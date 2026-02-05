use std::process::{Command, Stdio, Child};
use std::io::{BufRead, BufReader, Write};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use reqwest;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatLon {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteLeg {
    pub mode: String,
    pub from_name: String,
    pub to_name: String,
    pub from: LatLon,
    pub to: LatLon,
    pub duration_seconds: i32,
    pub distance_meters: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_short_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headsign: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteResult {
    pub duration_seconds: i32,
    pub transfers: i32,
    pub legs: Vec<RouteLeg>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationResult {
    pub name: String,
    pub place_id: String,
    pub lat: f64,
    pub lon: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

// Backend connection modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BackendMode {
    Ipc,     // Subprocess with stdin/stdout - no HTTP needed
    Server,  // HTTP localhost server - uses web API
}

// Global state
static BACKEND_MODE: Lazy<Mutex<BackendMode>> = Lazy::new(|| Mutex::new(BackendMode::Ipc));
static IPC_PROCESS: Lazy<Mutex<Option<IpcBackend>>> = Lazy::new(|| Mutex::new(None));
static SERVER_URL: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("http://localhost:8080".to_string()));

pub struct IpcBackend {
    _child: Child,
    stdin: std::process::ChildStdin,
}

impl IpcBackend {
    fn send_command(&mut self, cmd: &str) -> Result<String, Box<dyn std::error::Error>> {
        writeln!(self.stdin, "{}", cmd)?;
        
        let stdout = self._child.stdout.as_mut().ok_or("No stdout")?;
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        Ok(line)
    }
}

/// Try to make a file executable (copy to /tmp if on FAT32)
fn ensure_executable(exe_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new(exe_path);
    
    // Check if file exists
    if !path.exists() {
        return Err(format!("Executable not found: {}", exe_path).into());
    }
    
    // Try to check if it's executable by attempting to run it with --version
    // This is a lightweight check that works on Unix
    #[cfg(unix)]
    {
        // Check if we can execute it directly
        match Command::new(exe_path).arg("--version").output() {
            Ok(_) => return Ok(exe_path.to_string()),
            Err(e) => {
                eprintln!("[MOTIS-GUI] Cannot execute directly (permission issue?): {}", e);
                
                // Try to copy to /tmp and make executable
                let tmp_dir = std::env::temp_dir();
                let exe_name = path.file_name()
                    .ok_or("Invalid exe path")?
                    .to_str()
                    .ok_or("Invalid exe name")?;
                let tmp_path = tmp_dir.join(exe_name);
                
                eprintln!("[MOTIS-GUI] Copying to /tmp: {:?}", tmp_path);
                
                // Copy the file
                std::fs::copy(exe_path, &tmp_path)?;
                
                // Make it executable
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(&tmp_path)?.permissions();
                perms.set_mode(0o755);
                std::fs::set_permissions(&tmp_path, perms)?;
                
                let tmp_path_str = tmp_path.to_str()
                    .ok_or("Invalid tmp path")?
                    .to_string();
                
                eprintln!("[MOTIS-GUI] Made executable at: {}", tmp_path_str);
                return Ok(tmp_path_str);
            }
        }
    }
    
    #[cfg(not(unix))]
    {
        // On Windows, just return the original path
        Ok(exe_path.to_string())
    }
}

pub fn init_ipc(exe_path: &str, data_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("[MOTIS-GUI] Starting motis-ipc...");
    eprintln!("[MOTIS-GUI] Original exe path: {}", exe_path);
    eprintln!("[MOTIS-GUI] Data path: {}", data_path);
    
    // Check data path exists
    if !Path::new(data_path).exists() {
        return Err(format!("Data directory not found: {}", data_path).into());
    }
    
    // Ensure executable is actually executable (handles FAT32 USB)
    let actual_exe_path = ensure_executable(exe_path)?;
    eprintln!("[MOTIS-GUI] Using executable: {}", actual_exe_path);

    let mut child = Command::new(&actual_exe_path)
        .arg(data_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())  // Show motis-ipc's stderr for debugging
        .spawn()
        .map_err(|e| format!("Failed to spawn motis-ipc: {}. Path: {}", e, actual_exe_path))?;

    eprintln!("[MOTIS-GUI] Process spawned, PID: {:?}", child.id());
    
    let stdin = child.stdin.take().ok_or("Failed to get stdin")?;
    
    // Store the backend first
    let backend = IpcBackend { _child: child, stdin };
    
    let mut guard = IPC_PROCESS.lock()?;
    *guard = Some(backend);
    
    let mut mode_guard = BACKEND_MODE.lock()?;
    *mode_guard = BackendMode::Ipc;
    
    eprintln!("[MOTIS-GUI] IPC backend initialized (data loading in progress...)");
    
    Ok(())
}

pub fn init_server(url: &str) {
    let mut guard = SERVER_URL.lock().unwrap();
    *guard = url.to_string();
    
    let mut mode_guard = BACKEND_MODE.lock().unwrap();
    *mode_guard = BackendMode::Server;
}

pub fn get_mode() -> BackendMode {
    *BACKEND_MODE.lock().unwrap()
}

// Geocode - works in both modes
pub async fn geocode(query: &str) -> Result<Vec<LocationResult>, Box<dyn std::error::Error>> {
    let mode = get_mode();
    
    match mode {
        BackendMode::Ipc => {
            eprintln!("[MOTIS-GUI] geocode() called with query: '{}'", query);
            
            let cmd = format!(r#"{{"cmd":"geocode","query":"{}"}}"#, query.replace('"', "\\\""));
            eprintln!("[MOTIS-GUI] Sending command: {}", cmd);
            
            // Retry logic for IPC
            let max_retries = 5;
            let retry_delay = std::time::Duration::from_millis(1000);
            
            for attempt in 1..=max_retries {
                let mut guard = IPC_PROCESS.lock()?;
                let backend = guard.as_mut().ok_or("IPC not initialized")?;
                
                match backend.send_command(&cmd) {
                    Ok(response) => {
                        eprintln!("[MOTIS-GUI] Got response: {}", response.trim());
                        
                        let result: serde_json::Value = serde_json::from_str(&response)?;
                        if result["status"] == "ok" {
                            let locations: Vec<LocationResult> = serde_json::from_value(result["data"].clone())?;
                            eprintln!("[MOTIS-GUI] Found {} locations", locations.len());
                            return Ok(locations);
                        } else {
                            let msg = result["message"].as_str().unwrap_or("Unknown error");
                            eprintln!("[MOTIS-GUI] Backend error: {}", msg);
                            return Err(msg.into());
                        }
                    }
                    Err(e) => {
                        eprintln!("[MOTIS-GUI] Attempt {}/{} failed: {}", attempt, max_retries, e);
                        if attempt < max_retries {
                            eprintln!("[MOTIS-GUI] Retrying in 1 second...");
                            drop(guard); // Release lock before sleeping
                            std::thread::sleep(retry_delay);
                            continue;
                        } else {
                            return Err(e);
                        }
                    }
                }
            }
            
            Err("Max retries exceeded".into())
        }
        BackendMode::Server => {
            let url = SERVER_URL.lock()?.clone();
            eprintln!("[MOTIS-GUI] geocode() using server: {}", url);
            
            let client = reqwest::Client::new();
            let resp = client
                .get(format!("{}/api/v1/geocode", url))
                .query(&[("text", query)])
                .send()
                .await?;
            
            let locations: Vec<LocationResult> = resp.json().await?;
            eprintln!("[MOTIS-GUI] Found {} locations", locations.len());
            Ok(locations)
        }
    }
}

// Plan route - works in both modes
pub async fn plan_route(
    from_lat: f64, from_lon: f64,
    to_lat: f64, to_lon: f64
) -> Result<Vec<RouteResult>, Box<dyn std::error::Error>> {
    let mode = get_mode();
    
    match mode {
        BackendMode::Ipc => {
            eprintln!("[MOTIS-GUI] plan_route() called: ({}, {}) to ({}, {})", from_lat, from_lon, to_lat, to_lon);
            
            let cmd = format!(
                r#"{{"cmd":"plan_route","from_lat":{},"from_lon":{},"to_lat":{},"to_lon":{}}}"#,
                from_lat, from_lon, to_lat, to_lon
            );
            eprintln!("[MOTIS-GUI] Sending command: {}", cmd);
            
            // Retry logic for IPC
            let max_retries = 5;
            let retry_delay = std::time::Duration::from_millis(1000);
            
            for attempt in 1..=max_retries {
                let mut guard = IPC_PROCESS.lock()?;
                let backend = guard.as_mut().ok_or("IPC not initialized")?;
                
                match backend.send_command(&cmd) {
                    Ok(response) => {
                        eprintln!("[MOTIS-GUI] Got response: {}", response.trim());
                        
                        let result: serde_json::Value = serde_json::from_str(&response)?;
                        if result["status"] == "ok" {
                            let routes: Vec<RouteResult> = serde_json::from_value(result["data"].clone())?;
                            eprintln!("[MOTIS-GUI] Found {} routes", routes.len());
                            return Ok(routes);
                        } else {
                            let msg = result["message"].as_str().unwrap_or("Unknown error");
                            eprintln!("[MOTIS-GUI] Backend error: {}", msg);
                            return Err(msg.into());
                        }
                    }
                    Err(e) => {
                        eprintln!("[MOTIS-GUI] Attempt {}/{} failed: {}", attempt, max_retries, e);
                        if attempt < max_retries {
                            eprintln!("[MOTIS-GUI] Retrying in 1 second...");
                            drop(guard); // Release lock before sleeping
                            std::thread::sleep(retry_delay);
                            continue;
                        } else {
                            return Err(e);
                        }
                    }
                }
            }
            
            Err("Max retries exceeded".into())
        }
        BackendMode::Server => {
            let url = SERVER_URL.lock()?.clone();
            eprintln!("[MOTIS-GUI] plan_route() using server: {}", url);
            
            let client = reqwest::Client::new();
            let resp = client
                .get(format!("{}/api/v1/plan", url))
                .query(&[
                    ("fromPlace", &format!("{}, {}", from_lat, from_lon)),
                    ("toPlace", &format!("{}, {}", to_lat, to_lon)),
                ])
                .send()
                .await?;
            
            let result: serde_json::Value = resp.json().await?;
            // Parse from MOTIS plan response format
            let itineraries = result["itineraries"].as_array()
                .ok_or("No itineraries")?;
            
            let mut routes = Vec::new();
            for itin in itineraries {
                let route = RouteResult {
                    duration_seconds: itin["duration"].as_i64().unwrap_or(0) as i32,
                    transfers: itin["transfers"].as_i64().unwrap_or(0) as i32,
                    legs: Vec::new(), // Simplified - would parse legs here
                };
                routes.push(route);
            }
            eprintln!("[MOTIS-GUI] Found {} routes", routes.len());
            Ok(routes)
        }
    }
}

// Reverse geocode
pub async fn reverse_geocode(lat: f64, lon: f64) -> Result<Option<LocationResult>, Box<dyn std::error::Error>> {
    let mode = get_mode();
    
    match mode {
        BackendMode::Ipc => {
            let mut guard = IPC_PROCESS.lock()?;
            let backend = guard.as_mut().ok_or("IPC not initialized")?;
            
            let cmd = format!(r#"{{"cmd":"reverse_geocode","lat":{},"lon":{}}}"#, lat, lon);
            let response = backend.send_command(&cmd)?;
            
            let result: serde_json::Value = serde_json::from_str(&response)?;
            if result["status"] == "ok" && !result["data"].is_null() {
                let loc: LocationResult = serde_json::from_value(result["data"].clone())?;
                Ok(Some(loc))
            } else {
                Ok(None)
            }
        }
        BackendMode::Server => {
            let url = SERVER_URL.lock()?.clone();
            let client = reqwest::Client::new();
            let resp = client
                .get(format!("{}/api/v1/reverse-geocode", url))
                .query(&[("lat", lat.to_string()), ("lon", lon.to_string())])
                .send()
                .await?;
            
            let loc: Option<LocationResult> = resp.json().await?;
            Ok(loc)
        }
    }
}

pub fn destroy() {
    let mut guard = IPC_PROCESS.lock().unwrap();
    if let Some(_) = guard.take() {
        // Process will be killed when dropped
    }
}

// Auto-detect mode based on what's available
// For USB portable: ONLY use IPC mode, never fall back to server
pub async fn auto_init(exe_path: Option<&str>, data_path: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    eprintln!("[MOTIS-GUI] auto_init() called");
    eprintln!("[MOTIS-GUI] exe_path: {:?}", exe_path);
    eprintln!("[MOTIS-GUI] data_path: {:?}", data_path);
    
    // USB PORTABLE MODE: Only use IPC, never fall back to server
    let exe = exe_path.ok_or("No executable path provided")?;
    let data = data_path.ok_or("No data path provided")?;
    
    eprintln!("[MOTIS-GUI] Checking if files exist...");
    let exe_exists = Path::new(exe).exists();
    let data_exists = Path::new(data).exists();
    eprintln!("[MOTIS-GUI] exe exists: {}, data exists: {}", exe_exists, data_exists);
    
    if !exe_exists {
        return Err(format!(
            "motis-ipc executable not found: {}\n\nPlease ensure:\n1. motis-ipc is in the same folder as motis-gui\n2. You're running from the correct directory", 
            exe
        ).into());
    }
    
    if !data_exists {
        return Err(format!(
            "Data directory not found: {}\n\nPlease ensure:\n1. The 'data' folder exists next to motis-gui\n2. You've imported GTFS/OSM data using ./motis-transit import", 
            data
        ).into());
    }
    
    eprintln!("[MOTIS-GUI] Attempting IPC initialization...");
    match init_ipc(exe, data) {
        Ok(()) => {
            eprintln!("[MOTIS-GUI] IPC mode initialized successfully");
            Ok("IPC mode initialized".to_string())
        }
        Err(e) => {
            eprintln!("[MOTIS-GUI] IPC init failed: {}", e);
            Err(format!(
                "Failed to start IPC backend: {}\n\nTroubleshooting:\n1. Check that motis-ipc has executable permissions:\n   chmod +x {}\n2. If on NTFS/USB, ensure it's mounted with exec permissions\n3. Try running: {} --version\n4. Check that data directory contains valid MOTIS data", 
                e, exe, exe
            ).into())
        }
    }
}
