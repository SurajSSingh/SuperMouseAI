use mouce::{
    common::{MouseButton, MouseEvent},
    Mouse, MouseActions,
};
use mutter::{Model, ModelError};
use serde::{Deserialize, Serialize};
use tauri::{path::BaseDirectory, AppHandle, Emitter, Manager, State};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};
// use webview2_com::Microsoft::Web::WebView2::Win32::{
//     ICoreWebView2Profile4, ICoreWebView2_13, COREWEBVIEW2_PERMISSION_KIND_MICROPHONE,
//     COREWEBVIEW2_PERMISSION_STATE_DEFAULT,
// };
// use windows::core::{Interface, PCWSTR};
// use rodio::Decoder;
// use std::io::Cursor;

mod mutter;
mod transcript;

/// "Global" App state
struct AppState {
    model: Model,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
/// Take MP3 audio data and transcribe it with Whisper model
fn transcribe(app_state: State<'_, AppState>, audio_data: Vec<u8>) -> Result<String, String> {
    let translate = false;
    let individual_word_timestamps = false;
    let threads = None;
    // TODO: Possible thing to allow user to change
    let initial_prompt = None;
    let language = Some("en");
    let transcription = app_state
        .model
        .transcribe_audio(
            &audio_data,
            translate,
            individual_word_timestamps,
            initial_prompt,
            language,
            threads,
        )
        .map_err(|err| match err {
            ModelError::WhisperError(whisper_error) => whisper_error.to_string(),
            ModelError::DecodingError(decoder_error) => decoder_error.to_string(),
        })?;
    Ok(transcription.as_text())
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

/// Function to listen for any clicks from mouse
///
/// Adapted from https://github.com/crabnebula-dev/koi-pond/blob/main/src-tauri/src/lib.rs under MIT License
fn listen_for_mouse_click(app_handle: AppHandle) {
    let mut mouse_manager = Mouse::new();
    mouse_manager
        .hook(Box::new(move |e| match e {
            MouseEvent::Press(button) => app_handle
                .emit("mouse_press", MouseButtonType::from(button))
                .expect("App Handle should emit press event with button playload"),
            // MouseEvent::Release(mouse_button) => todo!(),
            _ => (),
        }))
        .expect("Able to listen to mouse clicks!");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mod_key = Shortcut::new(Some(Modifiers::ALT), Code::Space);

    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcuts([
                    // alt_left,
                    // alt_right,
                    // ctrl_left,
                    // ctrl_right,
                    // shift_left,
                    // shift_right,
                    mod_key,
                ])
                .expect("Shortcuts should be valid")
                .with_handler(|app, _shortcut, event| {
                    app.emit("mod_key_event", event.state)
                        .expect("Keyboard should emit");
                })
                .build(),
        )
        .setup(|app| {
            //  Load the model
            let resource_path = app
                .path()
                .resolve("resources/whisper-model.bin", BaseDirectory::Resource)?;
            let model_path = resource_path
                .into_os_string()
                .into_string()
                .map_err(|os_str| format!("\"{:?}\" cannot be convered to string!", os_str))?;
            let model = Model::new(&model_path)?;
            app.manage(AppState { model });
            listen_for_mouse_click(app.handle().clone());
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, transcribe])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
