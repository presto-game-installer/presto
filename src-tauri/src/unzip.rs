use std::process::Command;

#[cfg(target_os = "macos")]
use std::path::PathBuf;
use crate::filemgmt::create_directory;

#[cfg(target_os = "windows")]
use crate::filemgmt::convert_to_windows_path;

#[tauri::command]
#[allow(unused_variables)] // To prevent unused parameter warnings on non-macOS
pub async fn unzip_file(
    archive_path: String,
    temp_path: String, 
    final_path: String,
    uses_dmg: Option<bool>,
    game_executable: Option<String>) -> Result<String, String> {

    let result = async {
        #[cfg(target_os = "windows")]
        {
            // Convert paths to Windows format
            let archive_path = convert_to_windows_path(&archive_path);
            let final_path = convert_to_windows_path(&final_path);
            // Extract directly to the final path
            Command::new("powershell")
                .args(&[
                    "-Command",
                    &format!(
                        "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
                        archive_path,
                        final_path
                    )
                ])
                .output()
                .map_err(|e| format!("Failed to extract on Windows: {}", e))?;
            Ok(final_path)
        }

        #[cfg(target_os = "macos")]
        {
            let mut return_path = String::new();
            let mut dest_path = final_path.clone();
            // If the game uses a DMG, we need to extract to the temp path
            if uses_dmg.expect("uses_dmg is required") {
                // Create extraction directory if it doesn't exist
                create_directory(&temp_path)?;

                dest_path = temp_path.clone();
            }

            if archive_path.ends_with(".zip") {
            Command::new("unzip")
                .args(&[
                    "-o",
                    &archive_path,
                    "-d",
                    &dest_path
                ])
                .output()
                .map_err(|e| format!("Failed to extract: {}", e))?;
            }

            if archive_path.ends_with(".tar.xz") {
                Command::new("tar")
                    .args(&["-xJf", &archive_path, "--strip=1", "-C", &dest_path])
                    .output()
                    .map_err(|e| format!("Failed to extract: {}", e))?;
            }

            // If the game uses a DMG, we need to copy the .app to the final path
            if uses_dmg.expect("uses_dmg is required") {
                match mount_and_copy_dmg(temp_path.clone(), final_path).await {
                    Ok(path) => return_path = path,
                    Err(e) => return Err(e)
                }
            }

            // Set the gameExecutable permissions
            Command::new("chmod")
                .args(&[
                    "+x",
                    &format!("{}/{}",
                     dest_path,
                     game_executable.expect("game_executable is required")
                    )
                ])
                .output()
                .map_err(|e| format!("Failed to set gameExecutable permissions: {}", e))?;
            
            Ok(return_path)
        }

        #[cfg(target_os = "linux")]
        {
            if archive_path.ends_with(".zip") {
            Command::new("unzip")
                .args(&[
                    "-o",
                    &archive_path,
                    "-d",
                    &final_path
                ])
                .output()
                .map_err(|e| format!("Failed to extract: {}", e))?;
            }

            if archive_path.ends_with(".tar.xz") {
                Command::new("tar")
                    .args(&["-xvf", &archive_path, "--strip=1", "-C", &final_path])
                    .output()
                    .map_err(|e| format!("Failed to extract: {}", e))?;
            }

            if archive_path.ends_with(".flatpak") {
                Command::new("flatpak")
                    .args(&["install", &archive_path, "-y"])
                    .output()
                    .map_err(|e| format!("Failed to extract: {}", e))?;
            }

            // Set the gameExecutable permissions
            Command::new("chmod")
                .args(&[
                    "+x",
                    &format!("{}/{}",
                     final_path,
                     game_executable.expect("game_executable is required")
                    )
                ])
                .output()
                .map_err(|e| format!("Failed to set gameExecutable permissions: {}", e))?;

            Ok(final_path)
        }
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
