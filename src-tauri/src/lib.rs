use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use mouce::{
    common::{MouseButton, MouseEvent},
    Mouse, MouseActions,
};
use mutter::{Model, ModelError};
use rodio::{Decoder, OutputStream, Sink};
use serde::{Deserialize, Serialize};
use tauri::{path::BaseDirectory, AppHandle, Emitter, Manager, State};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};

mod mutter;
mod transcript;

macro_rules! load_audio {
    ($app:ident, $map:ident, $name: ident) => {{
        let path = $app.path().resolve(
            format!("resources/{}.mp3", stringify!($name)),
            BaseDirectory::Resource,
        )?;
        $map.insert(format!("default_{}", stringify!($name)), path.clone());
        $map.insert(stringify!($name).into(), path);
    }};
    ($app:ident, $map:ident, $fileName: ident, $mapName: ident) => {{
        let path = $app.path().resolve(
            format!("resources/{}.mp3", stringify!($fileName)),
            BaseDirectory::Resource,
        )?;
        $map.insert(format!("default_{}", stringify!($mapName)), path.clone());
        $map.insert(stringify!($mapName).into(), path);
    }};
}

/// "Global" App state
struct AppState {
    model: Model,
    sound_map: HashMap<String, PathBuf>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
/// Take MP3 audio data and transcribe it with Whisper model
fn transcribe(
    app_state: State<'_, AppState>,
    audio_data: Vec<u8>,
    translate: Option<bool>,
    individual_word_timestamps: Option<bool>,
    threads: Option<u16>,
    initial_prompt: Option<String>,
    language: Option<String>,
) -> Result<String, String> {
    let translate = translate.unwrap_or(false);
    let individual_word_timestamps = individual_word_timestamps.unwrap_or(false);
    let transcription = app_state
        .model
        .transcribe_audio(
            &audio_data,
            translate,
            individual_word_timestamps,
            initial_prompt.as_deref(),
            language.as_deref(),
            threads,
        )
        .map_err(|err| match err {
            ModelError::WhisperError(whisper_error) => whisper_error.to_string(),
            ModelError::DecodingError(decoder_error) => decoder_error.to_string(),
        })?;
    Ok(transcription.as_text())
}

#[tauri::command]
fn play_sound(app_state: State<'_, AppState>, sound_name: String) -> Result<(), String> {
    let sound_map = &app_state.sound_map;
    let source = if let Some(path) = sound_map.get(&sound_name) {
        // Play sound
        let file = BufReader::new(File::open(path).map_err(|err| dbg!(err).to_string())?);
        Decoder::new(file).map_err(|err| dbg!(err).to_string())?
    } else if let Some(path) = sound_map.get(&format!("default_{}", &sound_name)) {
        // Play default sound
        let file = BufReader::new(File::open(path).map_err(|err| dbg!(err).to_string())?);
        Decoder::new(file).map_err(|err| dbg!(err).to_string())?
    } else {
        return Err(format!("No sound with name: {}", &sound_name));
    };
    // app_state.sink.play();
    // Ok(())
    let (_stream, sound_handle) = OutputStream::try_default().map_err(|err| err.to_string())?;
    let sink = Sink::try_new(&sound_handle).map_err(|err| err.to_string())?;
    sink.append(source);
    sink.sleep_until_end();
    sink.detach();
    Ok(())
    // sound_handle
    //     .play_raw(source.convert_samples())
    //     .map_err(|err| dbg!(err).to_string())
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
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
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
            let sound_map = {
                let mut map = HashMap::with_capacity(8);
                load_audio!(app, map, alert);
                load_audio!(app, map, start_record, start);
                load_audio!(app, map, stop_record, stop);
                load_audio!(app, map, transcribed, finish);
                map
            };
            app.manage(AppState { model, sound_map });
            listen_for_mouse_click(app.handle().clone());
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, transcribe, play_sound])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
