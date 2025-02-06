use mutter::{Model, ModelError};
use tauri::{path::BaseDirectory, Manager, State};
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

// TODO: Cannot register mouse click in hotkey directly using the plugin
//       (see https://github.com/tauri-apps/global-hotkey/issues/50),
//       consider: https://crabnebula.dev/blog/building-a-desktop-pet-with-tauri/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let resource_path = app.path().resolve(
                "resources/whisper-small.en-q8_0.bin",
                BaseDirectory::Resource,
            )?;
            let model_path = resource_path
                .into_os_string()
                .into_string()
                .map_err(|os_str| format!("\"{:?}\" cannot be convered to string!", os_str))?;
            let model = Model::new(&model_path)?;
            app.manage(AppState { model });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, transcribe])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
