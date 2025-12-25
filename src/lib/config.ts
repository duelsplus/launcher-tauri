import { invoke } from "@tauri-apps/api/core";
import type { Config } from "@/types/config";
import { defaultSettings } from "@/settings/defaults";

export const config = {
  /**
   * Check if config file exists.
   */
  async exists(): Promise<boolean> {
    return invoke<boolean>("config_exists");
  },

  /**
   * Check if legacy config file exists.
   */
  async legacyExists(): Promise<boolean> {
    return invoke<boolean>("legacy_config_exists");
  },

  /**
   * Get full config.
   * If missing, returns defaults and optionally persists them.
   */
  async get(options?: { initIfMissing?: boolean }): Promise<Config> {
    const cfg = await invoke<Config | null>("get_config");
    if (cfg) return cfg;
    if (options?.initIfMissing !== false) {
      await invoke("save_config", { config: defaultSettings });
    }
    return defaultSettings;
  },

  /**
   * Read a single config key.
   */
  async getValue<K extends keyof Config>(key: K): Promise<Config[K]> {
    const value = await invoke<Config[K] | null>("get_config_value", { key });
    if (value !== null) return value;
    return defaultSettings[key];
  },

  /**
   * Set a single config key.
   * RPC-related settings are automatically synced by the backend.
   */
  async setValue<K extends keyof Config>(
    key: K,
    value: Config[K],
  ): Promise<void> {
    await invoke("set_config_key", {
      key,
      value,
    });
  },

  /**
   * Save entire config object.
   */
  async save(cfg: Config): Promise<void> {
    await invoke("save_config", { config: cfg });
  },
};
