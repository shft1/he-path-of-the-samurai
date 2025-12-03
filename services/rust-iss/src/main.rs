mod clients;
mod config;
mod domain;
mod error;
mod handlers;
mod repo;
mod routes;
mod services;

use anyhow::Context;
use config::AppConfig;
use repo::Repositories;
use services::{spawn_jobs, AppState, ServiceRegistry};
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::clients::ExternalClients;
use crate::routes::build_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();
    dotenvy::dotenv().ok();

    let config = AppConfig::load()?;
    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .acquire_timeout(config.database.acquire_timeout)
        .connect(&config.database.url)
        .await
        .context("failed to connect PostgreSQL")?;

    let repos = Repositories::new(pool);
    repos.migrate().await?;

    let clients = ExternalClients::new(&config)?;
    let services = ServiceRegistry::new(&repos, &clients, &config);
    let state = AppState::new(config.clone(), services);

    spawn_jobs(&state);

    let router = build_router(state.clone());
    let addr = config.server.socket_addr()?;
    info!("rust_iss listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}

fn init_tracing() {
    let fmt_layer = tracing_subscriber::fmt::layer().with_target(false);
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();
}
