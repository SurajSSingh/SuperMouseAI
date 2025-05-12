//! # Based on original Mutter project:
//! <https://github.com/sigaloid/mutter>
//!
//! Used under [MIT OR Apache-2.0 License](https://github.com/sigaloid/mutter/blob/main/Cargo.toml#L5C1-L6C1)
use std::time::Instant;

use crate::{
    transcript::{Transcript, Utterance},
    types::AudioProcessingOptions,
};
use log::{debug, error, trace, warn};
use nnnoiseless::{DenoiseState, RnnModel};
use rodio::{buffer::SamplesBuffer, source::UniformSourceIterator, Decoder, Source};
use std::io::Cursor;
use whisper_rs::{
    FullParams, SamplingStrategy, SegmentCallbackData, WhisperContext, WhisperContextParameters,
    WhisperError,
};

/// Model struct. Can be constructed with [`Model::new`] or [`Model::download`].
/// Contains the Whisper model and its context.
pub struct Model {
    context: WhisperContext,
}

impl Model {
    /// Creates a new model from a model path. Must be a path to a valid Whisper model,
    /// in GGML format, that is compatible with Whisper.cpp.
    /// # Arguments
    /// - `path`: Path to the model.
    /// # Errors
    /// - [`WhisperError`]
    pub fn new(path: &str) -> Result<Self, WhisperError> {
        trace!("Loading model using default parameters");
        let params: WhisperContextParameters = WhisperContextParameters::default();
        Self::new_with_params(path, params)
    }

    /// Creates a new model from a model path and parameters. Must be a path to a valid
    /// Whisper model, in GGML format, that is compatible with Whisper.cpp.
    /// # Arguments
    /// - `path`: Path to the model.
    /// - `path`: Context parameters.
    /// # Errors
    /// - [`WhisperError`]
    pub fn new_with_params(
        path: &str,
        params: WhisperContextParameters,
    ) -> Result<Self, WhisperError> {
        trace!("Loading model {path}");
        // Sanity check - make sure the path exists
        let path_converted = std::path::Path::new(path);
        if !path_converted.exists() {
            return Err(WhisperError::InitError);
        }

        Ok({
            Self {
                context: WhisperContext::new_with_params(path, params)?,
            }
        })
    }

    /// Transcribes audio to text, given the audio is a byte array of a file.
    /// Supported codecs: MP3 (Symphonia), WAV (Hound), OGG Vorbis (lewton),
    /// FLAC (claxon).
    ///
    /// # Arguments
    /// - `audio`: Audio to transcribe. An array of bytes.
    /// - `translate`: Whether to translate the text.
    /// - `word_timestamps`: Whether to output word timestamps.
    /// - `initial_prompt`: Optinal initial prompt to whisper model.
    /// - `language`: Optinal language setting for whisper model.
    /// - `threads`: Number of threads to use. `None` will use the number of cores from
    ///   the `num_cpus` crate.
    /// # Errors
    /// - [`ModelError`]
    /// # Returns
    /// [Transcript]    
    #[allow(
        clippy::too_many_arguments,
        reason = "Temporary allowed to prevent full refactoring into options struct"
    )]
    pub fn transcribe_audio(
        &self,
        audio: impl AsRef<[u8]>,
        translate: bool,
        word_timestamps: bool,
        initial_prompt: Option<&str>,
        language: Option<&str>,
        threads: Option<u16>,
        patience: Option<f32>,
        decode_options: AudioProcessingOptions,
        abort_callback: Option<impl FnMut() -> bool + 'static>,
        progress_callback: Option<impl FnMut(i32) + 'static>,
        new_segment_lossy_callback: Option<impl FnMut(SegmentCallbackData) + 'static>,
        new_segment_callback: Option<impl FnMut(SegmentCallbackData) + 'static>,
    ) -> Result<Transcript, ModelError> {
        trace!("Decoding audio.");
        let samples = if decode_options.denoise_audio.is_none_or(|is_true| is_true) {
            decode_and_denoise(audio.as_ref().to_vec(), decode_options)?
        } else {
            decode(audio.as_ref().to_vec())?
        };
        trace!("Transcribing audio.");
        self.transcribe_pcm_s16le(
            &samples,
            translate,
            word_timestamps,
            initial_prompt,
            language,
            threads,
            patience,
            abort_callback,
            progress_callback,
            new_segment_lossy_callback,
            new_segment_callback,
            None,
        )
    }

    /// Transcribes audio to text, given the audio is an [f32] float array of codec
    /// `pcm_s16le` and in single-channel format.
    ///
    /// You probably want to use [`Model::transcribe_audio`] instead, unless you've already
    /// converted it into the correct format.
    ///
    /// # Arguments
    /// - `audio`: Audio to transcribe. Must be a [f32] array.
    /// - `translate`: Whether to translate the text.
    /// - `word_timestamps`: Whether to output word timestamps.
    /// - `initial_prompt`: Optinal initial prompt to whisper model.
    /// - `language`: Optinal language setting for whisper model.
    /// - `threads`: Number of threads to use. `None` will use the number of cores from
    ///
    /// # Errors
    /// - [`ModelError`]
    /// # Panics
    /// This function shouldn't panic, but may due to the underlying -sys c bindings.
    /// # Returns
    /// [Transcript]
    #[allow(
        clippy::too_many_arguments,
        unused_variables,
        clippy::needless_pass_by_value,
        reason = "Temporary allowed to prevent full refactoring into options struct"
    )]
    pub fn transcribe_pcm_s16le(
        &self,
        audio: &[f32],
        translate: bool,
        word_timestamps: bool,
        initial_prompt: Option<&str>,
        language: Option<&str>,
        threads: Option<u16>,
        patience: Option<f32>,
        abort_callback: Option<impl FnMut() -> bool + 'static>,
        progress_callback: Option<impl FnMut(i32) + 'static>,
        new_segment_lossy_callback: Option<impl FnMut(SegmentCallbackData) + 'static>,
        new_segment_callback: Option<impl FnMut(SegmentCallbackData) + 'static>,
        beam_size: Option<i32>,
    ) -> Result<Transcript, ModelError> {
        debug!("Start transcribing audio");
        trace!(
            "Transcribing audio (len = {}) with translate: {translate} and timestamps: {word_timestamps}",
            audio.len()
        );

        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_possible_wrap,
            reason = "Number of cores on current CPUs should be well below 2^31"
        )]
        let cpu_count = num_cpus::get() as i32;

        let mut params = FullParams::new(SamplingStrategy::BeamSearch {
            beam_size: beam_size.map_or(5, |decoder| decoder.min(cpu_count)),
            patience: patience.map_or_else(|| 1.0, |p| p.min(0.0)),
        });

        if let Some(prompt) = initial_prompt {
            params.set_initial_prompt(prompt);
        }

        params.set_language(language);

        params.set_translate(translate);
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_token_timestamps(word_timestamps);
        params.set_split_on_word(true);
        params.set_single_segment(true);

        // TODO: Uncomment when I can figure out how to fix crashing bug
        // trace!("Adding Callbacks");
        // if let Some(closure) = abort_callback {
        //     params.set_abort_callback_safe(closure);
        // }
        // if let Some(closure) = progress_callback {
        //     params.set_progress_callback_safe(closure);
        // }
        // if let Some(closure) = new_segment_lossy_callback {
        //     params.set_segment_callback_safe_lossy(closure);
        // }
        // if let Some(closure) = new_segment_callback {
        //     params.set_segment_callback_safe(closure);
        // }

        trace!("Basic params for Whisper Set");

        #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
        let threads = threads.map_or_else(|| cpu_count, i32::from);

        trace!("Using {threads} threads");

        params.set_n_threads(threads);

        let st = Instant::now();
        let mut state = self.context.create_state().map_err(|e| {
            error!("Failed to create Whisper state");
            ModelError::WhisperError(e)
        })?;
        trace!("Transcribing audio with WhisperState");
        state.full(params, audio).map_err(|e| {
            error!("Failed to transcribe");
            ModelError::WhisperError(e)
        })?;

        let num_segments = state.full_n_segments().map_err(|e| {
            error!("Failed to get segments");
            ModelError::WhisperError(e)
        })?;
        trace!("Number of segments: {num_segments}");

        let mut words = Vec::new();
        let mut utterances = Vec::new();
        for segment_idx in 0..num_segments {
            let text = state
                .full_get_segment_text(segment_idx)
                .map_err(ModelError::WhisperError)?;
            let start = state
                .full_get_segment_t0(segment_idx)
                .map_err(ModelError::WhisperError)?;
            let stop = state
                .full_get_segment_t1(segment_idx)
                .map_err(ModelError::WhisperError)?;

            utterances.push(Utterance { start, stop, text });

            if !word_timestamps {
                trace!("Skipping word timestamps");
                continue;
            }

            trace!("Getting word timestamps for segment {segment_idx}");

            let num_tokens = state
                .full_n_tokens(segment_idx)
                .map_err(ModelError::WhisperError)?;

            for t in 0..num_tokens {
                let text = state
                    .full_get_token_text(segment_idx, t)
                    .map_err(ModelError::WhisperError)?;
                let token_data = state
                    .full_get_token_data(segment_idx, t)
                    .map_err(ModelError::WhisperError)?;

                if text.starts_with("[_") {
                    continue;
                }

                words.push(Utterance {
                    text,
                    start: token_data.t0,
                    stop: token_data.t1,
                });
            }
        }
        let processing_time = Instant::now().duration_since(st);
        debug!("Finished transcription");
        trace!("Processing Time = {processing_time:?}");
        Ok(Transcript {
            utterances,
            processing_time,
            word_utterances: if word_timestamps { Some(words) } else { None },
        })
    }
}
/// Crate error that contains an enum of all possible errors related to the model.
#[derive(Debug)]
pub enum ModelError {
    /// [`WhisperError`]. Error either loading model, or during transcription, in the
    /// actual whisper.cpp library
    WhisperError(WhisperError),
    // /// [`ureq::Error`]. Error downloading model.
    // DownloadError(Box<ureq::Error>),
    // /// [`std::io::Error`]. Error reading model.
    // IoError(std::io::Error),
    /// [`rodio::decoder::DecoderError`]. Error decoding audio.
    DecodingError(rodio::decoder::DecoderError),
}

/// Decode a byte array of audio into a float array
///
/// Adapted from <https://github.com/sigaloid/mutter/blob/main/src/transcode.rs>
pub fn decode(bytes: Vec<u8>) -> Result<Vec<f32>, ModelError> {
    debug!("Start Decoding");
    let input = Cursor::new(bytes);
    trace!("Created cursor of bytes");
    // We know it is m4a by Mime type in browser (and we set it to be that as well)
    let source = Decoder::new_wav(input).map_err(ModelError::DecodingError)?;
    trace!("Produced WAV source from bytes");
    let output_sample_rate = 16000;
    let channels = 1;
    // Resample to output sample rate and channels
    let resample = UniformSourceIterator::new(source, channels, output_sample_rate);
    // High and low pass filters to enhance the audio
    let pass_filter = resample.low_pass(3000).high_pass(200).convert_samples();
    trace!("Finished Resampling");
    let samples: Vec<i16> = pass_filter.collect::<Vec<i16>>();
    let mut output: Vec<f32> = vec![0.0f32; samples.len()];
    let result: Result<(), whisper_rs::WhisperError> =
        whisper_rs::convert_integer_to_float_audio(&samples, &mut output);
    debug!("Normalizing output");
    let max_amp = output
        .iter()
        .fold(f32::MIN, |current, &sample| current.max(sample.abs()));
    let norm_factor = if max_amp <= 0.0 {
        trace!("Max amplitude (={max_amp}) does not make sense, use 1.0 to leave unchanged");
        1.0
    } else {
        1.0 / max_amp
    };
    trace!("Normalizing value with factor={norm_factor}");
    output.iter_mut().for_each(|x| *x *= norm_factor);
    debug!("Decoding Finished");
    result.map(|()| output).map_err(ModelError::WhisperError)
}

/// Decode a byte array of audio into a float array
///
/// Adapted from <https://github.com/sigaloid/mutter/blob/main/src/transcode.rs>
pub fn decode_and_denoise(
    bytes: Vec<u8>,
    options: AudioProcessingOptions,
) -> Result<Vec<f32>, ModelError> {
    trace!("Options given for decode and denoise: {options:?}");
    debug!("Start Decoding");
    let input = Cursor::new(bytes);
    trace!("Created cursor of bytes");
    // We know it is m4a by Mime type in browser (and we set it to be that as well)
    let source = Decoder::new_wav(input).map_err(ModelError::DecodingError)?;
    trace!("Produced WAV source from bytes");
    let input_sample_rate = 48_000;
    let output_sample_rate = 16_000;
    let channels = 1;
    let input_wav_sample =
        UniformSourceIterator::<rodio::Decoder<std::io::Cursor<Vec<u8>>>, f32>::new(
            source,
            channels,
            input_sample_rate,
        )
        .convert_samples()
        .collect::<Vec<i16>>();
    let denoise_sample = if options.normalize_result.unwrap_or(false) {
        trace!("Run first normalization");
        // NOTE: This treat f32 like full range i16 rather than range between -1.0 and 1.0
        let max_amp = input_wav_sample
            .iter()
            .fold(i16::MIN, |current, &sample| current.max(sample.abs()));
        let norm_factor = if max_amp <= 0 {
            trace!("Max amplitude (={max_amp}) does not make sense, use 1.0 to leave unchanged");
            1.0
        } else {
            f32::from(i16::MAX) / f32::from(max_amp)
        };
        debug!("Normalizing value with factor={norm_factor}");
        let res: Vec<f32> = input_wav_sample
            .iter()
            .map(|&x| (f32::from(x) * norm_factor).clamp(f32::from(i16::MIN), f32::from(i16::MAX)))
            .collect();
        trace!("Finished Normalizing");
        res
    } else {
        trace!("Skip normalizing, using same as denoised ouput");
        input_wav_sample.iter().map(|&x| f32::from(x)).collect()
    };

    #[cfg(debug_assertions)]
    write_wav(
        "../input.wav",
        &denoise_sample
            .iter()
            .map(|&x| x / f32::from(i16::MAX))
            .collect::<Vec<_>>(),
        (Some(input_sample_rate), Some(32), Some(SampleFormat::Float)),
    );

    let denoised_output: Vec<f32> = if options.denoise_audio.unwrap_or(true) {
        denoise_input(&denoise_sample)
    } else {
        trace!("Skip denoising, just resize output");
        denoise_sample
            .into_iter()
            .map(|x| x / (f32::from(i16::MAX) + 1.0))
            .collect()
    };
    #[cfg(debug_assertions)]
    write_wav(
        "../denoised.wav",
        &denoised_output,
        // .iter()
        // .map(|&x| x / f32::from(i16::MAX))
        // .collect(),
        (Some(input_sample_rate), Some(32), Some(SampleFormat::Float)),
    );

    debug!("WAV resample data: sample_rate={output_sample_rate}, channels={channels}");
    // Resample to output sample rate and channels
    let resample = UniformSourceIterator::new(
        rodio::buffer::SamplesBuffer::new(channels, input_sample_rate, denoised_output),
        channels,
        output_sample_rate,
    );
    let pass_filter = resample
        .low_pass(options.low_pass_value.unwrap_or(3000))
        .high_pass(options.high_pass_value.unwrap_or(200))
        .convert_samples();
    trace!("Finished Resampling");
    let samples: Vec<i16> = pass_filter.collect::<Vec<i16>>();
    let mut decoded_output: Vec<f32> = vec![0.0f32; samples.len()];
    whisper_rs::convert_integer_to_float_audio(&samples, &mut decoded_output)
        .map_err(ModelError::WhisperError)?;
    debug!("Decoding Finished");
    #[cfg(debug_assertions)]
    write_wav("../decoded.wav", &decoded_output, (None, None, None));
    if options.normalize_result.unwrap_or(false) {
        normalize(&mut decoded_output);
    } else {
        trace!("Skip normalizing, using same as denoised ouput");
    }
    #[cfg(debug_assertions)]
    write_wav("../ouput.wav", &decoded_output, (None, None, None));
    Ok(decoded_output)
}

/// Denoise a float array (without decoding)
///
/// Adapted from <https://github.com/sigaloid/mutter/blob/main/src/transcode.rs>
pub fn directly_denoise(
    source: Vec<f32>,
    input_channels: u16,
    input_sample_rate: u32,
    options: AudioProcessingOptions,
) -> Result<Vec<f32>, ModelError> {
    trace!("Options given for denoise: {options:?}");
    debug!(
        "Input info: len={}, channels={input_channels}, rate={input_sample_rate}",
        source.len()
    );
    let output_sample_rate = 16_000;
    let channels = 1;
    let input_wav_sample = UniformSourceIterator::<_, f32>::new(
        SamplesBuffer::new(input_channels, input_sample_rate, source),
        channels,
        input_sample_rate,
    )
    .convert_samples()
    .collect::<Vec<i16>>();
    let denoise_sample = if options.normalize_result.unwrap_or(false) {
        trace!("Run first normalization");
        // NOTE: This treat f32 like full range i16 rather than range between -1.0 and 1.0
        let max_amp = input_wav_sample
            .iter()
            .fold(i16::MIN, |current, &sample| current.max(sample.abs()));
        let norm_factor = if max_amp <= 0 {
            trace!("Max amplitude (={max_amp}) does not make sense, use 1.0 to leave unchanged");
            1.0
        } else {
            f32::from(i16::MAX) / f32::from(max_amp)
        };
        debug!("Normalizing value with factor={norm_factor}");
        let res: Vec<f32> = input_wav_sample
            .iter()
            .map(|&x| (f32::from(x) * norm_factor).clamp(f32::from(i16::MIN), f32::from(i16::MAX)))
            .collect();
        trace!("Finished Normalizing");
        res
    } else {
        trace!("Skip normalizing, using same as denoised ouput");
        input_wav_sample.iter().map(|&x| f32::from(x)).collect()
    };

    #[cfg(debug_assertions)]
    write_wav(
        "../input.wav",
        &denoise_sample
            .iter()
            .map(|&x| x / f32::from(i16::MAX))
            .collect::<Vec<_>>(),
        (Some(input_sample_rate), Some(32), Some(SampleFormat::Float)),
    );

    let denoised_output: Vec<f32> = if options.denoise_audio.unwrap_or(true) {
        denoise_input(&denoise_sample)
    } else {
        trace!("Skip denoising, just resize output");
        denoise_sample
            .into_iter()
            .map(|x| x / (f32::from(i16::MAX) + 1.0))
            .collect()
    };
    #[cfg(debug_assertions)]
    write_wav(
        "../denoised.wav",
        &denoised_output,
        // .iter()
        // .map(|&x| x / f32::from(i16::MAX))
        // .collect(),
        (Some(input_sample_rate), Some(32), Some(SampleFormat::Float)),
    );

    debug!("WAV resample data: sample_rate={output_sample_rate}, channels={channels}");
    // Resample to output sample rate and channels
    let resample = UniformSourceIterator::new(
        rodio::buffer::SamplesBuffer::new(channels, input_sample_rate, denoised_output),
        channels,
        output_sample_rate,
    );
    let pass_filter = resample
        .low_pass(options.low_pass_value.unwrap_or(3000))
        .high_pass(options.high_pass_value.unwrap_or(200))
        .convert_samples();
    trace!("Finished Resampling");
    let samples: Vec<i16> = pass_filter.collect::<Vec<i16>>();
    let mut decoded_output: Vec<f32> = vec![0.0f32; samples.len()];
    whisper_rs::convert_integer_to_float_audio(&samples, &mut decoded_output)
        .map_err(ModelError::WhisperError)?;
    debug!("Decoding Finished");
    #[cfg(debug_assertions)]
    write_wav("../decoded.wav", &decoded_output, (None, None, None));
    if options.normalize_result.unwrap_or(false) {
        normalize(&mut decoded_output);
    } else {
        trace!("Skip normalizing, using same as denoised ouput");
    }
    #[cfg(debug_assertions)]
    write_wav("../ouput.wav", &decoded_output, (None, None, None));
    Ok(decoded_output)
}

fn normalize(decoded_output: &mut [f32]) {
    trace!("Run second normalization");
    let max_amp = decoded_output
        .iter()
        .fold(f32::MIN, |current, &sample| current.max(sample.abs()));
    let norm_factor = if max_amp <= 0.0 {
        trace!("Max amplitude (={max_amp}) does not make sense, use 1.0 to leave unchanged");
        1.0
    } else {
        1.0 / max_amp
    };
    trace!("Normalizing value with factor={norm_factor}");
    decoded_output.iter_mut().for_each(|s| *s *= norm_factor);
    trace!("Finished Normalizing");
}

fn denoise_input(denoise_sample: &[f32]) -> Vec<f32> {
    const FRAME_SIZE: usize = DenoiseState::FRAME_SIZE;
    debug!("Denoising input");
    let mut output = Vec::new();
    let mut out_buf = [0.0; FRAME_SIZE];
    // TODO: Be able to load from model
    // let bytes = include_bytes!("../rnn_models/sh.rnnn");
    // let model = RnnModel::from_static_bytes(bytes).expect("Corrupted model file");
    // let mut nn_denoiser = DenoiseState::from_model(model);
    let model = RnnModel::default();
    let mut nn_denoiser = DenoiseState::from_model(model);
    let mut first = true;
    for chunk in denoise_sample.chunks_exact(FRAME_SIZE) {
        nn_denoiser.process_frame(&mut out_buf[..], chunk);

        // For first, I add unprocessed chunk, to keep same size
        if first {
            output.extend_from_slice(chunk);
        } else {
            output.extend_from_slice(&out_buf[..]);
        }
        first = false;
    }
    trace!("Finished Denoising");
    output
        .into_iter()
        .map(|x| x / (f32::from(i16::MAX) + 1.0))
        .collect()
}

use audrey::hound::{SampleFormat, WavSpec, WavWriter};
fn write_wav<T: audrey::hound::Sample + Copy>(
    path: &str,
    data: &[T],
    info: (Option<u32>, Option<u16>, Option<SampleFormat>),
) {
    debug!("Saving audio data to file.");
    // let maybe_file = std::fs::OpenOptions::new()
    //     .write(true)
    //     .create(true)
    //     .open(path);
    // if let Ok(file) = maybe_file {
    let maybe_writer = WavWriter::create(
        path,
        WavSpec {
            channels: 1,
            sample_rate: info.0.unwrap_or(16000),
            bits_per_sample: info.1.unwrap_or(32),
            sample_format: info.2.unwrap_or(SampleFormat::Float),
        },
    );
    match maybe_writer {
        Ok(mut writer) => {
            if let Err(err) = data
                .iter()
                .try_for_each(|sample| writer.write_sample(*sample))
            {
                warn!("Could not write samples to file (error: {err:?})");
            } else if let Err(err) = writer.flush() {
                warn!("Could not flush samples to file (error: {err:?})");
            } else if let Err(err) = writer.finalize() {
                warn!("Could not finalize samples to file (error: {err:?})");
            } else {
                debug!("Finished writing to {path} file.");
            }
        }
        Err(err) => warn!("Could not open writer (error: {err:?}). SKIPPED!"),
    }
    // } else {
    //     warn!("Could not get file (error: {maybe_file:?}). SKIPPED!")
    // };
}
