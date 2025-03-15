#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use events::{ModKeyEvent, MouseClickEvent};
use specta_typescript::Typescript;
use tauri_specta::{collect_commands, collect_events, Builder, Event};

use std::{collections::HashMap, path::PathBuf};

use command::{
    listen_for_mouse_click, paste_text, play_sound, process_text, set_window_top, transcribe,
    transcribe_with_post_process, write_text,
};
use mutter::Model;
use tauri::{path::BaseDirectory, Manager};
use types::{is_modkey, AppState, ModKeyPayload};

mod command;
mod events;
mod mutter;
mod transcript;
mod types;

/// Macro to load audio path into the app's map with given name.
///
/// Includes both the exact name and `default_` prepended name.
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

/// Number of milliseconds between checks of key events
const KEY_QUERY_MILLIS: u64 = 100;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
/// Tauri entry point to run app
pub fn run() {
    // Following <https://docs.rs/tauri-specta/2.0.0-rc.21/tauri_specta/index.html>
    let builder = Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        .commands(collect_commands![
            transcribe,
            play_sound,
            paste_text,
            process_text,
            transcribe_with_post_process,
            set_window_top,
            write_text
        ])
        .events(collect_events![MouseClickEvent, ModKeyEvent]);
    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(Typescript::default(), "../src/lib/bindings.ts")
        .expect("Failed to export typescript bindings");
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("super-mouse-ai-logs".to_string()),
                    },
                ))
                .level(if cfg!(debug_assertions) {
                    log::LevelFilter::max()
                } else {
                    log::LevelFilter::Warn
                })
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            // This is also required if you want to use events
            builder.mount_events(app);
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
                        .unwrap_or_else(|| log::info!("No start path found: {:?}", start_path));
                    stop_path
                        .exists()
                        .then(|| {
                            map.insert("stop".into(), stop_path.clone());
                        })
                        .unwrap_or_else(|| log::info!("No stop path found: {:?}", stop_path));
                    magic_path
                        .exists()
                        .then(|| {
                            map.insert("finish".into(), magic_path.clone());
                        })
                        .unwrap_or_else(|| log::info!("No magic path found: {:?}", magic_path));
                }
                map
            };
            app.manage(AppState::new(model, sound_map));
            listen_for_mouse_click(app.handle().clone())?;
            let app_key_listener_handler = app.handle().clone();
            // Listen for mod keys directly and emit when found
            std::mem::drop(tauri::async_runtime::spawn_blocking(move || {
                use device_query::{DeviceEvents, DeviceEventsHandler};
                use std::time::Duration;

                let device_state =
                    DeviceEventsHandler::new(Duration::from_millis(KEY_QUERY_MILLIS))
                        .expect("Failed to start event loop");
                let app_handle_up = app_key_listener_handler.clone();
                let app_handle_down = app_key_listener_handler.clone();
                let _up_guard = device_state.on_key_up(move |key| {
                    is_modkey(key).then(|| {
                        let _ = ModKeyEvent::with_payload(ModKeyPayload::released(key.to_string()))
                            .emit(&app_handle_up)
                            .map_err(|err| log::error!("Error for mod key event release: {err}"));
                    });
                });
                let _down_guard = device_state.on_key_down(move |key| {
                    let _ = ModKeyEvent::with_payload(ModKeyPayload::pressed(key.to_string()))
                        .emit(&app_handle_down)
                        .map_err(|err| log::error!("Error for mod key event press: {err}"));
                });
                std::thread::yield_now();
            }));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
