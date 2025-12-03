use std::{future::Future, sync::Arc, time::Duration};

use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde_json::{json, Value};
use tokio::sync::Mutex;
use tracing::{error, info};

use crate::{
    clients::{ExternalClients, IssClient, NasaClient, OsdrClient, SpacexClient},
    config::AppConfig,
    domain::{IssEntry, IssTrend, OsdrRecord, RefreshResult, SpaceLatestResponse, SpaceSummary},
    repo::{CacheRepo, IssRepo, OsdrRepo, Repositories},
};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub services: Arc<ServiceRegistry>,
}

#[derive(Clone)]
pub struct ServiceRegistry {
    pub iss: Arc<IssService>,
    pub osdr: Arc<OsdrService>,
    pub space: Arc<SpaceCacheService>,
}

pub struct IssService {
    repo: IssRepo,
    client: IssClient,
    fetch_lock: Arc<Mutex<()>>,
}

pub struct OsdrService {
    repo: OsdrRepo,
    client: OsdrClient,
    fetch_lock: Arc<Mutex<()>>,
}

pub struct SpaceCacheService {
    cache_repo: CacheRepo,
    iss_repo: IssRepo,
    osdr_repo: OsdrRepo,
    nasa: NasaClient,
    spacex: SpacexClient,
    refresh_lock: Arc<Mutex<()>>,
    default_sources: Vec<String>,
}

impl AppState {
    pub fn new(config: AppConfig, registry: ServiceRegistry) -> Self {
        Self {
            config,
            services: Arc::new(registry),
        }
    }
}

impl ServiceRegistry {
    pub fn new(repos: &Repositories, clients: &ExternalClients, config: &AppConfig) -> Self {
        Self {
            iss: Arc::new(IssService::new(repos.iss.clone(), clients.iss.clone())),
            osdr: Arc::new(OsdrService::new(repos.osdr.clone(), clients.osdr.clone())),
            space: Arc::new(SpaceCacheService::new(
                repos.cache.clone(),
                repos.iss.clone(),
                repos.osdr.clone(),
                clients.nasa.clone(),
                clients.spacex.clone(),
                config.refresh_defaults.clone(),
            )),
        }
    }
}

impl IssService {
    pub fn new(repo: IssRepo, client: IssClient) -> Self {
        Self {
            repo,
            client,
            fetch_lock: Arc::new(Mutex::new(())),
        }
    }

    pub async fn last(&self) -> Result<Option<IssEntry>> {
        Ok(self.repo.last_entry().await?)
    }

    pub async fn trigger_fetch(&self) -> Result<IssEntry> {
        self.fetch_and_store().await
    }

    pub async fn trend(&self) -> Result<IssTrend> {
        let rows = self.repo.last_points(2).await?;
        if rows.len() < 2 {
            return Ok(IssTrend {
                movement: false,
                delta_km: 0.0,
                dt_sec: 0.0,
                velocity_kmh: None,
                from_time: None,
                to_time: None,
                from_lat: None,
                from_lon: None,
                to_lat: None,
                to_lon: None,
            });
        }
        Ok(compute_trend(&rows[1], &rows[0]))
    }

    pub async fn fetch_job(&self) -> Result<()> {
        let entry = self.fetch_and_store().await?;
        info!("iss snapshot stored at {}", entry.fetched_at);
        Ok(())
    }

    fn source_url(&self) -> &str {
        self.client.url()
    }

    async fn fetch_and_store(&self) -> Result<IssEntry> {
        let _guard = self.fetch_lock.lock().await;
        let payload = self.client.fetch().await?;
        let entry = self.repo.insert_entry(self.source_url(), payload).await?;
        Ok(entry)
    }
}

impl OsdrService {
    pub fn new(repo: OsdrRepo, client: OsdrClient) -> Self {
        Self {
            repo,
            client,
            fetch_lock: Arc::new(Mutex::new(())),
        }
    }

    pub async fn sync(&self) -> Result<usize> {
        let _guard = self.fetch_lock.lock().await;
        let items = self.client.fetch().await?;
        let mut written = 0usize;
        for item in items {
            let dataset_id = pick_string(&item, &["dataset_id", "id", "uuid", "studyId", "accession", "osdr_id"]);
            let title = pick_string(&item, &["title", "name", "label"]);
            let status = pick_string(&item, &["status", "state", "lifecycle"]);
            let updated = pick_datetime(&item, &["updated", "updated_at", "modified", "lastUpdated", "timestamp"]);
            self.repo
                .upsert_item(dataset_id, title, status, updated, item)
                .await?;
            written += 1;
        }
        Ok(written)
    }

    pub async fn list(&self, limit: i64) -> Result<Vec<OsdrRecord>> {
        Ok(self.repo.list(limit).await?)
    }

    pub async fn count(&self) -> Result<i64> {
        Ok(self.repo.count().await?)
    }

    pub async fn fetch_job(&self) -> Result<()> {
        let count = self.sync().await?;
        info!("osdr synced {count} records");
        Ok(())
    }
}

impl SpaceCacheService {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cache_repo: CacheRepo,
        iss_repo: IssRepo,
        osdr_repo: OsdrRepo,
        nasa: NasaClient,
        spacex: SpacexClient,
        default_sources: Vec<String>,
    ) -> Self {
        Self {
            cache_repo,
            iss_repo,
            osdr_repo,
            nasa,
            spacex,
            refresh_lock: Arc::new(Mutex::new(())),
            default_sources,
        }
    }

    pub fn default_sources(&self) -> Vec<String> {
        self.default_sources.clone()
    }

    pub async fn latest(&self, source: &str) -> Result<SpaceLatestResponse> {
        let source = source.to_lowercase();
        if let Some(item) = self.cache_repo.latest(&source).await? {
            return Ok(SpaceLatestResponse::from_item(item));
        }
        Ok(SpaceLatestResponse::empty(source))
    }

    pub async fn refresh_sources(&self, sources: &[String]) -> Result<RefreshResult> {
        let _guard = self.refresh_lock.lock().await;
        let mut refreshed = Vec::new();
        for src in sources {
            let normalized = src.trim().to_lowercase();
            match self.refresh_source(&normalized).await {
                Ok(_) => refreshed.push(normalized),
                Err(err) => {
                    error!(target: "space_cache", source = src, error = %err, "refresh failed");
                }
            }
        }

        if refreshed.is_empty() {
            return Err(anyhow!("no sources refreshed successfully"));
        }

        Ok(RefreshResult { refreshed })
    }

    pub async fn summary(&self) -> Result<SpaceSummary> {
        let sources = vec![
            String::from("apod"),
            String::from("neo"),
            String::from("flr"),
            String::from("cme"),
            String::from("spacex"),
        ];
        let latest = self.cache_repo.latest_by_sources(&sources).await?;
        let apod = latest.get("apod").cloned().unwrap_or_else(|| json!({}));
        let neo = latest.get("neo").cloned().unwrap_or_else(|| json!({}));
        let flr = latest.get("flr").cloned().unwrap_or_else(|| json!({}));
        let cme = latest.get("cme").cloned().unwrap_or_else(|| json!({}));
        let spacex = latest.get("spacex").cloned().unwrap_or_else(|| json!({}));

        let iss_json = if let Some(last) = self.iss_repo.last_entry().await? {
            json!({ "at": last.fetched_at, "payload": last.payload })
        } else {
            json!({})
        };
        let osdr_count = self.osdr_repo.count().await?;

        Ok(SpaceSummary {
            apod,
            neo,
            flr,
            cme,
            spacex,
            iss: iss_json,
            osdr_count,
        })
    }

    pub async fn refresh_defaults(&self) -> Result<RefreshResult> {
        let sources = self.default_sources();
        self.refresh_sources(&sources).await
    }

    async fn refresh_source(&self, source: &str) -> Result<()> {
        match source {
            "apod" => {
                let payload = self.nasa.apod().await?;
                self.cache_repo.write("apod", payload).await?;
            }
            "neo" => {
                let (start, end) = last_days(2);
                let payload = self.nasa.neo_feed(&start, &end).await?;
                self.cache_repo.write("neo", payload).await?;
            }
            "flr" => {
                let (start, end) = last_days(5);
                let payload = self.nasa.donki_flr(&start, &end).await?;
                self.cache_repo.write("flr", payload).await?;
            }
            "cme" => {
                let (start, end) = last_days(5);
                let payload = self.nasa.donki_cme(&start, &end).await?;
                self.cache_repo.write("cme", payload).await?;
            }
            "spacex" => {
                let payload = self.spacex.next_launch().await?;
                self.cache_repo.write("spacex", payload).await?;
            }
            other => return Err(anyhow!("unsupported source {other}")),
        }
        Ok(())
    }
}

pub fn spawn_jobs(state: &AppState) {
    let schedule = state.config.scheduler.clone();
    let services = state.services.clone();

    spawn_periodic("iss_fetch", schedule.iss, {
        let iss = services.iss.clone();
        move || {
            let iss = iss.clone();
            async move { iss.fetch_job().await }
        }
    });

    spawn_periodic("osdr_sync", schedule.osdr, {
        let osdr = services.osdr.clone();
        move || {
            let osdr = osdr.clone();
            async move { osdr.fetch_job().await }
        }
    });

    spawn_periodic("space_apod", schedule.apod, {
        let space = services.space.clone();
        move || {
            let space = space.clone();
            async move {
                let sources = vec![String::from("apod")];
                space.refresh_sources(&sources).await.map(|_| ())
            }
        }
    });

    spawn_periodic("space_neo", schedule.neo, {
        let space = services.space.clone();
        move || {
            let space = space.clone();
            async move {
                let sources = vec![String::from("neo")];
                space.refresh_sources(&sources).await.map(|_| ())
            }
        }
    });

    spawn_periodic("space_donki", schedule.donki, {
        let space = services.space.clone();
        move || {
            let space = space.clone();
            async move {
                let sources = vec![String::from("flr"), String::from("cme")];
                space.refresh_sources(&sources).await.map(|_| ())
            }
        }
    });

    spawn_periodic("space_spacex", schedule.spacex, {
        let space = services.space.clone();
        move || {
            let space = space.clone();
            async move {
                let sources = vec![String::from("spacex")];
                space.refresh_sources(&sources).await.map(|_| ())
            }
        }
    });
}

fn spawn_periodic<F, Fut>(name: &'static str, interval: Duration, mut job_factory: F)
where
    F: FnMut() -> Fut + Send + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    tokio::spawn(async move {
        loop {
            if let Err(err) = job_factory().await {
                error!(target = "scheduler", job = name, error = %err, "background job failed");
            }
            tokio::time::sleep(interval).await;
        }
    });
}

fn compute_trend(previous: &IssEntry, newest: &IssEntry) -> IssTrend {
    let lat1 = newest.payload.get("latitude").and_then(Value::as_f64);
    let lon1 = newest.payload.get("longitude").and_then(Value::as_f64);
    let lat0 = previous.payload.get("latitude").and_then(Value::as_f64);
    let lon0 = previous.payload.get("longitude").and_then(Value::as_f64);
    let velocity = newest.payload.get("velocity").and_then(Value::as_f64);

    let (movement, delta_km) = if let (Some(a1), Some(o1), Some(a0), Some(o0)) = (lat1, lon1, lat0, lon0) {
        let dist = haversine_km(a0, o0, a1, o1);
        (dist > 0.1, dist)
    } else {
        (false, 0.0)
    };
    let dt_sec = (newest.fetched_at - previous.fetched_at).num_milliseconds() as f64 / 1_000.0;

    IssTrend {
        movement,
        delta_km,
        dt_sec,
        velocity_kmh: velocity,
        from_time: Some(previous.fetched_at),
        to_time: Some(newest.fetched_at),
        from_lat: lat0,
        from_lon: lon0,
        to_lat: lat1,
        to_lon: lon1,
    }
}

fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let rlat1 = lat1.to_radians();
    let rlat2 = lat2.to_radians();
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2) + rlat1.cos() * rlat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    6371.0 * c
}

fn last_days(days: i64) -> (String, String) {
    let end = Utc::now().date_naive();
    let start = end - chrono::Days::new(days as u64);
    (start.to_string(), end.to_string())
}

fn pick_string(value: &Value, keys: &[&str]) -> Option<String> {
    for key in keys {
        if let Some(v) = value.get(*key) {
            if let Some(s) = v.as_str() {
                if !s.is_empty() {
                    return Some(s.to_string());
                }
            } else if v.is_number() {
                return Some(v.to_string());
            }
        }
    }
    None
}

fn pick_datetime(value: &Value, keys: &[&str]) -> Option<DateTime<Utc>> {
    for key in keys {
        if let Some(v) = value.get(*key) {
            if let Some(s) = v.as_str() {
                if let Ok(dt) = s.parse::<DateTime<Utc>>() {
                    return Some(dt);
                }
                if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
                    return Some(Utc.from_utc_datetime(&ndt));
                }
            } else if let Some(epoch) = v.as_i64() {
                if let Some(dt) = Utc.timestamp_opt(epoch, 0).single() {
                    return Some(dt);
                }
            }
        }
    }
    None
}

