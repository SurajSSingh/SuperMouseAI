use mutter::Model;
use tauri::{path::BaseDirectory, Manager, State};
// use rodio::Decoder;
// use std::io::Cursor;

/// "Global" App state
struct ModelPath(String);

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
/// Take MP3 audio data and transcribe it with Whisper model
fn transcribe(model_path: State<'_, ModelPath>, audio_data: Vec<u8>) -> String {
    println!("Loading from {}", model_path.inner().0);
    let model = Model::new(&model_path.inner().0).expect("Valid Model Path");
    let translate = false;
    let individual_word_timestamps = false;
    let threads = None;
    // dbg!(audio_data.len());
    // println!("{:?}", audio_data.iter().take(100).collect::<Vec<_>>());
    // let input = Cursor::new(audio_data.clone());
    // let source = Decoder::new(input);
    // if let Err(e) = source {
    //     println!("{:?}", e)
    // }
    let transcription = model
        .transcribe_audio(&audio_data, translate, individual_word_timestamps, threads)
        .unwrap();
    transcription.as_text()
    // format!(
    //     "Passed vec of size {}, first four bytes: {:?}",
    //     audio_data.len(),
    //     audio_data.iter().take(4).collect::<Vec<_>>()
    // )
}

use webview2_com::Microsoft::Web::WebView2::Win32::{
    ICoreWebView2Profile4, ICoreWebView2_13, COREWEBVIEW2_PERMISSION_KIND_MICROPHONE,
    COREWEBVIEW2_PERMISSION_STATE_DEFAULT,
};
use windows::core::{Interface, PCWSTR};

/// Reset microphone permissions
///
/// Adapted from <https://github.com/tauri-apps/tauri/issues/5042#issuecomment-2269455318>
#[tauri::command]
fn reset_permission(origin: &str, app: AppHandle) {
    let webview = app.get_webview_window("main").unwrap();
    let mut origin = origin.to_string();
    origin.push('\0');
    let origin = origin.encode_utf16().collect::<Vec<u16>>();
    webview
        .with_webview(move |webview| unsafe {
            let core = webview.controller().CoreWebView2().unwrap();
            let core = Interface::cast::<ICoreWebView2_13>(&core).unwrap();
            let profile = core.Profile().unwrap();
            let profile = Interface::cast::<ICoreWebView2Profile4>(&profile).unwrap();
            let origin = PCWSTR::from_raw(origin.as_ptr());
            profile
                .SetPermissionState(
                    COREWEBVIEW2_PERMISSION_KIND_MICROPHONE,
                    origin,
                    COREWEBVIEW2_PERMISSION_STATE_DEFAULT,
                    None,
                )
                .unwrap();
        })
        .unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let resource_path = app.path().resolve(
                "resources/whisper-small.en-q8_0.bin",
                BaseDirectory::Resource,
            )?;
            app.manage(ModelPath(
                resource_path.as_os_str().to_string_lossy().to_string(),
            ));
            println!(
                "{:?}",
                resource_path.as_os_str().to_string_lossy().to_string()
            );
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            transcribe,
            reset_permission
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
