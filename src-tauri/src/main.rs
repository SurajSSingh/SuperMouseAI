// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if cfg!(feature = "export-bindings-only") {
        super_mouse_ai_lib::export_bindings(
            &tauri_specta::Builder::<tauri::Wry>::new()
                // Then register them (separated by a comma)
                .commands(super_mouse_ai_lib::get_collected_commands())
                .events(super_mouse_ai_lib::get_collected_events()),
        )
    } else {
        super_mouse_ai_lib::run()
    }
}
