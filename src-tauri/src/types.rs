//! Data types and associated functions for those types.

use crate::mutter::{decode, Model};
// use ct2rs::Whisper as CT2RSWhisper;
use futures_util::StreamExt;
use log::{debug, warn};
use mouce::common::MouseButton;
use rodio::buffer::SamplesBuffer;
// use rwhisper::{Whisper as RWhisper, WhisperSource};
use serde::{Deserialize, Serialize};
// use sherpa_rs::whisper::{WhisperConfig, WhisperRecognizer};
use specta::Type;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::Instant,
};
use whisper_rs::WhisperContextParameters;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
/// Which type of ASR model is used
pub enum ModelType {
    WhisperCPP,
    // CT2RS,
    // SherpaONNX,
    // RWhisper,
    Unknown,
}

#[derive(Default)]
#[non_exhaustive]
/// Which model is used
pub enum WhisperModel {
    #[default]
    None,
    Unloaded(ModelType, Box<Path>),
    WhisperCPP(Model, Box<Path>),
    // CT2RS(Box<CT2RSWhisper>, Box<Path>),
    // SherpaONNX(WhisperRecognizer, Box<Path>),
    // RWhisper(RWhisper, Box<Path>),
}

impl WhisperModel {
    #[allow(unused, reason = "Useful later on")]
    pub fn path(&self) -> &Path {
        match self {
            WhisperModel::None => Path::new("").into(),
            WhisperModel::Unloaded(_, path)
            // | WhisperModel::CT2RS(_, path)
            // | WhisperModel::SherpaONNX(_, path)
            // | WhisperModel::RWhisper(_, path) 
            | WhisperModel::WhisperCPP(_, path) => path.as_ref(),
        }
    }

    #[allow(unused, reason = "Useful later on")]
    pub fn new() -> Self {
        WhisperModel::None
    }

    pub fn switch_path(self, new_path: Box<Path>) -> Result<Self, String> {
        if !new_path.exists() {
            return Err("Model not found at path".to_string());
        }
        debug!("Found model @ {new_path:?}");
        Ok(match self {
            WhisperModel::Unloaded(model, _) => WhisperModel::Unloaded(model, new_path),
            WhisperModel::WhisperCPP(model, _) => WhisperModel::WhisperCPP(model, new_path),
            // WhisperModel::CT2RS(model, _) => WhisperModel::CT2RS(model, new_path),
            // WhisperModel::SherpaONNX(model, _) => WhisperModel::SherpaONNX(model, new_path),
            // WhisperModel::RWhisper(model, _) => WhisperModel::RWhisper(model, new_path),
            other => other,
        })
    }

    pub fn new_unloaded_model(kind: ModelType, path: Box<Path>) -> Result<Self, String> {
        //     let mut model_path_buf = app_handle
        //     .path()
        //     .app_local_data_dir()
        //     .map_err(|err| err.to_string())?;
        // model_path_buf.join("models");
        // model_path_buf.join(path);
        if !path.exists() {
            return Err(format!("Model not found at path: {path:?}"));
        }
        debug!("Found model @ {path:?}",);
        Ok(Self::Unloaded(kind, path))
    }

    pub fn get_model_type(&self) -> ModelType {
        match self {
            WhisperModel::WhisperCPP(_, _) => ModelType::WhisperCPP,
            // WhisperModel::CT2RS(_, _) => ModelType::CT2RS,
            // WhisperModel::SherpaONNX(_, _) => ModelType::SherpaONNX,
            // WhisperModel::RWhisper(_, _) => ModelType::RWhisper,
            _ => ModelType::Unknown,
        }
    }

    pub async fn load_model(self) -> Result<(Self, f64), String> {
        match self {
            WhisperModel::Unloaded(ModelType::Unknown, path) => {
                Err(format!("Cannot load unknown model type from {path:?}"))
            }
            WhisperModel::Unloaded(model_type, path) => {
                let start_time = Instant::now();
                Ok(match model_type {
                    ModelType::WhisperCPP => (
                        Self::WhisperCPP(
                            Model::new_with_params(
                                path.as_os_str().to_string_lossy().as_ref(),
                                WhisperContextParameters::default(),
                            )
                            .map_err(|err| err.to_string())?,
                            path,
                        ),
                        start_time.elapsed().as_secs_f64(),
                    ),
                    // ModelType::CT2RS => (
                    //     Self::CT2RS(
                    //         Box::new(
                    //             CT2RSWhisper::new(&path, Default::default())
                    //                 .map_err(|err| err.to_string())?,
                    //         ),
                    //         path,
                    //     ),
                    //     start_time.elapsed().as_secs_f64(),
                    // ),
                    // ModelType::SherpaONNX => (
                    //     Self::SherpaONNX(
                    //         WhisperRecognizer::new(WhisperConfig {
                    //             decoder: path
                    //                 .join("tiny-decoder.onnx")
                    //                 .to_str()
                    //                 .ok_or("Decoder not found".to_string())?
                    //                 .to_string(),
                    //             encoder: path
                    //                 .join("tiny-encoder.onnx")
                    //                 .to_str()
                    //                 .ok_or("Encoder not found".to_string())?
                    //                 .to_string(),
                    //             tokens: path
                    //                 .join("tiny-tokens.txt")
                    //                 .to_str()
                    //                 .ok_or("Tokens not found".to_string())?
                    //                 .to_string(),
                    //             language: "en".into(),
                    //             bpe_vocab: None,
                    //             num_threads: Some(1),
                    //             tail_paddings: None,
                    //             provider: None,
                    //             debug: false,
                    //         })
                    //         .map_err(|err| err.to_string())?,
                    //         path,
                    //     ),
                    //     start_time.elapsed().as_secs_f64(),
                    // ),
                    // ModelType::RWhisper => (
                    //     Self::RWhisper(
                    //         RWhisper::builder()
                    //             .with_cache(kalosm_common::Cache::new(path.clone().into_path_buf()))
                    //             .with_source(WhisperSource::QuantizedTinyEn)
                    //             .build()
                    //             .await
                    //             .map_err(|err| err.to_string())?,
                    //         path,
                    //     ),
                    //     start_time.elapsed().as_secs_f64(),
                    // ),
                    ModelType::Unknown => unreachable!("See above branch"),
                })
            }
            WhisperModel::None => Err("No model is given to load".to_string()),
            model => Ok((model, 0.0)),
        }
    }

    fn unload_and_get_path(self) -> Box<Path> {
        match self {
            WhisperModel::None => Path::new("").into(),
            WhisperModel::Unloaded(_, path)
            // | WhisperModel::CT2RS(_, path)
            // | WhisperModel::SherpaONNX(_, path)
            // | WhisperModel::RWhisper(_, path) 
            | WhisperModel::WhisperCPP(_, path)=> path,
        }
    }

    pub fn unload_model(self) -> Result<Self, String> {
        match self {
            model @ WhisperModel::WhisperCPP(_, _)
            // | model @ WhisperModel::CT2RS(_, _)
            // | model @ WhisperModel::SherpaONNX(_, _)
            // model @ WhisperModel::WhisperCPP(_, _)| model @ WhisperModel::RWhisper(_, _) 
            => Ok(Self::Unloaded(
                model.get_model_type(),
                model.unload_and_get_path(),
            )),
            WhisperModel::None => Err("No model is given to unload".to_string()),
            WhisperModel::Unloaded(_, path) => Err(format!(
                "No model is loaded from {path:?}, load a model first"
            )),
            // _ => Err("An unknown model is given".to_string()),
        }
    }

    pub async fn default_transcribe(
        &mut self,
        audio_data: Vec<u8>,
    ) -> Result<(String, f64), String> {
        let sep = " ++ ";
        match self {
            WhisperModel::None => Err("No model given to run transcription.".to_string()),
            WhisperModel::Unloaded(model_type, path) => {
                Err(format!("Model {model_type:?} not yet loaded from {path:?}"))
            }
            WhisperModel::WhisperCPP(model, _) => {
                let transcript = model
                    .transcribe_audio(
                        audio_data,
                        false,
                        false,
                        None,
                        Some("en"),
                        Some(1),
                        Some(1.0),
                        None::<fn() -> bool>,
                        None::<fn(i32)>,
                        None::<fn(whisper_rs::SegmentCallbackData)>,
                        None::<fn(whisper_rs::SegmentCallbackData)>,
                    )
                    .map_err(|err| err.to_string())?;
                debug!(
                    "Additional Segment Info: WordUtterances={:?}, Utterances={:?}",
                    transcript.word_utterances, transcript.utterances
                );
                Ok((
                    transcript.utterances.into_iter().map(|utt| utt.text).fold(
                        String::new(),
                        |mut text, utterance| {
                            text.push_str(&utterance);
                            text.push_str(sep);
                            text
                        },
                    ),
                    transcript.processing_time.as_secs_f64(),
                ))
            } // WhisperModel::CT2RS(whisper, _) => {
              //     let samples = decode(audio_data).map_err(|err| err.to_string())?;
              //     let start_time = std::time::Instant::now();
              //     let full_text = whisper
              //         .generate(&samples, Some("en"), false, &Default::default())
              //         .map_err(|err| err.to_string())?
              //         .join(sep);
              //     Ok((full_text, start_time.elapsed().as_secs_f64()))
              // }
              // WhisperModel::SherpaONNX(whisper_recognizer, _) => {
              //     let decoded = decode(audio_data).map_err(|err| err.to_string())?;
              //     let samples = decoded[..].chunks(16000 * 20).collect::<Vec<_>>();
              //     let start_time = std::time::Instant::now();
              //     let full_text = samples
              //         .into_iter()
              //         .map(|sample| whisper_recognizer.transcribe(16000, &sample))
              //         .fold(String::new(), |mut text, segment| {
              //             debug!(
              //                 "Additional Segment Info: Lang={}, Timestamps={:?}, Tokens={:?}",
              //                 segment.lang, segment.timestamps, segment.tokens
              //             );
              //             text.push_str(&segment.text);
              //             text.push_str(sep);
              //             text
              //         });
              //     Ok((full_text, start_time.elapsed().as_secs_f64()))
              // }
              // WhisperModel::RWhisper(whisper, _) => {
              //     let decoded = decode(audio_data).map_err(|err| err.to_string())?;
              //     let samples = SamplesBuffer::new(1, 16000, decoded);
              //     let start_time = std::time::Instant::now();
              //     let mut transcribe = whisper.transcribe(samples);
              //     let mut full_text = String::new();
              //     let mut elapsed = 0.0;
              //     while let Some(segment) = transcribe.next().await {
              //         elapsed += segment.elapsed_time().as_secs_f64();
              //         full_text.push_str(segment.text());
              //         full_text.push_str(sep);
              //         debug!(
              //             "Additional Segment Info: Confidence={}, Duration={:?}, NS_Prob={}, Progress={}, Remaining={:?}, Range={:?}, Start={}",
              //             segment.confidence(),
              //             segment.duration(),
              //             segment.probability_of_no_speech(),
              //             segment.progress(),
              //             segment.remaining_time(),
              //             segment.sample_range(),
              //             segment.start()
              //         )
              //     }
              //     debug!(
              //         "Finished RWhisper with {}",
              //         start_time.elapsed().as_secs_f64()
              //     );
              //     Ok((full_text, elapsed))
              // }
        }
    }
}

impl std::fmt::Display for WhisperModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WhisperModel::None => f.write_str("No Model"),
            WhisperModel::Unloaded(model_type, path) => f.write_fmt(format_args!(
                "Unloaded {:?} at `{}`",
                model_type,
                path.display()
            )),
            WhisperModel::WhisperCPP(_, path) => {
                f.write_fmt(format_args!("WhisperCPP model at `{}`", path.display()))
            } // WhisperModel::CT2RS(_, path) => {
              //     f.write_fmt(format_args!("CTranslate2 model at `{}`", path.display()))
              // }
              // WhisperModel::SherpaONNX(_, path) => {
              //     f.write_fmt(format_args!("Sherpa-Onnx model at `{}`", path.display()))
              // }
              // WhisperModel::RWhisper(_, path) => {
              //     f.write_fmt(format_args!("Candle model at `{}`", path.display()))
              // }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
pub struct ModelTranscribeData {
    pub model_name: String,
    pub text: String,
    pub memory_usage: f64,
    // TODO: Figure out how to make GPU send and sync
    // pub vram_usage: f64,
    pub loading_sec: f64,
    pub processing_sec: f64,
}

/// "Global" state for the application.
///
/// This holds all data that is expected to
/// persist throughout the app's runtime.
pub struct InnerAppState {
    // pub(crate) model_wcpp: ModelHolder,
    // pub(crate) model_ctrs: ct2rs::Whisper,
    pub(crate) default_path: PathBuf,
    pub(crate) model: Option<WhisperModel>,
    pub(crate) sound_map: HashMap<String, PathBuf>,
}

impl InnerAppState {
    pub fn new(default_path: PathBuf, sound_map: HashMap<String, PathBuf>) -> Self {
        // Load model into memory by evaluating short silence
        // FIXME: Need to do this in another thread, otherwise UI freezes
        // let _ = model.transcribe_pcm_s16le(&[0.0; 20_000], false, false, None, None, None);
        let model = Some(WhisperModel::Unloaded(
            ModelType::WhisperCPP,
            default_path.clone().as_path().into(),
        ));
        Self {
            default_path,
            sound_map,
            model,
        }
    }

    pub fn give_default_model<P: AsRef<Path>>(default_path: P) -> WhisperModel {
        WhisperModel::Unloaded(ModelType::WhisperCPP, default_path.as_ref().into())
    }

    /// Get sound by the provided name or by prepending `default_` to the beginning.
    pub fn get_sound_path(&self, sound_name: &str) -> Option<&PathBuf> {
        debug!("Getting sound: {sound_name}");
        self.sound_map.get(sound_name).or_else(|| {
            warn!("No sound with name '{sound_name}', falling back to 'default_{sound_name}'");
            self.sound_map.get(&format!("default_{}", &sound_name))
        })
    }

    /// Replace the custom model being used
    pub fn replace_custom_model(&mut self, path: String, use_gpu: bool) -> Result<(), String> {
        // let mut params = WhisperContextParameters::new();
        // params.use_gpu(use_gpu);
        // let new_model = Model::new_with_params(&path, params)?;
        // // _ will drop old model
        // let _ = self.model_wcpp.custom.replace((new_model, path));
        let model = self
            .model
            .take()
            .unwrap_or(Self::give_default_model(&self.default_path));
        self.model = Some(model.switch_path(Path::new(&path).into())?);
        Ok(())
    }

    /// Removes a custom model
    pub fn remove_custom_model(&mut self) {
        // _ will drop old model
        let _ = self.model.take();
        self.model = Some(Self::give_default_model(&self.default_path))
    }

    pub async fn get_model(&mut self) -> Result<(&mut WhisperModel, f64), String> {
        let model = self
            .model
            .take()
            .unwrap_or(Self::give_default_model(&self.default_path));
        let (loaded_model, loading_time) = model.load_model().await?;
        self.model = Some(loaded_model);
        // SAFETY: I have literally assigned model above, and unless `as_mut` causes it to be None (which I believe it cannot),
        //         this cannot be None
        Ok((
            unsafe { self.model.as_mut().unwrap_unchecked() },
            loading_time,
        ))
    }

    // pub fn get_model_ctrs(&self) -> &ct2rs::Whisper {
    //     &self.model_ctrs
    // }

    pub fn get_model_info(&self) -> String {
        self.model
            .as_ref()
            .map(|m| m.to_string())
            .unwrap_or("Default (Model)".to_string())
    }

    pub async fn get_and_load_model_from<P: AsRef<Path>>(
        &mut self,
        model_type: ModelType,
        path: P,
    ) -> Result<(&mut WhisperModel, f64), String> {
        let _ = self.model.take();
        let new_model = WhisperModel::new_unloaded_model(model_type, path.as_ref().into())?;
        self.model = Some(new_model);
        self.get_model().await
    }

    pub fn unload_model(&mut self) -> Result<(), String> {
        let model = self.model.take();
        match model {
            Some(model) => {
                let model = model.unload_model()?;
                self.model = Some(model);
            }
            None => {
                self.model = Some(Self::give_default_model(&self.default_path));
            }
        }
        Ok(())
    }
}

pub type AppState = tauri::async_runtime::Mutex<InnerAppState>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
/// Enum representing mouse button type
///
/// Main three are left, middle, and right
#[non_exhaustive]
pub enum MouseButtonType {
    Left,
    Middle,
    Right,
}

impl From<&MouseButton> for MouseButtonType {
    fn from(mb: &MouseButton) -> Self {
        match mb {
            MouseButton::Left => Self::Left,
            MouseButton::Middle => Self::Middle,
            MouseButton::Right => Self::Right,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Type)]
/// Information about modifier key event (pressed or released)
pub struct ModKeyPayload {
    key: String,
    is_pressed: bool,
}

impl ModKeyPayload {
    /// New Modifier Key Event
    pub const fn new(key: String, is_pressed: bool) -> Self {
        Self { key, is_pressed }
    }

    /// New pressed event for given key
    pub const fn pressed(key: String) -> Self {
        Self::new(key, true)
    }

    /// New release event for given key
    pub const fn released(key: String) -> Self {
        Self::new(key, false)
    }
}

/// Given a key, check if it matches one of the (specific) modifier keys.
///
/// The main modifiers are: Alt, Control, Meta, Option, and Shift (both left and right).
pub const fn is_modkey(key: device_query::Keycode) -> bool {
    use device_query::Keycode as K;
    matches!(
        key,
        K::Command
            | K::LAlt
            | K::LControl
            | K::LMeta
            | K::LOption
            | K::LShift
            | K::RAlt
            | K::RControl
            | K::RMeta
            | K::ROption
            | K::RShift
    )
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Type)]
#[non_exhaustive]
/// Format type for a transcription
pub enum TranscriptionFormat {
    #[default]
    Text,
    #[allow(clippy::upper_case_acronyms, reason = "Proper name of format type")]
    SRT,
    #[allow(clippy::upper_case_acronyms, reason = "Proper name of format type")]
    VTT,
}

impl TranscriptionFormat {
    /// Convert a given transcript to its string form based on the current format type.
    pub fn convert_transcript(self, transcript: &crate::transcript::Transcript) -> String {
        match self {
            Self::Text => transcript.as_text(),
            Self::SRT => transcript.as_srt(),
            Self::VTT => transcript.as_vtt(),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize, Type)]
/// Options for the transcribing function.
///
/// All items are optional. Based on arguments for [`crate::mutter::Model::transcribe_audio`].
pub struct TranscribeOptions {
    pub translate: Option<bool>,
    pub individual_word_timestamps: Option<bool>,
    pub threads: Option<u16>,
    pub initial_prompt: Option<String>,
    pub language: Option<String>,
    pub format: Option<TranscriptionFormat>,
    pub patience: Option<f32>,
    pub include_callback: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
#[non_exhaustive]
/// Ways a text can be decorated
pub enum TextDecoration {
    Bold,
    Italics,
    Underline,
    Strikethrough,
    Mark,
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize, Type)]
/// Options for the text post-processing function.
///
/// All items are optional.
pub struct TextProcessOptions {
    /// Words that will be removed (or striken) from the string
    pub removed_words: Option<Vec<String>>,
    /// Words to modify in someway that does not change meaning of word,
    /// but adds some decoration
    pub decorated_words: Option<Vec<(TextDecoration, String)>>,
    pub replace_inter_sentence_newlines: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Type)]
/// Basic information about the current system
pub struct SystemInfo {
    /// Total number of cores on this system
    pub cpu_core_count: f64,
    /// Total amount of system memory (RAM) in the system in GB
    pub total_memory_gb: f64,
    /// Total amount of graphic memory (VRAM) in the system in GB
    pub total_vram_gb: f64,
}
