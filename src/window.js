const windowLink = window.__TAURI__.window;
let invoke = window.__TAURI__.primitives.invoke;

async function check_for_config() {
    let config = await window.__TAURI__.fs.readTextFile("config.json");
    if (config == "") {
        windowLink.open("Setup");
    }
}

new windowLink.Window("Setup", { url: "setup.html", width: 750, height: 500});