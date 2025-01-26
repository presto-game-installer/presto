use std::path::PathBuf;
use std::process::Command;
use crate::filemgmt::create_directory;

#[cfg(target_os = "windows")]
use crate::filemgmt::convert_to_windows_path;

#[tauri::command]
pub async fn unzip_file(zip_path: String,
      temp_path: String,
      final_path: String,
      uses_dmg: bool) -> Result<String, String> {

    let temp_path_buf = PathBuf::from(temp_path.clone());
    let final_path_buf = PathBuf::from(final_path.clone());
    let mut return_path = String::new();

    // Create extraction directory if it doesn't exist
    if !temp_path_buf.exists() {
        create_directory(&temp_path_buf)?;
    }

    if !final_path_buf.exists() {
        create_directory(&final_path_buf)?;
    }

    let result = async {
        #[cfg(target_os = "windows")]
        {
            let zip_path = convert_to_windows_path(&zip_path);
            let final_path = convert_to_windows_path(&final_path);
            // Extract directly to the final path
            Command::new("powershell")
                .args(&[
                    "-Command",
                    &format!(
                        "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
                        zip_path,
                        final_path
                    )
                ])
                .output()
                .map_err(|e| format!("Failed to extract on Windows: {}", e))?;
        }

        #[cfg(target_os = "macos")]
        {
            let mut dest_path = final_path.clone();
            if uses_dmg {
                dest_path = temp_path.clone();
            }
            Command::new("unzip")
                .args(&[
                    "-o",
                    &zip_path,
                    "-d",
                    &dest_path
                ])
                .output()
                .map_err(|e| format!("Failed to extract: {}", e))?;

            if uses_dmg {
                match mount_and_copy_dmg(temp_path.clone(), final_path).await {
                    Ok(path) => return_path = path,
                    Err(e) => return Err(e)
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            Command::new("unzip")
                .args(&[
                    "-o",
                    &zip_path,
                    "-d",
                    &final_path
                ])
                .output()
                .map_err(|e| format!("Failed to extract: {}", e))?;
        }
        Ok(return_path)
    }.await;    
    result
}

#[cfg(target_os = "macos")]
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
