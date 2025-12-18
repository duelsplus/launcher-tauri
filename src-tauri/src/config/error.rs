//! Error types for configuration operations.

use thiserror::Error;

/// Configuration-related errors.
///
/// This enum covers all possible errors that can occur during
/// configuration operations, including I/O and JSON parsing errors.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// File system I/O error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization error
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    /// Generic error for unexpected situations
    #[error("Unknown error: {0}")]
    Unknown(String),

    /// Configuration key not found
    #[error("Key not found: {0}")]
    #[allow(dead_code)]
    KeyNotFound(String),
}

impl serde::Serialize for ConfigError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
