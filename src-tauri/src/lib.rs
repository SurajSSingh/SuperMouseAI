use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use mouce::{
    common::{MouseButton, MouseEvent},
    Mouse, MouseActions,
};
use mutter::{Model, ModelError};
use rodio::{Decoder, OutputStream, Sink};
use serde::{Deserialize, Serialize};
use tauri::{path::BaseDirectory, AppHandle, Emitter, Manager, State};

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

const KEY_QUERY_MILLIS: u64 = 100;

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

/// Given a key, check if it matches one of the modifier keys.
///
/// Modifiers are: Alt, Control, Meta, Option, and Shift (both left and right).
fn is_modkey(key: &device_query::Keycode) -> bool {
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
struct ModKeyEvent {
    key: String,
    is_pressed: bool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            // Initialize Shortcuts plugin
            #[cfg(desktop)]
            app.handle()
                .plugin(tauri_plugin_global_shortcut::Builder::new().build())?;
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
                {
                    // TEMP: Add client sound paths, see reference
                    //       Must be configurable after release
                    let mut path_buf = PathBuf::new();
                    path_buf.push(r"C:\");
                    path_buf.push("MyFastPrograms");
                    path_buf.push("Python");
                    path_buf.push("virtuale");
                    let mut start_path = path_buf.clone();
                    let mut stop_path = path_buf.clone();
                    let mut magic_path = path_buf;
                    start_path.push("start_sound");
                    stop_path.push("stop_sound");
                    magic_path.push("magicsound");
                    start_path.set_extension("wav");
                    stop_path.set_extension("wav");
                    magic_path.set_extension("wav");
                    start_path
                        .exists()
                        .then(|| {
                            map.insert("start".into(), start_path.clone());
                        })
                        .unwrap_or_else(|| println!("No start path found: {:?}", start_path));
                    stop_path
                        .exists()
                        .then(|| {
                            map.insert("stop".into(), stop_path.clone());
                        })
                        .unwrap_or_else(|| println!("No stop path found: {:?}", stop_path));
                    magic_path
                        .exists()
                        .then(|| {
                            map.insert("finish".into(), magic_path.clone());
                        })
                        .unwrap_or_else(|| println!("No magic path found: {:?}", magic_path));
                }
                map
            };
            app.manage(AppState { model, sound_map });
            listen_for_mouse_click(app.handle().clone());
            let app_key_listener_handler = app.handle().clone();
            // Listen for mod keys directly and emit when found
            let _ = tauri::async_runtime::spawn_blocking(move || {
                use device_query::{DeviceEvents, DeviceEventsHandler};
                use std::time::Duration;

                let device_state =
                    DeviceEventsHandler::new(Duration::from_millis(KEY_QUERY_MILLIS))
                        .expect("Failed to start event loop");
                let app_handle_up = app_key_listener_handler.clone();
                let app_handle_down = app_key_listener_handler.clone();
                let _up_guard = device_state.on_key_up(move |key| {
                    is_modkey(key).then(|| {
                        app_handle_up.emit(
                            "mod_key",
                            ModKeyEvent {
                                key: key.to_string(),
                                is_pressed: false,
                            },
                        )
                    });
                });
                let _down_guard = device_state.on_key_down(move |key| {
                    is_modkey(key).then(|| {
                        app_handle_down.emit(
                            "mod_key",
                            ModKeyEvent {
                                key: key.to_string(),
                                is_pressed: true,
                            },
                        )
                    });
                });
                loop {}
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, transcribe, play_sound])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
