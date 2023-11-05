let check = window.__TAURI__.updater.check();
let process_plugin = window.__TAURI__.process;
async function update() {
  const update = await check;
  if (update.response.available == true) {
    await update.downloadAndInstall();
    await relaunch();
  }
}