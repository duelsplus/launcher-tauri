//! Data models for configuration.

use serde::{Deserialize, Serialize};

/// Application configuration structure.
///
/// This structure represents all application settings that can be
/// stored in the configuration file.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Whether to minimize the application to the system tray
    #[serde(default = "default_false")]
    pub minimizeToTray: bool,

    /// Whether to automatically check for and install updates
    #[serde(default = "default_true")]
    pub autoUpdate: bool,

    /// Whether to open logs when the application launches
    #[serde(default = "default_true")]
    pub openLogsOnLaunch: bool,

    /// Whether to reduce motion/animations for accessibility
    #[serde(default = "default_false")]
    pub reducedMotion: bool,

    /// Whether to enable Discord Rich Presence
    #[serde(default = "default_true")]
    pub enableRpc: bool,

    /// Port number for the proxy server (as string)
    #[serde(default = "default_proxy_port")]
    pub proxyPort: String,

    /// Whether to enable MSA (Microsoft Account) authentication
    #[serde(default = "default_false")]
    pub enableMsa: bool,
}

fn default_false() -> bool {
    false
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
            minimizeToTray: false,
            autoUpdate: true,
            openLogsOnLaunch: true,
            reducedMotion: false,
            enableRpc: true,
            proxyPort: "25565".to_string(),
            enableMsa: false,
        }
    }
}
