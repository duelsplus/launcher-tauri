//! Tauri commands for frontend-backend communication.
//!
//! This module exposes authentication functions as Tauri commands that can be
//! invoked from the frontend JavaScript/TypeScript code.

use crate::auth;
use crate::config;
use crate::proxy::{download, models, ProxyManager};
use crate::rpc::RpcManager;
use tauri::{AppHandle, State};

/// Starts the Discord OAuth sign-in flow.
///
/// This command:
/// 1. Starts a temporary localhost server on a random port
/// 2. Requests the Discord OAuth URL from the API
/// 3. Opens the URL in the user's browser
/// 4. Waits for the callback and emits the token via `discord-auth-result` event
///
/// # Arguments
///
/// * `app` - The Tauri app handle
///
/// # Returns
///
/// Returns `Ok(())` if the flow started successfully.
/// The token will be delivered via the `discord-auth-result` event.
#[tauri::command]
pub async fn start_discord_signin(app: AppHandle) -> Result<(), String> {
    auth::discord::start_discord_signin(app)
        .await
        .map_err(|e| e.to_string())
}

/// Checks if a token file exists on disk.
///
/// Returns `true` if the token file exists, `false` otherwise.
#[tauri::command]
pub async fn token_exists() -> Result<bool, String> {
    auth::token::token_exists().await.map_err(|e| e.to_string())
}

/// Retrieves the stored authentication token from disk.
///
/// Returns `Some(token)` if the token exists, `None` if it doesn't.
#[tauri::command]
pub async fn get_token() -> Result<Option<String>, String> {
    auth::token::get_token().await.map_err(|e| e.to_string())
}

/// Saves an authentication token to disk.
///
/// The token is saved with appropriate file permissions and a verification timestamp.
///
/// # Arguments
///
/// * `token` - The authentication token string to save
#[tauri::command]
pub async fn save_token(token: String) -> Result<(), String> {
    auth::token::save_token(token)
        .await
        .map_err(|e| e.to_string())
}

/// Deletes the token file from disk.
///
/// Returns `true` if the file was deleted, `false` if it didn't exist.
#[tauri::command]
pub async fn delete_token() -> Result<bool, String> {
    auth::token::delete_token().await.map_err(|e| e.to_string())
}

/// Verifies an authentication token with the API.
///
/// Checks if the token is valid and if the user is banned.
///
/// # Arguments
///
/// * `token` - The authentication token to verify
///
/// # Returns
///
/// Returns a `VerifyTokenResponse` with success status and user information.
#[tauri::command]
pub async fn verify_token(token: String) -> Result<auth::models::VerifyTokenResponse, String> {
    auth::api::verify_token(&token)
        .await
        .map_err(|e| e.to_string())
}

/// Retrieves user data from the API.
///
/// Gets the full user object from the API.
///
/// # Arguments
///
/// * `token` - The authentication token
///
/// # Returns
///
/// Returns a `GetUserResponse` with success status and user data.
#[tauri::command]
pub async fn get_user(token: String) -> Result<auth::models::GetUserResponse, String> {
    auth::api::get_user(&token).await.map_err(|e| e.to_string())
}

/// Retrieves user statistics from the API.
///
/// Gets the user's stats from the API using the cached/stored token.
///
/// # Returns
///
/// Returns a `GetStatsResponse` with success status and stats data.
#[tauri::command]
pub async fn get_user_stats() -> Result<auth::models::GetStatsResponse, String> {
    let token = auth::token::get_token()
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "No token found".to_string())?;

    auth::api::get_stats(&token)
        .await
        .map_err(|e| e.to_string())
}

/// Retrieves global statistics from the public API.
///
/// Gets global stats from the public API (no authentication required).
///
/// # Returns
///
/// Returns a `GetGlobalStatsResponse` with success status and global stats data.
#[tauri::command]
pub async fn get_global_stats() -> Result<auth::models::GetGlobalStatsResponse, String> {
    auth::api::get_global_stats()
        .await
        .map_err(|e| e.to_string())
}

/// Checks if the API is online and healthy.
///
/// Sends a GET request to the /health endpoint.
///
/// # Returns
///
/// Returns `true` if the API responds with status "ok", `false` otherwise.
#[tauri::command]
pub async fn check_api_status() -> bool {
    auth::api::check_api_status().await
}

/// Launches the proxy process.
///
/// This command checks for updates, downloads if necessary, and starts the proxy.
///
/// # Arguments
///
/// * `app` - The Tauri app handle for emitting events
/// * `manager` - The proxy manager state
/// * `port` - The port number for the proxy (default: 25565)
#[tauri::command]
pub async fn launch_proxy(
    app: AppHandle,
    manager: State<'_, ProxyManager>,
    port: Option<u16>,
) -> Result<(), String> {
    let port = port.unwrap_or(25565);
    manager
        .check_and_launch(app, port)
        .await
        .map_err(|e| e.to_string())
}

/// Stops the proxy process.
///
/// # Arguments
///
/// * `manager` - The proxy manager state
#[tauri::command]
pub async fn stop_proxy(manager: State<'_, ProxyManager>) -> Result<(), String> {
    manager.stop().await.map_err(|e| e.to_string())
}

/// Gets the current proxy status.
///
/// Returns `true` if the proxy is running, `false` otherwise.
///
/// # Arguments
///
/// * `manager` - The proxy manager state
#[tauri::command]
pub async fn get_proxy_status(manager: State<'_, ProxyManager>) -> Result<bool, String> {
    Ok(manager.is_running().await)
}

/// Fetches the list of releases from the API.
///
/// Returns a list of all available releases with their version, assets, and metadata.
///
/// # Returns
///
/// Returns a `Vec<Release>` containing all releases from the API.
#[tauri::command]
pub async fn fetch_releases() -> Result<Vec<models::Release>, String> {
    download::fetch_releases().await.map_err(|e| e.to_string())
}

/// Checks if the legacy configuration file exists.
///
/// Returns `true` if the legacy config file exists, `false` otherwise.
#[tauri::command]
pub async fn legacy_config_exists() -> Result<bool, String> {
    config::manager::legacy_config_exists()
        .await
        .map_err(|e| e.to_string())
}

/// Checks if the configuration file exists.
///
/// Returns `true` if the config file exists, `false` otherwise.
#[tauri::command]
pub async fn config_exists() -> Result<bool, String> {
    config::manager::config_exists()
        .await
        .map_err(|e| e.to_string())
}

/// Reads the legacy configuration file.
///
/// Returns the config if it exists, `None` otherwise.
#[tauri::command]
pub async fn get_legacy_config() -> Result<Option<config::models::Config>, String> {
    config::manager::get_legacy_config()
        .await
        .map_err(|e| e.to_string())
}

/// Reads the configuration file.
///
/// Returns the config if it exists, `None` otherwise.
#[tauri::command]
pub async fn get_config() -> Result<Option<config::models::Config>, String> {
    config::manager::get_config()
        .await
        .map_err(|e| e.to_string())
}

/// Reads a specific key from the legacy configuration file.
///
/// # Arguments
///
/// * `key` - The configuration key to read
///
/// # Returns
///
/// Returns the value if the key exists, `None` otherwise.
#[tauri::command]
pub async fn get_legacy_config_value(key: String) -> Result<Option<serde_json::Value>, String> {
    config::manager::get_legacy_config_value(&key)
        .await
        .map_err(|e| e.to_string())
}

/// Reads a specific key from the configuration file.
///
/// # Arguments
///
/// * `key` - The configuration key to read
///
/// # Returns
///
/// Returns the value if the key exists, `None` otherwise.
#[tauri::command]
pub async fn get_config_value(key: String) -> Result<Option<serde_json::Value>, String> {
    config::manager::get_config_value(&key)
        .await
        .map_err(|e| e.to_string())
}

/// Sets a specific key in the configuration file.
///
/// Also syncs RPC-related settings with the RPC manager automatically.
///
/// # Arguments
///
/// * `key` - The configuration key to set
/// * `value` - The value to set (must be a valid JSON value)
/// * `rpc` - The RPC manager state (for syncing RPC settings)
#[tauri::command]
pub async fn set_config_key(
    key: String,
    value: serde_json::Value,
    rpc: State<'_, RpcManager>,
) -> Result<(), String> {
    config::manager::set_config_key(&key, value.clone())
        .await
        .map_err(|e| e.to_string())?;

    // Sync RPC-related settings with the RPC manager
    match key.as_str() {
        "enableRpc" => {
            if let Some(enabled) = value.as_bool() {
                rpc.set_enabled(enabled);
            }
        }
        "rpcAnonymizeProfile" => {
            if let Some(anonymize) = value.as_bool() {
                rpc.set_anonymize_profile(anonymize);
            }
        }
        "rpcAnonymizeLocation" => {
            if let Some(anonymize) = value.as_bool() {
                rpc.set_anonymize_location(anonymize);
            }
        }
        "rpcImage" => {
            if let Some(image_key) = value.as_str() {
                let _ = rpc.set_image(image_key);
            }
        }
        _ => {}
    }

    Ok(())
}

/// Saves the entire configuration structure to the configuration file.
///
/// # Arguments
///
/// * `config` - The complete configuration structure to write
#[tauri::command]
pub async fn save_config(config: config::models::Config) -> Result<(), String> {
    config::manager::save_config(config)
        .await
        .map_err(|e| e.to_string())
}

// ============================================================================
// Discord RPC Commands
// ============================================================================

/// Enables or disables Discord Rich Presence.
///
/// # Arguments
///
/// * `rpc` - The RPC manager state
/// * `enabled` - Whether to enable or disable RPC
#[tauri::command]
pub fn rpc_set_enabled(rpc: State<'_, RpcManager>, enabled: bool) {
    rpc.set_enabled(enabled);
}

/// Returns whether Discord Rich Presence is enabled.
///
/// # Arguments
///
/// * `rpc` - The RPC manager state
///
/// # Returns
///
/// Returns `true` if RPC is enabled, `false` otherwise.
#[tauri::command]
pub fn rpc_is_enabled(rpc: State<'_, RpcManager>) -> bool {
    rpc.is_enabled()
}

/// Sets the Discord Rich Presence activity.
///
/// # Arguments
///
/// * `rpc` - The RPC manager state
/// * `activity` - The activity type: "launcher", "launching", "playing", or "clear"
/// * `ign` - Optional in-game name (for "playing" activity)
/// * `uuid` - Optional player UUID (for "playing" activity)
#[tauri::command]
pub fn rpc_set_activity(
    rpc: State<'_, RpcManager>,
    activity: String,
    ign: Option<String>,
    uuid: Option<String>,
) {
    match activity.as_str() {
        "launcher" => rpc.set_in_launcher(),
        "launching" => rpc.set_launching(),
        "playing" => rpc.set_playing(ign, uuid),
        "clear" => rpc.clear_activity(),
        _ => {}
    }
}

/// Sets the Discord Rich Presence image.
///
/// # Arguments
///
/// * `rpc` - The RPC manager state
/// * `image_key` - The asset key for the image (e.g., "logo-v1", "logo-diamond")
///
/// # Returns
///
/// * `Ok(())` if the image was set successfully
/// * `Err(String)` if the image key is invalid
///
/// # Valid Image Keys
///
/// - `logo-emerald`
/// - `logo-emerald-plus`
/// - `logo-golden-mark`
/// - `logo-golden-plus`
/// - `logo-green`
/// - `logo-shiny`
/// - `logo-v1`
/// - `logo-v1-purple`
/// - `logo-blue`
/// - `logo-blue-plus`
/// - `logo-white`
/// - `logo-gray`
#[tauri::command]
pub async fn rpc_set_image(rpc: State<'_, RpcManager>, image_key: String) -> Result<(), String> {
    // Validate and set the image in RPC manager
    rpc.set_image(&image_key)?;

    // Persist to config
    config::manager::set_config_key("rpcImage", serde_json::Value::String(image_key))
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Returns the list of valid RPC image keys.
///
/// # Returns
///
/// A vector of valid image key strings.
#[tauri::command]
pub fn rpc_get_valid_image_keys() -> Vec<&'static str> {
    RpcManager::get_valid_image_keys().to_vec()
}
