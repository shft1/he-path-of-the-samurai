use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use sqlx::{PgPool, Row};

use crate::domain::{IssEntry, OsdrRecord, SpaceCacheItem};

#[derive(Clone)]
pub struct Repositories {
    pool: PgPool,
    pub iss: IssRepo,
    pub osdr: OsdrRepo,
    pub cache: CacheRepo,
}

#[derive(Clone)]
pub struct IssRepo {
    pool: PgPool,
}

#[derive(Clone)]
pub struct OsdrRepo {
    pool: PgPool,
}

#[derive(Clone)]
pub struct CacheRepo {
    pool: PgPool,
}

impl Repositories {
    pub fn new(pool: PgPool) -> Self {
        Self {
            iss: IssRepo { pool: pool.clone() },
            osdr: OsdrRepo { pool: pool.clone() },
            cache: CacheRepo { pool: pool.clone() },
            pool,
        }
    }

    pub fn pool(&self) -> PgPool {
        self.pool.clone()
    }

    pub async fn migrate(&self) -> anyhow::Result<()> {
        self.iss.init().await?;
        self.osdr.init().await?;
        self.cache.init().await?;
        Ok(())
    }
}

impl IssRepo {
    async fn init(&self) -> sqlx::Result<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS iss_fetch_log(
                id BIGSERIAL PRIMARY KEY,
                fetched_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                source_url TEXT NOT NULL,
                payload JSONB NOT NULL
            )",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_entry(&self, source_url: &str, payload: Value) -> sqlx::Result<IssEntry> {
        sqlx::query(
            "INSERT INTO iss_fetch_log(source_url, payload)
             VALUES ($1,$2)
             RETURNING id, fetched_at, source_url, payload",
        )
        .bind(source_url)
        .bind(payload)
        .map(|row: sqlx::postgres::PgRow| IssEntry {
            id: row.get("id"),
            fetched_at: row.get("fetched_at"),
            source_url: row.get("source_url"),
            payload: row.get("payload"),
        })
        .fetch_one(&self.pool)
        .await
    }

    pub async fn last_entry(&self) -> sqlx::Result<Option<IssEntry>> {
        sqlx::query(
            "SELECT id, fetched_at, source_url, payload
             FROM iss_fetch_log
             ORDER BY id DESC LIMIT 1",
        )
        .map(|row: sqlx::postgres::PgRow| IssEntry {
            id: row.get("id"),
            fetched_at: row.get("fetched_at"),
            source_url: row.get("source_url"),
            payload: row.get("payload"),
        })
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn last_points(&self, limit: i64) -> sqlx::Result<Vec<IssEntry>> {
        sqlx::query(
            "SELECT id, fetched_at, source_url, payload
             FROM iss_fetch_log
             ORDER BY id DESC LIMIT $1",
        )
        .bind(limit)
        .map(|row: sqlx::postgres::PgRow| IssEntry {
            id: row.get("id"),
            fetched_at: row.get("fetched_at"),
            source_url: row.get("source_url"),
            payload: row.get("payload"),
        })
        .fetch_all(&self.pool)
        .await
    }
}

impl OsdrRepo {
    async fn init(&self) -> sqlx::Result<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS osdr_items(
                id BIGSERIAL PRIMARY KEY,
                dataset_id TEXT,
                title TEXT,
                status TEXT,
                updated_at TIMESTAMPTZ,
                inserted_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                raw JSONB NOT NULL
            )",
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "CREATE UNIQUE INDEX IF NOT EXISTS ux_osdr_dataset_id
             ON osdr_items(dataset_id) WHERE dataset_id IS NOT NULL",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn upsert_item(
        &self,
        dataset_id: Option<String>,
        title: Option<String>,
        status: Option<String>,
        updated_at: Option<DateTime<Utc>>,
        raw: Value,
    ) -> sqlx::Result<()> {
        if let Some(ds) = dataset_id {
            sqlx::query(
                "INSERT INTO osdr_items(dataset_id, title, status, updated_at, raw)
                 VALUES($1,$2,$3,$4,$5)
                 ON CONFLICT (dataset_id) DO UPDATE
                 SET title=EXCLUDED.title,
                     status=EXCLUDED.status,
                     updated_at=EXCLUDED.updated_at,
                     raw=EXCLUDED.raw",
            )
            .bind(ds)
            .bind(title)
            .bind(status)
            .bind(updated_at)
            .bind(raw)
            .execute(&self.pool)
            .await?;
        } else {
            sqlx::query(
                "INSERT INTO osdr_items(dataset_id, title, status, updated_at, raw)
                 VALUES($1,$2,$3,$4,$5)",
            )
            .bind::<Option<String>>(None)
            .bind(title)
            .bind(status)
            .bind(updated_at)
            .bind(raw)
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    pub async fn count(&self) -> sqlx::Result<i64> {
        sqlx::query("SELECT count(*) AS c FROM osdr_items")
            .map(|row: sqlx::postgres::PgRow| row.get::<i64, _>("c"))
            .fetch_one(&self.pool)
            .await
    }

    pub async fn list(&self, limit: i64) -> sqlx::Result<Vec<OsdrRecord>> {
        sqlx::query(
            "SELECT id, dataset_id, title, status, updated_at, inserted_at, raw
             FROM osdr_items
             ORDER BY inserted_at DESC
             LIMIT $1",
        )
        .bind(limit)
        .map(|row: sqlx::postgres::PgRow| OsdrRecord {
            id: row.get("id"),
            dataset_id: row.get("dataset_id"),
            title: row.get("title"),
            status: row.get("status"),
            updated_at: row.get("updated_at"),
            inserted_at: row.get("inserted_at"),
            raw: row.get("raw"),
        })
        .fetch_all(&self.pool)
        .await
    }
}

impl CacheRepo {
    async fn init(&self) -> sqlx::Result<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS space_cache(
                id BIGSERIAL PRIMARY KEY,
                source TEXT NOT NULL,
                fetched_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                payload JSONB NOT NULL
            )",
        )
        .execute(&self.pool)
        .await?;
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS ix_space_cache_source
             ON space_cache(source, fetched_at DESC)",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn write(&self, source: &str, payload: Value) -> sqlx::Result<()> {
        sqlx::query("INSERT INTO space_cache(source, payload) VALUES ($1,$2)")
            .bind(source)
            .bind(payload)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn latest(&self, source: &str) -> sqlx::Result<Option<SpaceCacheItem>> {
        sqlx::query(
            "SELECT source, fetched_at, payload
             FROM space_cache
             WHERE source = $1
             ORDER BY id DESC LIMIT 1",
        )
        .bind(source)
        .map(|row: sqlx::postgres::PgRow| SpaceCacheItem {
            source: row.get("source"),
            fetched_at: row.get("fetched_at"),
            payload: row.get("payload"),
        })
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn latest_with_metadata(&self, source: &str) -> sqlx::Result<Value> {
        Ok(self
            .latest(source)
            .await?
            .map(|item| {
                json!({
                    "source": item.source,
                    "at": item.fetched_at,
                    "payload": item.payload
                })
            })
            .unwrap_or_else(|| json!({})))
    }

    pub async fn latest_by_sources(&self, sources: &[String]) -> sqlx::Result<HashMap<String, Value>> {
        if sources.is_empty() {
            return Ok(HashMap::new());
        }
        let args: Vec<&str> = sources.iter().map(|s| s.as_str()).collect();
        let rows = sqlx::query(
            "SELECT DISTINCT ON (source) source, fetched_at, payload
             FROM space_cache
             WHERE source = ANY($1)
             ORDER BY source, fetched_at DESC",
        )
        .bind(&args)
        .fetch_all(&self.pool)
        .await?;

        let mut out = HashMap::new();
        for row in rows {
            let source: String = row.get("source");
            let value = json!({
                "source": source,
                "at": row.get::<DateTime<Utc>, _>("fetched_at"),
                "payload": row.get::<Value, _>("payload")
            });
            out.insert(source, value);
        }
        Ok(out)
    }
}

