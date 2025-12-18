//! Proxy download and update management.

use super::error::ProxyError;
use super::models::{Asset, DownloadProgress, Release};
use directories::ProjectDirs;
use reqwest;
use std::fs;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;

const API_BASE: &str = "https://duelsplus.com/api/releases";
const APP_NAME: &str = "duelsplus";
const MIN_FILE_SIZE_MB: f64 = 50.0;

/// Gets the platform-specific tag for binary selection
pub fn get_platform_tag() -> Result<String, ProxyError> {
    let tag = match std::env::consts::OS {
        "windows" => "win-x64",
        "macos" => "macos-x64",
        "linux" => "linux-x64",
        other => {
            return Err(ProxyError::UnsupportedPlatform(other.to_string()));
        }
    };
    Ok(tag.to_string())
}

/// Gets the installation directory for the proxy
pub fn get_install_dir() -> Result<PathBuf, ProxyError> {
    let project_dirs = ProjectDirs::from("", "", APP_NAME)
        .ok_or_else(|| ProxyError::Unknown("Failed to get home directory".to_string()))?;

    let install_dir = project_dirs.data_dir().join("proxy");
    Ok(install_dir)
}

/// Fetches the list of releases from the API
pub async fn fetch_releases() -> Result<Vec<Release>, ProxyError> {
    let client = reqwest::Client::new();
    let response = client.get(API_BASE).send().await?;

    if !response.status().is_success() {
        return Err(ProxyError::Network(
            response.error_for_status().unwrap_err(),
        ));
    }

    let releases: Vec<Release> = response.json().await?;
    Ok(releases)
}

/// Finds the latest release
pub fn find_latest_release(releases: &[Release]) -> Result<&Release, ProxyError> {
    releases
        .iter()
        .find(|r| r.is_latest)
        .ok_or(ProxyError::NoReleaseFound)
}

/// Finds the asset for the current platform
pub fn find_platform_asset<'a>(
    release: &'a Release,
    platform_tag: &str,
) -> Result<&'a Asset, ProxyError> {
    release
        .assets
        .iter()
        .find(|a| a.name.contains(platform_tag))
        .ok_or_else(|| ProxyError::NoAssetFound(platform_tag.to_string()))
}

/// Checks if the file exists and is valid (not corrupted/incomplete)
pub fn is_file_valid(path: &PathBuf) -> bool {
    if !path.exists() {
        return false;
    }

    if let Ok(metadata) = fs::metadata(path) {
        let size_mb = metadata.len() as f64 / (1024.0 * 1024.0);
        size_mb >= MIN_FILE_SIZE_MB
    } else {
        false
    }
}

/// Downloads an artifact with progress tracking
pub async fn download_artifact<F>(
    asset_id: &str,
    dest_path: &PathBuf,
    mut progress_callback: F,
) -> Result<(), ProxyError>
where
    F: FnMut(DownloadProgress),
{
    let url = format!("{}/artifact?assetId={}", API_BASE, asset_id);
    let client = reqwest::Client::new();

    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(ProxyError::Network(
            response.error_for_status().unwrap_err(),
        ));
    }

    let total_size = response.content_length().unwrap_or(0);

    // Create parent directory if it doesn't exist
    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = tokio::fs::File::create(dest_path).await?;
    let mut downloaded: u64 = 0;
    let start_time = std::time::Instant::now();

    let mut stream = response.bytes_stream();
    use futures_util::StreamExt;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;

        let elapsed = start_time.elapsed().as_secs_f64();
        let speed = if elapsed > 0.0 {
            downloaded as f64 / elapsed
        } else {
            0.0
        };

        progress_callback(DownloadProgress {
            downloaded,
            total: total_size,
            speed,
        });
    }

    file.flush().await?;

    // Set executable permissions on Unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(dest_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(dest_path, perms)?;
    }

    Ok(())
}

/// Cleans up old executables in the install directory
pub fn cleanup_old_executables(
    install_dir: &PathBuf,
    current_file: &str,
) -> Result<(), ProxyError> {
    if !install_dir.exists() {
        return Ok(());
    }

    let entries = fs::read_dir(install_dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                let file_name_str = file_name.to_string_lossy();

                // On Windows, delete old .exe files
                #[cfg(windows)]
                if file_name_str.ends_with(".exe") && file_name_str != current_file {
                    let _ = fs::remove_file(&path); // Ignore errors
                }

                // On Unix, delete old executables (no extension or matching pattern)
                #[cfg(unix)]
                if file_name_str != current_file && !file_name_str.contains(".") {
                    let _ = fs::remove_file(&path); // Ignore errors
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_find_latest_release_none() {
        let releases = vec![Release {
            id: "1".to_string(),
            version: "1.0.0".to_string(),
            release_date: "2025-01-01T00:00:00Z".to_string(),
            is_beta: false,
            is_latest: false,
            changelog: "".to_string(),
            whats_new: vec![],
            assets: vec![],
        }];

        let result = find_latest_release(&releases);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ProxyError::NoReleaseFound));
    }

    #[test]
    fn test_find_platform_asset_not_found() {
        let release = Release {
            id: "1".to_string(),
            version: "1.0.0".to_string(),
            release_date: "2025-01-01T00:00:00Z".to_string(),
            is_beta: false,
            is_latest: true,
            changelog: "".to_string(),
            whats_new: vec![],
            assets: vec![Asset {
                id: "1".to_string(),
                name: "proxy-win-x64.exe".to_string(),
                url: "https://example.com/proxy.exe".to_string(),
            }],
        };

        let result = find_platform_asset(&release, "unsupported-platform");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ProxyError::NoAssetFound(_)));
    }

    #[test]
    fn test_cleanup_old_executables() {
        let temp_dir = TempDir::new().unwrap();
        let install_dir = temp_dir.path().to_path_buf();

        // Create several executable files
        #[cfg(windows)]
        {
            fs::write(install_dir.join("old_proxy_v1.exe"), "test").unwrap();
            fs::write(install_dir.join("old_proxy_v2.exe"), "test").unwrap();
            fs::write(install_dir.join("current_proxy.exe"), "test").unwrap();
            fs::write(install_dir.join("readme.txt"), "test").unwrap();
        }

        #[cfg(unix)]
        {
            fs::write(install_dir.join("old_proxy_v1"), "test").unwrap();
            fs::write(install_dir.join("old_proxy_v2"), "test").unwrap();
            fs::write(install_dir.join("current_proxy"), "test").unwrap();
            fs::write(install_dir.join("readme.txt"), "test").unwrap();
        }

        // Clean up old executables, keeping current_proxy
        #[cfg(windows)]
        cleanup_old_executables(&install_dir, "current_proxy.exe").unwrap();

        #[cfg(unix)]
        cleanup_old_executables(&install_dir, "current_proxy").unwrap();

        // Check that old executables are gone but current one remains
        #[cfg(windows)]
        {
            assert!(!install_dir.join("old_proxy_v1.exe").exists());
            assert!(!install_dir.join("old_proxy_v2.exe").exists());
            assert!(install_dir.join("current_proxy.exe").exists());
            assert!(install_dir.join("readme.txt").exists()); // Non-exe files kept
        }

        #[cfg(unix)]
        {
            assert!(!install_dir.join("old_proxy_v1").exists());
            assert!(!install_dir.join("old_proxy_v2").exists());
            assert!(install_dir.join("current_proxy").exists());
            assert!(install_dir.join("readme.txt").exists()); // Files with extensions kept
        }
    }
}
