//! Error types for authentication operations.

use thiserror::Error;

/// Authentication-related errors.
///
/// This enum covers all possible errors that can occur during
/// authentication operations, including I/O, JSON parsing, and network errors.
#[derive(Debug, Error)]
pub enum AuthError {
    /// File system I/O error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization error
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    /// Network request error
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// Token file not found when attempting to read
    #[error("Token not found")]
    TokenNotFound,

    /// Generic error for unexpected situations
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl serde::Serialize for AuthError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_io_error_display() {
        let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "Permission denied");
        let auth_error: AuthError = io_error.into();

        let error_string = auth_error.to_string();
        assert!(error_string.contains("IO error"));
        assert!(error_string.contains("Permission denied"));
    }

    #[test]
    fn test_json_error_display() {
        let json_error = serde_json::from_str::<serde_json::Value>("{invalid}").unwrap_err();
        let auth_error: AuthError = json_error.into();

        let error_string = auth_error.to_string();
        assert!(error_string.contains("JSON parse error"));
    }
}
