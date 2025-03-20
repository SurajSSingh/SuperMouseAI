// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if cfg!(feature = "export-bindings-only") {
        todo!("Add default bulder"); // super_mouse_ai_lib::export_bindings(builder)
    } else {
        super_mouse_ai_lib::run()
    }
}
