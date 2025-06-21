// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod backup;
mod elevation;
mod hosts;

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{command, State};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HostEntry {
    ip: String,
    hostname: String,
    comment: String,
    enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupInfo {
    name: String,
    created_at: String,
    path: String,
}

// Application state
pub struct AppState {
    pub hosts_entries: Mutex<Vec<HostEntry>>,
    pub is_elevated: Mutex<bool>,
}

#[command]
async fn get_raw_hosts_content() -> Result<String, String> {
    let hosts_path = hosts::get_hosts_file_path();
    match tokio::fs::read_to_string(&hosts_path).await {
        Ok(content) => Ok(content),
        Err(e) => Err(format!("Failed to read hosts file: {}", e)),
    }
}

#[command]
async fn save_raw_hosts_content(
    content: String,
    state: State<'_, AppState>,
) -> Result<Vec<HostEntry>, String> {
    // Check if we need elevation
    if !*state.is_elevated.lock().unwrap() {
        if let Err(e) = elevation::request_elevation().await {
            return Err(format!("Failed to get elevation: {}", e));
        }
        *state.is_elevated.lock().unwrap() = true;
    }

    let hosts_path = hosts::get_hosts_file_path();

    // Create backup before writing
    if let Err(e) = backup::create_backup("auto_before_raw_edit").await {
        eprintln!("Warning: Failed to create automatic backup: {}", e);
    }

    match tokio::fs::write(&hosts_path, &content).await {
        Ok(_) => {
            // Parse the new content and update state
            let entries = hosts::parse_hosts_content(&content);
            *state.hosts_entries.lock().unwrap() = entries.clone();
            Ok(entries)
        }
        Err(e) => Err(format!("Failed to save hosts file: {}", e)),
    }
}

#[command]
async fn load_hosts_file(state: State<'_, AppState>) -> Result<Vec<HostEntry>, String> {
    match hosts::load_hosts_file().await {
        Ok(entries) => {
            *state.hosts_entries.lock().unwrap() = entries.clone();
            Ok(entries)
        }
        Err(e) => Err(e.to_string()),
    }
}

#[command]
async fn save_hosts_file(
    entries: Vec<HostEntry>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // Check if we need elevation
    if !*state.is_elevated.lock().unwrap() {
        if let Err(e) = elevation::request_elevation().await {
            return Err(format!("Failed to get elevation: {}", e));
        }
        *state.is_elevated.lock().unwrap() = true;
    }

    match hosts::save_hosts_file(&entries).await {
        Ok(_) => {
            *state.hosts_entries.lock().unwrap() = entries;
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[command]
async fn create_backup(name: String) -> Result<BackupInfo, String> {
    backup::create_backup(&name)
        .await
        .map_err(|e| e.to_string())
}

#[command]
async fn list_backups() -> Result<Vec<BackupInfo>, String> {
    backup::list_backups().await.map_err(|e| e.to_string())
}

#[command]
async fn restore_backup(
    backup_name: String,
    state: State<'_, AppState>,
) -> Result<Vec<HostEntry>, String> {
    // Check if we need elevation
    if !*state.is_elevated.lock().unwrap() {
        if let Err(e) = elevation::request_elevation().await {
            return Err(format!("Failed to get elevation: {}", e));
        }
        *state.is_elevated.lock().unwrap() = true;
    }

    match backup::restore_backup(&backup_name).await {
        Ok(entries) => {
            *state.hosts_entries.lock().unwrap() = entries.clone();
            Ok(entries)
        }
        Err(e) => Err(e.to_string()),
    }
}

#[command]
async fn delete_backup(backup_name: String) -> Result<(), String> {
    backup::delete_backup(&backup_name)
        .await
        .map_err(|e| e.to_string())
}

#[command]
async fn check_elevation() -> Result<bool, String> {
    elevation::is_elevated().await.map_err(|e| e.to_string())
}

#[command]
async fn request_elevation_command(state: State<'_, AppState>) -> Result<bool, String> {
    match elevation::request_elevation().await {
        Ok(_) => {
            *state.is_elevated.lock().unwrap() = true;
            Ok(true)
        }
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    let app_state = AppState {
        hosts_entries: Mutex::new(Vec::new()),
        is_elevated: Mutex::new(false),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            load_hosts_file,
            save_hosts_file,
            get_raw_hosts_content,
            save_raw_hosts_content,
            create_backup,
            list_backups,
            restore_backup,
            delete_backup,
            check_elevation,
            request_elevation_command
        ])
        .setup(|_app| {
            // Initialize backup directory
            tauri::async_runtime::spawn(async {
                if let Err(e) = backup::init_backup_directory().await {
                    eprintln!("Failed to initialize backup directory: {}", e);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
