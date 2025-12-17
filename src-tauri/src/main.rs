// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// better than lilith
fn main() {
    launcher_tauri_lib::run()
}
