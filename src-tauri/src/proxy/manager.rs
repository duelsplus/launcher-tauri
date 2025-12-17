//! Proxy process management.

use super::download::{
    cleanup_old_executables, download_artifact, fetch_releases, find_latest_release,
    find_platform_asset, get_install_dir, get_platform_tag, is_file_valid,
};
use super::error::ProxyError;
use super::models::{ProxyStatus, RpcUserData};
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

/// Proxy process manager
pub struct ProxyManager {
    process: Arc<Mutex<Option<Child>>>,
    is_running: Arc<Mutex<bool>>,
}

impl ProxyManager {
    /// Creates a new proxy manager
    pub fn new() -> Self {
        Self {
            process: Arc::new(Mutex::new(None)),
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    /// Checks if the proxy is currently running
    pub async fn is_running(&self) -> bool {
        *self.is_running.lock().await
    }

    /// Checks for updates, downloads if necessary, and launches the proxy
    pub async fn check_and_launch(&self, app: AppHandle, port: u16) -> Result<(), ProxyError> {
        // Check if already running
        if self.is_running().await {
            return Err(ProxyError::AlreadyRunning);
        }

        // Emit status
        let _ = app.emit("updater:show", ());
        let _ = app.emit("updater:status", ProxyStatus::checking());

        // Get platform and install directory
        let platform_tag = get_platform_tag()?;
        let install_dir = get_install_dir()?;

        // Fetch releases
        let releases = fetch_releases().await?;
        let latest = find_latest_release(&releases)?;
        let asset = find_platform_asset(latest, &platform_tag)?;

        let file_path = install_dir.join(&asset.name);

        // Check if download is needed
        let needs_download = !is_file_valid(&file_path);

        if needs_download {
            let _ = app.emit(
                "log-message",
                format!("Downloading version {}", latest.version),
            );
            let _ = app.emit(
                "updater:status",
                ProxyStatus::downloading(latest.version.clone()),
            );

            // Download with progress tracking
            let app_clone = app.clone();
            download_artifact(&asset.id, &file_path, move |progress| {
                let _ = app_clone.emit("updater:progress", progress);
            })
            .await?;

            let _ = app.emit("log-message", "Download complete!");
        } else {
            let _ = app.emit(
                "log-message",
                format!("Proxy already up to date ({})", latest.version),
            );
        }

        // Clean up old executables
        cleanup_old_executables(&install_dir, &asset.name)?;

        // Launch the proxy
        let _ = app.emit("updater:status", ProxyStatus::launching());
        self.launch_process(app, file_path, port).await?;

        Ok(())
    }

    /// Launches the proxy process
    async fn launch_process(
        &self,
        app: AppHandle,
        executable_path: PathBuf,
        port: u16,
    ) -> Result<(), ProxyError> {
        let mut cmd = Command::new(&executable_path);
        cmd.arg("--port")
            .arg(port.to_string())
            .current_dir(executable_path.parent().unwrap())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::null());

        // Set environment variables for better encoding
        #[cfg(unix)]
        {
            cmd.env("LANG", "en_US.UTF-8");
            cmd.env("LC_ALL", "en_US.UTF-8");
        }

        let mut child = cmd
            .spawn()
            .map_err(|e| ProxyError::ProcessError(e.to_string()))?;

        // Get stdout and stderr
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| ProxyError::ProcessError("Failed to capture stdout".to_string()))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| ProxyError::ProcessError("Failed to capture stderr".to_string()))?;

        // Store the process
        *self.process.lock().await = Some(child);
        *self.is_running.lock().await = true;

        let _ = app.emit("updater:status", ProxyStatus::launched());
        let _ = app.emit("updater:hide", ());

        // Spawn tasks to handle stdout and stderr
        let app_clone = app.clone();
        let is_running_clone = self.is_running.clone();
        tokio::spawn(async move {
            Self::handle_output(app_clone, stdout, stderr, is_running_clone).await;
        });

        Ok(())
    }

    /// Handles stdout and stderr from the proxy process
    async fn handle_output(
        app: AppHandle,
        stdout: impl tokio::io::AsyncRead + Unpin,
        stderr: impl tokio::io::AsyncRead + Unpin,
        is_running: Arc<Mutex<bool>>,
    ) {
        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);

        let mut stdout_lines = stdout_reader.lines();
        let mut stderr_lines = stderr_reader.lines();

        let mut ign: Option<String> = None;
        let mut uuid: Option<String> = None;

        loop {
            tokio::select! {
                result = stdout_lines.next_line() => {
                    match result {
                        Ok(Some(line)) => {
                            let line = Self::fix_encoding(&line);

                            // Check for special launcher tags
                            if line.contains("[launcher:ign]") {
                                if let Some(extracted) = line.split("[launcher:ign]").nth(1) {
                                    ign = Some(extracted.trim().to_string());
                                }
                                continue;
                            }

                            if line.contains("[launcher:uuid]") {
                                if let Some(extracted) = line.split("[launcher:uuid]").nth(1) {
                                    uuid = Some(extracted.trim().to_string());
                                }
                                continue;
                            }

                            // Emit RPC user data if we have both
                            if let (Some(ign_val), Some(uuid_val)) = (&ign, &uuid) {
                                let _ = app.emit("rpc-user-data", RpcUserData {
                                    ign: ign_val.clone(),
                                    uuid: uuid_val.clone(),
                                });
                                ign = None;
                                uuid = None;
                            }

                            // Emit log message
                            if !line.contains("ExperimentalWarning") && !line.contains("--trace-warnings") {
                                let _ = app.emit("log-message", line);
                            }
                        }
                        Ok(None) => break,
                        Err(_) => break,
                    }
                }
                result = stderr_lines.next_line() => {
                    match result {
                        Ok(Some(line)) => {
                            let line = Self::fix_encoding(&line);

                            if !line.contains("ExperimentalWarning") && !line.contains("--trace-warnings") {
                                let _ = app.emit("log-message", line);
                            }
                        }
                        Ok(None) => break,
                        Err(_) => break,
                    }
                }
            }
        }

        *is_running.lock().await = false;
        let _ = app.emit("updater:status", ProxyStatus::error());
        let _ = app.emit("log-message", "Proxy process exited");
    }

    /// Fixes encoding issues on Windows
    fn fix_encoding(s: &str) -> String {
        s.replace("\u{0393}\u{00a3}\u{00f4}", "*") // garbled checkmark -> *
            .replace("\u{0393}\u{00a3}\u{00f9}", "x") // garbled x mark -> x
            .replace("\u{0393}\u{00a3}\u{00e6}", "*") // garbled check variant -> *
            .replace("\u{0393}\u{0102}\u{00b6}", "-") // garbled em dash -> -
            .replace("\u{0393}\u{0102}\u{00aa}", "...") // garbled ellipsis -> ...
            .replace("\u{0393}\u{0102}\u{00f4}", "-") // garbled en dash -> -
            .replace("\u{0393}\u{0102}\u{0153}", "\"") // garbled left quote -> "
            .replace("\u{0393}\u{0102}\u{009d}", "\"") // garbled right quote -> "
            .replace("\u{0393}\u{201e}\u{00bf}", "*") // garbled bullet -> *
            .replace("\u{0393}\u{0102}\u{00b3}", "*") // garbled bullet variant -> *
            .replace("\u{00c2}\u{00a9}", "(c)") // garbled copyright -> (c)
            .replace("✔", "*")
            .replace("✓", "*")
            .replace("✗", "x")
            .replace("✘", "x")
            .replace("—", "-")
            .replace("–", "-")
            .replace("…", "...")
            .replace("\u{201c}", "\"") // left double quote
            .replace("\u{201d}", "\"") // right double quote
            .replace("\u{2018}", "'") // left single quote
            .replace("\u{2019}", "'") // right single quote
            .replace("●", "*")
            .replace("•", "*")
            .replace("©", "(c)")
    }

    /// Stops the proxy process
    pub async fn stop(&self) -> Result<(), ProxyError> {
        let mut process_guard = self.process.lock().await;

        if let Some(mut child) = process_guard.take() {
            // Try graceful shutdown first
            #[cfg(unix)]
            {
                use nix::sys::signal::{kill, Signal};
                use nix::unistd::Pid;

                if let Some(pid) = child.id() {
                    let _ = kill(Pid::from_raw(pid as i32), Signal::SIGINT);
                }
            }

            #[cfg(windows)]
            {
                let _ = child.kill().await;
            }

            // Wait for process to exit
            let _ = tokio::time::timeout(std::time::Duration::from_secs(5), child.wait()).await;

            *self.is_running.lock().await = false;
            Ok(())
        } else {
            Err(ProxyError::NotRunning)
        }
    }
}

impl Default for ProxyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_proxy_manager_stop_when_not_running() {
        let manager = ProxyManager::new();
        let result = manager.stop().await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ProxyError::NotRunning));
    }
}
