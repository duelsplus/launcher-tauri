import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import type { Config } from "@/types/config";
import { settingDefinitions } from "@/settings/definitions";
import { SettingSwitch } from "@/components/settings/switch";
import { SettingsSection } from "@/components/settings/section";
import { SettingTheme } from "@/components/settings/theme";
import { defaultSettings } from "@/settings/defaults";
import { ArrowUpRightIcon, SpinnerIcon } from "@phosphor-icons/react";
import { config as configApi } from "@/lib/config";
import { Button } from "../ui/button";
import { useTheme } from "../theme-provider";

export function Settings() {
  const [config, setConfig] = useState<Config | null>(null);
  const [savingKey, setSavingKey] = useState<keyof Config | null>(null);

  const { theme: currentTheme, setTheme: setAppTheme } = useTheme();
  const [theme, setTheme] = useState<"dark" | "light">(
    currentTheme === "light" ? "light" : "dark",
  );

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

  const isDisabled = (setting: (typeof settingDefinitions)[number]) => {
    if (!config) return true;
    let disabled = savingKey === setting.key;

    if (setting.dependsOn) {
      if (typeof setting.dependsOn === "string") {
        disabled ||= !config[setting.dependsOn];
      } else if (typeof setting.dependsOn === "function") {
        disabled ||= !setting.dependsOn(config);
      }
    }

    return disabled;
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

      {grouped["General"] && (
        <SettingsSection title="General">
          <SettingTheme
            title="Theme"
            value={theme}
            onChange={(t) => {
              setTheme(t);
              setAppTheme(t);
            }}
          />
          {grouped["General"].map((setting) => (
            <SettingSwitch
              key={setting.key}
              title={setting.title}
              description={setting.description}
              checked={config[setting.key] as boolean}
              disabled={isDisabled(setting)}
              onCheckedChange={(value) => updateSetting(setting.key, value)}
            />
          ))}
        </SettingsSection>
      )}

      {Object.entries(grouped)
        .filter(([section]) => section !== "General")
        .map(([section, settings]) => (
          <SettingsSection key={section} title={section}>
            {settings.map((setting) => (
              <SettingSwitch
                key={setting.key}
                title={setting.title}
                description={setting.description}
                checked={config[setting.key] as boolean}
                disabled={isDisabled(setting)}
                onCheckedChange={(value) => updateSetting(setting.key, value)}
              />
            ))}
          </SettingsSection>
        ))}

      <div className="w-full flex justify-center">
        <a
          href="https://dash.duelsplus.com"
          target="_blank"
          rel="noopener noreferrer"
        >
          <Button variant="input" size="sm">
            Manage your preferences
            <ArrowUpRightIcon />
          </Button>
        </a>
      </div>
    </div>
  );
}
