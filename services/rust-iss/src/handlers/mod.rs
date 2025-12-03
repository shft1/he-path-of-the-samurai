use axum::extract::{Path, Query, State};
use chrono::Utc;
use serde::Deserialize;
use serde_json::json;

use crate::{
    domain::{HealthDto},
    error::{respond, ApiError, ApiResult},
    services::AppState,
};

#[derive(Deserialize)]
pub struct RefreshQuery {
    src: Option<String>,
}

#[derive(Deserialize)]
pub struct OsdrListQuery {
    limit: Option<i64>,
}

pub async fn health() -> ApiResult<HealthDto> {
    respond(HealthDto {
        status: "ok",
        now: Utc::now(),
    })
}

pub async fn last_iss(State(state): State<AppState>) -> ApiResult<serde_json::Value> {
    let svc = state.services.iss.clone();
    let payload = svc
        .last()
        .await
        .map_err(|err| ApiError::from_error("DB_ISS_LAST_FAILED", err))?;
    if let Some(entry) = payload {
        respond(json!({
            "id": entry.id,
            "fetched_at": entry.fetched_at,
            "source_url": entry.source_url,
            "payload": entry.payload
        }))
    } else {
        respond(json!({ "message": "no data" }))
    }
}

pub async fn trigger_iss(State(state): State<AppState>) -> ApiResult<serde_json::Value> {
    let svc = state.services.iss.clone();
    let entry = svc
        .trigger_fetch()
        .await
        .map_err(|err| ApiError::from_error("ISS_FETCH_FAILED", err))?;
    respond(json!({
        "id": entry.id,
        "fetched_at": entry.fetched_at,
        "source_url": entry.source_url,
        "payload": entry.payload
    }))
}

pub async fn iss_trend(State(state): State<AppState>) -> ApiResult<crate::domain::IssTrend> {
    let svc = state.services.iss.clone();
    let trend = svc
        .trend()
        .await
        .map_err(|err| ApiError::from_error("ISS_TREND_FAILED", err))?;
    respond(trend)
}

pub async fn osdr_sync(State(state): State<AppState>) -> ApiResult<serde_json::Value> {
    let svc = state.services.osdr.clone();
    let written = svc
        .sync()
        .await
        .map_err(|err| ApiError::from_error("OSDR_SYNC_FAILED", err))?;
    respond(json!({ "written": written }))
}

pub async fn osdr_list(
    State(state): State<AppState>,
    Query(query): Query<OsdrListQuery>,
) -> ApiResult<serde_json::Value> {
    let limit = query
        .limit
        .unwrap_or(state.config.osdr_list_limit)
        .clamp(1, 200);
    let items = state
        .services
        .osdr
        .list(limit)
        .await
        .map_err(|err| ApiError::from_error("OSDR_LIST_FAILED", err))?;
    respond(json!({ "items": items }))
}

pub async fn space_latest(
    Path(source): Path<String>,
    State(state): State<AppState>,
) -> ApiResult<crate::domain::SpaceLatestResponse> {
    let payload = state
        .services
        .space
        .latest(&source)
        .await
        .map_err(|err| ApiError::from_error("SPACE_LATEST_FAILED", err))?;
    respond(payload)
}

pub async fn space_refresh(
    Query(query): Query<RefreshQuery>,
    State(state): State<AppState>,
) -> ApiResult<crate::domain::RefreshResult> {
    let sources = query
        .src
        .map(|raw| raw.split(',').map(|s| s.trim().to_lowercase()).filter(|s| !s.is_empty()).collect())
        .filter(|list: &Vec<String>| !list.is_empty())
        .unwrap_or_else(|| state.config.refresh_defaults.clone());

    let result = state
        .services
        .space
        .refresh_sources(&sources)
        .await
        .map_err(|err| ApiError::from_error("SPACE_REFRESH_FAILED", err))?;
    respond(result)
}

pub async fn space_summary(State(state): State<AppState>) -> ApiResult<crate::domain::SpaceSummary> {
    let summary = state
        .services
        .space
        .summary()
        .await
        .map_err(|err| ApiError::from_error("SPACE_SUMMARY_FAILED", err))?;
    respond(summary)
}

