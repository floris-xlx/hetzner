use crate::HetznerClient;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Record {
    id: String,
    name: String,
    ttl: u64,
    type_: String,
    value: String,
    zone_id: String,
}

#[derive(Deserialize)]
struct ApiResponse {
    records: Vec<Record>,
}

impl HetznerClient {
    pub async fn get_all_records(
        &self,
        zone_id: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let client: Client = Client::new();
        let url: String = format!("https://dns.hetzner.com/api/v1/records?zone_id={}", zone_id);
        let response = client
            .get(&url)
            .header("Auth-API-Token", &self.auth_api_token)
            .send()
            .await?;

        let api_response: serde_json::Value = response.json().await?;
        Ok(api_response)
    }
}
