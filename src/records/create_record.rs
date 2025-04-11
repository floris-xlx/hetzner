use crate::HetznerClient;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Serialize, Debug, Clone)]
struct CreateRecordRequest {
    value: String,
    ttl: u64,
    r#type: String,
    name: String,
    zone_id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreatedRecord {
    pub record: RecordDetails,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RecordDetails {
    pub type_: String,
    pub id: String,
    pub created: String,
    pub modified: String,
    pub zone_id: String,
    pub name: String,
    pub value: String,
    pub ttl: u64,
}

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    error: ErrorDetails,
}

#[derive(Deserialize, Debug)]
struct ErrorDetails {
    code: u16,
    message: String,
}

impl HetznerClient {
    pub async fn create_record(
        &self,
        value: &str,
        ttl: u64,
        type_: &str,
        name: &str,
        zone_id: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let client: Client = Client::new();
        let request_body: CreateRecordRequest = CreateRecordRequest {
            value: value.to_string(),
            ttl,
            r#type: type_.to_string(),
            name: name.to_string(),
            zone_id: zone_id.to_string(),
        };

        info!("Creating record with request body: {:#?}", request_body);

        let response = client
            .post("https://dns.hetzner.com/api/v1/records")
            .header("Content-Type", "application/json")
            .header("Auth-API-Token", &self.auth_api_token)
            .json(&request_body)
            .send()
            .await?;

        let status = response.status();
        let response_json: serde_json::Value = response.json().await?;
        if status.is_success() {
            Ok(response_json)
        } else {
            let error_message = response_json["error"]["message"].as_str().unwrap_or("Unknown error");
            let error_code = response_json["error"]["code"].as_u64().unwrap_or(0);
            Err(format!("Error {}: {}", error_code, error_message).into())
        }
    }
}
