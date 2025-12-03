use anyhow::Result;
use reqwest::Client;
use serde_json::Value;

use crate::config::AppConfig;

#[derive(Clone)]
pub struct ExternalClients {
    pub iss: IssClient,
    pub osdr: OsdrClient,
    pub nasa: NasaClient,
    pub spacex: SpacexClient,
}

impl ExternalClients {
    pub fn new(config: &AppConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(config.http_timeout)
            .user_agent(&config.user_agent)
            .build()?;

        Ok(Self {
            iss: IssClient::new(client.clone(), &config.iss_url),
            osdr: OsdrClient::new(client.clone(), &config.osdr_url),
            nasa: NasaClient::new(client.clone(), config.nasa_api_key.clone()),
            spacex: SpacexClient::new(client.clone()),
        })
    }
}

#[derive(Clone)]
pub struct IssClient {
    client: Client,
    url: String,
}

impl IssClient {
    fn new(client: Client, url: &str) -> Self {
        Self {
            client,
            url: url.to_string(),
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub async fn fetch(&self) -> Result<Value> {
        Ok(self
            .client
            .get(&self.url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}

#[derive(Clone)]
pub struct OsdrClient {
    client: Client,
    url: String,
}

impl OsdrClient {
    fn new(client: Client, url: &str) -> Self {
        Self {
            client,
            url: url.to_string(),
        }
    }

    pub async fn fetch(&self) -> Result<Vec<Value>> {
        let json: Value = self
            .client
            .get(&self.url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        if let Some(items) = json.as_array() {
            return Ok(items.clone());
        }
        if let Some(items) = json.get("items").and_then(|v| v.as_array()) {
            return Ok(items.clone());
        }
        if let Some(items) = json.get("results").and_then(|v| v.as_array()) {
            return Ok(items.clone());
        }
        Ok(vec![json])
    }
}

#[derive(Clone)]
pub struct NasaClient {
    client: Client,
    api_key: Option<String>,
}

impl NasaClient {
    fn new(client: Client, api_key: Option<String>) -> Self {
        Self { client, api_key }
    }

    pub async fn apod(&self) -> Result<Value> {
        self.get_json(
            "https://api.nasa.gov/planetary/apod",
            &[("thumbs", "true".to_string())],
        )
        .await
    }

    pub async fn neo_feed(&self, start: &str, end: &str) -> Result<Value> {
        self.get_json(
            "https://api.nasa.gov/neo/rest/v1/feed",
            &[("start_date", start.to_string()), ("end_date", end.to_string())],
        )
        .await
    }

    pub async fn donki_flr(&self, start: &str, end: &str) -> Result<Value> {
        self.get_json(
            "https://api.nasa.gov/DONKI/FLR",
            &[("startDate", start.to_string()), ("endDate", end.to_string())],
        )
        .await
    }

    pub async fn donki_cme(&self, start: &str, end: &str) -> Result<Value> {
        self.get_json(
            "https://api.nasa.gov/DONKI/CME",
            &[("startDate", start.to_string()), ("endDate", end.to_string())],
        )
        .await
    }

    async fn get_json(&self, url: &str, params: &[(&str, String)]) -> Result<Value> {
        let mut query: Vec<(&str, String)> = params.iter().map(|(k, v)| (*k, v.clone())).collect();
        if let Some(key) = &self.api_key {
            query.push(("api_key", key.clone()));
        }
        Ok(self
            .client
            .get(url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}

#[derive(Clone)]
pub struct SpacexClient {
    client: Client,
}

impl SpacexClient {
    fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn next_launch(&self) -> Result<Value> {
        Ok(self
            .client
            .get("https://api.spacexdata.com/v4/launches/next")
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}

