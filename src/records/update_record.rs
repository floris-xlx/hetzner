use crate::HetznerClient;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

/// Represents a request to update a DNS record.
#[derive(Serialize)]
struct UpdateRecordRequest {
    /// The zone ID associated with the DNS record.
    zone_id: String,
    /// The type of the DNS record (e.g., A, AAAA, CNAME).
    r#type: String,
    /// The name of the DNS record.
    name: String,
    /// The value of the DNS record.
    value: String,
    /// The time-to-live (TTL) value of the DNS record.
    ttl: u64,
}

impl HetznerClient {
    /// Updates an existing DNS record.
    ///
    /// # Arguments
    ///
    /// * `record_id` - A string slice that holds the ID of the record to be updated.
    /// * `zone_id` - The zone ID associated with the DNS record.
    /// * `type_` - The type of the DNS record (e.g., A, AAAA, CNAME).
    /// * `name` - The name of the DNS record.
    /// * `value` - The value of the DNS record.
    /// * `ttl` - The time-to-live (TTL) value of the DNS record.
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    /// * `Ok` containing the JSON response if the record is updated successfully.
    /// * `Err` containing an error message if the update fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use hetzner::HetznerClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = HetznerClient::new("your_api_token".to_string());
    ///
    /// let result = client.update_record("record_id", "zone_id", "A", "example.com", "127.0.0.1", 3600).await;
    /// match result {
    ///     Ok(response) => println!("Record updated: {:?}", response),
    ///     Err(e) => eprintln!("Error updating record: {}", e),
    /// }
    /// # Ok(())
    /// # }
    /// ```
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
        let response: reqwest::Response = client
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
