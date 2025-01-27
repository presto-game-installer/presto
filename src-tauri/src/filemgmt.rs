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
        let source = convert_to_windows_path(&source);
        let destination = convert_to_windows_path(&destination);
        
        Command::new("cmd")
            .args(&["/C", "move", "/Y", &source, &destination])
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
            let folder = convert_to_windows_path(folder);
            
            Command::new("powershell")
                .args(&[
                    "-Command",
                    &format!("Remove-Item -Path '{}' -Recurse -Force", folder)
                ])
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
            Command::new("powershell")
                .args(&[
                    "-Command",
                    &format!("Remove-Item -Path '{}' -Force", file)
                ])
                .output()
                .map_err(|e| format!("Failed to cleanup file: {}", e))?;
        }
    }
    
    Ok(())
}

#[tauri::command]
pub async fn uninstall_game(game_path: &str, _data_path: &str) -> Result<(), String> {
    // Remove game directory if it exists
    if PathBuf::from(game_path).exists() {
        cleanup_folder(game_path).await?;
    }

    #[cfg(target_os = "macos")] {
        if PathBuf::from(_data_path).exists() {
            cleanup_folder(_data_path).await?;
        }
    }

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn convert_to_windows_path(path: &str) -> String {
    path.replace("/", "\\")
}

pub fn create_directory(path: &PathBuf) -> Result<(), String> {
    std::fs::create_dir_all(path)
        .map_err(|e| format!("Failed to create directory: {}", e))
}

#[tauri::command]
pub async fn run_executable(executable: &str, install_path: &str) -> Result<String, String> {

    #[cfg(target_os = "macos")]
    {
        let path = PathBuf::from(install_path).join(executable);
        let run_game_result = Command::new("open")
        .args(&[path])
        .output()
        .map_err(|e| format!("Failed to run executable: {}", e))?;

        if !run_game_result.status.success() {
            return Err(format!("Failed to run app: {}", 
                String::from_utf8_lossy(&run_game_result.stderr)));
        }
    }

    #[cfg(target_os = "linux")]
    {
        let path = PathBuf::from(install_path).join(executable);
        let run_game_result = Command::new(&path)
            .output()
            .map_err(|e| format!("Failed to run executable: {}", e))?;

        if !run_game_result.status.success() {
            return Err(format!("Failed to run app: {}", 
                String::from_utf8_lossy(&run_game_result.stderr)));
        }
    }

    #[cfg(target_os = "windows")]
    {
        log::info!("{}",convert_to_windows_path(&install_path));
        let run_game_result = Command::new("powershell")
            .args(&[
                "Start-Process",
                "-FilePath",
                &executable,
                "-WorkingDirectory",
                &convert_to_windows_path(&install_path),
                "-verb RunAs"
            ])
            .output()
            .map_err(|e| format!("Failed to run executable: {}", e))?;

        if !run_game_result.status.success() {
            return Err(format!("Failed to run app: {}", 
                String::from_utf8_lossy(&run_game_result.stderr)));
        }
    }

    Ok("Success".to_string())
}