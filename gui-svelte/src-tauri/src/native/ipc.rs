#[path = "ipc_paths.rs"]
pub(crate) mod paths;

use once_cell::sync::Lazy;
#[cfg(unix)]
use std::io::ErrorKind;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::Mutex;
use std::time::Duration;

use self::paths::{
    copy_to_tmp_and_make_executable, ensure_executable, validate_data_directory,
    validate_ipc_executable_path,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BackendMode {
    Ipc,
}

#[derive(Debug, Clone)]
struct IpcLaunchConfig {
    exe_path: String,
    data_path: String,
}

pub(crate) const MAX_IPC_COMMAND_BYTES: usize = 64 * 1024;
const IPC_RECOVERY_MAX_ATTEMPTS: usize = 2;
const IPC_RECOVERY_DELAYS_MS: [u64; IPC_RECOVERY_MAX_ATTEMPTS] = [250, 1000];

static BACKEND_MODE: Lazy<Mutex<BackendMode>> = Lazy::new(|| Mutex::new(BackendMode::Ipc));
static IPC_PROCESS: Lazy<Mutex<Option<IpcBackend>>> = Lazy::new(|| Mutex::new(None));
static IPC_LAUNCH_CONFIG: Lazy<Mutex<Option<IpcLaunchConfig>>> = Lazy::new(|| Mutex::new(None));
static STARTUP_DIAGNOSTICS: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

struct IpcBackend {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

fn remember_startup_diagnostics(message: impl Into<String>) {
    let message = message.into();
    eprintln!("[MOTIS-GUI] startup-diagnostics: {}", message);
    if let Ok(mut guard) = STARTUP_DIAGNOSTICS.lock() {
        *guard = Some(message);
    }
}

fn clear_startup_diagnostics() {
    if let Ok(mut guard) = STARTUP_DIAGNOSTICS.lock() {
        *guard = None;
    }
}

pub fn get_startup_diagnostics() -> Option<String> {
    STARTUP_DIAGNOSTICS
        .lock()
        .ok()
        .and_then(|guard| guard.clone())
}

pub(crate) fn validate_ipc_command(cmd: &str) -> io::Result<()> {
    if cmd.contains('\n') || cmd.contains('\r') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "IPC command must be single-line JSON",
        ));
    }
    if cmd.len() > MAX_IPC_COMMAND_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "IPC command exceeds {} bytes (got {})",
                MAX_IPC_COMMAND_BYTES,
                cmd.len()
            ),
        ));
    }
    Ok(())
}

impl IpcBackend {
    fn send_command(&mut self, cmd: &str) -> io::Result<String> {
        validate_ipc_command(cmd)?;

        if let Some(status) = self.child.try_wait()? {
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                format!("motis-ipc exited before command send: {}", status),
            ));
        }

        writeln!(self.stdin, "{}", cmd)?;
        self.stdin.flush()?;

        let mut line = String::new();
        let bytes_read = self.stdout.read_line(&mut line)?;
        if bytes_read == 0 {
            if let Some(status) = self.child.try_wait()? {
                return Err(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    format!("motis-ipc exited before response: {}", status),
                ));
            }
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "motis-ipc returned empty response",
            ));
        }
        Ok(line)
    }

    fn terminate(&mut self, reason: &str) {
        match self.child.try_wait() {
            Ok(Some(status)) => {
                eprintln!("[MOTIS-GUI] motis-ipc already exited ({reason}): {status}");
            }
            Ok(None) => {
                eprintln!(
                    "[MOTIS-GUI] Stopping motis-ipc PID {} ({reason})",
                    self.child.id()
                );
                if let Err(err) = self.child.kill() {
                    eprintln!("[MOTIS-GUI] Failed to kill motis-ipc: {}", err);
                }
                if let Err(err) = self.child.wait() {
                    eprintln!("[MOTIS-GUI] Failed waiting for motis-ipc exit: {}", err);
                }
            }
            Err(err) => {
                eprintln!("[MOTIS-GUI] Failed to query motis-ipc state: {}", err);
            }
        }
    }
}

impl Drop for IpcBackend {
    fn drop(&mut self) {
        self.terminate("drop");
    }
}

fn spawn_ipc_backend(
    exe_path: &str,
    data_path: &str,
) -> Result<IpcBackend, Box<dyn std::error::Error>> {
    let actual_exe_path = ensure_executable(exe_path)?;
    eprintln!("[MOTIS-GUI] Using executable: {}", actual_exe_path);

    let spawn = |binary: &str| {
        let mut cmd = Command::new(binary);
        cmd.arg(data_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit());
        cmd.spawn()
    };

    let mut child = match spawn(&actual_exe_path) {
        Ok(child) => child,
        #[cfg(unix)]
        Err(err) if err.kind() == ErrorKind::PermissionDenied => {
            eprintln!(
                "[MOTIS-GUI] Spawn failed due to permission/noexec mount, retrying from /tmp: {}",
                err
            );
            let tmp_exe = copy_to_tmp_and_make_executable(exe_path)?;
            eprintln!("[MOTIS-GUI] Retrying spawn with executable: {}", tmp_exe);
            spawn(&tmp_exe).map_err(|e| format!("Failed to spawn motis-ipc from /tmp: {}", e))?
        }
        Err(err) => {
            return Err(
                format!("Failed to spawn motis-ipc: {}. Path: {}", err, actual_exe_path).into(),
            )
        }
    };

    eprintln!("[MOTIS-GUI] motis-ipc process spawned, PID: {}", child.id());

    std::thread::sleep(Duration::from_millis(150));
    if let Some(status) = child.try_wait()? {
        return Err(format!(
            "motis-ipc exited immediately (status: {}). Check executable compatibility and startup logs.",
            status
        )
        .into());
    }

    let stdin = child.stdin.take().ok_or("Failed to get stdin")?;
    let stdout = child.stdout.take().ok_or("Failed to get stdout")?;

    Ok(IpcBackend {
        child,
        stdin,
        stdout: BufReader::new(stdout),
    })
}

fn replace_ipc_backend(backend: IpcBackend, reason: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut guard = IPC_PROCESS.lock()?;
    if let Some(mut old) = guard.take() {
        old.terminate(reason);
    }
    *guard = Some(backend);
    Ok(())
}

pub(crate) fn recover_ipc_backend(reason: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let launch = IPC_LAUNCH_CONFIG.lock()?.clone();
    let Some(launch) = launch else {
        eprintln!("[MOTIS-GUI] IPC recovery skipped (no launch config): {reason}");
        return Ok(false);
    };

    eprintln!("[MOTIS-GUI] IPC recovery started: {reason}");
    for attempt in 1..=IPC_RECOVERY_MAX_ATTEMPTS {
        match spawn_ipc_backend(&launch.exe_path, &launch.data_path) {
            Ok(backend) => {
                replace_ipc_backend(backend, "recovery-replace")?;
                eprintln!("[MOTIS-GUI] IPC recovery succeeded on attempt {}", attempt);
                return Ok(true);
            }
            Err(err) => {
                eprintln!(
                    "[MOTIS-GUI] IPC recovery attempt {}/{} failed: {}",
                    attempt, IPC_RECOVERY_MAX_ATTEMPTS, err
                );
                if attempt < IPC_RECOVERY_MAX_ATTEMPTS {
                    std::thread::sleep(Duration::from_millis(IPC_RECOVERY_DELAYS_MS[attempt - 1]));
                }
            }
        }
    }

    eprintln!(
        "[MOTIS-GUI] IPC recovery failed after {} attempts",
        IPC_RECOVERY_MAX_ATTEMPTS
    );
    Ok(false)
}

fn send_ipc_command_with_recovery(cmd: &str) -> Result<String, Box<dyn std::error::Error>> {
    let total_attempts = IPC_RECOVERY_MAX_ATTEMPTS + 1;

    for attempt in 1..=total_attempts {
        let response = {
            let mut guard = IPC_PROCESS.lock()?;
            let backend = guard.as_mut().ok_or("IPC not initialized")?;
            backend.send_command(cmd)
        };

        match response {
            Ok(line) => return Ok(line),
            Err(err) => {
                eprintln!(
                    "[MOTIS-GUI] IPC command attempt {}/{} failed: {}",
                    attempt, total_attempts, err
                );
                if attempt == total_attempts {
                    return Err(
                        format!("IPC command failed after {} attempts: {}", total_attempts, err)
                            .into(),
                    );
                }
                if !recover_ipc_backend(&err.to_string())? {
                    return Err(format!("IPC recovery failed after command error: {}", err).into());
                }
            }
        }
    }

    Err("IPC command failed with unknown recovery state".into())
}

pub(crate) fn send_ipc_json_command(
    cmd: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let response = send_ipc_command_with_recovery(cmd)?;
    let result: serde_json::Value = serde_json::from_str(&response)
        .map_err(|e| format!("Invalid IPC JSON response: {} (raw: {})", e, response.trim()))?;

    if result["status"] == "ok" {
        Ok(result["data"].clone())
    } else {
        let msg = result["message"].as_str().unwrap_or("Unknown error");
        Err(msg.to_string().into())
    }
}

pub fn init_ipc(exe_path: &str, data_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("[MOTIS-GUI] Starting motis-ipc...");
    eprintln!("[MOTIS-GUI] Original exe path: {}", exe_path);
    eprintln!("[MOTIS-GUI] Data path: {}", data_path);

    if let Err(message) = validate_ipc_executable_path(exe_path) {
        remember_startup_diagnostics(message.clone());
        return Err(message.into());
    }

    if let Err(message) = validate_data_directory(data_path) {
        remember_startup_diagnostics(message.clone());
        return Err(message.into());
    }

    let backend = match spawn_ipc_backend(exe_path, data_path) {
        Ok(backend) => backend,
        Err(e) => {
            let message = format!(
                "Failed to start motis-ipc (exe='{}', data='{}'): {}. Next action: run RUN.sh --launcher-self-test and check launcher.log/error.txt.",
                exe_path, data_path, e
            );
            remember_startup_diagnostics(message.clone());
            return Err(message.into());
        }
    };
    replace_ipc_backend(backend, "reinit")?;

    {
        let mut cfg = IPC_LAUNCH_CONFIG.lock()?;
        *cfg = Some(IpcLaunchConfig {
            exe_path: exe_path.to_string(),
            data_path: data_path.to_string(),
        });
    }

    let mut mode_guard = BACKEND_MODE.lock()?;
    *mode_guard = BackendMode::Ipc;

    clear_startup_diagnostics();
    eprintln!("[MOTIS-GUI] IPC backend initialized (data loading in progress...)");
    Ok(())
}

pub fn get_mode() -> BackendMode {
    *BACKEND_MODE.lock().unwrap()
}

pub fn destroy() {
    eprintln!("[MOTIS-GUI] destroy() called");

    if let Ok(mut guard) = IPC_PROCESS.lock() {
        if let Some(mut backend) = guard.take() {
            backend.terminate("destroy");
        }
    }

    if let Ok(mut cfg) = IPC_LAUNCH_CONFIG.lock() {
        if cfg.is_some() {
            eprintln!("[MOTIS-GUI] Clearing IPC launch config");
        }
        *cfg = None;
    }
}

pub async fn auto_init(
    exe_path: Option<&str>,
    data_path: Option<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    eprintln!("[MOTIS-GUI] auto_init() called");
    eprintln!("[MOTIS-GUI] exe_path: {:?}", exe_path);
    eprintln!("[MOTIS-GUI] data_path: {:?}", data_path);

    let exe = match exe_path {
        Some(path) => path,
        None => {
            let message = "No executable path provided for initialization. Next action: set MOTIS_IPC_PATH or launch via RUN.sh.".to_string();
            remember_startup_diagnostics(message.clone());
            return Err(message.into());
        }
    };
    let data = match data_path {
        Some(path) => path,
        None => {
            let message = "No data path provided for initialization. Next action: set MOTIS_DATA_PATH or launch via RUN.sh.".to_string();
            remember_startup_diagnostics(message.clone());
            return Err(message.into());
        }
    };

    eprintln!("[MOTIS-GUI] Checking if files exist...");
    let exe_exists = Path::new(exe).exists();
    let data_exists = Path::new(data).exists();
    eprintln!(
        "[MOTIS-GUI] exe exists: {}, data exists: {}",
        exe_exists, data_exists
    );

    if !exe_exists {
        let message = format!(
            "motis-ipc executable not found at '{}'. Next action: verify the file exists and launch via RUN.sh.",
            exe
        );
        remember_startup_diagnostics(message.clone());
        return Err(message.into());
    }

    if !data_exists {
        let message = format!(
            "Data directory not found at '{}'. Next action: provide a valid MOTIS data directory (with config.yml).",
            data
        );
        remember_startup_diagnostics(message.clone());
        return Err(message.into());
    }

    eprintln!("[MOTIS-GUI] Attempting IPC initialization...");
    match init_ipc(exe, data) {
        Ok(()) => {
            eprintln!("[MOTIS-GUI] IPC mode initialized successfully");
            Ok("IPC mode initialized".to_string())
        }
        Err(e) => {
            let message = format!("Initialization failed: {}", e);
            eprintln!("[MOTIS-GUI] IPC init failed: {}", message);
            remember_startup_diagnostics(message.clone());
            Err(message.into())
        }
    }
}

pub fn is_ipc_initialized() -> bool {
    IPC_PROCESS
        .lock()
        .map(|guard| guard.is_some())
        .unwrap_or(false)
}

pub fn try_auto_init() -> bool {
    if is_ipc_initialized() {
        return true;
    }

    eprintln!("[MOTIS-GUI] Trying auto-initialization...");

    let ipc_path = std::env::var("MOTIS_IPC_PATH").ok();
    let data_path = std::env::var("MOTIS_DATA_PATH").ok().or_else(|| {
        let exe_dir = std::env::current_exe().ok()?.parent()?.to_path_buf();
        let data_dir = exe_dir.join("data");
        if data_dir.exists() {
            data_dir.to_str().map(|s| s.to_string())
        } else {
            None
        }
    });

    if let (Some(ipc), Some(data)) = (&ipc_path, &data_path) {
        eprintln!("[MOTIS-GUI] Auto-init with IPC: {}, Data: {}", ipc, data);
        if let Err(e) = init_ipc(ipc, data) {
            let message = format!(
                "Auto-initialization failed with MOTIS_IPC_PATH='{}' and MOTIS_DATA_PATH='{}': {}",
                ipc, data, e
            );
            remember_startup_diagnostics(message.clone());
            eprintln!("[MOTIS-GUI] Auto-init failed: {}", message);
            return false;
        }
        eprintln!("[MOTIS-GUI] Auto-init succeeded!");
        clear_startup_diagnostics();
        return true;
    }

    let message = format!(
        "Auto-initialization could not determine required paths. MOTIS_IPC_PATH={:?}, MOTIS_DATA_PATH={:?}. Next action: launch via RUN.sh or set both environment variables.",
        ipc_path, data_path
    );
    remember_startup_diagnostics(message);
    eprintln!(
        "[MOTIS-GUI] Auto-init: missing paths. IPC: {:?}, Data: {:?}",
        ipc_path, data_path
    );
    false
}
