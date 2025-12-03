use std::{env, net::SocketAddr, time::Duration};

use anyhow::{anyhow, Context};

#[derive(Clone)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub osdr_url: String,
    pub iss_url: String,
    pub nasa_api_key: Option<String>,
    pub osdr_list_limit: i64,
    pub http_timeout: Duration,
    pub user_agent: String,
    pub scheduler: SchedulerConfig,
    pub refresh_defaults: Vec<String>,
}

#[derive(Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub acquire_timeout: Duration,
}

#[derive(Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Clone)]
pub struct SchedulerConfig {
    pub osdr: Duration,
    pub iss: Duration,
    pub apod: Duration,
    pub neo: Duration,
    pub donki: Duration,
    pub spacex: Duration,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let database = DatabaseConfig::load()?;
        let server = ServerConfig::load();

        let osdr_url = env::var("NASA_API_URL")
            .unwrap_or_else(|_| "https://visualization.osdr.nasa.gov/biodata/api/v2/datasets/?format=json".to_string());
        let iss_url = env::var("WHERE_ISS_URL")
            .unwrap_or_else(|_| "https://api.wheretheiss.at/v1/satellites/25544".to_string());
        let nasa_api_key = env::var("NASA_API_KEY").ok().filter(|v| !v.is_empty());
        let osdr_list_limit = env_i64("OSDR_LIST_LIMIT", 20);
        let http_timeout = env_duration("HTTP_TIMEOUT_SECONDS", 30);
        let user_agent = env::var("HTTP_USER_AGENT").unwrap_or_else(|_| "rust_iss/1.0".to_string());
        let scheduler = SchedulerConfig::load();
        let refresh_defaults = env::var("SPACE_REFRESH_DEFAULTS")
            .map(|raw| parse_sources(&raw))
            .unwrap_or_else(|_| vec!["apod", "neo", "flr", "cme", "spacex"].into_iter().map(String::from).collect());

        Ok(Self {
            database,
            server,
            osdr_url,
            iss_url,
            nasa_api_key,
            osdr_list_limit,
            http_timeout,
            user_agent,
            scheduler,
            refresh_defaults,
        })
    }
}

impl DatabaseConfig {
    fn load() -> anyhow::Result<Self> {
        let url = env::var("DATABASE_URL").context("DATABASE_URL is required")?;
        let max_connections = env_u32("DB_MAX_CONNECTIONS", 5);
        let acquire_timeout = env_duration("DB_ACQUIRE_TIMEOUT_SEC", 10);
        Ok(Self {
            url,
            max_connections,
            acquire_timeout,
        })
    }
}

impl ServerConfig {
    fn load() -> Self {
        let host = env::var("RUST_ISS_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env_u16("RUST_ISS_PORT", 3000);
        Self { host, port }
    }

    pub fn socket_addr(&self) -> anyhow::Result<SocketAddr> {
        let addr = format!("{}:{}", self.host, self.port);
        addr.parse().map_err(|e| anyhow!("invalid bind address {addr}: {e}"))
    }
}

impl SchedulerConfig {
    fn load() -> Self {
        Self {
            osdr: env_duration("FETCH_EVERY_SECONDS", 600),
            iss: env_duration("ISS_EVERY_SECONDS", 120),
            apod: env_duration("APOD_EVERY_SECONDS", 43_200),
            neo: env_duration("NEO_EVERY_SECONDS", 7_200),
            donki: env_duration("DONKI_EVERY_SECONDS", 3_600),
            spacex: env_duration("SPACEX_EVERY_SECONDS", 3_600),
        }
    }
}

fn env_duration(key: &str, default_secs: u64) -> Duration {
    env::var(key)
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .map(Duration::from_secs)
        .unwrap_or_else(|| Duration::from_secs(default_secs))
}

fn env_u32(key: &str, default: u32) -> u32 {
    env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn env_u16(key: &str, default: u16) -> u16 {
    env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn env_i64(key: &str, default: i64) -> i64 {
    env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn parse_sources(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect()
}

