// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri_plugin_updater::UpdaterExt;
use tauri_plugin_http::reqwest;
use csv;
use std::write;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
} 

#[derive(Debug)]
struct QSO {
    callsign: String,
    firstname: String,
    lastname: String,
    city: String,
    state: String,
    zip: String
}

fn download_csv_if_not_present() -> std::path::PathBuf {
    let system_os = std::env::consts::OS;
    let docDir: std::path::PathBuf;
    if system_os == "windows" {
        let username = std::env::var("USERNAME").unwrap();
        docDir = std::path::Path::new(&("C:\\Users\\".to_owned() + &username + "\\Documents\\multi-op-qso-logger")).to_path_buf();
    } else if system_os == "linux" {
        docDir = std::path::Path::new("/etc/multi-op-qso-logger").to_path_buf();
    } else if system_os == "macos" {
        let username = std::process::Command::new("whoami").output().unwrap();
        let username = String::from_utf8(username.stdout).unwrap();
        docDir = std::path::Path::new(&("/Users/".to_owned() + &username + "/Library/multi-op-qso-logger")).to_path_buf();
    } else {
        docDir = std::path::Path::new("/etc/multi-op-qso-logger").to_path_buf();
    }
    if !docDir.exists() {
        // if the command errors, print the error but don't exit
        let _ = std::fs::create_dir(docDir.clone()).unwrap();
    }
    let csvPath = docDir.join("EN.csv");
    println!("{:?}", csvPath);
    if !csvPath.exists() {
        let file = std::fs::File::create(csvPath.clone()).unwrap();
    }
    csvPath.to_path_buf()
}

#[tauri::command]
fn parse_csv() {
    let mut qso_vec: Vec<QSO> = Vec::new();
    let csvPath = download_csv_if_not_present();
    let mut rdr = csv::Reader::from_path(csvPath).unwrap();
    //the header contains these columns, "lang,number1,blank1,blank2,callsign,letter1,string1,fullname,firstname,middleinit,lastname,blank4,blank5,blank6,blank7,streetaddr,city,state,zip,blank8,blank9,blank10,number2,letter2,blank11,blank12,blank13,blank14,blank15"
    for result in rdr.records() {
        //let record = result;
        println!("{:?}", result);
        // print the format of the record
        //println!("{:?}", record.expect("REASON").as_slice());
        //let callsign = record.get(4).unwrap();
        //let firstname = record.get(8).unwrap();
        //let lastname = record.get(10).unwrap();
        //let city = record.get(16).unwrap();
        //let state = record.get(17).unwrap();
        //let zip = record.get(18).unwrap();
        //let qso = QSO {
        //    callsign: callsign.to_string(),
        //    firstname: firstname.to_string(),
        //    lastname: lastname.to_string(),
        //    city: city.to_string(),
        //    state: state.to_string(),
        //    zip: zip.to_string()
        //};
        //qso_vec.push(qso);
    }
    //println!("{:?}", qso_vec);
}

async fn download_and_save(csvPath: std::path::PathBuf) {
    let mut response = reqwest::get("https://gitlab.austinh.dev/root/ham_radio_logger/-/blob/56d1b79827015d9a2e15b11575c5948c39e5be1f/EN.csv").await.unwrap();
    println!("Status: {}", response.status());
    println!("Headers:\n{:#?}", response.headers());
    println!("Body:\n{}", response.text().await.unwrap());
    //write!(csvPath, response).unwrap();
}

//invoke("qso_vec", {callsign: "KE8YGW", frequency: 14.255, mode: "SSB", rst_sent: 59, rst_recieved: 59, operator: "KE8YGW", comment: "OHIO"});
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        //.plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let response = handle.updater().expect("REASON").check().await; // If .await?; works in the setup hook you can remove the if let Ok line - can't try that myself right now.
                if let Ok(response) = response {
                  if let Some(response) = response {
                    // The first || {} ignores the download progress
                    // The second || {} ignores the download finished event
                    // If you wanna handle them you can write actual functions instead
                    let _ = response.download_and_install(|_,_| {}, || {}).await; // this returns a result you may wanna handle
                    println!("Update downloaded and installed");
                    println!("{:?}", response);
                  }
                }
            });
            Ok(())
        })
        .plugin(tauri_plugin_window::init())
        .invoke_handler(tauri::generate_handler![greet, parse_csv])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
