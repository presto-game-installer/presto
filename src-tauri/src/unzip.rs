use std::path::PathBuf;
use std::process::Command;

#[tauri::command]
pub async fn unzip_file(zip_path: String,
      extract_path: String,
      final_path: String) -> Result<String, String> {
    let zip_path_buf = PathBuf::from(zip_path.clone());
    let extract_path_temp = PathBuf::from(extract_path.clone());
    let mut return_path = String::new();

    // Create extraction directory if it doesn't exist
    if !extract_path_temp.exists() {
        std::fs::create_dir_all(&extract_path_temp)
            .map_err(|e| format!("Failed to create extraction directory: {}", e))?;
    }

    let result = async {
        #[cfg(target_os = "windows")]
        {
            Command::new("powershell")
                .args(&[
                    "Expand-Archive",
                    "-Path", zip_path_buf.to_str().unwrap(),
                    "-DestinationPath", extract_path_temp.to_str().unwrap(),
                    "-Force"
                ])
                .output()
                .map_err(|e| format!("Failed to extract on Windows: {}", e))?;

            // create the install directory if it doesn't exist
            if !PathBuf::from(&final_path).exists() {
                let copy_result = Command::new("cmd")
                    .args(&["/C", "mkdir", &final_path])
                    .output()
                    .map_err(|e| format!("Failed to create install directory for app: {}", e))?;

                if !copy_result.status.success() {
                    return Err(format!("Failed to create install directory for app: {}", 
                        String::from_utf8_lossy(&copy_result.stderr)));
                }
            }

            // Copy files using xcopy (more suitable for directories than copy)
            let copy_result = Command::new("cmd")
                .args(&["/C", "xcopy", &extract_path, &final_path, "/E", "/I", "/H", "/Y"])
                .output()
                .map_err(|e| format!("Failed to copy app: {}", e))?;
        }

        #[cfg(target_os = "macos")]
        {
            Command::new("unzip")
                .args(&[
                    "-o",
                    zip_path_buf.to_str().unwrap(),
                    "-d",
                    extract_path_temp.to_str().unwrap()
                ])
                .output()
                .map_err(|e| format!("Failed to extract: {}", e))?;

            match mount_and_copy_dmg(extract_path.clone(), final_path).await {
                Ok(path) => return_path = path,
                Err(e) => return Err(e)
            }
        }

        #[cfg(target_os = "linux")]
        {
            Command::new("unzip")
                .args(&[
                    "-o",
                    zip_path_buf.to_str().unwrap(),
                    "-d",
                    extract_path_temp.to_str().unwrap()
                ])
                .output()
                .map_err(|e| format!("Failed to extract: {}", e))?;

            // create the install directory if it doesn't exist
            if !PathBuf::from(&final_path).exists() {
                let copy_result = Command::new("mkdir")
                    .args([&final_path])
                    .output()
                    .map_err(|e| format!("Failed to create install directory for app: {}", e))?;

                if !copy_result.status.success() {
                    return Err(format!("Failed to create install directory for app: {}", 
                        String::from_utf8_lossy(&copy_result.stderr)));
                }
            }

            let copy_result = Command::new("cp")
                .args(&["-r", &extract_path, &final_path])
                .output()
                .map_err(|e| format!("Failed to copy app: {}", e))?;
        }
        Ok(return_path)
    }.await;    
    result
}

#[tauri::command]
async fn mount_and_copy_dmg(dmg_path: String, install_path: String) -> Result<String, String> {
    // Find the .dmg file
    let dmg_file = Command::new("find")
        .args(&[&dmg_path, "-name", "*.dmg", "-maxdepth", "1"])
        .output()
        .map_err(|e| format!("Failed to find DMG: {}", e))?;

    let dmg_file = String::from_utf8_lossy(&dmg_file.stdout)
        .trim()
        .to_string();

    if dmg_file.is_empty() {
        return Err("No .dmg file found".to_string());
    }

    // Mount the DMG
    let mount_output = Command::new("hdiutil")
        .args(&["attach", &dmg_file])  // Use found dmg_file instead of dmg_path
        .output()
        .map_err(|e| format!("Failed to mount DMG: {}", e))?;

    if !mount_output.status.success() {
        return Err(format!("Failed to mount DMG: {}", 
            String::from_utf8_lossy(&mount_output.stderr)));
    }

    // Get the mount point from the output
    let mount_info = String::from_utf8_lossy(&mount_output.stdout);
    let mount_point = mount_info
        .lines()
        .last()
        .ok_or("Could not find mount point")?
        .split('\t')
        .last()
        .ok_or("Invalid mount point format")?
        .trim();

    // Find the .app in the mounted volume
    let app_path = Command::new("find")
        .args(&[mount_point, "-name", "*.app", "-maxdepth", "1"])
        .output()
        .map_err(|e| format!("Failed to find app: {}", e))?;

    let app_path = String::from_utf8_lossy(&app_path.stdout)
        .trim()
        .to_string();

    if app_path.is_empty() {
        // Unmount before returning error
        let _ = Command::new("hdiutil")
            .args(&["detach", mount_point])
            .output();
        return Err("No .app found in DMG".to_string());
    }

    // create the install directory if it doesn't exist
    if !PathBuf::from(&install_path).exists() {
        let copy_result = Command::new("mkdir")
            .args([&install_path])
            .output()
            .map_err(|e| format!("Failed to create install directory for app: {}", e))?;

        if !copy_result.status.success() {
            return Err(format!("Failed to create install directory for app: {}", 
                String::from_utf8_lossy(&copy_result.stderr)));
        }
    }

    let final_path = install_path + "/" + app_path.split('/').last().unwrap();

    // Copy the .app to the install location
    let copy_result = Command::new("cp")
        .args(&["-r", &app_path, &final_path])
        .output()
        .map_err(|e| format!("Failed to copy app: {}", e))?;

    // Unmount the DMG
    let unmount_result = Command::new("hdiutil")
        .args(&["detach", mount_point])
        .output()
        .map_err(|e| format!("Failed to unmount DMG: {}", e))?;

    if !unmount_result.status.success() {
        return Err(format!("Failed to unmount DMG: {}", 
            String::from_utf8_lossy(&unmount_result.stderr)));
    }

    if !copy_result.status.success() {
        return Err(format!("Failed to copy app: {}", 
            String::from_utf8_lossy(&copy_result.stderr)));
    }

    Ok(final_path)
}
