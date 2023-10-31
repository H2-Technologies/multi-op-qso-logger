// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Serialize, Deserialize};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct qso {
    callsign: String,
    frequency: u16,
    mode: String,
    rst_sent: u8,
    rst_recieved: u8,
    operator: String,
    comment: String
}

#[tauri::command]
async fn command_name() -> Result<(), String> {
  Ok(())
}
//invoke("qso_vec", {callsign: "KE8YGW", frequency: 14.255, mode: "SSB", rst_sent: 59, rst_recieved: 59, operator: "KE8YGW", comment: "OHIO"});
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let handle = app.handle();
            tauri::async_runtime::spawn(async move {
                let response = handle.updater().check().await;
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
