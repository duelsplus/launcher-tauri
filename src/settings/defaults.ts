import type { Config } from "@/types/config";

export const defaultSettings: Config = {
  minimizeToTray: false,
  autoUpdate: true,
  openLogsOnLaunch: true,
  reducedMotion: false,
  enableRpc: true,
  rpcAnonymizeProfile: false,
  rpcAnonymizeLocation: false,
  rpcImage: "logo-v1",
  proxyPort: "25565",
  enableMsa: false,
  receiveBetaReleases: false,
};
