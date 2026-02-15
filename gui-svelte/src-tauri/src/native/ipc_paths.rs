use std::path::{Path, PathBuf};
#[cfg(unix)]
use std::io::ErrorKind;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(unix)]
pub(crate) fn copy_to_tmp_and_make_executable(
    exe_path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new(exe_path);

    let tmp_dir = create_private_temp_dir("motis-ipc")?;
    let exe_name = path
        .file_name()
        .ok_or("Invalid exe path")?
        .to_str()
        .ok_or("Invalid exe name")?;
    let tmp_path = tmp_dir.join(exe_name);

    eprintln!("[MOTIS-GUI] Copying executable to /tmp: {:?}", tmp_path);
    std::fs::copy(exe_path, &tmp_path)?;

    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(&tmp_path)?.permissions();
    perms.set_mode(0o700);
    std::fs::set_permissions(&tmp_path, perms)?;

    tmp_path
        .to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "Invalid tmp path".into())
}

#[cfg(unix)]
pub(crate) fn create_private_temp_dir(
    prefix: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    use std::os::unix::fs::PermissionsExt;

    let base = std::env::temp_dir();
    for attempt in 0..16 {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let candidate = base.join(format!("{prefix}-{}-{nonce}-{attempt}", std::process::id()));
        match std::fs::create_dir(&candidate) {
            Ok(()) => {
                std::fs::set_permissions(&candidate, std::fs::Permissions::from_mode(0o700))?;
                return Ok(candidate);
            }
            Err(err) if err.kind() == ErrorKind::AlreadyExists => continue,
            Err(err) => return Err(err.into()),
        }
    }

    Err("Failed to create secure temp directory for motis-ipc".into())
}

#[cfg(not(unix))]
pub(crate) fn create_private_temp_dir(
    _prefix: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    Err("secure temp directory helper is unix-only".into())
}

pub(crate) fn ensure_executable(exe_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new(exe_path);

    if !path.exists() {
        return Err(format!("Executable not found: {}", exe_path).into());
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(path)?.permissions().mode();
        if mode & 0o111 != 0 {
            return Ok(exe_path.to_string());
        }
        eprintln!(
            "[MOTIS-GUI] Executable bit missing for {}, using /tmp copy",
            exe_path
        );
        return copy_to_tmp_and_make_executable(exe_path);
    }

    #[cfg(not(unix))]
    {
        Ok(exe_path.to_string())
    }
}

pub(crate) fn validate_ipc_executable_path(exe_path: &str) -> Result<(), String> {
    let path = Path::new(exe_path);
    if !path.exists() {
        return Err(format!(
            "motis-ipc executable not found at '{}'. Next action: verify the file exists and launch via RUN.sh.",
            exe_path
        ));
    }
    if !path.is_file() {
        return Err(format!(
            "motis-ipc path is not a file: '{}'. Next action: check bundle layout and MOTIS_IPC_PATH.",
            exe_path
        ));
    }
    Ok(())
}

pub(crate) fn validate_data_directory(data_path: &str) -> Result<(), String> {
    let path = Path::new(data_path);
    if !path.exists() {
        return Err(format!(
            "Data directory not found at '{}'. Next action: set MOTIS_DATA_PATH correctly or place 'data/' next to the app.",
            data_path
        ));
    }
    if !path.is_dir() {
        return Err(format!(
            "Data path is not a directory: '{}'. Next action: point MOTIS_DATA_PATH to a MOTIS data directory.",
            data_path
        ));
    }
    let config = path.join("config.yml");
    if !config.exists() {
        return Err(format!(
            "Data directory '{}' is missing 'config.yml'. Next action: run import (motis-import.sh) or copy a complete data bundle.",
            data_path
        ));
    }
    Ok(())
}
