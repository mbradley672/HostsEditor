use std::process::Command;

// Check if the current process is running with elevated privileges
pub async fn is_elevated() -> Result<bool, Box<dyn std::error::Error>> {
    #[cfg(windows)]
    {
        is_elevated_windows().await
    }
    #[cfg(not(windows))]
    {
        is_elevated_unix().await
    }
}

// Request elevation (restart the application with elevated privileges)
pub async fn request_elevation() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(windows)]
    {
        request_elevation_windows().await
    }
    #[cfg(not(windows))]
    {
        request_elevation_unix().await
    }
}

#[cfg(windows)]
async fn is_elevated_windows() -> Result<bool, Box<dyn std::error::Error>> {
    
    // This is a simplified check - in a real implementation, you'd want to use
    // Windows API calls to check for admin privileges
    // For now, we'll try to write to a system directory as a test
    let test_path = std::path::Path::new(r"C:\Windows\System32\drivers\etc\hosts");
    match std::fs::OpenOptions::new().append(true).open(test_path) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[cfg(not(windows))]
async fn is_elevated_unix() -> Result<bool, Box<dyn std::error::Error>> {
    // Check if running as root (uid 0)
    Ok(unsafe { libc::getuid() } == 0)
}

#[cfg(windows)]
async fn request_elevation_windows() -> Result<(), Box<dyn std::error::Error>> {
    let current_exe = std::env::current_exe()?;
    
    // Use PowerShell to request elevation
    let output = Command::new("powershell")
        .args(&[
            "-Command",
            &format!(
                "Start-Process '{}' -Verb RunAs -Wait",
                current_exe.to_string_lossy()
            ),
        ])
        .output()?;
    
    if output.status.success() {
        // The elevated process should have started
        // We'll exit this instance
        std::process::exit(0);
    } else {
        return Err("Failed to request elevation".into());
    }
}

#[cfg(not(windows))]
async fn request_elevation_unix() -> Result<(), Box<dyn std::error::Error>> {
    let current_exe = std::env::current_exe()?;
    
    // Try different elevation methods
    let elevation_commands = vec![
        ("pkexec", vec![current_exe.to_string_lossy().to_string()]),
        ("sudo", vec![current_exe.to_string_lossy().to_string()]),
        ("gksudo", vec![current_exe.to_string_lossy().to_string()]),
        ("kdesu", vec![current_exe.to_string_lossy().to_string()]),
    ];
    
    for (cmd, args) in elevation_commands {
        if Command::new(cmd).args(&args).spawn().is_ok() {
            // The elevated process should have started
            // We'll exit this instance
            std::process::exit(0);
        }
    }
    
    Err("No suitable elevation method found".into())
}

// Alternative approach: Try to modify the hosts file and request elevation if needed
#[allow(dead_code)]
pub async fn ensure_hosts_file_access() -> Result<bool, Box<dyn std::error::Error>> {
    let hosts_path = crate::hosts::get_hosts_file_path();
    
    // Try to open the file for writing
    match std::fs::OpenOptions::new().append(true).open(&hosts_path) {
        Ok(_) => Ok(true), // We have access
        Err(_) => {
            // We don't have access, request elevation
            request_elevation().await?;
            Ok(false) // This line won't be reached if elevation succeeds
        }
    }
}
