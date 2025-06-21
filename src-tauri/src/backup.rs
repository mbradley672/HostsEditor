use crate::{HostEntry, BackupInfo, hosts};
use std::path::PathBuf;
use tokio::fs;
use chrono::{DateTime, Local};

// Get the backup directory path
pub fn get_backup_directory() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".hosts-editor");
    path.push("backups");
    path
}

// Initialize the backup directory
pub async fn init_backup_directory() -> Result<(), Box<dyn std::error::Error>> {
    let backup_dir = get_backup_directory();
    if !backup_dir.exists() {
        fs::create_dir_all(&backup_dir).await?;
    }
    Ok(())
}

// Create a backup of the current hosts file
pub async fn create_backup(name: &str) -> Result<BackupInfo, Box<dyn std::error::Error>> {
    let backup_dir = get_backup_directory();
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let safe_name = sanitize_filename(name);
    let filename = format!("{}_{}.hosts", safe_name, timestamp);
    let backup_path = backup_dir.join(&filename);
    
    // Ensure backup directory exists
    fs::create_dir_all(&backup_dir).await?;
    
    // Read current hosts file
    let hosts_path = hosts::get_hosts_file_path();
    let content = fs::read_to_string(&hosts_path).await?;
    
    // Write backup
    fs::write(&backup_path, &content).await?;
    
    // Create metadata file
    let metadata = BackupMetadata {
        name: name.to_string(),
        created_at: Local::now(),
        original_path: hosts_path.to_string_lossy().to_string(),
    };
    
    let metadata_path = backup_dir.join(format!("{}.meta", filename.trim_end_matches(".hosts")));
    let metadata_json = serde_json::to_string_pretty(&metadata)?;
    fs::write(&metadata_path, metadata_json).await?;
    
    Ok(BackupInfo {
        name: name.to_string(),
        created_at: metadata.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        path: backup_path.to_string_lossy().to_string(),
    })
}

// List all available backups
pub async fn list_backups() -> Result<Vec<BackupInfo>, Box<dyn std::error::Error>> {
    let backup_dir = get_backup_directory();
    
    if !backup_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut backups = Vec::new();
    let mut entries = fs::read_dir(&backup_dir).await?;
    
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("meta") {
            if let Ok(content) = fs::read_to_string(&path).await {
                if let Ok(metadata) = serde_json::from_str::<BackupMetadata>(&content) {
                    let hosts_file = path.with_extension("hosts");
                    if hosts_file.exists() {
                        backups.push(BackupInfo {
                            name: metadata.name,
                            created_at: metadata.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                            path: hosts_file.to_string_lossy().to_string(),
                        });
                    }
                }
            }
        }
    }
    
    // Sort by creation time (newest first)
    backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    
    Ok(backups)
}

// Restore a backup
pub async fn restore_backup(backup_name: &str) -> Result<Vec<HostEntry>, Box<dyn std::error::Error>> {
    let backups = list_backups().await?;
    
    let backup = backups
        .iter()
        .find(|b| b.name == backup_name)
        .ok_or_else(|| format!("Backup '{}' not found", backup_name))?;
    
    // Read backup content
    let backup_content = fs::read_to_string(&backup.path).await?;
    
    // Parse the backup content to get entries
    let entries = hosts::parse_hosts_content(&backup_content);
    
    // Write to hosts file
    hosts::save_hosts_file(&entries).await?;
    
    Ok(entries)
}

// Delete a backup
pub async fn delete_backup(backup_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let backups = list_backups().await?;
    
    let backup = backups
        .iter()
        .find(|b| b.name == backup_name)
        .ok_or_else(|| format!("Backup '{}' not found", backup_name))?;
    
    let backup_path = PathBuf::from(&backup.path);
    let meta_path = backup_path.with_extension("meta");
    
    // Delete both files
    if backup_path.exists() {
        fs::remove_file(&backup_path).await?;
    }
    
    if meta_path.exists() {
        fs::remove_file(&meta_path).await?;
    }
    
    Ok(())
}

// Sanitize filename for cross-platform compatibility
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '|' | '?' | '*' | '/' | '\\' => '_',
            _ => c,
        })
        .collect::<String>()
        .trim()
        .to_string()
}

#[derive(serde::Serialize, serde::Deserialize)]
struct BackupMetadata {
    name: String,
    created_at: DateTime<Local>,
    original_path: String,
}
