//! All things related to Rust of the Super Mouse AI app

// Crate level use (imports)
use crate::mutter::{Model, ModelError};
use mouce::{
    common::{MouseButton, MouseEvent},
    Mouse, MouseActions,
};
use rodio::{Decoder, OutputStream, Sink};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};
use tauri::{AppHandle, Emitter, State};

/// "Global" state for the application.
///
/// This holds all data that is expected to
/// persist throughout the app's runtime.
pub struct AppState {
    model: Model,
    sound_map: HashMap<String, PathBuf>,
}

impl AppState {
    pub fn new(model: Model, sound_map: HashMap<String, PathBuf>) -> Self {
        AppState { model, sound_map }
    }

    /// Get sound by the provided name or by prepending `default_` to the beginning.
    pub fn get_sound_path(&self, sound_name: &str) -> Option<&PathBuf> {
        self.sound_map
            .get(sound_name)
            .or_else(|| self.sound_map.get(&format!("default_{}", &sound_name)))
    }
}

#[tauri::command]
/// Take WAV audio data and transcribe it with application Whisper model.
///
/// Check [`crate::mutter::Model`] for details on argument
pub fn transcribe(
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
            threads,
        )
        .map_err(|err| {
            log::error!("Transcription Error: {:?}", err);
            match err {
                ModelError::WhisperError(whisper_error) => whisper_error.to_string(),
                ModelError::DecodingError(decoder_error) => decoder_error.to_string(),
            }
        })?;
    match format.as_ref().map(String::as_str) {
        Some("vtt") => Ok(transcription.as_vtt()),
        Some("srt") => Ok(transcription.as_srt()),
        Some("text") | None => Ok(transcription.as_text()),
        Some("str") => Err("Unknown Format `str`, did you mean `srt` instead?".into()),
        Some("txt") => Err("Unknown Format `txt`, did you mean `text` instead?".into()),
        Some(unk) => Err(format!("Unknown Format: {}, should be `text`,", unk)),
    }
}

#[tauri::command]
/// Play the provided sound given its name that is stored in the app_state
pub fn play_sound(app_state: State<'_, AppState>, sound_name: String) -> Result<(), String> {
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Enum representing mouse button type
///
/// Main three are left, middle, and right
#[non_exhaustive]
enum MouseButtonType {
    Left,
    Middle,
    Right,
}

impl From<&MouseButton> for MouseButtonType {
    fn from(mb: &MouseButton) -> Self {
        match mb {
            MouseButton::Left => MouseButtonType::Left,
            MouseButton::Middle => MouseButtonType::Middle,
            MouseButton::Right => MouseButtonType::Right,
        }
    }
}

/// Function to listen for any clicks from mouse and emits Tauri event.
///
/// Returns either the callback id or an error message.
///
/// Adapted from https://github.com/crabnebula-dev/koi-pond/blob/main/src-tauri/src/lib.rs under MIT License
pub fn listen_for_mouse_click(app_handle: AppHandle) -> Result<u8, String> {
    Mouse::new()
        .hook(Box::new(move |e| match e {
            MouseEvent::Press(button) => app_handle
                .emit("mouse_press", MouseButtonType::from(button))
                .expect("App Handle should emit press event with button playload"),
            MouseEvent::Release(_button) => { /* Do Nothing Yet */ }
            _ => (),
        }))
        .map_err(|err| err.to_string())
}

/// Given a key, check if it matches one of the (specific) modifier keys.
///
/// The main modifiers are: Alt, Control, Meta, Option, and Shift (both left and right).
pub fn is_modkey(key: &device_query::Keycode) -> bool {
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

#[derive(Clone, Debug, Default, Serialize)]
/// Information about modifier key event (pressed or released)
pub struct ModKeyEvent {
    key: String,
    is_pressed: bool,
}

impl ModKeyEvent {
    /// New Modifier Key Event
    pub fn new(key: String, is_pressed: bool) -> Self {
        Self { key, is_pressed }
    }

    /// New pressed event for given key
    pub fn pressed(key: String) -> Self {
        Self::new(key, true)
    }

    /// New release event for given key
    pub fn released(key: String) -> Self {
        Self::new(key, false)
    }
}
