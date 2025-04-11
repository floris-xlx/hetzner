use crate::{HetznerClient, PrimaryServer, Record, RecordType};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct Zone {
    id: String,
    created: String,
    modified: String,
    legacy_dns_host: String,
    legacy_ns: Vec<String>,
    name: String,
    ns: Vec<String>,
    owner: String,
    paused: bool,
    permission: String,
    project: String,
    registrar: String,
    status: String,
    ttl: u32,
    verified: String,
    records_count: u32,
    is_secondary_dns: bool,
    txt_verification: serde_json::Value,
}

#[derive(Deserialize)]
struct Meta {
    pagination: Pagination,
}

#[derive(Deserialize)]
struct Pagination {
    page: u32,
    per_page: u32,
    last_page: u32,
    total_entries: u32,
}

#[derive(Deserialize)]
pub struct ApiResponse {
    zones: Vec<Zone>,
    meta: Meta,
}

impl HetznerClient {
    pub async fn get_all_zones(&self) -> Result<Vec<Zone>, reqwest::Error> {
        let client = Client::new();
        let response = client
            .get("https://dns.hetzner.com/api/v1/zones")
            .header("Auth-API-Token", &self.auth_api_token)
            .send()
            .await?;

        let api_response: ApiResponse = response.json().await?;
        Ok(api_response.zones)
    }
}
