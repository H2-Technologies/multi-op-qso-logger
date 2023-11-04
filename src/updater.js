import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
const update = await check();
if (update.response.available) {
  await update.downloadAndInstall();
  await relaunch();
}