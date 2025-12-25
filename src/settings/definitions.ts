import type { Config } from "@/types/config";

export type SettingDefinition = {
  key: keyof Config;
  title: string;
  description?: string;
  section: string;
  dependsOn?: keyof Config | ((config: Config) => boolean);
};

export const settingDefinitions: SettingDefinition[] = [
  /*{
    key: "minimizeToTray",
    title: "Minimize to Tray",
    description: "Keep the launcher running in the background when closed.",
    section: "General",
  },*/
  {
    key: "autoUpdate",
    title: "Automatic Updates",
    description: "Automatically download and install launcher updates.",
    section: "General",
  },
  {
    key: "openLogsOnLaunch",
    title: "Open Logs on Launch",
    description: "Switch to the logs tab on proxy launch.",
    section: "General",
  },
  /*{
    key: "reducedMotion",
    title: "Reduced Motion",
    description: "Reduce animations for accessibility.",
    section: "Accessibility",
  },*/
  {
    key: "enableRpc",
    title: "Discord Rich Presence",
    description: "Show your activity on Discord.",
    section: "Integrations",
  },
  {
    key: "rpcAnonymizeProfile",
    title: "Anonymize Profile",
    description: "Hide your in-game profile from Discord Rich Presence.",
    section: "Integrations",
    dependsOn: "enableRpc",
  },
  {
    key: "rpcAnonymizeLocation",
    title: "Anonymize Location",
    description: "Hide your in-game location from Discord Rich Presence.",
    section: "Integrations",
    dependsOn: "enableRpc",
  },
  /*{
    key: "enableMsa",
    title: "Microsoft Account Authentication",
    description: "Enable Microsoft Account login support.",
    section: "Advanced",
  },*/
];
