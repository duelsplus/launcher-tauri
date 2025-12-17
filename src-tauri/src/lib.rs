//! Main library entry point for the Tauri application.
//!
//! This module initializes the Tauri application and registers all
//! available commands that can be invoked from the frontend.

mod auth;
mod commands;
mod proxy;

use commands::*;
use proxy::ProxyManager;

/// Initializes and runs the Tauri application.
///
/// Sets up the Tauri builder with required plugins and registers
/// all authentication-related commands for frontend access.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(ProxyManager::new())
        .invoke_handler(tauri::generate_handler![
            // Authentication handling
            token_exists,
            get_token,
            save_token,
            delete_token,
            verify_token,
            get_user,
            // Process management
            launch_proxy,
            stop_proxy,
            get_proxy_status
        ])
        .setup(|_app| {
            // Cleanup will be handled by the on_window_event hook
            // or by explicit stop_proxy calls from the frontend
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
