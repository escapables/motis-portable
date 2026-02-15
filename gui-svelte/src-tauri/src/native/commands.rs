use super::ipc::send_ipc_json_command;
use super::types::{LocationResult, Match, RouteResult};

pub(crate) fn build_geocode_command(query: &str) -> String {
    serde_json::json!({
        "cmd": "geocode",
        "query": query,
    })
    .to_string()
}

fn build_plan_route_command(from_lat: f64, from_lon: f64, to_lat: f64, to_lon: f64) -> String {
    serde_json::json!({
        "cmd": "plan_route",
        "from_lat": from_lat,
        "from_lon": from_lon,
        "to_lat": to_lat,
        "to_lon": to_lon,
    })
    .to_string()
}

fn build_reverse_geocode_command(lat: f64, lon: f64) -> String {
    serde_json::json!({
        "cmd": "reverse_geocode",
        "lat": lat,
        "lon": lon,
    })
    .to_string()
}

pub async fn geocode(query: &str) -> Result<Vec<Match>, Box<dyn std::error::Error>> {
    eprintln!("[MOTIS-GUI] geocode() called with query: '{}'", query);

    let cmd = build_geocode_command(query);
    eprintln!("[MOTIS-GUI] Sending command: {}", cmd);
    let data = send_ipc_json_command(&cmd)?;
    let locations: Vec<LocationResult> = serde_json::from_value(data)?;
    eprintln!("[MOTIS-GUI] Found {} locations", locations.len());
    let matches: Vec<Match> = locations.iter().map(Match::from_location_result).collect();
    Ok(matches)
}

pub async fn plan_route(
    from_lat: f64,
    from_lon: f64,
    to_lat: f64,
    to_lon: f64,
) -> Result<Vec<RouteResult>, Box<dyn std::error::Error>> {
    eprintln!(
        "[MOTIS-GUI] plan_route() called: ({}, {}) to ({}, {})",
        from_lat, from_lon, to_lat, to_lon
    );

    let cmd = build_plan_route_command(from_lat, from_lon, to_lat, to_lon);
    eprintln!("[MOTIS-GUI] Sending command: {}", cmd);
    let data = send_ipc_json_command(&cmd)?;
    let routes: Vec<RouteResult> = serde_json::from_value(data)?;
    eprintln!("[MOTIS-GUI] Found {} routes", routes.len());
    Ok(routes)
}

pub async fn reverse_geocode(
    lat: f64,
    lon: f64,
) -> Result<Option<Match>, Box<dyn std::error::Error>> {
    let cmd = build_reverse_geocode_command(lat, lon);
    let data = send_ipc_json_command(&cmd)?;
    if !data.is_null() {
        let loc: LocationResult = serde_json::from_value(data)?;
        Ok(Some(Match::from_location_result(&loc)))
    } else {
        Ok(None)
    }
}

pub fn geocode_sync(query: &str) -> Result<Vec<Match>, Box<dyn std::error::Error>> {
    let cmd = build_geocode_command(query);
    let data = send_ipc_json_command(&cmd)?;
    let locations: Vec<LocationResult> = serde_json::from_value(data)?;
    let matches: Vec<Match> = locations.iter().map(Match::from_location_result).collect();
    Ok(matches)
}

pub fn plan_route_sync(
    from_lat: f64,
    from_lon: f64,
    to_lat: f64,
    to_lon: f64,
) -> Result<Vec<RouteResult>, Box<dyn std::error::Error>> {
    let cmd = build_plan_route_command(from_lat, from_lon, to_lat, to_lon);
    let data = send_ipc_json_command(&cmd)?;
    let routes: Vec<RouteResult> = serde_json::from_value(data)?;
    Ok(routes)
}

pub fn reverse_geocode_sync(
    lat: f64,
    lon: f64,
) -> Result<Option<Match>, Box<dyn std::error::Error>> {
    let cmd = build_reverse_geocode_command(lat, lon);
    let data = send_ipc_json_command(&cmd)?;
    if !data.is_null() {
        let loc: LocationResult = serde_json::from_value(data)?;
        Ok(Some(Match::from_location_result(&loc)))
    } else {
        Ok(None)
    }
}

pub fn get_tile_sync(
    z: i32,
    x: i32,
    y: i32,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let cmd = serde_json::json!({
        "cmd": "get_tile",
        "z": z,
        "x": x,
        "y": y
    })
    .to_string();

    let data = send_ipc_json_command(&cmd)?;
    if data["found"].as_bool().unwrap_or(false) {
        let base64_data = data["data_base64"].as_str().ok_or("Invalid tile data")?;
        Ok(Some(base64_data.to_string()))
    } else {
        Ok(None)
    }
}

pub fn get_glyph_sync(path: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let cmd = serde_json::json!({
        "cmd": "get_glyph",
        "path": path
    })
    .to_string();

    let data = send_ipc_json_command(&cmd)?;

    if data["found"].as_bool().unwrap_or(false) {
        let base64_data = data["data_base64"].as_str().ok_or("Invalid glyph data")?;
        Ok(Some(base64_data.to_string()))
    } else {
        Ok(None)
    }
}

pub fn api_get_sync(path_and_query: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let cmd = serde_json::json!({
        "cmd": "api_get",
        "path": path_and_query
    })
    .to_string();

    send_ipc_json_command(&cmd)
}
