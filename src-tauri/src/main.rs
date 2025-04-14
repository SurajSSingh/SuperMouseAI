// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if cfg!(feature = "export-bindings-only") {
        super_mouse_ai_lib::export_bindings();
    } else {
        super_mouse_ai_lib::run()
    }
}
