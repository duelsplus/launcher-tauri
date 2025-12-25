import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";

interface NotificationOptions {
  title: string;
  body: string;
}

export async function notify({
  title,
  body,
}: NotificationOptions) {
  try {
    let permissionGranted = await isPermissionGranted();

    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === "granted";
    }
    if (!permissionGranted) return false;

    sendNotification({ title, body });
    return true;
  } catch (err) {
    return false;
  }
}
