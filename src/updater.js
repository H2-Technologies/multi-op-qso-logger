let updater = window.__TAURI__.updater;
let update = await updater.check();
if (update.response.available) {
  await update.downloadAndInstall();
  await update.relaunch();
}