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
fn transcribe(model_path: State<'_, ModelPath>, audio_data: Vec<u8>) -> String {
    println!("Loading from {}", model_path.inner().0);
    let model = Model::new(&model_path.inner().0).expect("Valid Model Path");
    let translate = false;
    let individual_word_timestamps = false;
    let threads = Some(8);
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
        .invoke_handler(tauri::generate_handler![greet, transcribe])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
