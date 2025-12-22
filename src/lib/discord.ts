import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export type DiscordAuthResult = {
  success: boolean;
  token?: string; // Present on success
  error?: string; // Present on failure
};

export type DiscordAuthOutcome = {
  success: boolean;
  token?: string; // Present on success
  message?: string; // Present on failure
};

export async function startDiscordAuth(): Promise<DiscordAuthOutcome> {
  return new Promise(async (resolve) => {
    let unlisten: (() => void) | null = null;

    try {
      unlisten = await listen<DiscordAuthResult>(
        "discord-auth-result",
        async (event) => {
          const payload = event.payload;
          if (payload.success && payload.token) {
            try {
              const verifyResult = await invoke<{ success: boolean }>(
                "verify_token",
                { token: payload.token },
              );
              if (verifyResult.success) {
                await invoke("save_token", { token: payload.token });
                resolve({ success: true, token: payload.token });
              } else {
                throw new Error;
              }
            } catch {
              resolve({ success: false, message: "Verification failed" });
            }
          } else {
            resolve({
              success: false,
              message: payload.error || "",
            });
          }
          unlisten?.();
        },
      );

      await invoke("start_discord_signin");
    } catch {
      unlisten?.();

      resolve({ success: false, message: "Failed to start Discord sign-in" });
    }
  });
}
