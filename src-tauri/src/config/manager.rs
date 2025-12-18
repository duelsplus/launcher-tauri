//! Configuration file management operations.
//!
//! This module handles reading, writing, and managing configuration files
//! from both legacy and new locations.

use crate::config::error::ConfigError;
use crate::config::models::Config;
use crate::utils;
use std::fs;
use std::path::PathBuf;

#[cfg(test)]
use std::sync::Mutex;

#[cfg(test)]
pub(crate) static TEST_CONFIG_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);

#[cfg(test)]
pub(crate) static TEST_LEGACY_CONFIG_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);

/// Name of the configuration file
const CONFIG_FILE: &str = "config.json";

/// Gets the full path to the legacy configuration file.
///
/// Legacy config file locations:
/// - Linux: `~/.config/Duels+/config.json`
/// - macOS: `~/Library/Application Support/Duels+/config.json`
/// - Windows: `%APPDATA%\Duels+\config.json`
///
/// # Returns
///
/// Returns the full path to the legacy config file, or an error if the
/// directory cannot be determined.
fn get_legacy_config_path() -> Result<PathBuf, ConfigError> {
    #[cfg(test)]
    {
        if let Some(test_dir) = TEST_LEGACY_CONFIG_DIR
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
        {
            return Ok(test_dir.join(CONFIG_FILE));
        }
    }

    let config_path = if cfg!(target_os = "linux") {
        // Linux: ~/.config/Duels+/config.json
        let home_dir = utils::get_home_dir().map_err(|e| ConfigError::Unknown(e))?;
        home_dir.join(".config").join("Duels+").join(CONFIG_FILE)
    } else if cfg!(target_os = "macos") {
        // macOS: ~/Library/Application Support/Duels+/config.json
        let home_dir = utils::get_home_dir().map_err(|e| ConfigError::Unknown(e))?;
        home_dir
            .join("Library")
            .join("Application Support")
            .join("Duels+")
            .join(CONFIG_FILE)
    } else if cfg!(windows) {
        // Windows: %APPDATA%\Duels+\config.json
        let appdata = std::env::var("APPDATA")
            .map(PathBuf::from)
            .map_err(|_| ConfigError::Unknown("Failed to get APPDATA directory".to_string()))?;
        appdata.join("Duels+").join(CONFIG_FILE)
    } else {
        return Err(ConfigError::Unknown("Unsupported platform".to_string()));
    };

    Ok(config_path)
}

/// Gets the full path to the configuration file.
///
/// Config file location:
/// - All platforms: `~/.config/duelsplus/config.json`
///
/// # Returns
///
/// Returns the full path to the config file, or an error if the
/// directory cannot be determined.
fn get_config_path() -> Result<PathBuf, ConfigError> {
    #[cfg(test)]
    {
        if let Some(test_dir) = TEST_CONFIG_DIR
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
        {
            return Ok(test_dir.join(CONFIG_FILE));
        }
    }

    let home_dir = utils::get_home_dir().map_err(|e| ConfigError::Unknown(e))?;

    Ok(home_dir.join(".config").join("duelsplus").join(CONFIG_FILE))
}

/// Checks if the legacy configuration file exists.
///
/// # Returns
///
/// - `Ok(true)` if the legacy config file exists
/// - `Ok(false)` if the legacy config file does not exist
/// - `Err(ConfigError)` if there was an error determining the config path
pub async fn legacy_config_exists() -> Result<bool, ConfigError> {
    let config_path = get_legacy_config_path()?;
    Ok(config_path.exists())
}

/// Checks if the configuration file exists.
///
/// # Returns
///
/// - `Ok(true)` if the config file exists
/// - `Ok(false)` if the config file does not exist
/// - `Err(ConfigError)` if there was an error determining the config path
pub async fn config_exists() -> Result<bool, ConfigError> {
    let config_path = get_config_path()?;
    Ok(config_path.exists())
}

/// Reads the legacy configuration file.
///
/// # Returns
///
/// - `Ok(Some(config))` if the config file exists and was successfully read
/// - `Ok(None)` if the config file does not exist
/// - `Err(ConfigError)` if there was an error reading or parsing the config file
pub async fn get_legacy_config() -> Result<Option<Config>, ConfigError> {
    let config_path = get_legacy_config_path()?;

    if !config_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&config_path)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(Some(config))
}

/// Reads the configuration file.
///
/// # Returns
///
/// - `Ok(Some(config))` if the config file exists and was successfully read
/// - `Ok(None)` if the config file does not exist
/// - `Err(ConfigError)` if there was an error reading or parsing the config file
pub async fn get_config() -> Result<Option<Config>, ConfigError> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&config_path)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(Some(config))
}

/// Reads a specific key from the legacy configuration file.
///
/// # Arguments
///
/// * `key` - The configuration key to read (e.g., "minimizeToTray", "autoUpdate")
///
/// # Returns
///
/// - `Ok(Some(value))` if the key exists and was successfully read
/// - `Ok(None)` if the config file or key does not exist
/// - `Err(ConfigError)` if there was an error reading or parsing the config file
pub async fn get_legacy_config_value(key: &str) -> Result<Option<serde_json::Value>, ConfigError> {
    let config = get_legacy_config().await?;

    if let Some(config) = config {
        let json_value = serde_json::to_value(&config)?;
        if let Some(value) = json_value.get(key) {
            return Ok(Some(value.clone()));
        }
    }

    Ok(None)
}

/// Reads a specific key from the configuration file.
///
/// # Arguments
///
/// * `key` - The configuration key to read (e.g., "minimizeToTray", "autoUpdate")
///
/// # Returns
///
/// - `Ok(Some(value))` if the key exists and was successfully read
/// - `Ok(None)` if the config file or key does not exist
/// - `Err(ConfigError)` if there was an error reading or parsing the config file
pub async fn get_config_value(key: &str) -> Result<Option<serde_json::Value>, ConfigError> {
    let config = get_config().await?;

    if let Some(config) = config {
        let json_value = serde_json::to_value(&config)?;
        if let Some(value) = json_value.get(key) {
            return Ok(Some(value.clone()));
        }
    }

    Ok(None)
}

/// Sets a specific key in the configuration file.
///
/// This function reads the existing config (or creates a default one),
/// updates the specified key, and writes it back to the file.
///
/// # Arguments
///
/// * `key` - The configuration key to set (e.g., "minimizeToTray", "autoUpdate")
/// * `value` - The value to set (must be a valid JSON value)
///
/// # Returns
///
/// - `Ok(())` if the key was successfully set
/// - `Err(ConfigError)` if there was an error reading, updating, or writing the config file
pub async fn set_config_key(key: &str, value: serde_json::Value) -> Result<(), ConfigError> {
    let config_path = get_config_path()?;
    let config_dir = config_path
        .parent()
        .ok_or_else(|| ConfigError::Unknown("Invalid config path".to_string()))?;

    // Create directory if it doesn't exist
    fs::create_dir_all(config_dir)?;

    // Read existing config or use default
    let mut config = get_config().await?.unwrap_or_default();

    // Update the config with the new value
    let mut json_value = serde_json::to_value(&config)?;
    json_value[key] = value;

    // Deserialize back to Config to validate
    config = serde_json::from_value(json_value)?;

    // Serialize and write to file
    let json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, json)?;

    // Set file permissions to 0o600 (owner read/write only) on Unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&config_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&config_path, perms)?;
    }

    Ok(())
}

/// Saves the entire configuration structure to the configuration file.
///
/// This function writes the complete config structure to the file,
/// overwriting any existing configuration.
///
/// # Arguments
///
/// * `config` - The complete configuration structure to write
///
/// # Returns
///
/// - `Ok(())` if the config was successfully written
/// - `Err(ConfigError)` if there was an error writing the config file
pub async fn save_config(config: Config) -> Result<(), ConfigError> {
    let config_path = get_config_path()?;
    let config_dir = config_path
        .parent()
        .ok_or_else(|| ConfigError::Unknown("Invalid config path".to_string()))?;

    // Create directory if it doesn't exist
    fs::create_dir_all(config_dir)?;

    // Serialize and write to file
    let json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, json)?;

    // Set file permissions to 0o600 (owner read/write only) on Unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&config_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&config_path, perms)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // Global lock to prevent tests from running in parallel
    static TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    // Helper to set up isolated test environment
    struct TestContext {
        _temp_dir: TempDir,
        _temp_legacy_dir: TempDir,
        _lock: std::sync::MutexGuard<'static, ()>,
    }

    impl TestContext {
        fn new() -> Self {
            // Acquire global test lock to serialize tests
            let lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());

            let temp_dir = TempDir::new().unwrap();
            let temp_legacy_dir = TempDir::new().unwrap();
            let temp_path = temp_dir.path().to_path_buf();
            let temp_legacy_path = temp_legacy_dir.path().to_path_buf();

            // Set the test config directories for this test
            *TEST_CONFIG_DIR.lock().unwrap_or_else(|e| e.into_inner()) = Some(temp_path);
            *TEST_LEGACY_CONFIG_DIR
                .lock()
                .unwrap_or_else(|e| e.into_inner()) = Some(temp_legacy_path);

            Self {
                _temp_dir: temp_dir,
                _temp_legacy_dir: temp_legacy_dir,
                _lock: lock,
            }
        }
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            // Clear the test config directories
            *TEST_CONFIG_DIR.lock().unwrap_or_else(|e| e.into_inner()) = None;
            *TEST_LEGACY_CONFIG_DIR
                .lock()
                .unwrap_or_else(|e| e.into_inner()) = None;
        }
    }

    #[tokio::test]
    async fn test_legacy_config_exists_not_exists() {
        let _ctx = TestContext::new();
        let exists = legacy_config_exists().await.unwrap();
        assert!(!exists);
    }

    #[tokio::test]
    async fn test_config_exists_not_exists() {
        let _ctx = TestContext::new();
        let exists = config_exists().await.unwrap();
        assert!(!exists);
    }

    #[tokio::test]
    async fn test_save_and_get_config() {
        let _ctx = TestContext::new();

        let config = Config::default();
        save_config(config.clone()).await.unwrap();

        // Verify config exists
        assert!(config_exists().await.unwrap());

        // Read config
        let read_config = get_config().await.unwrap();
        assert!(read_config.is_some());
        let read_config = read_config.unwrap();

        assert_eq!(read_config.minimizeToTray, config.minimizeToTray);
        assert_eq!(read_config.autoUpdate, config.autoUpdate);
        assert_eq!(read_config.proxyPort, config.proxyPort);
    }

    #[tokio::test]
    async fn test_set_config_key() {
        let _ctx = TestContext::new();

        // Set a key
        set_config_key("minimizeToTray", serde_json::json!(true))
            .await
            .unwrap();

        // Read the key
        let value = get_config_value("minimizeToTray").await.unwrap();
        assert_eq!(value, Some(serde_json::json!(true)));

        // Verify the full config was updated
        let config = get_config().await.unwrap().unwrap();
        assert!(config.minimizeToTray);
    }

    #[tokio::test]
    async fn test_get_legacy_config_value() {
        let _ctx = TestContext::new();

        // Create a legacy config file manually
        let legacy_path = get_legacy_config_path().unwrap();
        let legacy_dir = legacy_path.parent().unwrap();
        fs::create_dir_all(legacy_dir).unwrap();

        let config = Config {
            minimizeToTray: true,
            autoUpdate: false,
            ..Default::default()
        };
        let json = serde_json::to_string_pretty(&config).unwrap();
        fs::write(&legacy_path, json).unwrap();

        // Read a key
        let value = get_legacy_config_value("minimizeToTray").await.unwrap();
        assert_eq!(value, Some(serde_json::json!(true)));

        let value = get_legacy_config_value("autoUpdate").await.unwrap();
        assert_eq!(value, Some(serde_json::json!(false)));
    }

    #[tokio::test]
    async fn test_set_config_key_preserves_other_keys() {
        let _ctx = TestContext::new();

        // Save initial config with multiple values
        let initial_config = Config {
            minimizeToTray: false,
            autoUpdate: true,
            proxyPort: "25565".to_string(),
            enableRpc: true,
            ..Default::default()
        };
        save_config(initial_config.clone()).await.unwrap();

        // Update only one key
        set_config_key("minimizeToTray", serde_json::json!(true))
            .await
            .unwrap();

        // Verify the updated key changed
        let config = get_config().await.unwrap().unwrap();
        assert!(config.minimizeToTray);

        // Verify other keys were preserved
        assert_eq!(config.autoUpdate, initial_config.autoUpdate);
        assert_eq!(config.proxyPort, initial_config.proxyPort);
        assert_eq!(config.enableRpc, initial_config.enableRpc);
    }

    #[tokio::test]
    async fn test_set_config_key_multiple_sequential_updates() {
        let _ctx = TestContext::new();

        // Set first key
        set_config_key("minimizeToTray", serde_json::json!(true))
            .await
            .unwrap();

        // Set second key
        set_config_key("autoUpdate", serde_json::json!(false))
            .await
            .unwrap();

        // Set third key
        set_config_key("proxyPort", serde_json::json!("8080"))
            .await
            .unwrap();

        // Verify all keys are set correctly
        let config = get_config().await.unwrap().unwrap();
        assert!(config.minimizeToTray);
        assert!(!config.autoUpdate);
        assert_eq!(config.proxyPort, "8080");
    }

    #[tokio::test]
    async fn test_get_config_value_nonexistent_key() {
        let _ctx = TestContext::new();

        // Save a config
        save_config(Config::default()).await.unwrap();

        // Try to get a key that doesn't exist
        let value = get_config_value("nonexistentKey").await.unwrap();
        assert_eq!(value, None);
    }

    #[tokio::test]
    async fn test_invalid_json_handling() {
        let _ctx = TestContext::new();

        // Create a config file with invalid JSON
        let config_path = get_config_path().unwrap();
        let config_dir = config_path.parent().unwrap();
        fs::create_dir_all(config_dir).unwrap();
        fs::write(&config_path, "{ invalid json }").unwrap();

        // Reading should return an error
        let result = get_config().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("JSON parse error"));
    }

    #[tokio::test]
    async fn test_migrate_from_legacy_to_new_config() {
        let _ctx = TestContext::new();

        // Create a legacy config
        let legacy_path = get_legacy_config_path().unwrap();
        let legacy_dir = legacy_path.parent().unwrap();
        fs::create_dir_all(legacy_dir).unwrap();

        let legacy_config = Config {
            minimizeToTray: true,
            autoUpdate: false,
            proxyPort: "25565".to_string(),
            enableRpc: true,
            ..Default::default()
        };
        let json = serde_json::to_string_pretty(&legacy_config).unwrap();
        fs::write(&legacy_path, json).unwrap();

        // Read from legacy
        let read_legacy = get_legacy_config().await.unwrap().unwrap();

        // Save to new config
        save_config(read_legacy.clone()).await.unwrap();

        // Verify new config matches legacy
        let new_config = get_config().await.unwrap().unwrap();
        assert_eq!(new_config.minimizeToTray, read_legacy.minimizeToTray);
        assert_eq!(new_config.autoUpdate, read_legacy.autoUpdate);
        assert_eq!(new_config.proxyPort, read_legacy.proxyPort);
        assert_eq!(new_config.enableRpc, read_legacy.enableRpc);
    }
}
