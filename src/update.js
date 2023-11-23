let current_verison = window.__TAURI__.app.version();
let updater = window.__TAURI__.updater;

function check_for_updates() {
    updater.check().then((update) => {
        console.log(update);
        if (update.available) {
            updater.download();
        }
    });
}
    
function insert_versions() {
}