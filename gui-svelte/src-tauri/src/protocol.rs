//! Custom protocol handler for motis:// requests
//! 
//! This module intercepts HTTP requests from the Svelte UI and routes them
//! to the IPC backend. This allows the Svelte UI to use standard fetch()
//! while communicating via IPC subprocess (no localhost HTTP server needed).

use tauri::http::{Request, Response, StatusCode, header::CONTENT_TYPE};
use std::borrow::Cow;
use serde_json::json;
use crate::native;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Read;

/// Handle motis:// scheme requests
pub fn handle_motis_request(
    request: Request<Vec<u8>>,
) -> Response<Cow<'static, [u8]>> {
    let path = request.uri().path();
    let query = request.uri().query().unwrap_or("");
    
    eprintln!("[MOTIS-PROTOCOL] Request: {}?{}", path, query);
    
    // Check if IPC is initialized (except for initial config which can work without)
    let mut is_initialized = native::is_ipc_initialized();
    
    // Try auto-initialization if not initialized (for USB/FAT32 launcher compatibility)
    if !is_initialized && !path.contains("/map/initial") {
        is_initialized = native::try_auto_init();
    }
    
    eprintln!("[MOTIS-PROTOCOL] IPC initialized: {}, path: {}", is_initialized, path);
    
    if !is_initialized && !path.contains("/map/initial") {
        let error = json!({
            "error": "MOTIS IPC not initialized",
            "message": "Please call init_native() first"
        });
        return Response::builder()
            .status(StatusCode::SERVICE_UNAVAILABLE)
            .header(CONTENT_TYPE, "application/json")
            .body(Cow::Owned(error.to_string().into_bytes()))
            .unwrap();
    }
    
    // Parse query parameters
    let params: std::collections::HashMap<String, String> = query
        .split('&')
        .filter(|s| !s.is_empty())
        .filter_map(|s| {
            let mut parts = s.splitn(2, '=');
            let key = parts.next()?.to_string();
            let value = parts.next().unwrap_or("").to_string();
            Some((key, urlencoding::decode(&value).unwrap_or_default().to_string()))
        })
        .collect();
    
    // Debug: log all parameters
    if !params.is_empty() {
        eprintln!("[MOTIS-PROTOCOL] Params: {:?}", params);
    }
    
    // Route to appropriate handler
    let result = match path {
        // Geocoding
        "/api/v1/geocode" | "/api/v5/geocode" => handle_geocode(&params),
        "/api/v1/reverse-geocode" | "/api/v5/reverse-geocode" => handle_reverse_geocode(&params),
        
        // Route planning
        "/api/v1/plan" | "/api/v5/plan" => handle_plan(&params),
        "/api/v1/trip" | "/api/v5/trip" => handle_api_passthrough(path, query),
        
        // Stop times
        "/api/v1/stoptimes" | "/api/v4/stoptimes" | "/api/v5/stoptimes" => {
            handle_api_passthrough(path, query)
        }
        
        // Map/RailViz
        "/api/v1/map/trips" | "/api/v4/map/trips" | "/api/v5/map/trips" => {
            handle_api_passthrough(path, query)
        }
        "/api/v1/map/initial" => handle_api_passthrough(path, query),
        "/api/v1/map/stops" => handle_api_passthrough(path, query),
        "/api/v1/map/levels" => handle_api_passthrough(path, query),
        
        // Routing variants
        "/api/v1/one-to-many" => handle_api_passthrough(path, query),
        "/api/v1/one-to-all" | "/api/experimental/one-to-all" => {
            handle_api_passthrough(path, query)
        }
        
        // Rentals (bike sharing, etc.)
        "/api/v1/rentals" | "/api/v1/map/rentals" => handle_api_passthrough(path, query),
        
        // Glyph requests used by MapLibre text rendering
        _ if path.starts_with("/tiles/glyphs/") => handle_glyphs(path),

        // Tiles (vector tiles for map)
        _ if path.starts_with("/api/v1/tiles/") 
            || path.starts_with("/api/v5/tiles/")
            || (path.starts_with("/tiles/") && path.ends_with(".mvt")) => {
            handle_tiles(path)
        }
        
        // Debug
        "/api/debug/transfers" => handle_debug_transfers(&params),
        
        // Catch all
        _ => Err(format!("Unknown endpoint: {}", path))
    };
    
    match result {
        Ok((body, content_type)) => {
            Response::builder()
                .status(StatusCode::OK)
                .header(CONTENT_TYPE, content_type)
                .header("Access-Control-Allow-Origin", "*")
                .body(Cow::Owned(body))
                .unwrap()
        }
        Err(e) => {
            eprintln!("[MOTIS-PROTOCOL] Error: {}", e);
            let error_json = json!({"error": e}).to_string();
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header(CONTENT_TYPE, "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(Cow::Owned(error_json.into_bytes()))
                .unwrap()
        }
    }
}

fn handle_geocode(params: &std::collections::HashMap<String, String>) 
    -> Result<(Vec<u8>, &'static str), String> {
    
    // Support multiple common parameter names
    let query = params.get("text")
        .or_else(|| params.get("q"))
        .or_else(|| params.get("query"))
        .or_else(|| params.get("search"))
        .ok_or("Missing query parameter (expected 'text', 'q', 'query', or 'search')")?;
    
    eprintln!("[MOTIS-PROTOCOL] Geocoding query: '{}'", query);
    
    match native::geocode_sync(query) {
        Ok(locations) => {
            eprintln!("[MOTIS-PROTOCOL] Geocode found {} results", locations.len());
            // Return array directly (not wrapped in content)
            Ok((serde_json::to_vec(&locations).unwrap(), "application/json"))
        }
        Err(e) => {
            eprintln!("[MOTIS-PROTOCOL] Geocode error: {}", e);
            Err(e.to_string())
        }
    }
}

fn handle_reverse_geocode(params: &std::collections::HashMap<String, String>)
    -> Result<(Vec<u8>, &'static str), String> {
    
    let lat: f64 = params.get("lat")
        .and_then(|s| s.parse().ok())
        .ok_or("Missing or invalid 'lat' parameter")?;
    
    let lon: f64 = params.get("lon")
        .and_then(|s| s.parse().ok())
        .ok_or("Missing or invalid 'lon' parameter")?;
    
    match native::reverse_geocode_sync(lat, lon) {
        Ok(Some(loc)) => {
            Ok((serde_json::to_vec(&loc).unwrap(), "application/json"))
        }
        Ok(None) => {
            Ok(("null".as_bytes().to_vec(), "application/json"))
        }
        Err(e) => Err(e.to_string())
    }
}

/// Get current time as ISO 8601 string
fn now_iso8601() -> String {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = since_epoch.as_secs() as i64;
    
    // Convert to ISO 8601 format: 2024-01-15T10:30:00+01:00
    let dt = chrono::DateTime::from_timestamp(secs, 0)
        .unwrap_or_else(|| chrono::DateTime::UNIX_EPOCH);
    dt.to_rfc3339()
}

/// Add seconds to current time and return ISO 8601 string
fn add_seconds_iso8601(seconds: i64) -> String {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = since_epoch.as_secs() as i64 + seconds;
    
    let dt = chrono::DateTime::from_timestamp(secs, 0)
        .unwrap_or_else(|| chrono::DateTime::UNIX_EPOCH);
    dt.to_rfc3339()
}

/// Generate display name for a leg
fn leg_display_name(mode: &str, route_short_name: &Option<String>) -> Option<String> {
    match mode {
        "WALK" => Some("Walk".to_string()),
        "BIKE" => Some("Bike".to_string()),
        "CAR" => Some("Drive".to_string()),
        _ => route_short_name.clone(),
    }
}

fn handle_plan(params: &std::collections::HashMap<String, String>)
    -> Result<(Vec<u8>, &'static str), String> {
    
    let from_place = params.get("fromPlace")
        .ok_or("Missing 'fromPlace' parameter")?;
    let to_place = params.get("toPlace")
        .ok_or("Missing 'toPlace' parameter")?;
    
    let (from_lat, from_lon) = parse_place(from_place)?;
    let (to_lat, to_lon) = parse_place(to_place)?;
    
    match native::plan_route_sync(from_lat, from_lon, to_lat, to_lon) {
        Ok(routes) => {
            let current_time = 0i64;
            
            // Convert legs to proper format with all required fields
            let convert_legs = |legs: &Vec<native::RouteLeg>, start_time: i64| -> (Vec<serde_json::Value>, i64) {
                let mut leg_time = start_time;
                let converted: Vec<_> = legs.iter().map(|l| {
                    let leg_start = add_seconds_iso8601(leg_time);
                    let leg_end = add_seconds_iso8601(leg_time + l.duration_seconds as i64);
                    leg_time += l.duration_seconds as i64;
                    
                    let display_name = leg_display_name(&l.mode, &l.route_short_name);
                    let is_transit = l.mode != "WALK" && l.mode != "BIKE" && l.mode != "CAR";
                    
                    json!({
                        "mode": l.mode,
                        "from": {
                            "name": l.from_name,
                            "lat": l.from.lat,
                            "lon": l.from.lon,
                            "vertexType": "NORMAL",
                            "tz": "Europe/Stockholm"
                        },
                        "to": {
                            "name": l.to_name,
                            "lat": l.to.lat,
                            "lon": l.to.lon,
                            "vertexType": "NORMAL",
                            "tz": "Europe/Stockholm"
                        },
                        "duration": l.duration_seconds,
                        "distance": l.distance_meters,
                        "startTime": leg_start,
                        "endTime": leg_end,
                        "scheduledStartTime": leg_start,
                        "scheduledEndTime": leg_end,
                        "realTime": false,
                        "scheduled": true,
                        "routeShortName": l.route_short_name,
                        "headsign": l.headsign,
                        "displayName": display_name,
                        "legGeometry": {
                            "points": "",
                            "length": 0
                        },
                        "interlineWithPreviousLeg": false,
                        "rentedBike": false,
                        "transitLeg": is_transit,
                        "walkingBike": false
                    })
                }).collect();
                (converted, leg_time)
            };
            
            // Build itineraries with proper timestamps
            let itineraries: Vec<_> = routes.iter().enumerate().map(|(idx, r)| {
                let (legs, end_offset) = convert_legs(&r.legs, current_time);
                let start_time = now_iso8601();
                let end_time = add_seconds_iso8601(end_offset);
                
                json!({
                    "id": format!("itinerary-{}", idx),
                    "duration": r.duration_seconds,
                    "startTime": start_time,
                    "endTime": end_time,
                    "transfers": r.transfers,
                    "legs": legs,
                    "fareTransfers": [],
                    "tooSloped": false
                })
            }).collect();
            
            // Build direct connections (non-transit routes like walking, biking, driving)
            let direct: Vec<_> = routes.iter().filter(|r| {
                r.legs.iter().all(|l| l.mode == "WALK" || l.mode == "CAR" || l.mode == "BIKE")
            }).enumerate().map(|(idx, r)| {
                let (legs, end_offset) = convert_legs(&r.legs, current_time);
                let start_time = now_iso8601();
                let end_time = add_seconds_iso8601(end_offset);
                
                json!({
                    "id": format!("direct-{}", idx),
                    "duration": r.duration_seconds,
                    "startTime": start_time,
                    "endTime": end_time,
                    "transfers": r.transfers,
                    "legs": legs,
                    "fareTransfers": [],
                    "tooSloped": false
                })
            }).collect();
            
            // Full MOTIS plan response format
            let response = json!({
                "requestParameters": params,
                "debugOutput": {},
                "from": {
                    "name": "Origin",
                    "lat": from_lat,
                    "lon": from_lon,
                    "vertexType": "NORMAL",
                    "tz": "Europe/Stockholm"
                },
                "to": {
                    "name": "Destination", 
                    "lat": to_lat,
                    "lon": to_lon,
                    "vertexType": "NORMAL",
                    "tz": "Europe/Stockholm"
                },
                "direct": direct,
                "itineraries": itineraries,
                "previousPageCursor": "",
                "nextPageCursor": ""
            });
            Ok((response.to_string().into_bytes(), "application/json"))
        }
        Err(e) => Err(e.to_string())
    }
}

fn handle_trip(_params: &std::collections::HashMap<String, String>)
    -> Result<(Vec<u8>, &'static str), String> {
    Err("Legacy trip handler should not be used".to_string())
}

fn handle_stoptimes(_params: &std::collections::HashMap<String, String>)
    -> Result<(Vec<u8>, &'static str), String> {
    Err("Legacy stoptimes handler should not be used".to_string())
}

fn handle_map_trips(_params: &std::collections::HashMap<String, String>)
    -> Result<(Vec<u8>, &'static str), String> {
    Err("Legacy map trips handler should not be used".to_string())
}

fn handle_map_initial(_params: &std::collections::HashMap<String, String>)
    -> Result<(Vec<u8>, &'static str), String> {
    Err("Legacy map initial handler should not be used".to_string())
}

fn handle_map_stops(_params: &std::collections::HashMap<String, String>)
    -> Result<(Vec<u8>, &'static str), String> {
    Err("Legacy map stops handler should not be used".to_string())
}

fn handle_map_levels(_params: &std::collections::HashMap<String, String>)
    -> Result<(Vec<u8>, &'static str), String> {
    Err("Legacy map levels handler should not be used".to_string())
}

fn handle_one_to_many(_params: &std::collections::HashMap<String, String>)
    -> Result<(Vec<u8>, &'static str), String> {
    Err("Legacy one-to-many handler should not be used".to_string())
}

fn handle_one_to_all(_params: &std::collections::HashMap<String, String>)
    -> Result<(Vec<u8>, &'static str), String> {
    Err("Legacy one-to-all handler should not be used".to_string())
}

fn handle_rentals(_params: &std::collections::HashMap<String, String>)
    -> Result<(Vec<u8>, &'static str), String> {
    Err("Legacy rentals handler should not be used".to_string())
}

fn handle_api_passthrough(path: &str, query: &str) -> Result<(Vec<u8>, &'static str), String> {
    let path_and_query = if query.is_empty() {
        path.to_string()
    } else {
        format!("{}?{}", path, query)
    };
    let value = native::api_get_sync(&path_and_query).map_err(|e| e.to_string())?;
    Ok((serde_json::to_vec(&value).unwrap_or_else(|_| b"{}".to_vec()), "application/json"))
}

fn handle_glyphs(path: &str) -> Result<(Vec<u8>, &'static str), String> {
    eprintln!("[MOTIS-PROTOCOL] Glyph request: {}", path);
    match native::get_glyph_sync(path) {
        Ok(Some(base64_data)) => {
            match base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &base64_data) {
                Ok(binary_data) => {
                    return Ok((binary_data, "application/x-protobuf"));
                }
                Err(e) => {
                    eprintln!("[MOTIS-PROTOCOL] Glyph base64 decode error: {}", e);
                }
            }
        }
        Ok(None) => {
            eprintln!("[MOTIS-PROTOCOL] Glyph not found: {}", path);
        }
        Err(e) => {
            eprintln!("[MOTIS-PROTOCOL] Glyph fetch error: {}", e);
        }
    }

    // Return empty glyph payload on error so map rendering can continue.
    Ok((vec![], "application/x-protobuf"))
}

fn maybe_inflate_zlib(data: &[u8]) -> Vec<u8> {
    // MOTIS tile payloads are often zlib-compressed. MapLibre expects
    // decodable MVT protobuf bytes at this stage.
    if data.len() < 2 || data[0] != 0x78 {
        return data.to_vec();
    }

    let mut decoder = flate2::read::ZlibDecoder::new(data);
    let mut inflated = Vec::new();
    match decoder.read_to_end(&mut inflated) {
        Ok(_) if !inflated.is_empty() => inflated,
        _ => data.to_vec(),
    }
}

fn handle_tiles(path: &str) -> Result<(Vec<u8>, &'static str), String> {
    // Vector tiles are binary MVT format
    eprintln!("[MOTIS-PROTOCOL] Tile request: {}", path);
    
    // Parse tile coordinates from path: /api/v1/tiles/{z}/{x}/{y}.mvt or /tiles/{z}/{x}/{y}.mvt
    let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();
    eprintln!("[MOTIS-PROTOCOL] Path parts: {:?}", parts);
    
    // Need at least 3 parts: tiles/z/x/y.mvt
    if parts.len() >= 3 {
        let z: i32 = parts[parts.len()-3].parse().map_err(|e| format!("Invalid z: {}", e))?;
        let x: i32 = parts[parts.len()-2].parse().map_err(|e| format!("Invalid x: {}", e))?;
        let y_str = parts.last().unwrap_or(&"");
        let y: i32 = y_str.trim_end_matches(".mvt").parse().map_err(|e| format!("Invalid y: {}", e))?;
        
        eprintln!("[MOTIS-PROTOCOL] Tile: z={}, x={}, y={}", z, x, y);
        
        // Fetch tile from IPC backend
        match native::get_tile_sync(z, x, y) {
            Ok(Some(base64_data)) => {
                eprintln!("[MOTIS-PROTOCOL] Got base64 data: {} chars", base64_data.len());
                // Decode base64 to binary
                match base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &base64_data) {
                    Ok(binary_data) => {
                        let tile_bytes = maybe_inflate_zlib(&binary_data);
                        eprintln!(
                            "[MOTIS-PROTOCOL] Tile decoded: {} bytes (inflated: {} bytes)",
                            binary_data.len(),
                            tile_bytes.len()
                        );
                        if tile_bytes.is_empty() {
                            eprintln!("[MOTIS-PROTOCOL] Warning: tile data is empty!");
                        }
                        return Ok((tile_bytes, "application/vnd.mapbox-vector-tile"));
                    }
                    Err(e) => {
                        eprintln!("[MOTIS-PROTOCOL] Base64 decode error: {}", e);
                    }
                }
            }
            Ok(None) => {
                eprintln!("[MOTIS-PROTOCOL] Tile not found");
            }
            Err(e) => {
                eprintln!("[MOTIS-PROTOCOL] Tile fetch error: {}", e);
            }
        }
    }
    
    // Return empty tile on error (this is what map libraries expect for missing tiles)
    eprintln!("[MOTIS-PROTOCOL] Returning empty tile");
    Ok((vec![], "application/vnd.mapbox-vector-tile"))
}

fn handle_debug_transfers(_params: &std::collections::HashMap<String, String>)
    -> Result<(Vec<u8>, &'static str), String> {
    Ok(("[]".as_bytes().to_vec(), "application/json"))
}

// Helper function to parse "lat, lon" format
fn parse_place(s: &str) -> Result<(f64, f64), String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid place format: {}", s));
    }
    let lat = parts[0].trim().parse::<f64>()
        .map_err(|e| format!("Invalid lat: {}", e))?;
    let lon = parts[1].trim().parse::<f64>()
        .map_err(|e| format!("Invalid lon: {}", e))?;
    Ok((lat, lon))
}
