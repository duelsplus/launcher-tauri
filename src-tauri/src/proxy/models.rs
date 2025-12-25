//! Data models for proxy operations.

use serde::{Deserialize, Serialize};

/// Release information from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    pub id: String,
    pub version: String,
    pub release_date: String,
    pub is_beta: bool,
    pub is_latest: bool,
    pub changelog: String,
    pub whats_new: Vec<String>,
    pub assets: Vec<Asset>,
}

/// Asset information (downloadable binary)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub id: String,
    pub name: String,
    pub url: String,
}

/// Download progress information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadProgress {
    pub downloaded: u64,
    pub total: u64,
    /// Bytes per second
    pub speed: f64,
}

/// Proxy status information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "status")]
pub enum ProxyStatus {
    Checking,
    Downloading { version: String },
    Launching,
    Launched,
    Error,
}

/// User data extracted from proxy logs for RPC
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcUserData {
    pub ign: String,
    pub uuid: String,
}

/// Error severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Error categories for grouping
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ErrorCategory {
    Network,
    Authentication,
    Hypixel,
    Proxy,
    Api,
    Unknown,
}

/// User-friendly proxy error sent from the proxy to the launcher
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyErrorData {
    /// Error code (e.g., "ECONNRESET", "AUTH_EXPIRED")
    pub code: String,
    /// User-friendly error title
    pub title: String,
    /// User-friendly error description
    pub message: String,
    /// Suggested action for the user
    pub suggestion: String,
    /// Severity level of the error
    pub severity: ErrorSeverity,
    /// Category of the error for grouping
    pub category: ErrorCategory,
    /// Original technical error message
    pub original_message: String,
    /// Context where the error occurred (e.g., "authentication", "hypixel_connection")
    pub context: Option<String>,
    /// Timestamp when the error occurred
    pub timestamp: u64,
}
