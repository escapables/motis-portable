pub mod native;

use native::{LocationResult as Location, RouteResult as Route};
use std::path::PathBuf;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

// Global debug flag
static DEBUG_MODE: AtomicBool = AtomicBool::new(false);

fn get_exe_dir() -> Result<PathBuf, String> {
    std::env::current_exe()
        .map_err(|e| format!("Failed to get executable path: {}", e))?
        .parent()
        .map(PathBuf::from)
        .ok_or_else(|| "Failed to get executable directory".to_string())
}

#[tauri::command]
async fn get_default_data_path_cmd() -> Result<String, String> {
    get_exe_dir()
        .map(|d| d.join("data"))
        .and_then(|p| p.to_str().map(|s| s.to_string()).ok_or("Invalid path".to_string()))
}

#[tauri::command]
async fn check_data_path_exists(path: String) -> Result<bool, String> {
    Ok(Path::new(&path).exists())
}

#[tauri::command]
async fn init_native(data_path: Option<String>) -> Result<String, String> {
    // This is an alias for init_backend to match frontend expectations
    init_backend(None, data_path, None).await
}

#[tauri::command]
async fn init_backend(
    exe_path: Option<String>,
    data_path: Option<String>,
    _server_url: Option<String>  // Ignored - USB portable only uses IPC
) -> Result<String, String> {
    // USB PORTABLE: Always use IPC mode, never server mode
    let exe = exe_path.as_deref().or_else(|| {
        get_exe_dir().ok().map(|d| d.join("motis-ipc"))
            .and_then(|p| p.to_str().map(|s| s.to_string()))
            .map(|s| s.leak() as &str)
    });
    
    let data = data_path.as_deref().or_else(|| {
        get_exe_dir().ok().map(|d| d.join("data"))
            .and_then(|p| p.to_str().map(|s| s.to_string()))
            .map(|s| s.leak() as &str)
    });
    
    native::auto_init(exe, data).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_backend_mode() -> Result<String, String> {
    match native::get_mode() {
        native::BackendMode::Ipc => Ok("ipc".to_string()),
        native::BackendMode::Server => Ok("server".to_string()),
    }
}

#[tauri::command]
async fn plan_route_cmd(
    #[allow(non_snake_case)] fromLat: f64,
    #[allow(non_snake_case)] fromLon: f64,
    #[allow(non_snake_case)] toLat: f64,
    #[allow(non_snake_case)] toLon: f64,
) -> Result<Vec<Route>, String> {
    native::plan_route(fromLat, fromLon, toLat, toLon)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn geocode_cmd(query: String) -> Result<Vec<Location>, String> {
    native::geocode(&query).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn reverse_geocode_cmd(lat: f64, lon: f64) -> Result<Option<Location>, String> {
    native::reverse_geocode(lat, lon).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn destroy_backend() {
    native::destroy();
}

#[tauri::command]
async fn is_debug_mode() -> bool {
    DEBUG_MODE.load(Ordering::Relaxed)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            init_backend,
            init_native,
            get_default_data_path_cmd,
            check_data_path_exists,
            get_backend_mode,
            plan_route_cmd,
            geocode_cmd,
            reverse_geocode_cmd,
            destroy_backend,
            is_debug_mode,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let debug_mode = args.contains(&"--debug".to_string());
    DEBUG_MODE.store(debug_mode, Ordering::Relaxed);
    
    if debug_mode {
        eprintln!("[MOTIS-GUI] Debug mode enabled");
    }
    
    run();
}
