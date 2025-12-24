//! Discord RPC manager for activity presence.
//!
//! Manages the Discord Rich Presence connection and activity updates.
//! Uses a background thread to handle the Discord IPC connection since
//! the discord-rich-presence crate uses blocking I/O.

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

/// Discord Application Client ID for Duels+
const CLIENT_ID: &str = "1391866803889770526";

/// Commands sent to the RPC worker thread
#[derive(Debug, Clone)]
enum RpcCommand {
    /// Connect to Discord
    Connect,
    /// Disconnect from Discord
    Disconnect,
    /// Set activity to "In Launcher"
    SetInLauncher,
    /// Set activity to "Launching"
    SetLaunching,
    /// Set activity to "Playing" (basic, no user data)
    SetPlaying,
    /// Update user data (ign, uuid)
    SetUserData {
        ign: Option<String>,
        uuid: Option<String>,
    },
    /// Update game mode
    SetGameMode {
        mode: Option<String>,
        map: Option<String>,
    },
    /// User disconnected from Hypixel
    SetDisconnected,
    /// Clear activity (reset to base)
    ClearActivity,
    /// Shutdown the RPC thread
    Shutdown,
}

/// Discord RPC state
#[derive(Debug, Clone, Default)]
struct RpcState {
    connected: bool,
    enabled: bool,
    start_timestamp: i64,
    current_ign: Option<String>,
    current_uuid: Option<String>,
    current_mode: Option<String>,
    current_map: Option<String>,
    is_playing: bool,
}

/// Manager for Discord Rich Presence
pub struct RpcManager {
    sender: Mutex<Option<Sender<RpcCommand>>>,
    state: Arc<Mutex<RpcState>>,
    is_dev: Arc<Mutex<bool>>,
}

impl RpcManager {
    /// Creates a new RPC manager
    pub fn new(is_dev: bool) -> Self {
        let state = Arc::new(Mutex::new(RpcState {
            connected: false,
            enabled: true,
            start_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64,
            current_ign: None,
            current_uuid: None,
            current_mode: None,
            current_map: None,
            is_playing: false,
        }));

        Self {
            sender: Mutex::new(None),
            state,
            is_dev: Arc::new(Mutex::new(is_dev)),
        }
    }

    /// Updates the dev mode flag
    pub fn set_dev_mode(&self, is_dev: bool) {
        *self.is_dev.lock().unwrap() = is_dev;
    }

    /// Starts the RPC worker thread
    pub fn start(&self) {
        let mut sender_lock = self.sender.lock().unwrap();
        if sender_lock.is_some() {
            return; // Already started
        }

        let (tx, rx) = mpsc::channel::<RpcCommand>();
        let state = self.state.clone();
        let is_dev = self.is_dev.clone();

        thread::spawn(move || {
            let mut client: Option<DiscordIpcClient> = None;
            let mut should_run = true;

            while should_run {
                let command = match rx.recv() {
                    Ok(cmd) => cmd,
                    Err(_) => break, // Channel closed
                };

                match command {
                    RpcCommand::Connect => {
                        if client.is_none() {
                            match DiscordIpcClient::new(CLIENT_ID) {
                                Ok(mut c) => {
                                    if c.connect().is_ok() {
                                        let mut s = state.lock().unwrap();
                                        s.connected = true;
                                        client = Some(c);

                                        // Set initial activity
                                        if let Some(ref mut c) = client {
                                            let dev = *is_dev.lock().unwrap();
                                            Self::set_activity_internal(c, &s, dev);
                                        }
                                    }
                                }
                                Err(_) => {
                                    // Discord not running, ignore
                                }
                            }
                        }
                    }
                    RpcCommand::Disconnect => {
                        if let Some(mut c) = client.take() {
                            let _ = c.close();
                            let mut s = state.lock().unwrap();
                            s.connected = false;
                        }
                    }
                    RpcCommand::SetInLauncher => {
                        {
                            let mut s = state.lock().unwrap();
                            s.is_playing = false;
                            s.current_mode = None;
                            s.current_map = None;
                        }
                        let s = state.lock().unwrap();
                        if let Some(ref mut c) = client {
                            let dev = *is_dev.lock().unwrap();
                            Self::set_activity_internal(c, &s, dev);
                        }
                    }
                    RpcCommand::SetLaunching => {
                        {
                            let mut s = state.lock().unwrap();
                            s.is_playing = false;
                            s.current_mode = Some("Launching".to_string());
                        }
                        let s = state.lock().unwrap();
                        if let Some(ref mut c) = client {
                            let dev = *is_dev.lock().unwrap();
                            Self::set_activity_internal(c, &s, dev);
                        }
                    }
                    RpcCommand::SetPlaying => {
                        // Just mark as playing, don't update activity yet
                        // Activity will update when user data or game mode arrives
                        let mut s = state.lock().unwrap();
                        s.is_playing = true;
                        s.current_mode = None;
                    }
                    RpcCommand::SetUserData { ign, uuid } => {
                        {
                            let mut s = state.lock().unwrap();
                            // When we receive user data, user is actually connected
                            s.is_playing = true;
                            // Clear any "Launching" state - real mode will come from game_mode message
                            s.current_mode = None;
                            s.current_map = None;
                            if ign.is_some() {
                                s.current_ign = ign;
                            }
                            if uuid.is_some() {
                                s.current_uuid = uuid;
                            }
                        }
                        let s = state.lock().unwrap();
                        if let Some(ref mut c) = client {
                            let dev = *is_dev.lock().unwrap();
                            Self::set_activity_internal(c, &s, dev);
                        }
                    }
                    RpcCommand::SetGameMode { mode, map } => {
                        {
                            let mut s = state.lock().unwrap();
                            s.current_mode = mode;
                            s.current_map = map;
                        }
                        let s = state.lock().unwrap();
                        if let Some(ref mut c) = client {
                            let dev = *is_dev.lock().unwrap();
                            Self::set_activity_internal(c, &s, dev);
                        }
                    }
                    RpcCommand::SetDisconnected => {
                        // User disconnected from Hypixel - clear playing state but keep user info
                        {
                            let mut s = state.lock().unwrap();
                            s.is_playing = false;
                            s.current_mode = None;
                            s.current_map = None;
                        }
                        let s = state.lock().unwrap();
                        if let Some(ref mut c) = client {
                            let dev = *is_dev.lock().unwrap();
                            Self::set_activity_internal(c, &s, dev);
                        }
                    }
                    RpcCommand::ClearActivity => {
                        {
                            let mut s = state.lock().unwrap();
                            s.current_ign = None;
                            s.current_uuid = None;
                            s.current_mode = None;
                            s.current_map = None;
                            s.is_playing = false;
                        }
                        let s = state.lock().unwrap();
                        if let Some(ref mut c) = client {
                            let dev = *is_dev.lock().unwrap();
                            Self::set_activity_internal(c, &s, dev);
                        }
                    }
                    RpcCommand::Shutdown => {
                        if let Some(mut c) = client.take() {
                            let _ = c.close();
                        }
                        should_run = false;
                    }
                }
            }
        });

        *sender_lock = Some(tx);
    }

    /// Formats a game mode string to be human readable for the "Playing X" display
    fn format_mode(mode: &str) -> String {
        // Map of internal mode names to display names
        match mode {
            // Solo duel modes (1v1)
            "DUELS_COMBO_DUEL" => "Combo Duel".to_string(),
            "DUELS_CLASSIC_DUEL" => "Classic Duel".to_string(),
            "DUELS_POTION_DUEL" => "NoDebuff Duel".to_string(),
            "DUELS_BOXING_DUEL" => "Boxing Duel".to_string(),
            "DUELS_BOW_DUEL" => "Bow Duel".to_string(),
            "DUELS_SUMO_DUEL" => "Sumo Duel".to_string(),
            "DUELS_OP_DUEL" => "OP Duel".to_string(),
            "DUELS_UHC_DUEL" => "UHC Duel".to_string(),
            "DUELS_BRIDGE_DUEL" => "Bridge Duel".to_string(),
            "DUELS_SW_DUEL" => "SkyWars Duel".to_string(),
            "DUELS_MW_DUEL" => "MegaWalls Duel".to_string(),
            "DUELS_BLITZ_DUEL" => "Blitz Duel".to_string(),
            "DUELS_PARKOUR_DUEL" => "Parkour Duel".to_string(),
            "DUELS_BOWSPLEEF_DUEL" => "Bow Spleef Duel".to_string(),
            "DUELS_SPLEEF_DUEL" => "Spleef Duel".to_string(),
            "DUELS_QUAKE_DUEL" => "Quake Duel".to_string(),
            // Doubles modes (2v2)
            "DUELS_CLASSIC_DOUBLES" => "Classic Doubles".to_string(),
            "DUELS_OP_DOUBLES" => "OP Doubles".to_string(),
            "DUELS_UHC_DOUBLES" => "UHC Doubles".to_string(),
            "DUELS_BRIDGE_DOUBLES" => "Bridge Doubles".to_string(),
            "DUELS_SW_DOUBLES" => "SkyWars Doubles".to_string(),
            "DUELS_MW_DOUBLES" => "MegaWalls Doubles".to_string(),
            // Bridge team modes (not 1v1 duels)
            "DUELS_BRIDGE_THREES" => "Bridge 3v3".to_string(),
            "DUELS_BRIDGE_FOUR" => "Bridge 4v4".to_string(),
            "DUELS_BRIDGE_2V2V2V2" => "Bridge 2v2v2v2".to_string(),
            "DUELS_BRIDGE_3V3V3V3" => "Bridge 3v3v3v3".to_string(),
            "DUELS_CAPTURE_THREES" => "CTF 3v3".to_string(),
            // Arena/Party modes (not duels)
            "DUELS_DUEL_ARENA" => "Arena".to_string(),
            "DUELS_DISASTERS" => "Disasters".to_string(),
            "DUELS_PARKOUR_EIGHT" => "Parkour".to_string(),
            // BedWars Duels
            "BEDWARS_TWO_ONE_DUELS" => "BedWars Duel".to_string(),
            "BEDWARS_TWO_ONE_DUELS_RUSH" => "BedWars Rush".to_string(),
            // Fallback: clean up the string (remove DUEL/DUELS suffix for non-1v1 modes)
            _ => {
                let cleaned = mode
                    .strip_prefix("DUELS_")
                    .or_else(|| mode.strip_prefix("BEDWARS_"))
                    .unwrap_or(mode);
                cleaned
                    .split('_')
                    .filter(|&w| w != "DUEL" && w != "DUELS")
                    .map(|word| {
                        let mut chars = word.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(first) => {
                                first.to_uppercase().to_string() + &chars.as_str().to_lowercase()
                            }
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        }
    }

    /// Sets the Discord activity based on current state
    fn set_activity_internal(client: &mut DiscordIpcClient, state: &RpcState, is_dev: bool) {
        let large_image = if is_dev { "logo-v1-purple" } else { "logo-v1" };
        let large_text = if is_dev {
            "Launcher (dev build)"
        } else {
            "Duels+ Launcher"
        };

        // Format game mode
        let formatted_mode: Option<String> = if state.is_playing {
            state.current_mode.as_ref().map(|m| Self::format_mode(m))
        } else {
            None
        };

        // Determine details text based on current activity
        let details: String = if state.is_playing {
            if let Some(ref mode) = formatted_mode {
                // Playing with a known game mode - "Playing Combo Duel"
                format!("Playing {}", mode)
            } else {
                // Connected to Hypixel but in lobby
                "In Hypixel Lobby".to_string()
            }
        } else if state.current_mode.as_deref() == Some("Launching") {
            "Launching Proxy".to_string()
        } else {
            "Idle".to_string()
        };

        let mut activity_builder = activity::Activity::new()
            .details(&details)
            .timestamps(activity::Timestamps::new().start(state.start_timestamp));

        // Build assets - need to store avatar_url to extend its lifetime
        let avatar_url: String;
        let assets = if let (Some(ign), Some(uuid)) = (&state.current_ign, &state.current_uuid) {
            avatar_url = format!("https://mc-heads.net/avatar/{}/64.png", uuid);
            activity::Assets::new()
                .large_image(large_image)
                .large_text(large_text)
                .small_image(&avatar_url)
                .small_text(ign)
        } else {
            activity::Assets::new()
                .large_image(large_image)
                .large_text(large_text)
        };

        activity_builder = activity_builder.assets(assets);

        // Ignore errors - Discord might not be running
        let _ = client.set_activity(activity_builder);
    }

    /// Sends a command to the RPC worker
    fn send(&self, command: RpcCommand) {
        if let Ok(sender) = self.sender.lock() {
            if let Some(ref tx) = *sender {
                let _ = tx.send(command);
            }
        }
    }

    /// Connects to Discord RPC
    pub fn connect(&self) {
        let enabled = {
            let s = self.state.lock().unwrap();
            s.enabled
        };
        if enabled {
            self.send(RpcCommand::Connect);
        }
    }

    /// Disconnects from Discord RPC
    pub fn disconnect(&self) {
        self.send(RpcCommand::Disconnect);
    }

    /// Sets activity to "In Launcher"
    pub fn set_in_launcher(&self) {
        self.send(RpcCommand::SetInLauncher);
    }

    /// Sets activity to "Launching"
    pub fn set_launching(&self) {
        self.send(RpcCommand::SetLaunching);
    }

    /// Sets activity to "Playing" (basic)
    pub fn set_playing(&self, ign: Option<String>, uuid: Option<String>) {
        self.send(RpcCommand::SetPlaying);
        if ign.is_some() || uuid.is_some() {
            self.send(RpcCommand::SetUserData { ign, uuid });
        }
    }

    /// Updates user data (ign, uuid) for RPC display
    pub fn set_user_data(&self, ign: Option<String>, uuid: Option<String>) {
        self.send(RpcCommand::SetUserData { ign, uuid });
    }

    /// Updates game mode for RPC display
    pub fn set_game_mode(&self, mode: Option<String>, map: Option<String>) {
        self.send(RpcCommand::SetGameMode { mode, map });
    }

    /// Called when user disconnects from Hypixel, clears playing state but keeps user data
    pub fn set_disconnected(&self) {
        self.send(RpcCommand::SetDisconnected);
    }

    /// Clears user data and resets to base activity
    pub fn clear_activity(&self) {
        self.send(RpcCommand::ClearActivity);
    }

    /// Enables or disables RPC
    pub fn set_enabled(&self, enabled: bool) {
        {
            let mut s = self.state.lock().unwrap();
            s.enabled = enabled;
        }
        if enabled {
            self.connect();
        } else {
            self.disconnect();
        }
    }

    /// Returns whether RPC is enabled
    pub fn is_enabled(&self) -> bool {
        self.state.lock().unwrap().enabled
    }

    /// Returns whether RPC is connected
    #[allow(dead_code)]
    pub fn is_connected(&self) -> bool {
        self.state.lock().unwrap().connected
    }

    /// Shuts down the RPC manager
    pub fn shutdown(&self) {
        self.send(RpcCommand::Shutdown);
    }
}

impl Drop for RpcManager {
    fn drop(&mut self) {
        self.shutdown();
    }
}
