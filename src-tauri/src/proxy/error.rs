//! Error types for proxy operations.

use thiserror::Error;

/// Proxy-related errors.
#[derive(Debug, Error)]
pub enum ProxyError {
    /// File system I/O error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Network request error
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    /// Proxy process error
    #[error("Proxy process error: {0}")]
    ProcessError(String),

    /// Proxy already running
    #[error("Proxy is already running")]
    AlreadyRunning,

    /// Proxy not running
    #[error("Proxy is not running")]
    NotRunning,

    /// Unsupported platform
    #[error("Unsupported platform: {0}")]
    UnsupportedPlatform(String),

    /// No release found
    #[error("No release found")]
    NoReleaseFound,

    /// No asset found for platform
    #[error("No asset found for platform: {0}")]
    NoAssetFound(String),

    /// Generic error for unexpected situations
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl serde::Serialize for ProxyError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
