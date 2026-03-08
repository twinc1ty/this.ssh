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
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::Command;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct HostEntry {
    host: String,
    hostname: Option<String>,
    user: Option<String>,
    identity_file: Option<String>,
    identities_only: bool,
}

fn get_ssh_dir() -> Result<PathBuf, String> {
    dirs::home_dir()
        .map(|mut p| {
            p.push(".ssh");
            p
        })
        .ok_or_else(|| "Could not find home directory".to_string())
}

fn ssh_config_path() -> Result<PathBuf, String> {
    get_ssh_dir().map(|p| p.join("config"))
}

fn parse_ssh_config() -> Result<Vec<HostEntry>, String> {
    let config_path = ssh_config_path()?;
    if !config_path.exists() {
        return Ok(Vec::new());
    }

    let file = fs::File::open(&config_path)
        .map_err(|e| format!("Failed to open SSH config: {}", e))?;
    let reader = BufReader::new(file);

    let mut entries: Vec<HostEntry> = Vec::new();
    let mut current: Option<HostEntry> = None;

    for line in reader.lines() {
        let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Split on whitespace: "Key Value"
        let parts: Vec<&str> = trimmed.splitn(2, char::is_whitespace).collect();
        if parts.len() < 2 {
            continue;
        }

        let key = parts[0].to_lowercase();
        let value = parts[1].trim().to_string();

        if key == "host" {
            // Save previous entry
            if let Some(entry) = current.take() {
                entries.push(entry);
            }
            current = Some(HostEntry {
                host: value,
                hostname: None,
                user: None,
                identity_file: None,
                identities_only: false,
            });
        } else if let Some(ref mut entry) = current {
            match key.as_str() {
                "hostname" => entry.hostname = Some(value),
                "user" => entry.user = Some(value),
                "identityfile" => entry.identity_file = Some(value),
                "identitiesonly" => entry.identities_only = value.to_lowercase() == "yes",
                _ => {} // Ignore other directives for now
            }
        }
    }

    // Don't forget the last entry
    if let Some(entry) = current {
        entries.push(entry);
    }

    Ok(entries)
}

fn write_ssh_config(entries: &[HostEntry]) -> Result<(), String> {
    let config_path = ssh_config_path()?;
    let ssh_dir = get_ssh_dir()?;

    // Ensure .ssh directory exists
    if !ssh_dir.exists() {
        fs::create_dir_all(&ssh_dir)
            .map_err(|e| format!("Failed to create .ssh directory: {}", e))?;
    }

    // Write to a temp file first, then rename for atomic write
    let tmp_path = ssh_dir.join("config.tmp");
    {
        let mut file = fs::File::create(&tmp_path)
            .map_err(|e| format!("Failed to create temp config: {}", e))?;

        for (i, entry) in entries.iter().enumerate() {
            if i > 0 {
                writeln!(file).map_err(|e| format!("Write error: {}", e))?;
            }
            writeln!(file, "Host {}", entry.host)
                .map_err(|e| format!("Write error: {}", e))?;
            if let Some(ref hostname) = entry.hostname {
                writeln!(file, "    HostName {}", hostname)
                    .map_err(|e| format!("Write error: {}", e))?;
            }
            if let Some(ref user) = entry.user {
                writeln!(file, "    User {}", user)
                    .map_err(|e| format!("Write error: {}", e))?;
            }
            if let Some(ref identity_file) = entry.identity_file {
                writeln!(file, "    IdentityFile {}", identity_file)
                    .map_err(|e| format!("Write error: {}", e))?;
            }
            if entry.identities_only {
                writeln!(file, "    IdentitiesOnly yes")
                    .map_err(|e| format!("Write error: {}", e))?;
            }
        }
    }

    fs::rename(&tmp_path, &config_path)
        .map_err(|e| format!("Failed to save SSH config: {}", e))?;

    Ok(())
}

#[tauri::command]
fn get_ssh_config_hosts() -> Result<Vec<HostEntry>, String> {
    parse_ssh_config()
}

#[tauri::command]
fn assign_host_to_key(
    host: String,
    hostname: Option<String>,
    user: Option<String>,
    key_filename: String,
) -> Result<String, String> {
    let mut entries = parse_ssh_config()?;
    let ssh_dir = get_ssh_dir()?;
    let identity_file = format!("~/.ssh/{}", key_filename);

    // Verify the key actually exists
    let key_path = ssh_dir.join(&key_filename);
    if !key_path.exists() {
        return Err(format!("SSH key '{}' not found", key_filename));
    }

    // Check if a Host block already exists for this host
    if let Some(entry) = entries.iter_mut().find(|e| e.host == host) {
        entry.identity_file = Some(identity_file);
        entry.identities_only = true;
        if let Some(ref hn) = hostname {
            entry.hostname = Some(hn.clone());
        }
        if let Some(ref u) = user {
            entry.user = Some(u.clone());
        }
    } else {
        entries.push(HostEntry {
            host: host.clone(),
            hostname: hostname.or_else(|| Some(host.clone())),
            user,
            identity_file: Some(identity_file),
            identities_only: true,
        });
    }

    write_ssh_config(&entries)?;
    Ok(format!("Host '{}' assigned to key '{}'", host, key_filename))
}

#[tauri::command]
fn remove_host_entry(host: String) -> Result<String, String> {
    let mut entries = parse_ssh_config()?;
    let original_len = entries.len();
    entries.retain(|e| e.host != host);

    if entries.len() == original_len {
        return Err(format!("Host '{}' not found in SSH config", host));
    }

    write_ssh_config(&entries)?;
    Ok(format!("Host '{}' removed from SSH config", host))
}

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


#[tauri::command]
fn update_git_remote(repo_path: String, old_host: String, new_host: String) -> Result<String, String> {
    let repo = std::path::Path::new(&repo_path);
    if !repo.join(".git").exists() {
        return Err(format!("'{}' is not a git repository", repo_path));
    }

    // Get current remote URLs
    let output = Command::new("git")
        .args(["-C", &repo_path, "remote", "-v"])
        .output()
        .map_err(|e| format!("Failed to run git: {}", e))?;

    if !output.status.success() {
        return Err("Failed to list git remotes".to_string());
    }

    let remotes_output = String::from_utf8_lossy(&output.stdout).to_string();
    let mut updated = Vec::new();

    for line in remotes_output.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }
        let remote_name = parts[0];
        let remote_url = parts[1];

        // Check if this remote uses the old host (e.g., git@github.com:user/repo.git)
        let old_prefix = format!("git@{}:", old_host);
        if remote_url.starts_with(&old_prefix) && !updated.contains(&remote_name.to_string()) {
            let new_url = remote_url.replace(&old_prefix, &format!("git@{}:", new_host));
            let set_url = Command::new("git")
                .args(["-C", &repo_path, "remote", "set-url", remote_name, &new_url])
                .output()
                .map_err(|e| format!("Failed to update remote '{}': {}", remote_name, e))?;

            if !set_url.status.success() {
                let err = String::from_utf8_lossy(&set_url.stderr).to_string();
                return Err(format!("Failed to update remote '{}': {}", remote_name, err));
            }
            updated.push(remote_name.to_string());
        }
    }

    if updated.is_empty() {
        return Err(format!("No remotes found using '{}'", old_host));
    }

    Ok(format!("Updated remote(s): {}", updated.join(", ")))
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
    .invoke_handler(tauri::generate_handler![get_ssh_keys, is_ssh_agent_running, get_loaded_ssh_agent_keys, create_ssh_key, remove_ssh_key, get_ssh_config_hosts, assign_host_to_key, remove_host_entry, update_git_remote])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
