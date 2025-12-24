import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import type { Config } from "@/types/config";
import { settingDefinitions } from "@/settings/definitions";
import { SettingSwitch } from "@/components/settings/switch";
import { SettingsSection } from "@/components/settings/section";
import { defaultSettings } from "@/settings/defaults";
import { SpinnerIcon } from "@phosphor-icons/react";
import { config as configApi } from "@/lib/config";

export function Settings() {
  const [config, setConfig] = useState<Config | null>(null);
  const [savingKey, setSavingKey] = useState<keyof Config | null>(null);

  useEffect(() => {
    configApi
      .get()
      .then((cfg) => {
        setConfig(cfg);
      })
      .catch(() => {
        setConfig(defaultSettings);
      });
  }, []);

  const grouped = useMemo(() => {
    return settingDefinitions.reduce<Record<string, typeof settingDefinitions>>(
      (acc, setting) => {
        acc[setting.section] ??= [];
        acc[setting.section].push(setting);
        return acc;
      },
      {},
    );
  }, []);

  const updateSetting = async <K extends keyof Config>(
    key: K,
    value: Config[K],
  ) => {
    if (!config) return;
    setConfig((prev) => (prev ? { ...prev, [key]: value } : prev));
    setSavingKey(key);

    try {
      await configApi.setValue(key, value);
    } catch {
      //rollback on error
      setConfig(config);
    } finally {
      setSavingKey(null);
    }
  };

  if (!config) {
    return (
      <div className="space-y-4">
        <h2 className="text-base font-medium">Settings</h2>

        <div className="flex justify-center items-center w-full">
          <SpinnerIcon className="animate-spin text-muted-foreground" />
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <h2 className="text-base font-medium">Settings</h2>

      {Object.entries(grouped).map(([section, settings]) => (
        <SettingsSection key={section} title={section}>
          {settings.map((setting) => (
            <SettingSwitch
              key={setting.key}
              title={setting.title}
              description={setting.description}
              checked={config[setting.key] as boolean}
              disabled={savingKey === setting.key}
              onCheckedChange={(value) => updateSetting(setting.key, value)}
            />
          ))}
        </SettingsSection>
      ))}
    </div>
  );
}
