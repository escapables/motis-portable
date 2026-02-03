use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Write};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteLeg {
    pub mode: String,
    pub from_name: String,
    pub to_name: String,
    pub from_lat: f64,
    pub from_lon: f64,
    pub to_lat: f64,
    pub to_lon: f64,
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

#[derive(Debug, Serialize)]
#[serde(tag = "cmd")]
enum Request {
    #[serde(rename = "geocode")]
    Geocode { query: String },
    #[serde(rename = "plan_route")]
    PlanRoute { 
        from_lat: f64, 
        from_lon: f64, 
        to_lat: f64, 
        to_lon: f64 
    },
    #[serde(rename = "reverse_geocode")]
    ReverseGeocode { lat: f64, lon: f64 },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "status")]
enum Response {
    #[serde(rename = "ok")]
    Ok { data: serde_json::Value },
    #[serde(rename = "error")]
    Error { message: String },
}

pub struct MotisProcess {
    child: std::process::Child,
    stdin: std::process::ChildStdin,
}

impl MotisProcess {
    fn send_command(&mut self, req: Request) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&req)?;
        writeln!(self.stdin, "{}", json)?;
        
        let stdout = self.child.stdout.as_mut().ok_or("No stdout")?;
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        
        let resp: Response = serde_json::from_str(&line)?;
        match resp {
            Response::Ok { data } => Ok(data),
            Response::Error { message } => Err(message.into()),
        }
    }
}

// Global process handle
static MOTIS_PROC: Lazy<Mutex<Option<MotisProcess>>> = Lazy::new(|| Mutex::new(None));

pub fn init(data_path: &str, exe_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new(exe_path)
        .arg(data_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;
    
    let stdin = child.stdin.take().ok_or("Failed to get stdin")?;
    
    let mut guard = MOTIS_PROC.lock()?;
    *guard = Some(MotisProcess { child, stdin });
    
    Ok(())
}

pub fn geocode(query: &str) -> Result<Vec<LocationResult>, Box<dyn std::error::Error>> {
    let mut guard = MOTIS_PROC.lock()?;
    let proc = guard.as_mut().ok_or("Not initialized")?;
    
    let data = proc.send_command(Request::Geocode { query: query.to_string() })?;
    let locations: Vec<LocationResult> = serde_json::from_value(data)?;
    Ok(locations)
}

pub fn plan_route(
    from_lat: f64, from_lon: f64,
    to_lat: f64, to_lon: f64
) -> Result<Vec<RouteResult>, Box<dyn std::error::Error>> {
    let mut guard = MOTIS_PROC.lock()?;
    let proc = guard.as_mut().ok_or("Not initialized")?;
    
    let data = proc.send_command(Request::PlanRoute { 
        from_lat, from_lon, to_lat, to_lon 
    })?;
    let routes: Vec<RouteResult> = serde_json::from_value(data)?;
    Ok(routes)
}

pub fn reverse_geocode(lat: f64, lon: f64) -> Result<Option<LocationResult>, Box<dyn std::error::Error>> {
    let mut guard = MOTIS_PROC.lock()?;
    let proc = guard.as_mut().ok_or("Not initialized")?;
    
    let data = proc.send_command(Request::ReverseGeocode { lat, lon })?;
    if data.is_null() {
        Ok(None)
    } else {
        let loc: LocationResult = serde_json::from_value(data)?;
        Ok(Some(loc))
    }
}

pub fn destroy() {
    let mut guard = MOTIS_PROC.lock().unwrap();
    if let Some(mut proc) = guard.take() {
        let _ = proc.child.kill();
    }
}
