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
        gametype: Option<String>,
        lobbyname: Option<String>,
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
    current_gametype: Option<String>,
    in_lobby: bool,
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
            current_gametype: None,
            in_lobby: false,
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
                    RpcCommand::SetGameMode {
                        mode,
                        map,
                        gametype,
                        lobbyname,
                    } => {
                        {
                            let mut s = state.lock().unwrap();
                            s.current_mode = mode;
                            s.current_map = map;
                            s.current_gametype = gametype;
                            s.in_lobby = lobbyname.is_some();
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
                            s.current_gametype = None;
                            s.in_lobby = false;
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
                            s.current_gametype = None;
                            s.in_lobby = false;
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

    /// Formats a SkyBlock location/mode to be human readable
    fn format_skyblock_location(mode: &str) -> String {
        match mode {
            "hub" => "SkyBlock Hub".to_string(),
            "dynamic" => "SkyBlock Island".to_string(),
            "farming_1" => "SkyBlock Farm".to_string(),
            "mining_1" => "SkyBlock Deep Caverns".to_string(),
            "mining_2" => "SkyBlock Dwarven Mines".to_string(),
            "mining_3" => "SkyBlock Crystal Hollows".to_string(),
            "combat_1" => "SkyBlock Spider's Den".to_string(),
            "combat_2" => "SkyBlock Blazing Fortress".to_string(),
            "combat_3" => "SkyBlock The End".to_string(),
            "foraging_1" => "SkyBlock Park".to_string(),
            "dungeon_hub" => "SkyBlock Dungeon Hub".to_string(),
            "dungeon" => "SkyBlock Dungeons".to_string(),
            "crimson_isle" => "SkyBlock Crimson Isle".to_string(),
            "rift" => "SkyBlock Rift".to_string(),
            "garden" => "SkyBlock Garden".to_string(),
            "kuudra_normal" => "SkyBlock Kuudra (Basic)".to_string(),
            "kuudra_hot" => "SkyBlock Kuudra (Hot)".to_string(),
            "kuudra_burning" => "SkyBlock Kuudra (Burning)".to_string(),
            "kuudra_fiery" => "SkyBlock Kuudra (Fiery)".to_string(),
            "kuudra_infernal" => "SkyBlock Kuudra (Infernal)".to_string(),
            "instanced" => "SkyBlock Instance".to_string(),
            "dark_auction" => "SkyBlock Dark Auction".to_string(),
            "winter" => "SkyBlock Jerry's Workshop".to_string(),
            // Fallback: clean up the string
            _ => {
                let cleaned = mode.replace('_', " ");
                let mut result = String::new();
                for (i, word) in cleaned.split_whitespace().enumerate() {
                    if i > 0 {
                        result.push(' ');
                    }
                    let mut chars = word.chars();
                    if let Some(first) = chars.next() {
                        result.push_str(&first.to_uppercase().to_string());
                        result.push_str(&chars.as_str().to_lowercase());
                    }
                }
                format!("SkyBlock {}", result)
            }
        }
    }

    /// Formats a gametype string to be human readable for lobby display
    fn format_gametype(gametype: &str) -> String {
        match gametype {
            "DUELS" => "Duels".to_string(),
            "BEDWARS" => "BedWars".to_string(),
            "SKYWARS" => "SkyWars".to_string(),
            "ARCADE" => "Arcade".to_string(),
            "MURDER_MYSTERY" => "Murder Mystery".to_string(),
            "BUILD_BATTLE" => "Build Battle".to_string(),
            "HOUSING" => "Housing".to_string(),
            "SURVIVAL_GAMES" => "Blitz SG".to_string(),
            "SUPER_SMASH" => "Smash Heroes".to_string(),
            "WALLS3" => "Mega Walls".to_string(),
            "MCGO" => "Cops and Crims".to_string(),
            "UHC" => "UHC".to_string(),
            "SPEED_UHC" => "Speed UHC".to_string(),
            "TNTGAMES" => "TNT Games".to_string(),
            "BATTLEGROUND" => "Warlords".to_string(),
            "PIT" => "The Pit".to_string(),
            "PROTOTYPE" => "Prototype".to_string(),
            "SKYBLOCK" => "SkyBlock".to_string(),
            "WOOL_GAMES" => "Wool Wars".to_string(),
            "PAINTBALL" => "Paintball".to_string(),
            "QUAKECRAFT" => "Quake".to_string(),
            "VAMPIREZ" => "VampireZ".to_string(),
            "WALLS" => "The Walls".to_string(),
            "ARENA" => "Arena Brawl".to_string(),
            "LEGACY" => "Classic Games".to_string(),
            "SMP" => "SMP".to_string(),
            "LIMBO" => "Limbo".to_string(),
            "MAIN" => "Main".to_string(),
            "TOURNAMENT" => "Tournament".to_string(),
            "REPLAY" => "Replay".to_string(),
            // Fallback: title case the gametype
            _ => gametype
                .split('_')
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
                .join(" "),
        }
    }

    /// Formats a game mode string to be human readable for the "Playing X" display
    fn format_mode(mode: &str) -> String {
        // Map of internal mode names to display names
        match mode {
            // === DUELS - Solo modes (1v1) ===
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
            // === DUELS - Doubles modes (2v2) ===
            "DUELS_CLASSIC_DOUBLES" => "Classic Doubles".to_string(),
            "DUELS_OP_DOUBLES" => "OP Doubles".to_string(),
            "DUELS_UHC_DOUBLES" => "UHC Doubles".to_string(),
            "DUELS_BRIDGE_DOUBLES" => "Bridge Doubles".to_string(),
            "DUELS_SW_DOUBLES" => "SkyWars Doubles".to_string(),
            "DUELS_MW_DOUBLES" => "MegaWalls Doubles".to_string(),
            // === DUELS - Bridge team modes ===
            "DUELS_BRIDGE_THREES" => "Bridge 3v3".to_string(),
            "DUELS_BRIDGE_FOUR" => "Bridge 4v4".to_string(),
            "DUELS_BRIDGE_2V2V2V2" => "Bridge 2v2v2v2".to_string(),
            "DUELS_BRIDGE_3V3V3V3" => "Bridge 3v3v3v3".to_string(),
            "DUELS_CAPTURE_THREES" => "CTF 3v3".to_string(),
            // === DUELS - Party/Arena modes ===
            "DUELS_DUEL_ARENA" => "Arena".to_string(),
            "DUELS_DISASTERS" => "Disasters".to_string(),
            "DUELS_PARKOUR_EIGHT" => "Parkour".to_string(),

            // === BEDWARS ===
            "BEDWARS_TWO_ONE_DUELS" => "BedWars Duel".to_string(),
            "BEDWARS_TWO_ONE_DUELS_RUSH" => "BedWars Rush Duel".to_string(),
            "BEDWARS_EIGHT_ONE" => "BedWars Solo".to_string(),
            "BEDWARS_EIGHT_TWO" => "BedWars Doubles".to_string(),
            "BEDWARS_FOUR_THREE" => "BedWars 3v3v3v3".to_string(),
            "BEDWARS_FOUR_FOUR" => "BedWars 4v4v4v4".to_string(),
            "BEDWARS_TWO_FOUR" => "BedWars 4v4".to_string(),
            "BEDWARS_CASTLE" => "BedWars Castle".to_string(),
            "BEDWARS_EIGHT_ONE_RUSH" => "BedWars Rush Solo".to_string(),
            "BEDWARS_EIGHT_TWO_RUSH" => "BedWars Rush Doubles".to_string(),
            "BEDWARS_FOUR_FOUR_RUSH" => "BedWars Rush 4v4v4v4".to_string(),
            "BEDWARS_EIGHT_ONE_ULTIMATE" => "BedWars Ultimate Solo".to_string(),
            "BEDWARS_EIGHT_TWO_ULTIMATE" => "BedWars Ultimate Doubles".to_string(),
            "BEDWARS_FOUR_FOUR_ULTIMATE" => "BedWars Ultimate 4v4v4v4".to_string(),
            "BEDWARS_EIGHT_ONE_ARMED" => "BedWars Armed Solo".to_string(),
            "BEDWARS_EIGHT_TWO_ARMED" => "BedWars Armed Doubles".to_string(),
            "BEDWARS_FOUR_FOUR_ARMED" => "BedWars Armed 4v4v4v4".to_string(),
            "BEDWARS_EIGHT_ONE_LUCKY" => "BedWars Lucky Solo".to_string(),
            "BEDWARS_EIGHT_TWO_LUCKY" => "BedWars Lucky Doubles".to_string(),
            "BEDWARS_FOUR_FOUR_LUCKY" => "BedWars Lucky 4v4v4v4".to_string(),
            "BEDWARS_EIGHT_ONE_VOIDLESS" => "BedWars Voidless Solo".to_string(),
            "BEDWARS_EIGHT_TWO_VOIDLESS" => "BedWars Voidless Doubles".to_string(),
            "BEDWARS_FOUR_FOUR_VOIDLESS" => "BedWars Voidless 4v4v4v4".to_string(),
            "BEDWARS_PRACTICE" => "BedWars Practice".to_string(),

            // === SKYWARS ===
            "solo_normal" => "SkyWars Solo Normal".to_string(),
            "solo_insane" => "SkyWars Solo Insane".to_string(),
            "teams_normal" => "SkyWars Teams Normal".to_string(),
            "teams_insane" => "SkyWars Teams Insane".to_string(),
            "ranked_normal" => "SkyWars Ranked".to_string(),
            "mega_normal" => "SkyWars Mega".to_string(),
            "mega_doubles" => "SkyWars Mega Doubles".to_string(),
            "solo_insane_lucky" => "SkyWars Lucky Solo".to_string(),
            "teams_insane_lucky" => "SkyWars Lucky Teams".to_string(),
            "solo_insane_slime" => "SkyWars Slime Solo".to_string(),
            "teams_insane_slime" => "SkyWars Slime Teams".to_string(),
            "solo_insane_rush" => "SkyWars Rush Solo".to_string(),
            "teams_insane_rush" => "SkyWars Rush Teams".to_string(),
            "solo_insane_tnt_madness" => "SkyWars TNT Madness Solo".to_string(),
            "teams_insane_tnt_madness" => "SkyWars TNT Madness Teams".to_string(),

            // === MURDER MYSTERY ===
            "MURDER_CLASSIC" => "Murder Mystery Classic".to_string(),
            "MURDER_DOUBLE_UP" => "Murder Mystery Double Up".to_string(),
            "MURDER_ASSASSINS" => "Murder Mystery Assassins".to_string(),
            "MURDER_INFECTION" => "Murder Mystery Infection".to_string(),
            "MURDER_SHOWDOWN" => "Murder Mystery Showdown".to_string(),

            // === ARCADE ===
            "PARTY" => "Party Games".to_string(),
            "HOLE_IN_THE_WALL" => "Hole in the Wall".to_string(),
            "FARM_HUNT" => "Farm Hunt".to_string(),
            "SOCCER" => "Football".to_string(),
            "BOUNTY_HUNTERS" => "Bounty Hunters".to_string(),
            "MINI_WALLS" => "Mini Walls".to_string(),
            "HIDE_AND_SEEK_PROP_HUNT" => "Prop Hunt".to_string(),
            "HIDE_AND_SEEK_PARTY_POOPER" => "Party Pooper".to_string(),
            "ZOMBIES_DEAD_END" => "Zombies Dead End".to_string(),
            "ZOMBIES_BAD_BLOOD" => "Zombies Bad Blood".to_string(),
            "ZOMBIES_ALIEN_ARCADIUM" => "Zombies Alien Arcadium".to_string(),
            "PIXEL_PAINTERS" => "Pixel Painters".to_string(),
            "THROW_OUT" => "Throw Out".to_string(),
            "ENDER_SPLEEF" => "Ender Spleef".to_string(),
            "STARWARS" => "Galaxy Wars".to_string(),
            "DRAGON_WARS" => "Dragon Wars".to_string(),
            "BLOCKING_DEAD" => "Blocking Dead".to_string(),
            "CAPTURE_THE_WOOL" => "Capture the Wool".to_string(),
            "PVP_CTW" => "Capture the Wool".to_string(),
            "EASTER_SIMULATOR" => "Easter Simulator".to_string(),
            "SCUBA_SIMULATOR" => "Scuba Simulator".to_string(),
            "HALLOWEEN_SIMULATOR" => "Halloween Simulator".to_string(),
            "GRINCH_SIMULATOR_V2" => "Grinch Simulator".to_string(),
            "SANTA_SIMULATOR" => "Santa Simulator".to_string(),
            "HYPIXEL_SAYS" => "Hypixel Says".to_string(),
            "CREEPER_ATTACK" => "Creeper Attack".to_string(),
            "SIMON_SAYS" => "Simon Says".to_string(),
            "SANTA_SAYS" => "Santa Says".to_string(),
            "DAY_ONE" => "Day One".to_string(),

            // === WOOL GAMES ===
            "WOOL_WARS_TWO_FOUR" => "Wool Wars".to_string(),
            "SHEEP_WARS" => "Sheep Wars".to_string(),

            // === UHC ===
            "SOLO" => "UHC Solo".to_string(),
            "TEAMS" => "UHC Teams".to_string(),

            // === THE PIT ===
            "PIT" => "The Pit".to_string(),

            // === BUILD BATTLE ===
            "BUILD_BATTLE_SOLO_NORMAL" => "Build Battle Solo".to_string(),
            "BUILD_BATTLE_TEAMS_NORMAL" => "Build Battle Teams".to_string(),
            "BUILD_BATTLE_SOLO_PRO" => "Build Battle Pro".to_string(),
            "BUILD_BATTLE_GUESS_THE_BUILD" => "Guess the Build".to_string(),
            "BUILD_BATTLE_SOLO_NORMAL_LATEST" => "Build Battle Solo".to_string(),
            "BUILD_BATTLE_TEAMS_NORMAL_LATEST" => "Build Battle Teams".to_string(),

            // === SKYBLOCK ===
            "dynamic" => "SkyBlock".to_string(),
            "hub" => "SkyBlock Hub".to_string(),
            "farming_1" => "SkyBlock Farming".to_string(),
            "mining_1" => "SkyBlock Deep Caverns".to_string(),
            "mining_2" => "SkyBlock Dwarven Mines".to_string(),
            "mining_3" => "SkyBlock Crystal Hollows".to_string(),
            "combat_1" => "SkyBlock Spider's Den".to_string(),
            "combat_2" => "SkyBlock Blazing Fortress".to_string(),
            "combat_3" => "SkyBlock The End".to_string(),
            "foraging_1" => "SkyBlock Park".to_string(),
            "dungeon_hub" => "SkyBlock Dungeon Hub".to_string(),
            "dungeon" => "SkyBlock Dungeons".to_string(),
            "crimson_isle" => "SkyBlock Crimson Isle".to_string(),
            "rift" => "SkyBlock Rift".to_string(),
            "garden" => "SkyBlock Garden".to_string(),
            "kuudra_normal" => "Kuudra Basic".to_string(),
            "kuudra_hot" => "Kuudra Hot".to_string(),
            "kuudra_burning" => "Kuudra Burning".to_string(),
            "kuudra_fiery" => "Kuudra Fiery".to_string(),
            "kuudra_infernal" => "Kuudra Infernal".to_string(),
            "instanced" => "SkyBlock Instanced".to_string(),
            "dark_auction" => "SkyBlock Dark Auction".to_string(),
            "winter" => "SkyBlock Jerry's Workshop".to_string(),

            // === MEGA WALLS ===
            "standard" => "Mega Walls".to_string(),
            "face_off" => "Mega Walls Face Off".to_string(),

            // === COPS AND CRIMS ===
            "normal" => "Cops and Crims".to_string(),
            "deathmatch" => "Cops and Crims Deathmatch".to_string(),
            "normal_party" => "Cops and Crims Party".to_string(),

            // === TNT GAMES ===
            "TNTRUN" => "TNT Run".to_string(),
            "PVPRUN" => "PVP Run".to_string(),
            "BOWSPLEEF" => "Bow Spleef".to_string(),
            "TNTAG" => "TNT Tag".to_string(),
            "CAPTURE" => "TNT Wizards".to_string(),

            // === WARLORDS ===
            "ctf_mini" => "Warlords CTF".to_string(),
            "domination" => "Warlords Domination".to_string(),
            "team_deathmatch" => "Warlords TDM".to_string(),

            // === SMASH HEROES ===
            "1v1_normal" => "Smash 1v1".to_string(),
            "2v2_normal" => "Smash 2v2".to_string(),

            // Fallback: clean up the string
            _ => {
                let cleaned = mode
                    .strip_prefix("DUELS_")
                    .or_else(|| mode.strip_prefix("BEDWARS_"))
                    .or_else(|| mode.strip_prefix("SKYWARS_"))
                    .or_else(|| mode.strip_prefix("MURDER_"))
                    .or_else(|| mode.strip_prefix("BUILD_BATTLE_"))
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
            // Special handling for SkyBlock - modes are locations, not games
            if state.current_gametype.as_deref() == Some("SKYBLOCK") {
                if let Some(ref mode) = state.current_mode {
                    // SkyBlock location - "In SkyBlock Hub", "In SkyBlock Dungeons", etc.
                    format!("In {}", Self::format_skyblock_location(mode))
                } else {
                    "In SkyBlock".to_string()
                }
            } else if let Some(ref mode) = formatted_mode {
                // Playing with a known game mode - "Playing Combo Duel"
                format!("Playing {}", mode)
            } else if state.in_lobby {
                // In a specific game lobby - format based on gametype
                if let Some(ref gametype) = state.current_gametype {
                    format!("In {} Lobby", Self::format_gametype(gametype))
                } else {
                    "In Hypixel Lobby".to_string()
                }
            } else {
                // Connected to Hypixel but no specific location
                "In Hypixel Lobby".to_string()
            }
        } else if state.current_mode.as_deref() == Some("Launching") {
            "Launching".to_string()
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
    pub fn set_game_mode(
        &self,
        mode: Option<String>,
        map: Option<String>,
        gametype: Option<String>,
        lobbyname: Option<String>,
    ) {
        self.send(RpcCommand::SetGameMode {
            mode,
            map,
            gametype,
            lobbyname,
        });
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
