use mutter::Model;
use tauri::{path::BaseDirectory, AppHandle, Manager, State};
// use webview2_com::Microsoft::Web::WebView2::Win32::{
//     ICoreWebView2Profile4, ICoreWebView2_13, COREWEBVIEW2_PERMISSION_KIND_MICROPHONE,
//     COREWEBVIEW2_PERMISSION_STATE_DEFAULT,
// };
// use windows::core::{Interface, PCWSTR};
// use rodio::Decoder;
// use std::io::Cursor;

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
fn transcribe(app_state: State<'_, AppState>, audio_data: Vec<u8>) -> String {
    let translate = false;
    let individual_word_timestamps = false;
    let threads = None;
    // let sample = process_audio(audio_data);
    let transcription = app_state
        .model
        // TODO:
        // Use this over regular transcribe because decoding panics;
        // now manually creating sample
        // .transcribe_pcm_s16le(&sample, translate, individual_word_timestamps, threads)
        .transcribe_audio(&audio_data, translate, individual_word_timestamps, threads)
        .unwrap();
    transcription.as_text()
}

/// Process the audio to a format compatible with
/// Whisper.cpp: WAV, 16-bit, 16kHz, mono.
///
/// Input audio can be MP3, WEBM, or WAV
fn process_audio(audio_data: Vec<u8>) -> Vec<f32> {
    todo!()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
