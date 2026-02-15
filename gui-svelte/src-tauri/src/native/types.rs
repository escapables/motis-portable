use serde::{Deserialize, Serialize};

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
pub struct Area {
    pub name: String,
    pub admin_level: i32,
    pub matched: bool,
    pub unique: bool,
    #[serde(rename = "default")]
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationResult {
    pub name: String,
    pub place_id: String,
    pub lat: f64,
    pub lon: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default)]
    pub areas: Vec<MatchArea>,
    #[serde(default)]
    pub tokens: Vec<Vec<i32>>,
    pub score: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub house_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchArea {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

pub type Token = [i32; 2];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    pub id: String,
    pub lat: f64,
    pub lon: f64,
    pub tokens: Vec<Token>,
    pub areas: Vec<MatchArea>,
    pub score: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub house_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tz: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importance: Option<f64>,
}

impl Match {
    pub fn from_location_result(loc: &LocationResult) -> Self {
        let tokens: Vec<Token> = loc
            .tokens
            .iter()
            .filter(|t| t.len() >= 2)
            .map(|t| [t[0], t[1]])
            .collect();

        let type_ = loc.type_.clone().unwrap_or_else(|| "PLACE".to_string());

        Match {
            type_,
            name: loc.name.clone(),
            id: loc.place_id.clone(),
            lat: loc.lat,
            lon: loc.lon,
            tokens,
            areas: loc.areas.clone(),
            score: loc.score,
            category: loc.category.clone(),
            street: loc.street.clone(),
            house_number: loc.house_number.clone(),
            country: loc.country.clone(),
            zip: loc.zip.clone(),
            tz: None,
            level: None,
            modes: loc.modes.clone(),
            importance: loc.importance,
        }
    }
}
