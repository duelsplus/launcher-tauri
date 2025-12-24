import type { Config } from "@/types/config";

export type SettingDefinition = {
  key: keyof Config;
  title: string;
  description?: string;
  section: string;
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
  /*{
    key: "enableMsa",
    title: "Microsoft Account Authentication",
    description: "Enable Microsoft Account login support.",
    section: "Advanced",
  },*/
];
