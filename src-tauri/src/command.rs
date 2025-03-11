//! All things related to Rust of the Super Mouse AI app

// Crate level use (imports)
use crate::{
    events::MouseClickEvent,
    mutter::ModelError,
    types::{AppState, MouseButtonType},
};
use log::error;
use mouce::{common::MouseEvent, Mouse, MouseActions};
use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader};
use tauri::{AppHandle, State};
use tauri_specta::Event;

#[tauri::command]
#[specta::specta]
/// Take WAV audio data and transcribe it with application Whisper model.
///
/// Check [crate::mutter::Model::transcribe_audio] for details on arguments
pub async fn transcribe(
    app_state: State<'_, AppState>,
    audio_data: Vec<u8>,
    translate: Option<bool>,
    individual_word_timestamps: Option<bool>,
    threads: Option<u16>,
    initial_prompt: Option<String>,
    language: Option<String>,
    format: Option<String>,
) -> Result<String, String> {
    log::info!("Transcribing with parameters: translate={:?}, use_timestamp={:?}, threads={:?}, prompt={:?}, lang={:?}, fmt={:?}", 
    translate,
    individual_word_timestamps,
    threads,
    initial_prompt,
    language,
    format,
);
    let transcription = app_state
        .model
        .transcribe_audio(
            &audio_data,
            translate.unwrap_or(false),
            individual_word_timestamps.unwrap_or(false),
            initial_prompt.as_deref(),
            language.as_deref(),
            // Make sure not to pass 0 for CPU thread,
            // otherwise model crashes
            match threads {
                Some(0) => None,
                _ => threads,
            },
        )
        .map_err(|err| {
            log::error!("Transcription Error: {:?}", err);
            match err {
                ModelError::WhisperError(whisper_error) => whisper_error.to_string(),
                ModelError::DecodingError(decoder_error) => decoder_error.to_string(),
            }
        })?;
    match format.as_deref() {
        Some("vtt") => Ok(transcription.as_vtt()),
        Some("srt") => Ok(transcription.as_srt()),
        Some("text") | None => Ok(transcription.as_text()),
        Some("str") => Err("Unknown Format `str`, did you mean `srt` instead?".into()),
        Some("txt") => Err("Unknown Format `txt`, did you mean `text` instead?".into()),
        Some(unk) => Err(format!("Unknown Format: {}, should be `text`,", unk)),
    }
}

#[tauri::command]
#[specta::specta]
/// Play the provided sound given its name that is stored in the app_state
pub async fn play_sound(app_state: State<'_, AppState>, sound_name: String) -> Result<(), String> {
    // Get sound source
    let source = app_state
        .get_sound_path(&sound_name)
        .ok_or(format!("Could not find sound with name: {}", sound_name))
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
/// Adapted from https://github.com/crabnebula-dev/koi-pond/blob/main/src-tauri/src/lib.rs under MIT License
pub fn listen_for_mouse_click(app_handle: AppHandle) -> Result<u8, String> {
    Mouse::new()
        .hook(Box::new(move |e| match e {
            MouseEvent::Press(button) => 
            MouseClickEvent::with_payload(MouseButtonType::from(button))
                .emit(&app_handle)
                .map_err(|e| {
                    error!("App Handle expected to emit press event with button playload but could not: {}", e);
                })
                .unwrap_or_default(),
            MouseEvent::Release(_button) => { /* Do Nothing Yet */ }
            _ => (),
        }))
        .map_err(|err| err.to_string())
}
