pub mod native;

use native::{LocationResult as Location, RouteResult as Route};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::path::PathBuf;

// Global state
static DATA_PATH: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));
static EXE_PATH: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

fn get_exe_dir() -> Result<PathBuf, String> {
    std::env::current_exe()
        .map_err(|e| format!("Failed to get executable path: {}", e))?
        .parent()
        .map(PathBuf::from)
        .ok_or_else(|| "Failed to get executable directory".to_string())
}

fn get_default_data_path() -> Result<PathBuf, String> {
    let exe_dir = get_exe_dir()?;
    Ok(exe_dir.join("data"))
}

fn get_default_exe_path() -> Result<PathBuf, String> {
    let exe_dir = get_exe_dir()?;
    // Look for motis-ipc next to the GUI executable
    Ok(exe_dir.join("motis-ipc"))
}

#[tauri::command]
async fn get_default_data_path_cmd() -> Result<String, String> {
    let path = get_default_data_path()?;
    path.to_str()
        .map(String::from)
        .ok_or_else(|| "Invalid path encoding".to_string())
}

#[tauri::command]
async fn init_native(data_path: Option<String>, exe_path: Option<String>) -> Result<(), String> {
    let data = match data_path {
        Some(p) => PathBuf::from(p),
        None => get_default_data_path()?
    };
    
    let exe = match exe_path {
        Some(p) => PathBuf::from(p),
        None => get_default_exe_path()?
    };
    
    if !data.exists() {
        return Err(format!("Data path does not exist: {}", data.display()));
    }
    if !exe.exists() {
        return Err(format!(
            "Backend executable not found: {}. Make sure motis-native-example is built and copied next to the GUI executable.", 
            exe.display()
        ));
    }
    
    let data_str = data.to_str().ok_or("Invalid data path")?;
    let exe_str = exe.to_str().ok_or("Invalid exe path")?;
    
    native::init(data_str, exe_str).map_err(|e| e.to_string())?;
    
    *DATA_PATH.lock().map_err(|e| e.to_string())? = Some(data_str.to_string());
    *EXE_PATH.lock().map_err(|e| e.to_string())? = Some(exe_str.to_string());
    
    Ok(())
}

#[tauri::command]
async fn plan_route_cmd(
    from_lat: f64, from_lon: f64,
    to_lat: f64, to_lon: f64,
) -> Result<Vec<Route>, String> {
    native::plan_route(from_lat, from_lon, to_lat, to_lon)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn geocode_cmd(query: String) -> Result<Vec<Location>, String> {
    native::geocode(&query).map_err(|e| e.to_string())
}

#[tauri::command]
async fn reverse_geocode_cmd(lat: f64, lon: f64) -> Result<Option<Location>, String> {
    native::reverse_geocode(lat, lon).map_err(|e| e.to_string())
}

#[tauri::command]
async fn destroy_native() {
    native::destroy();
}

#[tauri::command]
async fn check_data_path_exists(path: String) -> Result<bool, String> {
    Ok(PathBuf::from(path).exists())
}

#[tauri::command]
async fn check_exe_path_exists(path: String) -> Result<bool, String> {
    Ok(PathBuf::from(path).exists())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            init_native,
            get_default_data_path_cmd,
            check_data_path_exists,
            check_exe_path_exists,
            plan_route_cmd,
            geocode_cmd,
            reverse_geocode_cmd,
            destroy_native,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
