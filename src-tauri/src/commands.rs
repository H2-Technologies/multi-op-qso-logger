use tauri_plugin_window;

pub fn create_setup_window() {
    tauri_plugin_window::create_window("setup", "setup.html", 800, 600);
}