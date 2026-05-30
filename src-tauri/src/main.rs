// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Runtime};
use std::fs::{self, OpenOptions, read_to_string, write};
use std::io::Write;

const HOSTS_PATH: &str = r"C:\Windows\System32\drivers\etc\hosts";

// --- 1. SOUND COMMANDS (AppData Folder) ---
#[tauri::command]
fn get_sound_files<R: Runtime>(app: tauri::AppHandle<R>) -> Vec<String> {
    let data_dir = app.path().app_local_data_dir().unwrap().join("sounds");
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(data_dir) {
        for entry in entries.flatten() {
            if let Ok(file_name) = entry.file_name().into_string() {
                if file_name.ends_with(".mp3") || file_name.ends_with(".wav") || file_name.ends_with(".ogg") {
                    files.push(file_name);
                }
            }
        }
    }
    files
}

#[tauri::command]
async fn load_audio_file<R: Runtime>(app: tauri::AppHandle<R>, filename: String) -> Result<Vec<u8>, String> {
    let file_path = app.path().app_local_data_dir().unwrap().join("sounds").join(filename);
    fs::read(file_path).map_err(|e| e.to_string())
}

// --- 2. BLOCKING COMMANDS (Hosts File Logic) ---
#[tauri::command]
fn start_blocking(sites: Vec<String>) -> String {
    let current_content = match read_to_string(HOSTS_PATH) {
        Ok(c) => c,
        Err(e) => return format!("ERROR: {}", e),
    };

    if current_content.contains("# ZENBLOCK START") {
        return "Already blocking!".to_string();
    }

    let mut file = match OpenOptions::new().append(true).open(HOSTS_PATH) {
        Ok(f) => f,
        Err(e) => return format!("ACCESS DENIED: Run as Admin! {}", e),
    };

    let _ = writeln!(file, "\n# ZENBLOCK START");
    for site in sites {
        let clean = site.trim();
        if !clean.is_empty() {
            let _ = writeln!(file, "127.0.0.1 {}", clean);
            let _ = writeln!(file, "::1 {}", clean);
            if !clean.starts_with("www.") {
                let _ = writeln!(file, "127.0.0.1 www.{}", clean);
                let _ = writeln!(file, "::1 www.{}", clean);
            }
        }
    }
    let _ = writeln!(file, "# ZENBLOCK END");
    "Blocked".to_string()
}

#[tauri::command]
fn stop_blocking() -> String {
    let current_content = match read_to_string(HOSTS_PATH) {
        Ok(c) => c,
        Err(e) => return e.to_string(),
    };

    let new_content: String = current_content
        .lines()
        .filter(|line| !line.contains("# ZENBLOCK") && !line.contains("127.0.0.1") && !line.contains("::1"))
        .collect::<Vec<&str>>()
        .join("\n");

    match write(HOSTS_PATH, new_content) {
        Ok(_) => "Unblocked".to_string(),
        Err(e) => format!("ERROR: {}", e),
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_path = app.path().app_local_data_dir().unwrap().join("sounds");
            
            // AppData mein folder banayein
            if !app_data_path.exists() {
                fs::create_dir_all(&app_data_path).unwrap();
                
                // default_sounds resources se copy karein
                if let Ok(resource_path) = app.path().resolve("default_sounds", tauri::path::BaseDirectory::Resource) {
                    if let Ok(entries) = fs::read_dir(resource_path) {
                        for entry in entries.flatten() {
                            let dest = app_data_path.join(entry.file_name());
                            fs::copy(entry.path(), dest).ok();
                        }
                    }
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_blocking, 
            stop_blocking, 
            get_sound_files, 
            load_audio_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}