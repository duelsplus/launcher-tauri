//! Tauri commands for frontend-backend communication.
//!
//! This module exposes authentication functions as Tauri commands that can be
//! invoked from the frontend JavaScript/TypeScript code.

use crate::auth;
use crate::proxy::ProxyManager;
use tauri::{AppHandle, State};

/// Checks if a token file exists on disk.
///
/// This is a Tauri command that can be called from the frontend.
/// Returns `true` if the token file exists, `false` otherwise.
#[tauri::command]
pub async fn token_exists() -> Result<bool, String> {
    auth::token::token_exists().await.map_err(|e| e.to_string())
}

/// Retrieves the stored authentication token from disk.
///
/// This is a Tauri command that can be called from the frontend.
/// Returns `Some(token)` if the token exists, `None` if it doesn't.
#[tauri::command]
pub async fn get_token() -> Result<Option<String>, String> {
    auth::token::get_token().await.map_err(|e| e.to_string())
}

/// Saves an authentication token to disk.
///
/// This is a Tauri command that can be called from the frontend.
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
/// This is a Tauri command that can be called from the frontend.
/// Returns `true` if the file was deleted, `false` if it didn't exist.
#[tauri::command]
pub async fn delete_token() -> Result<bool, String> {
    auth::token::delete_token().await.map_err(|e| e.to_string())
}

/// Verifies an authentication token with the API.
///
/// This is a Tauri command that can be called from the frontend.
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
/// This is a Tauri command that can be called from the frontend.
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
/// This is a Tauri command that can be called from the frontend.
/// Gets the user's stats from the API.
///
/// # Arguments
///
/// * `token` - The authentication token
///
/// # Returns
///
/// Returns a `GetStatsResponse` with success status and stats data.
#[tauri::command]
pub async fn get_stats(token: String) -> Result<auth::models::GetStatsResponse, String> {
    auth::api::get_stats(&token).await.map_err(|e| e.to_string())
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
