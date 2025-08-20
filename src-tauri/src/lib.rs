#[tauri::command]
fn get_loaded_ssh_agent_keys() -> Result<Vec<String>, String> {
    let output = std::process::Command::new("ssh-add")
        .arg("-l")
        .output();
    match output {
        Ok(out) => {
            if out.status.success() {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let lines: Vec<String> = stdout.lines().map(|l| l.to_string()).collect();
                Ok(lines)
            } else {
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                Err(stderr)
            }
        }
        Err(e) => Err(format!("Failed to run ssh-add -l: {}", e)),
    }
}

#[tauri::command]
fn is_ssh_agent_running() -> bool {
    match std::env::var("SSH_AUTH_SOCK") {
        Ok(val) => !val.trim().is_empty(),
        Err(_) => false,
    }
}
use std::fs;
use std::process::Command;

// Add this struct to hold both filename and key info
#[derive(serde::Serialize)]
struct SSHKeyInfo {
    filename: String,
    key_info: String,
}

#[tauri::command]
fn get_ssh_keys() -> Result<Vec<SSHKeyInfo>, String> {
    println!("Invoked list_ssh_keys method");
    let ssh_dir = dirs::home_dir()
        .map(|mut p| {
            p.push(".ssh");
            p
        })
        .ok_or("Could not find home directory")?;
    println!("SSH directory: {:?}", ssh_dir);

    let entries = fs::read_dir(&ssh_dir)
        .map_err(|_| format!("Could not read directory: {:?}", ssh_dir))?;

    let mut key_outputs = Vec::new();
    let mut found_files = Vec::new();

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            println!("Checking file: {:?}", path);
            if path.extension().and_then(|s| s.to_str()) == Some("pub") {
                println!("Found public key file: {:?}", path);
                found_files.push(path.clone());
                
                // Extract the filename without extension for the private key
                let filename = path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                let output = Command::new("ssh-keygen")
                    .arg("-lv")
                    .arg("-f")
                    .arg(&path)
                    .output();
                match output {
                    Ok(out) => {
                        if out.status.success() {
                            let result = String::from_utf8_lossy(&out.stdout).to_string();
                            println!("SSH key info: {}", result);
                            key_outputs.push(SSHKeyInfo {
                                filename,
                                key_info: result,
                            });
                        } else {
                            let err = String::from_utf8_lossy(&out.stderr).to_string();
                            println!("Error getting SSH key info: {}", err);
                            key_outputs.push(SSHKeyInfo {
                                filename,
                                key_info: format!("Error for {}: {}", path.display(), err),
                            });
                        }
                    }
                    Err(e) => {
                        println!("Failed to run ssh-keygen: {}", e);
                        key_outputs.push(SSHKeyInfo {
                            filename,
                            key_info: format!("Failed to run ssh-keygen for {}: {}", path.display(), e),
                        });
                    }
                }
            }
        }
    }

    println!("Found {} public key files: {:?}", found_files.len(), found_files);
    // println!("Generated {} key outputs: {:?}", key_outputs.len(), key_outputs);

    Ok(key_outputs)
}

#[tauri::command]
fn create_ssh_key(email: String, key_type: String, key_size: u32, passphrase: Option<String>) -> Result<String, String> {
    let ssh_dir = dirs::home_dir()
        .map(|mut p| {
            p.push(".ssh");
            p
        })
        .ok_or("Could not find home directory")?;

    // Create .ssh directory if it doesn't exist
    if !ssh_dir.exists() {
        fs::create_dir(&ssh_dir).map_err(|e| format!("Failed to create .ssh directory: {}", e))?;
    }

    // Generate filename based on key type and email
    let filename = format!("id_{}_{}", key_type, email.split('@').next().unwrap_or("key"));
    let key_path = ssh_dir.join(&filename);

    // Check if key already exists
    if key_path.exists() {
        return Err(format!("SSH key '{}' already exists", filename));
    }

    // Build ssh-keygen command
    let mut command = Command::new("ssh-keygen");
    command
        .arg("-t")
        .arg(&key_type)
        .arg("-b")
        .arg(&key_size.to_string())
        .arg("-C")
        .arg(&email)
        .arg("-f")
        .arg(&key_path);

    // Add passphrase if provided
    if let Some(pass) = passphrase {
        if !pass.trim().is_empty() {
            command.arg("-N").arg(&pass);
        } else {
            command.arg("-N").arg("");
        }
    } else {
        command.arg("-N").arg("");
    }

    // Execute the command
    let output = command.output().map_err(|e| format!("Failed to run ssh-keygen: {}", e))?;

    if output.status.success() {
        Ok(format!("SSH key created successfully: {}", filename))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Failed to create SSH key: {}", stderr))
    }
}

#[tauri::command]
fn remove_ssh_key(key_filename: String) -> Result<String, String> {
    let ssh_dir = dirs::home_dir()
        .map(|mut p| {
            p.push(".ssh");
            p
        })
        .ok_or("Could not find home directory")?;

    // Construct paths for both private and public key files
    let private_key_path = ssh_dir.join(&key_filename);
    let public_key_path = ssh_dir.join(format!("{}.pub", key_filename));

    // Check if the private key exists
    if !private_key_path.exists() {
        return Err(format!("SSH key '{}' not found", key_filename));
    }

    // Remove private key file
    if let Err(e) = fs::remove_file(&private_key_path) {
        return Err(format!("Failed to remove private key '{}': {}", key_filename, e));
    }

    // Remove public key file if it exists
    if public_key_path.exists() {
        if let Err(e) = fs::remove_file(&public_key_path) {
            // Log warning but don't fail the operation
            println!("Warning: Failed to remove public key '{}.pub': {}", key_filename, e);
        }
    }

    Ok(format!("SSH key '{}' removed successfully", key_filename))
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![get_ssh_keys, is_ssh_agent_running, get_loaded_ssh_agent_keys, create_ssh_key, remove_ssh_key])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
