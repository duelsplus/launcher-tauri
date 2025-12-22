import { getCurrentWindow } from "@tauri-apps/api/window";

export async function showWindow() {
  await getCurrentWindow().show();
  await getCurrentWindow().setFocus();
}
