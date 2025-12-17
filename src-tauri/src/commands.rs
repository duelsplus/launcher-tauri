//! Tauri commands for frontend-backend communication.
//!
//! This module exposes authentication functions as Tauri commands that can be
//! invoked from the frontend JavaScript/TypeScript code.

use crate::auth;

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
