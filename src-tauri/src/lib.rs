#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// External Crates
use log::{debug, error, info, trace, warn, LevelFilter};
use std::{collections::HashMap, path::PathBuf};
use tauri::{path::BaseDirectory, Manager};
use tauri_plugin_sentry::{minidump, sentry};
use tauri_specta::{Builder, Event};

// Internal Modules
mod command;
mod events;
mod mutter;
mod transcript;
mod types;

use command::listen_for_mouse_click;
use events::ModKeyEvent;
use mutter::Model;
use types::{is_modkey, AppState, ModKeyPayload};

pub use crate::command::get_collected_commands;
pub use crate::events::get_collected_events;

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
    info!("Start running Super Mouse AI");
    debug!("Start Sentry Setup");
    // Setup
    let client = sentry::init((
        "https://e48c5c52c4ca1341de4618624cc0f511@o4509002112958464.ingest.us.sentry.io/4509007972007936",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            auto_session_tracking: true,
            ..Default::default()
        },
    ));

    // Caution! Everything before here runs in both app and crash reporter processes
    #[cfg(not(target_os = "ios"))]
    let _guard = minidump::init(&client);
    // Everything after here runs in only the app process

    debug!("Finish Sentry Setup");

    // Following <https://docs.rs/tauri-specta/2.0.0-rc.21/tauri_specta/index.html>
    let builder = Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        .commands(get_collected_commands())
        .events(get_collected_events());
    export_bindings(&builder);
    info!("Start app building");
    tauri::Builder::default()
        .plugin(tauri_plugin_sentry::init(&client))
        .plugin(tauri_plugin_process::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("super-mouse-ai-logs".to_string()),
                    },
                ))
                .level(if cfg!(feature = "log-trace") {
                    LevelFilter::max()
                } else if cfg!(feature = "log-issues-only") {
                    LevelFilter::Warn
                } else if cfg!(feature = "log-none") {
                    LevelFilter::Off
                } else if cfg!(debug_assertions) {
                    LevelFilter::Debug
                } else {
                    LevelFilter::Info
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
            debug!("Start app setup function");
            // This is also required if you want to use events
            builder.mount_events(app);
            trace!("Mounted events for app");
            // Initialize Shortcuts plugin
            #[cfg(desktop)]
            app.handle()
                .plugin(tauri_plugin_global_shortcut::Builder::new().build())?;
            trace!("Initialized shortcut plugins.");
            debug!("Start loading model path");
            //  Load the model
            let resource_path = app
                .path()
                .resolve("resources/whisper-model.bin", BaseDirectory::Resource)?;
            trace!("Resolved resource model path.");
            let model_path = resource_path
                .into_os_string()
                .into_string()
                .map_err(|os_str| format!("\"{:?}\" cannot be convered to string!", os_str))?;
            trace!("Converted model path");
            let model = Model::new(&model_path)?;
            trace!("Created new model");
            debug!("Start loading sound paths");
            let sound_map = {
                let mut map = HashMap::with_capacity(8);
                trace!("Created new sound map");
                load_audio!(app, map, alert);
                load_audio!(app, map, start_record, start);
                load_audio!(app, map, stop_record, stop);
                load_audio!(app, map, transcribed, finish);
                trace!("Loaded all default sounds");
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
                        .unwrap_or_else(|| warn!("No start path found: {:?}", start_path));
                    stop_path
                        .exists()
                        .then(|| {
                            map.insert("stop".into(), stop_path.clone());
                        })
                        .unwrap_or_else(|| warn!("No stop path found: {:?}", stop_path));
                    magic_path
                        .exists()
                        .then(|| {
                            map.insert("finish".into(), magic_path.clone());
                        })
                        .unwrap_or_else(|| warn!("No magic path found: {:?}", magic_path));
                    trace!("Finished looking for extra sounds");
                }
                debug!("Finished creating sound map");
                map
            };
            app.manage(AppState::new(model, sound_map));
            trace!("Created initial app state");
            debug!("Setup mouse click listener");
            listen_for_mouse_click(app.handle().clone())?;
            debug!("Setup modifier key listener");
            let app_key_listener_handler = app.handle().clone();
            trace!("Cloned app handle for new listener thread");
            // Listen for mod keys directly and emit when found
            std::mem::drop(tauri::async_runtime::spawn_blocking(move || {
                use device_query::{DeviceEvents, DeviceEventsHandler};
                use std::time::Duration;

                let device_state =
                    DeviceEventsHandler::new(Duration::from_millis(KEY_QUERY_MILLIS))
                        .expect("Failed to start event loop");
                trace!("Created device state");
                let app_handle_up = app_key_listener_handler.clone();
                let app_handle_down = app_key_listener_handler.clone();
                trace!("Created clones for app handlers");
                let _up_guard = device_state.on_key_up(move |key| {
                    is_modkey(key).then(|| {
                        trace!("Mod Key UP Event with {:?}", key);
                        let _ = ModKeyEvent::with_payload(ModKeyPayload::released(key.to_string()))
                            .emit(&app_handle_up)
                            .map_err(|err| error!("Error for mod key event release: {err}"));
                    });
                });
                trace!("Start listening for key up events");
                let _down_guard = device_state.on_key_down(move |key| {
                    is_modkey(key).then(|| {
                        trace!("Mod Key Event DOWN with {:?}", key);
                        let _ = ModKeyEvent::with_payload(ModKeyPayload::pressed(key.to_string()))
                            .emit(&app_handle_down)
                            .map_err(|err| error!("Error for mod key event press: {err}"));
                    });
                });
                trace!("Start listening for key down events");
                // Require loop to ensure thread remains active
                #[allow(clippy::empty_loop)]
                loop {}
            }));
            if cfg!(feature = "no-overlay") {
                debug!("Removing overlay");
                if let Some(overlay) = app.get_webview_window("overlay") {
                    overlay.close()?;
                }
            } else {
                info!("Ignoring mouse events in overlay");
                app.get_webview_window("overlay")
                    .expect("Overlay should exist")
                    .set_ignore_cursor_events(true)
                    .expect("Setting to ignore cursor should work");
            }

            info!("Add close all windows event when main window is closed");
            let windows = app.webview_windows();
            app.get_webview_window("main")
                .expect("Main window should exist")
                .on_window_event(move |event| match event {
                    tauri::WindowEvent::CloseRequested { api: _api, .. } => {
                        debug!("Main window requested to be closed!");
                        windows.iter().for_each(|(label, window)| {
                            // NOTE: Only do non-main window, otherwise will get stuck in loop
                            if !label.eq_ignore_ascii_case("main") {
                                debug!("Window {label} will also be closed.");
                                window
                                    .close()
                                    .unwrap_or_else(|_| panic!("Window {label} should be closable"));
                            }
                        });
                    }
                    _ => { /* Do nothing*/ }
                });
            debug!("Finish app setup function");
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    info!("Finish app building");
}

/// Export TypeScript bindings for the application
pub fn export_bindings(builder: &Builder) {
    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(
            specta_typescript::Typescript::default(),
            "../src/lib/bindings.ts",
        )
        .expect("Failed to export typescript bindings");
    debug!("Exported TypeScript bindings");
}
