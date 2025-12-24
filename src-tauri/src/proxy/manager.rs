//! Proxy process management.

use super::download::{
    cleanup_old_executables, download_artifact, fetch_releases, find_latest_release,
    find_platform_asset, get_install_dir, get_platform_tag, is_file_valid,
};
use super::error::ProxyError;
use super::models::{ProxyStatus, RpcUserData};
use crate::rpc::RpcManager;
use crate::utils::get_home_dir;
use serde::Deserialize;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

/// Lock file data structure written by the proxy
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LockFileData {
    #[allow(dead_code)]
    pid: u32,
    #[allow(dead_code)]
    port: u16,
    control_port: Option<u16>,
}

/// Control socket message types
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ControlMessage {
    UserData { ign: String, uuid: String },
    GameMode { mode: Option<String>, map: Option<String> },
    Disconnect,
}

/// Gets the path to the proxy lock file
fn get_lock_file_path() -> Option<PathBuf> {
    get_home_dir().ok().map(|h| h.join(".duelsplus").join("proxy.lock"))
}

/// Reads the proxy lock file to get the control port
fn read_lock_file() -> Option<LockFileData> {
    let path = get_lock_file_path()?;
    let content = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&content).ok()
}

/// Sends a shutdown command to the proxy via TCP control socket
async fn send_shutdown_command(control_port: u16) -> bool {
    let addr = format!("127.0.0.1:{}", control_port);
    
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        async {
            let mut stream = TcpStream::connect(&addr).await?;
            stream.write_all(b"shutdown").await?;
            
            let mut buf = [0u8; 16];
            let n = stream.read(&mut buf).await?;
            let response = String::from_utf8_lossy(&buf[..n]);
            
            Ok::<bool, std::io::Error>(response.trim() == "ok")
        }
    ).await;
    
    match result {
        Ok(Ok(true)) => true,
        _ => false,
    }
}

/// Connects to the control socket and listens for user data messages
async fn listen_control_socket(app: AppHandle, is_running: Arc<Mutex<bool>>) {
    // Wait a bit for the proxy to start and write the lock file
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Retry connecting to the control socket
    let mut stream = None;
    for _ in 0..10 {
        if !*is_running.lock().await {
            return;
        }

        if let Some(lock_data) = read_lock_file() {
            if let Some(control_port) = lock_data.control_port {
                let addr = format!("127.0.0.1:{}", control_port);
                if let Ok(s) = TcpStream::connect(&addr).await {
                    stream = Some(s);
                    break;
                }
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }

    let stream = match stream {
        Some(s) => s,
        None => return,
    };

    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    while *is_running.lock().await {
        match lines.next_line().await {
            Ok(Some(line)) => {
                if let Ok(msg) = serde_json::from_str::<ControlMessage>(&line) {
                    match msg {
                        ControlMessage::UserData { ign, uuid } => {
                            // Emit event for frontend
                            let _ = app.emit(
                                "rpc-user-data",
                                RpcUserData {
                                    ign: ign.clone(),
                                    uuid: uuid.clone(),
                                },
                            );

                            // Update Discord RPC directly
                            if let Some(rpc) = app.try_state::<RpcManager>() {
                                rpc.set_user_data(Some(ign), Some(uuid));
                            }
                        }
                        ControlMessage::GameMode { mode, map } => {
                            // Update Discord RPC with game mode (mode can be null when in lobby)
                            if let Some(rpc) = app.try_state::<RpcManager>() {
                                rpc.set_game_mode(mode, map);
                            }
                        }
                        ControlMessage::Disconnect => {
                            // User disconnected from Hypixel, reset RPC to idle
                            if let Some(rpc) = app.try_state::<RpcManager>() {
                                rpc.set_disconnected();
                            }
                        }
                    }
                }
            }
            Ok(None) => break,
            Err(_) => break,
        }
    }
}

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
        let _ = app.emit("updater:status", ProxyStatus::Checking);

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
            let msg = format!("Downloading version {}", latest.version);
            println!("[proxy] {}", msg);
            let _ = app.emit("log-message", msg);
            let _ = app.emit(
                "updater:status",
                ProxyStatus::Downloading {
                    version: latest.version.clone(),
                },
            );

            // Download with progress tracking
            let app_clone = app.clone();
            download_artifact(&asset.id, &file_path, move |progress| {
                if let Err(e) = app_clone.emit("updater:progress", &progress) {
                    eprintln!("Failed to emit progress event: {:?}", e);
                }
            })
            .await?;

            println!("[proxy] Download complete!");
            let _ = app.emit("log-message", "Download complete!");
        } else {
            let msg = format!("Proxy already up to date ({})", latest.version);
            println!("[proxy] {}", msg);
            let _ = app.emit("log-message", msg);
        }

        // Clean up old executables
        cleanup_old_executables(&install_dir, &asset.name)?;

        // Launch the proxy
        let _ = app.emit("updater:status", ProxyStatus::Launching);

        // Update RPC to "Launching"
        if let Some(rpc) = app.try_state::<RpcManager>() {
            rpc.set_launching();
        }

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

        let _ = app.emit("updater:status", ProxyStatus::Launched);
        let _ = app.emit("updater:hide", ());

        // Clear "Launching" state - RPC goes back to "Idle" until user connects
        if let Some(rpc) = app.try_state::<RpcManager>() {
            rpc.set_in_launcher();
        }

        // Spawn tasks to handle stdout and stderr
        let app_clone = app.clone();
        let is_running_clone = self.is_running.clone();
        tokio::spawn(async move {
            Self::handle_output(app_clone, stdout, stderr, is_running_clone).await;
        });

        // Spawn control socket listener for user data
        let app_clone = app.clone();
        let is_running_clone = self.is_running.clone();
        tokio::spawn(async move {
            listen_control_socket(app_clone, is_running_clone).await;
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

        loop {
            tokio::select! {
                result = stdout_lines.next_line() => {
                    match result {
                        Ok(Some(line)) => {
                            let line = Self::fix_encoding(&line);

                            // Emit log message and print to console
                            if !line.contains("ExperimentalWarning") && !line.contains("--trace-warnings") {
                                println!("[proxy] {}", line);
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
                                eprintln!("[proxy:err] {}", line);
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
        let _ = app.emit("updater:status", ProxyStatus::Error);
        println!("[proxy] Proxy process exited");
        let _ = app.emit("log-message", "Proxy process exited");

        // Reset RPC to "In Launcher"
        if let Some(rpc) = app.try_state::<RpcManager>() {
            rpc.clear_activity();
        }
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
            // Try graceful shutdown via control socket first (works on all platforms)
            if let Some(lock_data) = read_lock_file() {
                if let Some(control_port) = lock_data.control_port {
                    if send_shutdown_command(control_port).await {
                        // Graceful shutdown initiated, wait for process to exit
                        let _ = tokio::time::timeout(
                            std::time::Duration::from_secs(5),
                            child.wait()
                        ).await;
                        *self.is_running.lock().await = false;
                        return Ok(());
                    }
                }
            }

            // Fallback: signal-based shutdown
            #[cfg(unix)]
            {
                use nix::sys::signal::{kill, Signal};
                use nix::unistd::Pid;

                if let Some(pid) = child.id() {
                    let _ = kill(Pid::from_raw(pid as i32), Signal::SIGTERM);
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
