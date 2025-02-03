use mutter::Model;
// use rodio::Decoder;
// use std::io::Cursor;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn transcribe(audio_data: Vec<u8>) -> String {
    let model =
        Model::new("/Users/secondary/Downloads/ggml-small.en-q8_0.bin").expect("Valid Model Path");
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
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, transcribe])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
