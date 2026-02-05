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
import type { Theme } from "../theme-provider";
import { SettingButton } from "../settings/button";
import { RpcCustomizeDialog } from "@/components/dialogs/rpc-customize";
import { SettingInput } from "../settings/input";
import { User, hasPerm } from "@/lib/perm";
import { getToken } from "@/lib/token";
import { RestartPendingDialog } from "../dialogs/restart-pending";
import { EnableBetaDialog } from "../dialogs/enable-beta";

type ApiResponse<T> = {
  success: boolean;
  data: T;
};

export function Settings() {
  const [user, setUser] = useState<User | null>(null);
  const [config, setConfig] = useState<Config | null>(null);
  const [savingKey, setSavingKey] = useState<keyof Config | null>(null);
  const [restartPending, setRestartPending] = useState(false);
  const [restartPendingName, setRestartPendingName] = useState("");
  const [enableBetaDialogOpen, setEnableBetaDialogOpen] = useState(false);
  const [pendingBetaValue, setPendingBetaValue] = useState<boolean | null>(
    null,
  );

  useEffect(() => {
    invoke<ApiResponse<User>>("get_user", {
      token: getToken(),
    })
      .then((u) => setUser(u.data))
      .catch(() => setUser(null));
  }, []);

  const { theme: currentTheme, setTheme: setAppTheme } = useTheme();
  const [theme, setTheme] = useState<Theme>(() => {
    if (
      currentTheme === "light" ||
      currentTheme === "dark" ||
      currentTheme === "classic"
    ) {
      return currentTheme;
    }

    return "dark";
  });

  const [rpcCustomizeOpen, setRpcCustomizeOpen] = useState(false);

  const isBetaEligible =
    hasPerm(user, "tester") ||
    hasPerm(user, "partner") ||
    hasPerm(user, "developer") ||
    hasPerm(user, "admin");

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
      const settingDef = settingDefinitions.find((s) => s.key === key);
      if (settingDef?.restartRequired) {
        setRestartPending(true);
        setRestartPendingName(settingDef.title);
      }
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

  const handlePortChange = (raw: string) => {
    if (!/^\d*$/.test(raw)) return;
    if (raw === "") return;
    let port = Number(raw);
    if (!Number.isInteger(port)) return;
    if (port < 1) port = 1;
    if (port > 65535) port = 65535;
    updateSetting("proxyPort", String(port));
  };

  if (!config || !user) {
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
          {isBetaEligible && (
            <SettingSwitch
              key="receiveBetaReleases"
              title="Receive Beta Releases"
              description="Receive new proxy updates early with new features and fixes."
              checked={config["receiveBetaReleases"] as boolean}
              onCheckedChange={(value) => {
                if (value) {
                  setPendingBetaValue(value);
                  setEnableBetaDialogOpen(true);
                } else {
                  updateSetting("receiveBetaReleases", value);
                }
              }}
            />
          )}
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

      {grouped["Integrations"] && (
        <SettingsSection title="Integrations">
          {grouped["Integrations"].map((setting) => (
            <SettingSwitch
              key={setting.key}
              title={setting.title}
              description={setting.description}
              checked={config[setting.key] as boolean}
              disabled={isDisabled(setting)}
              onCheckedChange={(value) => updateSetting(setting.key, value)}
            />
          ))}

          <SettingButton
            title="Customize Rich Presence"
            description="Change the appearance of the Discord Rich Presence."
            onClick={() => setRpcCustomizeOpen(true)}
          />
        </SettingsSection>
      )}

      <SettingsSection title="Advanced">
        <SettingInput
          title="Proxy Port"
          description="Set the localhost port."
          //type="number"
          value={config.proxyPort}
          min={1}
          max={100}
          step={1}
          onChange={handlePortChange}
        />
      </SettingsSection>

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

      <RpcCustomizeDialog
        open={rpcCustomizeOpen}
        onOpenChange={setRpcCustomizeOpen}
      />

      <RestartPendingDialog
        open={restartPending}
        onOpenChange={setRestartPending}
        name={restartPendingName}
      />

      <EnableBetaDialog
        open={enableBetaDialogOpen}
        onOpenChange={setEnableBetaDialogOpen}
        onContinue={() => {
          if (pendingBetaValue !== null) {
            setRestartPending(true);
            setRestartPendingName("Receive Beta Releases");
            updateSetting("receiveBetaReleases", pendingBetaValue);
            setPendingBetaValue(null);
          }
        }}
      />
    </div>
  );
}
