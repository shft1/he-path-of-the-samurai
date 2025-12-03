use axum::{routing::get, Router};

use crate::{handlers, services::AppState};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/last", get(handlers::last_iss))
        .route("/fetch", get(handlers::trigger_iss))
        .route("/iss/trend", get(handlers::iss_trend))
        .route("/osdr/sync", get(handlers::osdr_sync))
        .route("/osdr/list", get(handlers::osdr_list))
        .route("/space/:src/latest", get(handlers::space_latest))
        .route("/space/refresh", get(handlers::space_refresh))
        .route("/space/summary", get(handlers::space_summary))
        .with_state(state)
}

