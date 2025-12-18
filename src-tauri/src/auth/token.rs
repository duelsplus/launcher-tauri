//! Token storage and management operations.
//!
//! This module handles reading, writing, and deleting authentication tokens
//! from the local file system. Tokens are stored in the application's config directory.

use crate::auth::error::AuthError;
use crate::auth::models::TokenData;
use crate::utils;
use std::fs;
use std::path::PathBuf;

#[cfg(test)]
use std::sync::Mutex;

#[cfg(test)]
pub(crate) static TEST_TOKEN_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);

#[cfg(test)]
pub(crate) static TEST_LOCK: Mutex<()> = Mutex::new(());

/// Name of the token file stored in the config directory
const TOKEN_FILE: &str = "tokens.json";

/// Gets the full path to the token file.
///
/// The token file is stored in the home directory:
/// - All platforms: `~/.duelsplus/tokens.json`
///
/// # Returns
///
/// Returns the full path to the token file, or an error if the home directory
/// cannot be determined.
fn get_token_path() -> Result<PathBuf, AuthError> {
    #[cfg(test)]
    {
        // Handle poisoned mutex by recovering from it
        if let Some(test_dir) = TEST_TOKEN_DIR
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
        {
            return Ok(test_dir.join(TOKEN_FILE));
        }
    }

    let home_dir = utils::get_home_dir().map_err(|e| AuthError::Unknown(e))?;
    Ok(home_dir.join(".duelsplus").join(TOKEN_FILE))
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
    use std::sync::Mutex;
    use tempfile::TempDir;

    // Global lock to prevent tests from running in parallel
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    // Helper to set up isolated test environment
    struct TestContext {
        _temp_dir: TempDir,
        _lock: std::sync::MutexGuard<'static, ()>,
    }

    impl TestContext {
        fn new() -> Self {
            // Acquire global test lock to serialize tests
            // Handle poisoned mutex by recovering from it
            let lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());

            let temp_dir = TempDir::new().unwrap();
            let temp_path = temp_dir.path().to_path_buf();

            // Set the test token directory for this test
            // Handle poisoned mutex by recovering from it
            *TEST_TOKEN_DIR.lock().unwrap_or_else(|e| e.into_inner()) = Some(temp_path);

            Self {
                _temp_dir: temp_dir,
                _lock: lock,
            }
        }
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            // Clear the test token directory
            // Handle poisoned mutex by recovering from it
            *TEST_TOKEN_DIR.lock().unwrap_or_else(|e| e.into_inner()) = None;
            // Lock is automatically released when _lock is dropped
        }
    }

    #[tokio::test]
    async fn test_save_and_get_token() {
        let _ctx = TestContext::new();

        let test_token = "test_token_12345".to_string();

        // Save token
        save_token(test_token.clone()).await.unwrap();

        // Verify token exists
        let exists = token_exists().await.unwrap();
        assert!(exists);

        // Get token
        let retrieved_token = get_token().await.unwrap();
        assert_eq!(retrieved_token, Some(test_token));
    }

    #[tokio::test]
    async fn test_save_token_creates_directory() {
        let _ctx = TestContext::new();

        let test_token = "test_token_directory_creation".to_string();
        save_token(test_token.clone()).await.unwrap();

        // Verify the directory was created
        let token_path = get_token_path().unwrap();
        let token_dir = token_path.parent().unwrap();
        assert!(token_dir.exists());
    }

    #[tokio::test]
    async fn test_save_token_includes_timestamp() {
        let _ctx = TestContext::new();

        let test_token = "test_token_with_timestamp".to_string();
        save_token(test_token.clone()).await.unwrap();

        // Read the file directly to verify structure
        let token_path = get_token_path().unwrap();
        let content = fs::read_to_string(&token_path).unwrap();
        let token_data: TokenData = serde_json::from_str(&content).unwrap();

        assert_eq!(token_data.token, test_token);
        assert!(token_data.verified_at.is_some());
    }

    #[tokio::test]
    async fn test_delete_token_when_exists() {
        let _ctx = TestContext::new();

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
    async fn test_real_token_path_exists() {
        // This test uses the real token location (NOT isolated)
        // It only checks if the path can be retrieved, without reading/writing
        // This ensures we don't corrupt any existing token

        // Temporarily clear the test directory to use real path
        // Handle poisoned mutex by recovering from it
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        *TEST_TOKEN_DIR.lock().unwrap_or_else(|e| e.into_inner()) = None;

        // Should be able to get the real token path
        let token_path_result = get_token_path();
        assert!(
            token_path_result.is_ok(),
            "Should be able to get real token path"
        );

        let token_path = token_path_result.unwrap();

        // Verify it's in the expected location (contains app name)
        let path_str = token_path.to_string_lossy();
        assert!(
            path_str.contains("duelsplus"),
            "Token path should contain app name: {}",
            path_str
        );

        // Verify the filename is correct
        assert!(
            token_path.ends_with("tokens.json"),
            "Token path should end with tokens.json: {}",
            path_str
        );

        // Check if token exists (read-only, safe)
        let exists = token_path.exists();
        println!("Real token file exists: {}", exists);

        // If token exists, verify it's readable and valid JSON (read-only, safe)
        if exists {
            let read_result = std::fs::read_to_string(&token_path);
            assert!(
                read_result.is_ok(),
                "Should be able to read existing token file"
            );

            if let Ok(content) = read_result {
                println!("Token file content:\n{}", content);

                let parse_result = serde_json::from_str::<TokenData>(&content);
                assert!(
                    parse_result.is_ok(),
                    "Existing token file should be valid JSON"
                );

                if let Ok(token_data) = parse_result {
                    println!("\nParsed token data:");
                    println!("  Token length: {} characters", token_data.token.len());
                    println!(
                        "  Token preview: {}...",
                        if token_data.token.len() > 20 {
                            &token_data.token[..20]
                        } else {
                            &token_data.token
                        }
                    );

                    if let Some(verified_at) = token_data.verified_at {
                        use std::time::{Duration, UNIX_EPOCH};
                        let timestamp = UNIX_EPOCH + Duration::from_secs(verified_at);
                        println!("  Verified at: {:?}", timestamp);
                        println!("  Verified at (unix): {}", verified_at);
                    } else {
                        println!("  Verified at: None");
                    }
                }
            }
        }

        // Note: We do NOT call save_token or delete_token here
        // This is intentionally read-only to avoid corrupting real data
    }

    #[tokio::test]
    async fn test_invalid_json_token_file() {
        let _ctx = TestContext::new();

        // Create a token file with invalid JSON
        let token_path = get_token_path().unwrap();
        let token_dir = token_path.parent().unwrap();
        fs::create_dir_all(token_dir).unwrap();
        fs::write(&token_path, "{ invalid json }").unwrap();

        // Reading should return an error
        let result = get_token().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("JSON parse error"));
    }

    #[tokio::test]
    async fn test_get_token_when_file_missing() {
        let _ctx = TestContext::new();

        // Token file doesn't exist
        let token = get_token().await.unwrap();
        assert_eq!(token, None);
    }
}
