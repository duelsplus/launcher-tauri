//! Main library entry point for the Tauri application.
//!
//! This module initializes the Tauri application and registers all
//! available commands that can be invoked from the frontend.

mod auth;
mod commands;
mod config;
mod proxy;
mod rpc;
mod utils;

use commands::*;
use proxy::ProxyManager;
use rpc::RpcManager;
use tauri::{Manager, WindowEvent};

/// Initializes and runs the Tauri application.
///
/// Sets up the Tauri builder with required plugins and registers
/// all authentication-related commands for frontend access.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Create RPC manager (is_dev will be set in setup hook)
    let rpc_manager = RpcManager::new(false); // Temporary, will be updated in setup

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .manage(ProxyManager::new())
        .manage(rpc_manager)
        .invoke_handler(tauri::generate_handler![
            // Authentication handling
            token_exists,
            get_token,
            save_token,
            delete_token,
            verify_token,
            get_user,
            get_user_stats,
            get_global_stats,
            check_api_status,
            start_discord_signin,
            // Process management
            launch_proxy,
            stop_proxy,
            get_proxy_status,
            fetch_releases,
            // Configuration management
            legacy_config_exists,
            config_exists,
            get_legacy_config,
            get_config,
            get_legacy_config_value,
            get_config_value,
            set_config_key,
            save_config,
            // Discord RPC
            rpc_set_enabled,
            rpc_is_enabled,
            rpc_set_activity,
        ])
        .setup(|app| {
            // Detect if running in dev mode using Tauri's environment
            // In dev mode, Tauri sets TAURI_DEV environment variable
            let is_dev = std::env::var("TAURI_DEV").is_ok() || cfg!(debug_assertions);

            // Update RPC manager with correct dev flag
            if let Some(rpc) = app.try_state::<RpcManager>() {
                rpc.set_dev_mode(is_dev);
            }

            // Load config and apply RPC settings
            let rpc_state = app.try_state::<RpcManager>();
            if let Some(rpc) = rpc_state {
                // Try to load saved config and apply RPC settings
                // Use tokio runtime to run the async config loading
                let rt = tokio::runtime::Runtime::new().unwrap();
                if let Ok(Some(cfg)) = rt.block_on(config::manager::get_config()) {
                    rpc.set_enabled(cfg.enable_rpc);
                    rpc.set_anonymization(cfg.rpc_anonymize_profile, cfg.rpc_anonymize_location);
                }
            }

            // Start and connect RPC
            if let Some(rpc) = app.try_state::<RpcManager>() {
                rpc.start();
                rpc.connect();
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            // Stop the proxy when the window is closed
            if let WindowEvent::CloseRequested { .. } = event {
                let app = window.app_handle();
                if let Some(proxy) = app.try_state::<ProxyManager>() {
                    // Use tauri's async runtime to stop the proxy
                    tauri::async_runtime::block_on(async {
                        let _ = proxy.stop().await;
                    });
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
