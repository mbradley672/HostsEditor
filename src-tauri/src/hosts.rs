use crate::HostEntry;
use regex::Regex;
use std::path::PathBuf;
use tokio::fs;

// Get the hosts file path based on the operating system
pub fn get_hosts_file_path() -> PathBuf {
    if cfg!(windows) {
        PathBuf::from(r"C:\Windows\System32\drivers\etc\hosts")
    } else {
        PathBuf::from("/etc/hosts")
    }
}

// Load and parse the hosts file
pub async fn load_hosts_file() -> Result<Vec<HostEntry>, Box<dyn std::error::Error>> {
    let hosts_path = get_hosts_file_path();

    let content = match fs::read_to_string(&hosts_path).await {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read hosts file: {}", e);
            return Ok(Vec::new()); // Return empty if file doesn't exist or can't be read
        }
    };

    Ok(parse_hosts_content(&content))
}

// Parse hosts file content into HostEntry structs
pub fn parse_hosts_content(content: &str) -> Vec<HostEntry> {
    let mut entries = Vec::new();
    let entry_regex = Regex::new(r"^\s*([^#\s]+)\s+([^#\s]+)(?:\s*#\s*(.*))?$").unwrap();
    let disabled_entry_regex =
        Regex::new(r"^\s*#\s*([^#\s]+)\s+([^#\s]+)(?:\s*#\s*(.*))?$").unwrap();

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        // Check for disabled entries (commented out IP + hostname)
        if let Some(captures) = disabled_entry_regex.captures(line) {
            let ip = captures.get(1).unwrap().as_str().to_string();
            let hostname = captures.get(2).unwrap().as_str().to_string();
            let comment = captures
                .get(3)
                .map_or(String::new(), |m| m.as_str().to_string());

            // Only add if it looks like an IP address
            if is_valid_ip(&ip) {
                entries.push(HostEntry {
                    ip,
                    hostname,
                    comment,
                    enabled: false,
                });
                continue;
            }
        }

        // Check for enabled entries
        if let Some(captures) = entry_regex.captures(line) {
            let ip = captures.get(1).unwrap().as_str().to_string();
            let hostname = captures.get(2).unwrap().as_str().to_string();
            let comment = captures
                .get(3)
                .map_or(String::new(), |m| m.as_str().to_string());

            // Only add if it looks like an IP address
            if is_valid_ip(&ip) {
                entries.push(HostEntry {
                    ip,
                    hostname,
                    comment,
                    enabled: true,
                });
            }
        }
    }

    entries
}

// Simple IP validation
fn is_valid_ip(ip: &str) -> bool {
    // Check for IPv4
    if ip.split('.').count() == 4 {
        return ip.split('.').all(|part| part.parse::<u8>().is_ok());
    }

    // Check for IPv6 (basic check)
    if ip.contains(':') {
        return ip.chars().all(|c| c.is_ascii_hexdigit() || c == ':');
    }

    false
}

// Save hosts entries back to the hosts file
pub async fn save_hosts_file(entries: &[HostEntry]) -> Result<(), Box<dyn std::error::Error>> {
    let hosts_path = get_hosts_file_path();

    // Read the original file to preserve comments and other content
    let original_content = fs::read_to_string(&hosts_path).await.unwrap_or_default();

    // Generate new content
    let new_content = generate_hosts_content(entries, &original_content);

    // Create backup before writing
    if let Err(e) = create_emergency_backup().await {
        eprintln!("Warning: Failed to create emergency backup: {}", e);
    }

    // Write the new content
    fs::write(&hosts_path, new_content).await?;

    Ok(())
}

// Generate the new hosts file content, preserving system entries and comments
fn generate_hosts_content(entries: &[HostEntry], _original_content: &str) -> String {
    let mut result = String::new();

    // Add header comment
    result.push_str("# Hosts file managed by Hosts Editor\n");
    result.push_str("# Last modified: ");
    result.push_str(&chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
    result.push_str("\n\n");

    // Preserve system entries (localhost, etc.)
    result.push_str("# System entries\n");
    result.push_str("127.0.0.1\tlocalhost\n");
    result.push_str("::1\tlocalhost\n");

    if cfg!(windows) {
        result.push_str("127.0.0.1\tlocalhost.localdomain\n");
    }

    result.push_str("\n# Custom entries\n");

    // Add user entries
    for entry in entries {
        if entry.enabled {
            result.push_str(&format!("{}\t{}", entry.ip, entry.hostname));
            if !entry.comment.is_empty() {
                result.push_str(&format!("\t# {}", entry.comment));
            }
            result.push('\n');
        } else {
            result.push_str(&format!("# {}\t{}", entry.ip, entry.hostname));
            if !entry.comment.is_empty() {
                result.push_str(&format!("\t# {}", entry.comment));
            }
            result.push('\n');
        }
    }

    result
}

// Create an emergency backup before modifying the hosts file
async fn create_emergency_backup() -> Result<(), Box<dyn std::error::Error>> {
    let hosts_path = get_hosts_file_path();
    let backup_path = if cfg!(windows) {
        PathBuf::from(r"C:\Windows\System32\drivers\etc\hosts.backup")
    } else {
        PathBuf::from("/etc/hosts.backup")
    };

    if let Ok(content) = fs::read(&hosts_path).await {
        fs::write(backup_path, content).await?;
    }

    Ok(())
}
