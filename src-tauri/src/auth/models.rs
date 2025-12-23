//! Data models for authentication and API responses.

use serde::{Deserialize, Serialize};

/// User data structure returned from the API.
///
/// This is a minimal representation of the user object.
/// The API may return additional fields that are not included here,
/// but they will be preserved in the raw JSON when serialized.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// Unique user identifier
    pub id: String,
    /// User's username
    pub username: String,
    /// Whether the user is banned (optional, defaults to false if not present)
    #[serde(default)]
    pub is_banned: Option<bool>,
}

/// Token storage structure saved to disk.
///
/// Contains the authentication token and metadata about when it was verified.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenData {
    /// The authentication token string
    pub token: String,
    /// Unix timestamp (seconds since epoch) when the token was last verified
    #[serde(default)]
    pub verified_at: Option<u64>,
}

/// Response structure for token verification operations.
///
/// Returns success status, error codes, and user information if successful.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyTokenResponse {
    /// Whether the verification was successful
    pub success: bool,
    /// Error code if verification failed (can be string like "banned" or HTTP status code)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<VerifyCode>,
    /// User ID if verification succeeded
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Username if verification succeeded
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Error message if verification failed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Raw user data from the API response
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw: Option<serde_json::Value>,
}

/// Error code for token verification failures.
///
/// Can be either a string or an HTTP status code.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VerifyCode {
    /// String error code
    String(String),
    /// HTTP status code
    Number(u16),
}

/// Response structure for get user operations.
///
/// Returns success status, error codes, and user data if successful.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserResponse {
    /// Whether the request was successful
    pub success: bool,
    /// Error code if request failed (can be string or HTTP status code)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<GetUserCode>,
    /// User data as JSON if request succeeded
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// Error message if request failed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Error code for get user operation failures.
///
/// Can be either a string error code or an HTTP status code.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserCode {
    /// String error code
    String(String),
    /// HTTP status code
    Number(u16),
}

/// Response from the get stats endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStatsResponse {
    /// Whether the request was successful
    pub success: bool,
    /// Optional error code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<GetUserCode>,
    /// Stats data if successful
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<serde_json::Value>,
    /// Error message if any
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Response from the get global stats endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGlobalStatsResponse {
    /// Whether the request was successful
    pub success: bool,
    /// Optional error code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<GetUserCode>,
    /// Global stats data if successful
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// Error message if any
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
