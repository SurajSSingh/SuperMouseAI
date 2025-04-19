//! All things related to Rust of the Super Mouse AI app

#![allow(clippy::used_underscore_binding)]
// When Specta builds the type bindings, it uses it's own Result,
// which causes a lint warning. Thus, the aboe lint is disabled
// for this file

// Crate level use (imports)
use crate::{
    events::{
        new_lossy_transcript_segment_event, new_transcript_segment_event, MouseClickEvent,
        MouseMoveEvent, TranscriptionProgressEvent,
    },
    mutter::ModelError,
    types::{AppState, MouseButtonType, SystemInfo, TextProcessOptions, TranscribeOptions},
};
use enigo::{Enigo, Keyboard, Settings};
use log::{debug, error, info, trace};
use mouce::{common::MouseEvent, Mouse, MouseActions};
use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use tauri::{AppHandle, State, Wry};
use tauri_specta::{collect_commands, Commands, Event};

#[tauri::command]
#[specta::specta]
/// Take WAV audio data and transcribe it with application Whisper model.
///
/// Check [crate::mutter::Model::transcribe_audio] for details on arguments
pub async fn transcribe(
    app_state: State<'_, AppState>,
    app_handle: AppHandle,
    audio_data: Vec<u8>,
    options: Option<TranscribeOptions>,
) -> Result<(String, f64), String> {
    let options = options.unwrap_or_default();
    log::info!("Transcribing with parameters: translate={:?}, use_timestamp={:?}, threads={:?}, prompt={:?}, lang={:?}, fmt={:?}, patience={:?}",
        options.translate,
        options.individual_word_timestamps,
        options.threads,
        options.initial_prompt,
        options.language,
        options.format,
        options.patience,
    );
    info!("Running transcription command");
    let mut app_state = app_state.lock().await;
    info!("Transcribe using {}", app_state.get_model_info());
    let (model, _loading_time) = app_state.get_model().await?;
    trace!("Creating abort transcription callback");
    let _: Option<fn() -> bool> = if options.include_callback.is_some_and(|is_true| is_true) {
        // TODO: Figure out how to send off via an event from JS side
        Some(|| {
            trace!("Evaluating abort transcription => false");
            false
        })
    } else {
        None
    };
    let _ = if options.include_callback.is_some_and(|is_true| is_true) {
        trace!("Creating transcript progress callback");
        let handle = app_handle.clone();
        trace!("Cloned app handle");
        Some(move |precentage| {
            trace!("Creating transcription progress event");
            let event = TranscriptionProgressEvent::with_payload(precentage);
            trace!("Emitting transcription progress event");
            let _ = event
                .emit(&handle)
                .map_err(|err| error!("Transcription Progress event error: {err}"));
        })
    } else {
        None
    };
    let _ = if options.include_callback.is_some_and(|is_true| is_true) {
        let handle = app_handle.clone();
        Some(move |segment: whisper_rs::SegmentCallbackData| {
            let _ = new_lossy_transcript_segment_event(segment)
                .emit(&handle)
                .map_err(|err| error!("Transcription Segment event error: {err}"));
        })
    } else {
        None
    };
    let _ = if options.include_callback.is_some_and(|is_true| is_true) {
        #[allow(
            clippy::redundant_clone,
            reason = "May want to use app handle later on"
        )]
        let handle = app_handle.clone();
        Some(move |segment: whisper_rs::SegmentCallbackData| {
            let _ = new_transcript_segment_event(segment)
                .emit(&handle)
                .map_err(|err| error!("Transcription Segment event error: {err}"));
        })
    } else {
        None
    };
    Ok(model.default_transcribe(audio_data).await?)
    // let transcription = model
    //     .transcribe_audio(
    //         &audio_data,
    //         options.translate.unwrap_or(false),
    //         options.individual_word_timestamps.unwrap_or(false),
    //         options.initial_prompt.as_deref(),
    //         options.language.as_deref(),
    //         // Make sure not to pass 0 for CPU thread,
    //         // otherwise model crashes
    //         match options.threads {
    //             Some(0) => None,
    //             threads => threads,
    //         },
    //         options.patience,
    //         abort_callback,
    //         progress_callback,
    //         lossy_segment_callback,
    //         not_lossy_segment_callback,
    //     )
    //     .map_err(|err| {
    //         log::error!("Transcription Error: {err:?}");
    //         match err {
    //             ModelError::WhisperError(whisper_error) => whisper_error.to_string(),
    //             ModelError::DecodingError(decoder_error) => decoder_error.to_string(),
    //         }
    //     })?;
    // Ok((
    //     options
    //         .format
    //         .unwrap_or_default()
    //         .convert_transcript(&transcription),
    //     transcription.processing_time.as_secs_f64(),
    // ))
}

#[tauri::command]
#[specta::specta]
/// Process the text
pub async fn process_text(
    text: String,
    options: Option<TextProcessOptions>,
) -> Result<(String, f64), String> {
    info!("Running processing text command");
    let mut updated_text = text;
    let options = options.unwrap_or_default();
    log::info!("Processing text with parameters: replace_inter_sentence_newlines={:?}, removed_words={:?}, decorated_words={:?}", 
        options.removed_words,
        options.decorated_words,
        options.replace_inter_sentence_newlines,
    );
    options
        .removed_words
        .unwrap_or_default()
        .iter()
        .for_each(|word| {
            updated_text = updated_text.replace(word, "");
        });
    if options.replace_inter_sentence_newlines.unwrap_or(true) {
        let regex = regex::Regex::new(r"(\w)[ \t]*\n").map_err(|e| {
            log::error!("Regex error: {e}");
            e.to_string()
        })?;
        updated_text = regex.replace_all(&updated_text, "$1 ").to_string();
    }
    // TODO: Get actual processing time
    Ok((updated_text.trim().to_string(), 0.0))
}

#[tauri::command]
#[specta::specta]
/// Run [transcribe] function then pass to [process_text] for post processing.
pub async fn transcribe_with_post_process(
    app_state: State<'_, AppState>,
    app_handle: AppHandle,
    audio_data: Vec<u8>,
    transcribe_options: Option<TranscribeOptions>,
    processing_options: Option<TextProcessOptions>,
) -> Result<(String, f64), String> {
    info!("Running transcription & processing command");
    let (text, transcription_time) =
        transcribe(app_state, app_handle, audio_data, transcribe_options).await?;
    let (new_text, processing_time) = process_text(text, processing_options).await?;
    Ok((new_text, transcription_time + processing_time))
}

#[tauri::command]
#[specta::specta]
/// Play the provided sound given its name that is stored in the app_state
pub async fn play_sound(app_state: State<'_, AppState>, sound_name: String) -> Result<(), String> {
    info!("Running play sound command");
    // Get sound source
    let source = app_state
        .lock()
        .await
        .get_sound_path(&sound_name)
        .ok_or_else(|| format!("Could not find sound with name: {sound_name}"))
        .and_then(|path| File::open(path).map_err(|err| err.to_string()))
        .map(BufReader::new)
        .and_then(|file| Decoder::new(file).map_err(|err| err.to_string()))?;

    // Create sound player
    let (_stream, sound_handle) = OutputStream::try_default().map_err(|err| err.to_string())?;
    let sink = Sink::try_new(&sound_handle).map_err(|err| err.to_string())?;

    // Play sound on new thread to prevent app pauses
    sink.append(source);
    sink.sleep_until_end();
    sink.detach();
    Ok(())
}

/// Function to listen for any clicks from mouse and emits Tauri event.
///
/// Returns either the callback id or an error message.
///
/// Adapted from <https://github.com/crabnebula-dev/koi-pond/blob/main/src-tauri/src/lib.rs> under MIT License
pub fn listen_for_mouse_click(app_handle: AppHandle) -> Result<u8, String> {
    Mouse::new()
        .hook(Box::new(move |e| match e {
            MouseEvent::Press(button) => MouseClickEvent::with_payload(MouseButtonType::from(button))
                .emit(&app_handle)
                .map_err(|e| {
                    error!("App Handle expected to emit press event with button playload but could not: {e}");
                })
                .unwrap_or_default(),
            MouseEvent::Release(_button) => { /* Do Nothing Yet */ }
            MouseEvent::AbsoluteMove(x, y) =>  MouseMoveEvent::with_payload(*x, *y).emit(&app_handle).map_err(|e| {
                error!("App Handle expected to emit mouse move event but could not: {e}");
            })
            .unwrap_or_default(),
            _ => (),
        }))
        .map_err(|err| err.to_string())
}

#[tauri::command]
#[specta::specta]
/// Paste text from clipboard
pub async fn write_text(text: String) -> Result<(), String> {
    info!("Running auto-write text command");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    trace!("Enigo setup: {enigo:?}");
    enigo.text(&text).map_err(|e| e.to_string())?;
    // Use len rather then actual text to prevent leaking info in logs
    trace!("Enigo Wrote {} bytes", text.len());
    Ok(())
}

#[tauri::command]
#[specta::specta]
/// Paste text from clipboard
pub fn paste_text() -> Result<(), String> {
    info!("Running paste from clipboard command");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    trace!("Enigo setup: {enigo:?}");
    let cmd_or_ctrl = match std::env::consts::OS {
        "macos" => enigo::Key::Meta,
        "windows" | "linux" => enigo::Key::Control,
        _ => {
            error!("Pasting from an unsupported/unknown target");
            return Err("Unsupported action for your machine!".to_string());
        }
    };
    enigo
        .key(cmd_or_ctrl, enigo::Direction::Press)
        .map_err(|e| {
            error!("Input error: {e}");
            e.to_string()
        })?;
    enigo
        .key(enigo::Key::Unicode('v'), enigo::Direction::Click)
        .map_err(|e| {
            error!("Input error: {e}");
            e.to_string()
        })?;
    enigo
        .key(cmd_or_ctrl, enigo::Direction::Release)
        .map_err(|e| {
            error!("Input error: {e}");
            e.to_string()
        })?;
    trace!("Enigo Pasted text");
    Ok(())
}

#[tauri::command]
#[specta::specta]
/// Put window on top, can be overriden by optional parameter
pub async fn set_window_top(
    webview_window: tauri::WebviewWindow,
    override_value: Option<bool>,
) -> Result<(), String> {
    info!("Running set window float command");
    webview_window
        .set_always_on_top(override_value.unwrap_or(true))
        .map_err(|err| {
            log::error!("Could not set window to top value: {err}");
            err.to_string()
        })
}

#[tauri::command]
#[specta::specta]
/// Update the custom model information
pub async fn update_model(
    app_state: State<'_, AppState>,
    path: Option<String>,
    use_gpu: Option<bool>,
) -> Result<(), String> {
    info!("Updating model to use");
    let mut app_state = app_state.lock().await;
    if let Some(path) = path {
        info!("Replacing Custom Model");
        app_state
            .replace_custom_model(
                path,
                use_gpu.unwrap_or(cfg!(any(feature = "vulkan", target_os = "macos"))),
            )
            .map_err(|err| err.to_string())?;
    } else {
        info!("Removing Custom Model");
        app_state.remove_custom_model();
    }
    // Explict drop to unlock the Mutex (mostly to fix lint issue)
    drop(app_state);
    Ok(())
}

#[tauri::command]
#[specta::specta]
/// Get information about the user's system.
pub async fn get_system_info() -> SystemInfo {
    // CPU
    let cpu_info = CpuRefreshKind::nothing(); //.with_frequency();
    let mem_info = MemoryRefreshKind::nothing().with_ram();
    let sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(cpu_info)
            .with_memory(mem_info)
            .without_processes(),
    );

    #[allow(
        clippy::cast_precision_loss,
        reason = "Total CPU core is unlikely to exceed 2^52 for consumer systems"
    )]
    let cpu_core_count = sys.cpus().len() as f64;
    #[allow(
        clippy::cast_precision_loss,
        reason = "Total Memory in bytes is unlikely to exceed 2^52 for consumer systems"
    )]
    let total_memory_gb = (sys.total_memory() as f64) / 1_000_000_000_f64;
    // sys.cpus().first().unwrap().frequency();

    // GPU
    debug!("Get GPU Info");
    let total_vram_gb = match gfxinfo::active_gpu() {
        Ok(gpu) => {
            debug!(
                "Active GPU: ID={}; (Vendor, Model, Family)=({},{},{})",
                gpu.device_id(),
                gpu.vendor(),
                gpu.model(),
                gpu.family()
            );
            match (gpu.info().total_vram(), gpu.model()) {
                (0, maybe_apple) if maybe_apple.starts_with("Apple") => {
                    debug!("Active GPU ");
                    debug!("Got Apple GPU using Unified Memory for GPU, use RAM value");
                    total_memory_gb
                }
                #[allow(
                    clippy::cast_precision_loss,
                    reason = "Total VRAM in bytes is unlikely to exceed 2^52 for consumer systems"
                )]
                (bytes, _) => (bytes as f64) / 1_000_000_000_f64,
            }
        }
        Err(err) => {
            error!("Could not get GPU info because: {err}");
            debug!("Give NaN to signify error, but continue with CPU information");
            f64::NAN
        }
    };
    SystemInfo {
        cpu_core_count,
        total_memory_gb,
        total_vram_gb,
    }
}

// #[tauri::command]
// #[specta::specta]
// pub async fn transcribe_with_ct2rs(
//     app_state: State<'_, AppState>,
//     // app_handle: AppHandle,
//     audio_data: Vec<u8>,
// ) -> Result<(String, f64), String> {
//     use ct2rs::Whisper;
//     use tauri::Manager;
//     let app_state = app_state.lock().map_err(|err| err.to_string())?;
//     let whisper = app_state.get_model_ctrs();
//     let samples = crate::mutter::decode(audio_data).map_err(|err| err.to_string())?;
//     let st = std::time::Instant::now();
//     let res = whisper
//         .generate(&samples, Some("en"), false, &Default::default())
//         .map_err(|err| err.to_string())?;
//     Ok((
//         res.join(" |:==:| "),
//         std::time::Instant::now().duration_since(st).as_secs_f64(),
//     ))
// }

// #[tauri::command]
// #[specta::specta]
// pub async fn transcribe_with_sherpa(
//     // app_state: State<'_, AppState>,
//     app_handle: AppHandle,
//     audio_data: Vec<u8>,
// ) -> Result<(String, f64), String> {
//     use sherpa_rs::whisper::{WhisperConfig, WhisperRecognizer};
//     use tauri::Manager;
//     let mut model_path_buf = app_handle
//         .path()
//         .app_local_data_dir()
//         .map_err(|err| err.to_string())?;
//     model_path_buf.push("sherpa-onnx-whisper-tiny");
//     debug!("Found dir: {}", model_path_buf.exists());
//     if !model_path_buf.exists() {
//         return Err("Model not found at path".to_string().into());
//     }
//     let config = WhisperConfig {
//         decoder: model_path_buf
//             .clone()
//             .join("tiny-decoder.onnx")
//             .to_str()
//             .ok_or("Decoder not found".to_string())?
//             .to_string(),
//         encoder: model_path_buf
//             .clone()
//             .join("tiny-encoder.onnx")
//             .to_str()
//             .ok_or("Encoder not found".to_string())?
//             .to_string(),
//         tokens: model_path_buf
//             .join("tiny-tokens.txt")
//             .to_str()
//             .ok_or("Tokens not found".to_string())?
//             .to_string(),
//         language: "en".into(),
//         bpe_vocab: None,
//         num_threads: Some(1),
//         tail_paddings: None,
//         provider: None,
//         debug: false,
//     };
//     let mut whisper = WhisperRecognizer::new(config).map_err(|err| err.to_string())?;
//     let decoded = crate::mutter::decode(audio_data).map_err(|err| err.to_string())?;
//     let samples = decoded[..].chunks(16000 * 10).collect::<Vec<_>>();
//     let st = std::time::Instant::now();
//     let mut full_text = String::new();
//     for sample in samples {
//         let res = whisper.transcribe(16000, &sample);
//         debug!("{:?}", res.timestamps);
//         debug!("{:?}", res.tokens);
//         full_text.push_str(&res.text);
//     }
//     Ok((full_text, st.elapsed().as_secs_f64()))
// }

#[tauri::command]
#[specta::specta]
pub async fn transcribe_with_kalosm(
    // app_state: State<'_, AppState>,
    app_handle: AppHandle,
    audio_data: Vec<u8>,
) -> Result<(String, f64), String> {
    use kalosm::sound::*;
    use tauri::Manager;
    let mut model_path_buf = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|err| err.to_string())?;
    model_path_buf.push("kalosm");
    debug!("Found dir: {}", model_path_buf.exists());
    if !model_path_buf.exists() {
        return Err("Model not found at path".to_string().into());
    }
    let whisper = Whisper::builder()
        .with_cache(kalosm_common::Cache::new(model_path_buf))
        .with_source(WhisperSource::QuantizedTinyEn)
        .build()
        .await
        .map_err(|err| err.to_string())?;
    let decoded = crate::mutter::decode(audio_data).map_err(|err| err.to_string())?;
    let samples = kalosm::sound::rodio::buffer::SamplesBuffer::new(1, 16000, decoded);
    let st = std::time::Instant::now();
    let mut transcribe = whisper.transcribe(samples);
    let mut full_text = String::new();
    let mut elapsed = 0.0;
    while let Some(segment) = transcribe.next().await {
        elapsed += segment.elapsed_time().as_secs_f64();
        full_text.push_str(segment.text());
        debug!(
            "Additional Segment Info: Confidence={}, Duration={:?}, NS_Prob={}, Progress={}, Remaining={:?}, Range={:?}, Start={}",
            segment.confidence(),
            segment.duration(),
            segment.probability_of_no_speech(),
            segment.progress(),
            segment.remaining_time(),
            segment.sample_range(),
            segment.start()
        )
    }
    debug!("Finished Kalosm with {}", st.elapsed().as_secs_f64());
    Ok((full_text, elapsed))
}

#[tauri::command]
#[specta::specta]
pub async fn transcribe_whisper_run_each(
    app_state: State<'_, AppState>,
    app_handle: AppHandle,
    audio_data: Vec<u8>,
) -> Result<[(String, f64, f64); 4], String> {
    debug!("Running multiple models to test speed");
    let model_dir_path_buf = tauri::Manager::path(&app_handle)
        .app_local_data_dir()
        .map_err(|err| err.to_string())?;
    debug!("Top level model path dir: {model_dir_path_buf:?}");
    let mut app_state = app_state.lock().await;
    debug!(
        "Main Whisper model path dir: {:?}",
        model_dir_path_buf.join("models")
    );
    let mut result = [
        (String::new(), 0.0, 0.0),
        (String::new(), 0.0, 0.0),
        (String::new(), 0.0, 0.0),
        (String::new(), 0.0, 0.0),
    ];
    // Whisper.cpp
    {
        debug!("Load Whisper-rs model");
        let (model, loading) = app_state
            .get_and_load_model_from(
                crate::types::ModelType::WhisperCPP,
                model_dir_path_buf.join("whisper-cpp-model.bin"),
            )
            .await?;
        debug!("Loading Time for Whisper.cpp: {loading}");
        let (text, processing) = model.default_transcribe(audio_data.clone()).await?;
        result[0] = (text, loading, processing);
        app_state.unload_model()?;
    }
    // CTranslate2
    // {
    //     debug!("Load CT2-rs model");
    //     let (model, loading) = app_state
    //         .get_and_load_model_from(
    //             crate::types::ModelType::CT2RS,
    //             model_dir_path_buf.join("ct2rs"),
    //         )
    //         .await?;
    //     debug!("Loading Time for CT2RS: {loading}");
    //     let (text, processing) = model.default_transcribe(audio_data.clone()).await?;
    //     result[1] = (text, loading, processing);
    //     app_state.unload_model()?;
    // }
    // Sherpa-ONNX
    // {
    //     debug!("Load sherpa-onnx-rs model");
    //     let (model, loading) = app_state
    //         .get_and_load_model_from(
    //             crate::types::ModelType::SherpaONNX,
    //             model_dir_path_buf.join("sherpa-onnx"),
    //         )
    //         .await?;
    //     debug!("Loading Time for Sherpa-ONNX: {loading}");
    //     let (text, processing) = model.default_transcribe(audio_data.clone()).await?;
    //     result[2] = (text, loading, processing);
    //     app_state.unload_model()?;
    // }
    // Kalosm (Candle)
    {
        debug!("Load kalosm model");
        let (model, loading) = app_state
            .get_and_load_model_from(
                crate::types::ModelType::Candle,
                model_dir_path_buf.join("kalosm"),
            )
            .await?;
        debug!("Loading Time for Kalosm: {loading}");
        let (text, processing) = model.default_transcribe(audio_data.clone()).await?;
        result[3] = (text, loading, processing);
        app_state.unload_model()?;
    }
    Ok(result)
}

/// Gets all collected commands for Super Mouse AI application to be used by builder
#[must_use]
pub fn get_collected_commands() -> Commands<Wry> {
    collect_commands![
        transcribe,
        play_sound,
        paste_text,
        process_text,
        transcribe_with_post_process,
        set_window_top,
        write_text,
        update_model,
        get_system_info,
        // transcribe_with_ct2rs,
        // transcribe_with_sherpa,
        transcribe_with_kalosm,
        transcribe_whisper_run_each
    ]
}
