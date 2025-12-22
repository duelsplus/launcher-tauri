import { check } from "@tauri-apps/plugin-updater";
import { create } from "zustand";

export type UpdateStatus =
  | { state: "idle" }
  | { state: "checking" }
  | { state: "downloading"; chunkLength: number }
  | { state: "pending-restart" }
  | { state: "error" };

interface UpdaterStore {
  status: UpdateStatus;
  checkAndInstall: () => Promise<void>;
}

export const useUpdater = create<UpdaterStore>((set) => ({
  status: { state: "idle" },
  async checkAndInstall() {
    try {
      set({ status: { state: "checking" } });

      const update = await check();
      if (update === null) {
        set({ status: { state: "idle" } });
        return;
      }

      set({ status: { state: "downloading", chunkLength: 0 } });

      await update.downloadAndInstall((event) => {
        if (event.event === "Progress") {
          set({
            status: {
              state: "downloading",
              chunkLength: event.data.chunkLength,
            },
          });
        }
      });

      set({ status: { state: "pending-restart" } });
    } catch (err) {
      set({ status: { state: "error" } });
    }
  },
}));

export const useMockUpdater = create<UpdaterStore>((set) => ({
  status: { state: "idle" },
  async checkAndInstall() {
    set({ status: { state: "checking" } });
    await new Promise((r) => setTimeout(r, 5000));
    for (let i = 1; i <= 20; i++) {
      set({ status: { state: "downloading", chunkLength: i * 1024 * 50 } });
      await new Promise((r) => setTimeout(r, 300));
    }

    set({ status: { state: "pending-restart" } });
  },
}));
