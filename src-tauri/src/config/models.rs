//! Data models for configuration.

use serde::{Deserialize, Serialize};

/// Application configuration structure.
///
/// This structure represents all application settings that can be
/// stored in the configuration file.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// Whether to minimize the application to the system tray
    #[serde(default)]
    pub minimize_to_tray: bool,

    /// Whether to automatically check for and install updates
    #[serde(default = "default_true")]
    pub auto_update: bool,

    /// Whether to open logs when the application launches
    #[serde(default = "default_true")]
    pub open_logs_on_launch: bool,

    /// Whether to reduce motion/animations for accessibility
    #[serde(default)]
    pub reduced_motion: bool,

    /// Whether to enable Discord Rich Presence
    #[serde(default = "default_true")]
    pub enable_rpc: bool,

    /// Whether to hide profile (IGN/avatar) from Discord Rich Presence
    #[serde(default)]
    pub rpc_anonymize_profile: bool,

    /// Whether to hide location/game mode from Discord Rich Presence
    #[serde(default)]
    pub rpc_anonymize_location: bool,

    /// Port number for the proxy server (as string)
    #[serde(default = "default_proxy_port")]
    pub proxy_port: String,

    /// Whether to enable MSA (Microsoft Account) authentication
    #[serde(default)]
    pub enable_msa: bool,
}

fn default_true() -> bool {
    true
}

fn default_proxy_port() -> String {
    "25565".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            minimize_to_tray: false,
            auto_update: true,
            open_logs_on_launch: true,
            reduced_motion: false,
            enable_rpc: true,
            rpc_anonymize_profile: false,
            rpc_anonymize_location: false,
            proxy_port: "25565".to_string(),
            enable_msa: false,
        }
    }
}
