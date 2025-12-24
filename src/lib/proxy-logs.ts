import { create } from "zustand";
import { listen } from "@tauri-apps/api/event";

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
