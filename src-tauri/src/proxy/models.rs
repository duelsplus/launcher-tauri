//! Data models for proxy operations.

use serde::{Deserialize, Serialize};

/// Release information from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    pub version: String,
    pub is_latest: bool,
    pub assets: Vec<Asset>,
}

/// Asset information (downloadable binary)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub name: String,
}

/// Download progress information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub downloaded: u64,
    pub total: u64,
    pub speed: f64, // bytes per second
}

/// Proxy status information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyStatus {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub color: String,
    pub bar_color: String,
}

impl ProxyStatus {
    pub fn checking() -> Self {
        Self {
            status: "Checking for updates...".to_string(),
            version: None,
            color: "text-yellow-400".to_string(),
            bar_color: "bg-yellow-400".to_string(),
        }
    }

    pub fn downloading(version: String) -> Self {
        Self {
            status: "Downloading...".to_string(),
            version: Some(version),
            color: "text-blue-400".to_string(),
            bar_color: "bg-blue-500".to_string(),
        }
    }

    pub fn launching() -> Self {
        Self {
            status: "Launching...".to_string(),
            version: None,
            color: "text-green-400".to_string(),
            bar_color: "bg-green-400".to_string(),
        }
    }

    pub fn launched() -> Self {
        Self {
            status: "Launched".to_string(),
            version: None,
            color: "text-green-400".to_string(),
            bar_color: "bg-green-400".to_string(),
        }
    }

    pub fn error() -> Self {
        Self {
            status: "Error".to_string(),
            version: None,
            color: "text-red-500".to_string(),
            bar_color: "bg-red-500".to_string(),
        }
    }
}

/// User data extracted from proxy logs for RPC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcUserData {
    pub ign: String,
    pub uuid: String,
}
