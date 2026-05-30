use std::fs::{self, OpenOptions, read_to_string, write};
use std::io::Write;
// Note: 'path' warning ko ignore karne ke liye hum use kar rahe hain
use std::path::Path; 

const HOSTS_PATH: &str = r"C:\Windows\System32\drivers\etc\hosts";

// --- 1. SCAN FOLDER FOR NAMES ---
#[tauri::command]
fn get_sound_files() -> Vec<String> {
    let mut sounds = Vec::new();
    // Exe ke bagal me "sounds" folder dhoondho
    if let Ok(entries) = fs::read_dir("sounds") {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    // Sirf Audio files allow karo
                    if ext_str == "mp3" || ext_str == "wav" || ext_str == "ogg" {
                        if let Some(name) = path.file_name() {
                             sounds.push(name.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
    }
    sounds
}

// --- 2. NEW: LOAD AUDIO DATA (BYTES) ---
#[tauri::command]
fn load_audio_file(filename: String) -> Option<Vec<u8>> {
    let path = Path::new("sounds").join(filename);
    // File ko read karke bytes bhejo
    fs::read(path).ok()
}

// --- BLOCKING LOGIC ---
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

    if let Err(_) = writeln!(file, "\n# ZENBLOCK START") { return "Write Error".to_string(); }

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
    
    if let Err(_) = writeln!(file, "# ZENBLOCK END") { return "Write Error".to_string(); }
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_blocking, stop_blocking, get_sound_files, load_audio_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}