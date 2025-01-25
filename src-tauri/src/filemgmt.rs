use std::path::PathBuf;
use std::process::Command;

#[tauri::command]
pub async fn move_file(source: String, destination: String) -> Result<(), String> {
    let dest_path = PathBuf::from(&destination);
    
    // Create parent directory if it doesn't exist
    if let Some(parent) = dest_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }
    }

    // Move the file based on platform
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        Command::new("mv")
            .args(&[&source, &destination])
            .output()
            .map_err(|e| format!("Failed to move file: {}", e))?;
    }
    
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "move", &source, &destination])
            .output()
            .map_err(|e| format!("Failed to move file: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn cleanup_folder(folder: &str) -> Result<(), String> {
    // Remove temp directory if it exists
    if PathBuf::from(folder).exists() {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            Command::new("rm")
                .args(&["-rf", folder])
                .output()
                .map_err(|e| format!("Failed to cleanup directory: {}", e))?;
        }

        #[cfg(target_os = "windows")]
        {
            Command::new("cmd")
                .args(&["/C", "rmdir", "/S", "/Q", folder])
                .output()
                .map_err(|e| format!("Failed to cleanup directory: {}", e))?;
        }
    }
    
    Ok(())
}

#[tauri::command]
pub async fn cleanup_file(file: &str) -> Result<(), String> {
    // Remove zip file if it exists
    if PathBuf::from(file).exists() {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            Command::new("rm")
                .args(&["-f", file])
                .output()
                .map_err(|e| format!("Failed to cleanup file: {}", e))?;
        }

        #[cfg(target_os = "windows")]
        {
            Command::new("cmd")
                .args(&["/C", "del", "/F", "/Q", file])
                .output()
                .map_err(|e| format!("Failed to cleanup file: {}", e))?;
        }
    }
    
    Ok(())
}