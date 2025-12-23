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
