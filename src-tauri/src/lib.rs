#![deny(clippy::all, clippy::suspicious, clippy::complexity, clippy::perf)]
#![warn(clippy::style, clippy::cargo, clippy::pedantic, clippy::nursery)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// External Crates
use log::{debug, error, info, trace, warn, LevelFilter};
use std::{collections::HashMap, path::PathBuf};
use tauri::{path::BaseDirectory, Manager};
use tauri::{App, AppHandle};
// use tauri_plugin_sentry::sentry::protocol::Event as SentryEvent;
use tauri_plugin_sentry::sentry::ClientInitGuard;
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
use types::{is_modkey, InnerAppState, ModKeyPayload};

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
///
/// ## Panics
///
/// If Tauri app fails to build
pub fn run() {
    info!("Start running Super Mouse AI");
    let bindings_builder = export_bindings();
    info!("Start app building");
    let client = create_sentry_client();
    // Caution! Everything before here runs in both app and crash reporter processes
    #[cfg(not(target_os = "ios"))]
    let _guard = minidump::init(&client);
    debug!("Finish Sentry Setup");
    // Everything after here runs in only the app process
    let app_builder = create_app_builder(&client);
    debug!("Finish App plugin initialization");
    #[allow(
        clippy::large_stack_frames,
        reason = "This is required by Tauri and cannot be refactored right now"
    )]
    app_builder
        .invoke_handler(bindings_builder.invoke_handler())
        .setup(move |app| setup_app(app, &bindings_builder))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    info!("Finish app building");
}

/// Export TypeScript bindings for the application
#[must_use]
pub fn export_bindings() -> Builder {
    debug!("Exporting Rust binding to TypeScript");
    // Following <https://docs.rs/tauri-specta/2.0.0-rc.21/tauri_specta/index.html>
    let builder = Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        .commands(get_collected_commands())
        .events(get_collected_events());
    #[cfg(debug_assertions)] // <- Only export on non-release builds
    match builder.export(
        specta_typescript::Typescript::default(),
        "../src/lib/bindings.ts",
    ) {
        Ok(()) => debug!("Exported TypeScript bindings"),
        Err(err) => warn!("Export Binding Issue: {err}"),
    };
    builder
}

fn create_app_builder(client: &ClientInitGuard) -> tauri::Builder<tauri::Wry> {
    debug!("Creating Tauri app builder and adding initializing plugin");
    tauri::Builder::default()
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_sentry::init(client))
        .plugin(tauri_plugin_process::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("super-mouse-ai-logs".to_string()),
                    },
                ))
                .level(get_default_log_level())
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(single_instance_handler))
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
}

fn get_default_log_level() -> LevelFilter {
    if cfg!(feature = "log-trace") {
        LevelFilter::max()
    } else if cfg!(feature = "log-issues-only") {
        LevelFilter::Warn
    } else if cfg!(feature = "log-none") {
        LevelFilter::Off
    } else if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    }
}

fn single_instance_handler(app: &AppHandle, _args: Vec<String>, _cwd: String) {
    let _ = app
        .get_webview_window("main")
        .expect("no main window")
        .set_focus();
}

fn create_sentry_client() -> ClientInitGuard {
    debug!("Start Sentry Setup");
    sentry::init((
        "https://e48c5c52c4ca1341de4618624cc0f511@o4509002112958464.ingest.us.sentry.io/4509007972007936",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            auto_session_tracking: true,
            // before_send: Some(Arc::new(event_callback)),
            ..Default::default()
        },
    ))
}

// /// Arc<dyn Fn(Event<'static>) -> Option<Event<'static>> + Send + Sync, Global>
// fn event_callback(event: SentryEvent) -> Option<SentryEvent> {
//     todo!()
// }

fn setup_app(app: &App, bindings_builder: &Builder) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up application");
    trace!("Mounting events for app");
    bindings_builder.mount_events(app);
    trace!("Adding desktop plugins");
    #[cfg(desktop)]
    add_desktop_plugins(app)?;
    trace!("Start resolving resource model path");
    let default_model_path = resolve_model_path(app)?;
    trace!("Converted model path");
    let model = Model::new(&default_model_path)?;
    trace!("Created new model");
    debug!("Start loading sound paths");
    let sound_map = create_sound_map(app)?;
    debug!("Finished creating sound map");
    app.manage(std::sync::Mutex::new(InnerAppState::new(model, sound_map)));
    trace!("Created initial app state");
    debug!("Setup mouse click listener");
    let _mouse_click_listener_handler = listen_for_mouse_click(app.handle().clone())?;
    debug!("Setup modifier key listener");
    let app_key_listener_handler = app.handle().clone();
    trace!("Cloned app handle for new listener thread");
    setup_key_listeners(&app_key_listener_handler)?;
    debug!("Finished setting up key listeners");
    configure_overlay(app)?;
    setup_main_window_close_event(app)?;
    info!("Finish app setup function");
    Ok(())
}

/// Add desktop-only plugins to app
///
/// Currently adds Global Shortcut Plugin
fn add_desktop_plugins(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    app.handle()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())?;
    Ok(())
}

/// Resolves the path to the whisper model file.
///
/// # Errors
///
/// Returns an error if the path cannot be resolved or converted.
fn resolve_model_path(app: &App) -> Result<String, Box<dyn std::error::Error>> {
    let resource_path = app
        .path()
        .resolve("resources/whisper-model.bin", BaseDirectory::Resource)?;
    trace!("Resolved resource model path.");
    let model_path = resource_path
        .into_os_string()
        .into_string()
        .map_err(|os_str| format!("\"{os_str:?}\" cannot be converted to string!"))?;
    Ok(model_path)
}

/// Creates a map of sound paths.
///
/// # Errors
///
/// Returns an error if the paths cannot be resolved.
fn create_sound_map(app: &App) -> Result<HashMap<String, PathBuf>, Box<dyn std::error::Error>> {
    let mut map = HashMap::with_capacity(8);
    trace!("Created new sound map");
    load_audio!(app, map, alert);
    load_audio!(app, map, start_record, start);
    load_audio!(app, map, stop_record, stop);
    load_audio!(app, map, transcribed, finish);
    trace!("Loaded all default sounds");
    {
        // TEMP: Add client sound paths, see reference
        // Must be configurable after release
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
        start_path.exists().then(|| {
            map.insert("start".into(), start_path.clone());
        });
        stop_path.exists().then(|| {
            map.insert("stop".into(), stop_path.clone());
        });
        magic_path.exists().then(|| {
            map.insert("finish".into(), magic_path.clone());
        });
        if !start_path.exists() {
            warn!("No start sound path found: {start_path:?}");
        }
        if !stop_path.exists() {
            warn!("No stop sound path found: {stop_path:?}");
        }
        if !magic_path.exists() {
            warn!("No finish sound path found: {magic_path:?}");
        }
        trace!("Finished looking for extra sounds");
    }
    Ok(map)
}

/// Set up key listeners for modifier keys.
///
/// # Errors
///
/// Returns an error if the listener setup fails.
fn setup_key_listeners(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle_up = app_handle.clone();
    let app_handle_down = app_handle.clone();
    trace!("Created clones for app handlers");
    std::mem::drop(tauri::async_runtime::spawn_blocking(move || {
        use device_query::{DeviceEvents, DeviceEventsHandler};
        use std::time::Duration;

        let device_state = DeviceEventsHandler::new(Duration::from_millis(KEY_QUERY_MILLIS))
            .expect("Failed to start event loop");
        trace!("Created device state");

        let _up_guard = device_state.on_key_up(move |key| {
            if is_modkey(*key) {
                trace!("Mod Key UP Event with {key:?}");
                let _ = ModKeyEvent::with_payload(ModKeyPayload::released(key.to_string()))
                    .emit(&app_handle_up)
                    .map_err(|err| error!("Error for mod key event release: {err}"));
            }
        });
        trace!("Start listening for key up events");
        let _down_guard = device_state.on_key_down(move |key| {
            if is_modkey(*key) {
                trace!("Mod Key Event DOWN with {key:?}");
                let _ = ModKeyEvent::with_payload(ModKeyPayload::pressed(key.to_string()))
                    .emit(&app_handle_down)
                    .map_err(|err| error!("Error for mod key event press: {err}"));
            }
        });
        trace!("Start listening for key down events");
        // Keep the process alive by running an infinte loop
        #[allow(
            clippy::empty_loop,
            reason = "Require loop to ensure thread remains active"
        )]
        loop {}
    }));
    Ok(())
}

/// Configures the overlay window settings based on the app configuration.
///
/// # Errors
///
/// Returns an error if the overlay window setup fails.
fn configure_overlay(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    let windows = app.webview_windows();
    if cfg!(feature = "overlay") {
        info!("Ignoring mouse events in overlay");
        windows
            .get("overlay")
            .expect("Overlay should exist")
            .set_ignore_cursor_events(true)
            .expect("Setting to ignore cursor should work");
    } else {
        debug!("Removing overlay");
        if let Some(overlay) = windows.get("overlay") {
            overlay.close()?;
        }
    }
    Ok(())
}

/// Sets up an event listener to close all windows when the main window is closed.
///
/// # Errors
///
/// Returns an error if the event listener setup fails.
fn setup_main_window_close_event(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    let windows = app.webview_windows();
    app.get_webview_window("main")
        .expect("Main window should exist")
        .on_window_event(move |event| {
            if let tauri::WindowEvent::CloseRequested { api: _, .. } = event {
                debug!("Main window requested to be closed!");
                for (label, window) in &windows {
                    // NOTE: Only do non-main window, otherwise will get stuck in loop
                    if label.to_ascii_lowercase() != "main" {
                        debug!("Window {label} will also be closed.");
                        if let Err(err) = window.close() {
                            error!("Failed to close window {label}: {err}");
                        }
                    }
                }
            } else {
                /* Do nothing */
            }
        });
    Ok(())
}
