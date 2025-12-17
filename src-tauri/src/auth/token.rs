//! Token storage and management operations.
//!
//! This module handles reading, writing, and deleting authentication tokens
//! from the local file system. Tokens are stored in the application's config directory.

use crate::auth::error::AuthError;
use crate::auth::models::TokenData;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

/// Name of the token file stored in the config directory
const TOKEN_FILE: &str = "tokens.json";

/// Application name used for determining the config directory location
const APP_NAME: &str = "duelsplus";

/// Gets the full path to the token file.
///
/// The token file is stored in the application's config directory:
/// - Unix: `~/.config/duelsplus/tokens.json`
/// - Windows: `%APPDATA%\duelsplus\tokens.json`
/// - macOS: `~/Library/Application Support/duelsplus/tokens.json`
///
/// # Returns
///
/// Returns the full path to the token file, or an error if the home directory
/// cannot be determined.
fn get_token_path() -> Result<PathBuf, AuthError> {
    let project_dirs = ProjectDirs::from("", "", APP_NAME)
        .ok_or_else(|| AuthError::Unknown("Failed to get home directory".to_string()))?;

    Ok(project_dirs.config_dir().join(TOKEN_FILE))
}

/// Checks if a token file exists on disk.
///
/// # Returns
///
/// - `Ok(true)` if the token file exists
/// - `Ok(false)` if the token file does not exist
/// - `Err(AuthError)` if there was an error determining the token path
pub async fn token_exists() -> Result<bool, AuthError> {
    let token_path = get_token_path()?;
    Ok(token_path.exists())
}

/// Retrieves the stored authentication token from disk.
///
/// # Returns
///
/// - `Ok(Some(token))` if the token file exists and was successfully read
/// - `Ok(None)` if the token file does not exist
/// - `Err(AuthError)` if there was an error reading or parsing the token file
pub async fn get_token() -> Result<Option<String>, AuthError> {
    let token_path = get_token_path()?;

    if !token_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&token_path)?;
    let token_data: TokenData = serde_json::from_str(&content)?;

    Ok(Some(token_data.token))
}

/// Saves an authentication token to disk.
///
/// Creates the config directory if it doesn't exist, writes the token
/// along with a verification timestamp, and sets appropriate file permissions
/// (0o600 on Unix systems - owner read/write only).
///
/// # Arguments
///
/// * `token` - The authentication token string to save
///
/// # Returns
///
/// - `Ok(())` if the token was successfully saved
/// - `Err(AuthError)` if there was an error creating directories, writing the file, or setting permissions
pub async fn save_token(token: String) -> Result<(), AuthError> {
    let token_path = get_token_path()?;
    let token_dir = token_path
        .parent()
        .ok_or_else(|| AuthError::Unknown("Invalid token path".to_string()))?;

    // Create directory if it doesn't exist
    fs::create_dir_all(token_dir)?;

    // Create token data with current timestamp
    let token_data = TokenData {
        token: token.clone(),
        verified_at: Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        ),
    };

    // Serialize and write to file
    let json = serde_json::to_string_pretty(&token_data)?;
    fs::write(&token_path, json)?;

    // Set file permissions to 0o600 (owner read/write only) on Unix systems
    // This ensures the token file is only accessible by the owner
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&token_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&token_path, perms)?;
    }

    Ok(())
}

/// Deletes the token file from disk.
///
/// # Returns
///
/// - `Ok(true)` if the token file existed and was successfully deleted
/// - `Ok(false)` if the token file did not exist
/// - `Err(AuthError)` if there was an error deleting the file
pub async fn delete_token() -> Result<bool, AuthError> {
    let token_path = get_token_path()?;

    if token_path.exists() {
        fs::remove_file(&token_path)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // Helper function to get the token path for testing
    fn get_test_token_path() -> Result<PathBuf, AuthError> {
        get_token_path()
    }

    // Helper to clean up token file before and after tests
    async fn cleanup_token_file() {
        if let Ok(path) = get_test_token_path() {
            if path.exists() {
                let _ = fs::remove_file(&path);
            }
            // Try to remove the directory if it's empty (ignore errors)
            if let Some(parent) = path.parent() {
                let _ = fs::remove_dir(parent);
            }
        }
    }

    #[tokio::test]
    async fn test_save_and_get_token() {
        // Clean up any existing token file before test
        cleanup_token_file().await;

        let test_token = "test_token_12345".to_string();

        // Save token
        save_token(test_token.clone()).await.unwrap();

        // Verify token exists
        let exists = token_exists().await.unwrap();
        assert!(exists);

        // Get token
        let retrieved_token = get_token().await.unwrap();
        assert_eq!(retrieved_token, Some(test_token));

        // Clean up after test
        cleanup_token_file().await;
    }

    #[tokio::test]
    async fn test_save_token_creates_directory() {
        // Clean up any existing token file
        cleanup_token_file().await;

        let test_token = "test_token_directory_creation".to_string();
        save_token(test_token.clone()).await.unwrap();

        // Verify the directory was created
        let token_path = get_test_token_path().unwrap();
        let token_dir = token_path.parent().unwrap();
        assert!(token_dir.exists());

        // Clean up
        cleanup_token_file().await;
    }

    #[tokio::test]
    async fn test_save_token_includes_timestamp() {
        // Clean up any existing token file before test
        cleanup_token_file().await;

        let test_token = "test_token_with_timestamp".to_string();
        save_token(test_token.clone()).await.unwrap();

        // Read the file directly to verify structure
        let token_path = get_test_token_path().unwrap();
        let content = fs::read_to_string(&token_path).unwrap();
        let token_data: TokenData = serde_json::from_str(&content).unwrap();

        assert_eq!(token_data.token, test_token);
        assert!(token_data.verified_at.is_some());

        // Clean up after test
        cleanup_token_file().await;
    }

    #[tokio::test]
    async fn test_delete_token_when_exists() {
        // Clean up any existing token file
        cleanup_token_file().await;

        let test_token = "test_token_to_delete".to_string();
        save_token(test_token).await.unwrap();

        // Verify it exists
        assert!(token_exists().await.unwrap());

        // Delete token
        let deleted = delete_token().await.unwrap();
        assert!(deleted);

        // Verify it no longer exists
        assert!(!token_exists().await.unwrap());
        let token = get_token().await.unwrap();
        assert_eq!(token, None);
    }

    #[tokio::test]
    async fn test_save_overwrites_existing_token() {
        // Clean up any existing token file before test
        cleanup_token_file().await;

        let first_token = "first_token".to_string();
        let second_token = "second_token".to_string();

        // Save first token
        save_token(first_token.clone()).await.unwrap();
        assert_eq!(get_token().await.unwrap(), Some(first_token));

        // Save second token (should overwrite)
        save_token(second_token.clone()).await.unwrap();
        assert_eq!(get_token().await.unwrap(), Some(second_token));

        // Clean up after test
        cleanup_token_file().await;
    }
}
