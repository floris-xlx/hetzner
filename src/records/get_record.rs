use crate::HetznerClient;
use reqwest::Client;
use serde::Deserialize;
use tracing::{error, info};
use serde_json::Value;
#[derive(Deserialize, Debug, Clone)]
pub struct Record {
    r#type_: String,
    id: String,
    created: String,
    modified: String,
    zone_id: String,
    name: String,
    value: String,
    ttl: u64,
}



impl HetznerClient {
    pub async fn get_record(&self, record_id: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let client: Client = Client::new();
        let url = format!("https://dns.hetzner.com/api/v1/records/{}", record_id);

        info!("Fetching record with ID: {}", record_id);

        let response = client
            .get(&url)
            .header("Auth-API-Token", &self.auth_api_token)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let value = response.json::<serde_json::Value>().await?;
                Ok(value)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                error!("Unauthorized: Invalid API token.");
                Err("Unauthorized: Invalid API token.".into())
            }
            reqwest::StatusCode::FORBIDDEN => {
                error!("Forbidden: You do not have permission to access this record.");
                Err("Forbidden: You do not have permission to access this record.".into())
            }
            reqwest::StatusCode::NOT_FOUND => {
                error!("Not found: Record does not exist.");
                Err("Not found: Record does not exist.".into())
            }
            reqwest::StatusCode::NOT_ACCEPTABLE => {
                error!("Not acceptable: The request was not acceptable.");
                Err("Not acceptable: The request was not acceptable.".into())
            }
            _ => {
                let error_message = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                error!("Error fetching record: {}", error_message);
                Err(format!("Error fetching record: {}", error_message).into())
            }
        }
    }
}
