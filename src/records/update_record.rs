use crate::HetznerClient;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Serialize)]
struct UpdateRecordRequest {
    zone_id: String,
    r#type: String,
    name: String,
    value: String,
    ttl: u64,
}

impl HetznerClient {
    pub async fn update_record(
        &self,
        record_id: &str,
        zone_id: &str,
        type_: &str,
        name: &str,
        value: &str,
        ttl: u64,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let client = Client::new();
        let request_body = UpdateRecordRequest {
            zone_id: zone_id.to_string(),
            r#type: type_.to_string(),
            name: name.to_string(),
            value: value.to_string(),
            ttl,
        };

        info!("Updating record with ID: {}", record_id);

        let url = format!("https://dns.hetzner.com/api/v1/records/{}", record_id);
        let response = client
            .put(&url)
            .header("Content-Type", "application/json")
            .header("Auth-API-Token", &self.auth_api_token)
            .json(&request_body)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(response.json().await?),
            status => {
                let error_message = match status {
                    reqwest::StatusCode::UNAUTHORIZED => "Unauthorized: Invalid API token.",
                    reqwest::StatusCode::FORBIDDEN => {
                        "Forbidden: You do not have permission to update this record."
                    }
                    reqwest::StatusCode::NOT_FOUND => "Not found: Record does not exist.",
                    reqwest::StatusCode::NOT_ACCEPTABLE => {
                        "Not acceptable: The request was not acceptable."
                    }
                    reqwest::StatusCode::CONFLICT => {
                        "Conflict: The request could not be completed due to a conflict."
                    }
                    reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                        "Unprocessable entity: The request was well-formed but was unable to be followed due to semantic errors."
                    }
                    _ => "Unknown error",
                };
                error!("{}", error_message);
                Err(error_message.into())
            }
        }
    }
}
