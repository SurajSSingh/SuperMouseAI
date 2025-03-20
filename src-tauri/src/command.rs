//! All things related to Rust of the Super Mouse AI app

// Crate level use (imports)
use crate::{
    events::MouseClickEvent,
    mutter::ModelError,
    types::{AppState, MouseButtonType, TextProcessOptions, TranscribeOptions},
};
use enigo::{Enigo, Keyboard, Settings};
use log::error;
use mouce::{common::MouseEvent, Mouse, MouseActions};
use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader};
use tauri::{AppHandle, State, Wry};
use tauri_specta::{collect_commands, Commands, Event};

#[tauri::command]
#[specta::specta]
/// Take WAV audio data and transcribe it with application Whisper model.
///
/// Check [crate::mutter::Model::transcribe_audio] for details on arguments
pub async fn transcribe(
    app_state: State<'_, AppState>,
    audio_data: Vec<u8>,
    options: Option<TranscribeOptions>,
) -> Result<String, String> {
    let options = options.unwrap_or_default();
    log::info!("Transcribing with parameters: translate={:?}, use_timestamp={:?}, threads={:?}, prompt={:?}, lang={:?}, fmt={:?}", 
    options.translate,
    options.individual_word_timestamps,
    options.threads,
    options.initial_prompt,
    options.language,
    options.format,
);
    let transcription = app_state
        .model
        .transcribe_audio(
            &audio_data,
            options.translate.unwrap_or(false),
            options.individual_word_timestamps.unwrap_or(false),
            options.initial_prompt.as_deref(),
            options.language.as_deref(),
            // Make sure not to pass 0 for CPU thread,
            // otherwise model crashes
            match options.threads {
                Some(0) => None,
                threads => threads,
            },
        )
        .map_err(|err| {
            log::error!("Transcription Error: {:?}", err);
            match err {
                ModelError::WhisperError(whisper_error) => whisper_error.to_string(),
                ModelError::DecodingError(decoder_error) => decoder_error.to_string(),
            }
        })?;
    Ok(options
        .format
        .unwrap_or_default()
        .convert_transcript(transcription))
}

#[tauri::command]
#[specta::specta]
/// Process the text
pub async fn process_text(
    text: String,
    options: Option<TextProcessOptions>,
) -> Result<String, String> {
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
    Ok(updated_text.trim().to_string())
}

#[tauri::command]
#[specta::specta]
/// Run [transcribe] function then pass to [process_text] for post processing.
pub async fn transcribe_with_post_process(
    app_state: State<'_, AppState>,
    audio_data: Vec<u8>,
    transcribe_options: Option<TranscribeOptions>,
    processing_options: Option<TextProcessOptions>,
) -> Result<String, String> {
    process_text(
        transcribe(app_state, audio_data, transcribe_options).await?,
        processing_options,
    )
    .await
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
            MouseEvent::Press(button) => MouseClickEvent::with_payload(MouseButtonType::from(button))
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

#[tauri::command]
#[specta::specta]
/// Paste text from clipboard
pub async fn write_text(text: String) -> Result<(), String> {
    log::debug!("Start writing text");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    log::trace!("Enigo setup: {:?}", enigo);
    enigo.text(&text).map_err(|e| e.to_string())?;
    log::trace!("Enigo Wrote: `{}`", text);
    Ok(())
}

#[tauri::command]
#[specta::specta]
/// Paste text from clipboard
pub fn paste_text() -> Result<(), String> {
    log::debug!("Start Paste from clipboard");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    log::trace!("Enigo setup: {:?}", enigo);
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
            error!("Input error: {}", e);
            e.to_string()
        })?;
    enigo
        .key(enigo::Key::Unicode('v'), enigo::Direction::Click)
        .map_err(|e| {
            error!("Input error: {}", e);
            e.to_string()
        })?;
    enigo
        .key(cmd_or_ctrl, enigo::Direction::Release)
        .map_err(|e| {
            error!("Input error: {}", e);
            e.to_string()
        })?;
    log::trace!("Enigo Pasted text");
    Ok(())
}

#[tauri::command]
#[specta::specta]
/// Put window on top, can be overriden by optional parameter
pub async fn set_window_top(
    webview_window: tauri::WebviewWindow,
    override_value: Option<bool>,
) -> Result<(), String> {
    webview_window
        .set_always_on_top(override_value.unwrap_or(true))
        .map_err(|err| {
            log::error!("Could not set window to top value: {}", err);
            err.to_string()
        })
}

/// Gets all collected commands for Super Mouse AI application to be used by builder
pub fn get_collected_commands() -> Commands<Wry> {
    collect_commands![
        transcribe,
        play_sound,
        paste_text,
        process_text,
        transcribe_with_post_process,
        set_window_top,
        write_text
    ]
}
