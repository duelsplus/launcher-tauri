import { create } from "zustand";
import { listen } from "@tauri-apps/api/event";

export const LOG_LEVELS = [
  "DEBUG",
  "INFO",
  "WARN",
  "ERROR",
] as const;
export type LogLevel = (typeof LOG_LEVELS)[number];

export function getLogLevel(line: string): LogLevel | null {
  const match = line.match(/\[(DEBUG|INFO|WARN|WARNING|ERROR)\]/i);
  return match ? (match[1].toUpperCase() as LogLevel) : null;
}

interface LogsStore {
  logs: string[];
  addLog: (line: string) => void;
  clear: () => void;
}

export const useLogs = create<LogsStore>((set) => {
  listen<string>("log-message", (event) => {
    set((state) => ({
      logs: [...state.logs, event.payload],
    }));
  });

  return {
    logs: [],
    addLog: (line) =>
      set((state) => ({
        logs: [...state.logs, line],
      })),
    clear: () => set({ logs: [] }),
  };
});
