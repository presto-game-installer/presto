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
    let path = PathBuf::from(install_path).join(executable);
    #[cfg(target_os = "windows")]{
        let path = convert_to_windows_path(&path);
    }

    #[cfg(target_os = "macos")]
    {
        let run_game_result = Command::new("open")
        .args(&[path])
        .output()
        .map_err(|e| format!("Failed to run executable: {}", e))?;

        if !run_game_result.status.success() {
            return Err(format!("Failed to run app: {}", 
                String::from_utf8_lossy(&run_game_result.stderr)));
        }
    }

    Ok("Success".to_string())
}