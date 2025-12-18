//! Utility functions for common operations.

use std::path::PathBuf;

/// Gets the home directory path for the current user.
///
/// On Windows, tries `USERPROFILE` first, then falls back to `HOME`.
/// On Unix systems, uses `HOME`.
///
/// # Returns
///
/// Returns the home directory path, or an error message if it cannot be determined.
pub fn get_home_dir() -> Result<PathBuf, String> {
    let home_dir = if cfg!(windows) {
        std::env::var("USERPROFILE")
            .map(PathBuf::from)
            .or_else(|_| std::env::var("HOME").map(PathBuf::from))
    } else {
        std::env::var("HOME").map(PathBuf::from)
    }
    .map_err(|_| "Failed to get home directory".to_string())?;

    Ok(home_dir)
}

