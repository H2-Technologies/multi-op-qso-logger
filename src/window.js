const windowLink = window.__TAURI__.window;
let invoke = window.__TAURI__.primitives.invoke;

async function check_for_config() {
    invoke('check_for_config_file').then((res) => {
        if (res === false) {
            windowLink.Window("Setup", { url: "setup.html", width: 750, height: 500});
        }
    });
}