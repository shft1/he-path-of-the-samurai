use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct HealthDto {
    pub status: &'static str,
    pub now: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IssEntry {
    pub id: i64,
    pub fetched_at: DateTime<Utc>,
    pub source_url: String,
    pub payload: Value,
}

#[derive(Debug, Clone)]
pub struct IssTrendPoints {
    pub newest: IssEntry,
    pub previous: IssEntry,
}

#[derive(Debug, Clone, Serialize)]
pub struct IssTrend {
    pub movement: bool,
    pub delta_km: f64,
    pub dt_sec: f64,
    pub velocity_kmh: Option<f64>,
    pub from_time: Option<DateTime<Utc>>,
    pub to_time: Option<DateTime<Utc>>,
    pub from_lat: Option<f64>,
    pub from_lon: Option<f64>,
    pub to_lat: Option<f64>,
    pub to_lon: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OsdrRecord {
    pub id: i64,
    pub dataset_id: Option<String>,
    pub title: Option<String>,
    pub status: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
    pub inserted_at: DateTime<Utc>,
    pub raw: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct SpaceCacheItem {
    pub source: String,
    pub fetched_at: DateTime<Utc>,
    pub payload: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct SpaceLatestResponse {
    pub source: String,
    pub fetched_at: Option<DateTime<Utc>>,
    pub payload: Value,
    pub message: Option<&'static str>,
}

impl SpaceLatestResponse {
    pub fn empty(source: String) -> Self {
        Self {
            source,
            fetched_at: None,
            payload: Value::Null,
            message: Some("no data"),
        }
    }

    pub fn from_item(item: SpaceCacheItem) -> Self {
        Self {
            source: item.source,
            fetched_at: Some(item.fetched_at),
            payload: item.payload,
            message: None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SpaceSummary {
    pub apod: Value,
    pub neo: Value,
    pub flr: Value,
    pub cme: Value,
    pub spacex: Value,
    pub iss: Value,
    pub osdr_count: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RefreshResult {
    pub refreshed: Vec<String>,
}

