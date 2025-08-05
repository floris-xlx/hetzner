use crate::HetznerClient;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Represents a request to create a DNS record.
#[derive(Serialize, Debug, Clone)]
struct CreateRecordRequest {
    /// The value of the DNS record.
    value: String,
    /// The time-to-live (TTL) value of the DNS record.
    ttl: u64,
    /// The type of the DNS record (e.g., A, AAAA, CNAME).
    r#type: String,
    /// The name of the DNS record.
    name: String,
    /// The zone ID associated with the DNS record.
    zone_id: String,
}

/// Represents the response received after successfully creating a DNS record.
#[derive(Deserialize, Debug, Clone)]
pub struct CreatedRecord {
    /// Details of the created DNS record.
    pub record: RecordDetails,
}

/// Contains detailed information about a DNS record.
#[derive(Deserialize, Debug, Clone)]
pub struct RecordDetails {
    /// The type of the DNS record.
    pub type_: String,
    /// The unique identifier of the DNS record.
    pub id: String,
    /// The creation timestamp of the DNS record.
    pub created: String,
    /// The last modified timestamp of the DNS record.
    pub modified: String,
    /// The zone ID associated with the DNS record.
    pub zone_id: String,
    /// The name of the DNS record.
    pub name: String,
    /// The value of the DNS record.
    pub value: String,
    /// The time-to-live (TTL) value of the DNS record.
    pub ttl: u64,
}

impl HetznerClient {
    /// Creates a new DNS record.
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the DNS record.
    /// * `ttl` - The time-to-live (TTL) value of the DNS record.
    /// * `type_` - The type of the DNS record (e.g., A, AAAA, CNAME).
    /// * `name` - The name of the DNS record.
    /// * `zone_id` - The zone ID associated with the DNS record.
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    /// * `Ok` containing the JSON response if the record is created successfully.
    /// * `Err` containing an error message if the creation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use hetzner::HetznerClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = HetznerClient::new("your_api_token".to_string());
    ///
    /// let result = client.create_record("127.0.0.1", 3600, "A", "example.com", "zone_id").await;
    /// match result {
    ///     Ok(response) => println!("Record created: {:?}", response),
    ///     Err(e) => eprintln!("Error creating record: {}", e),
    /// }
    /// # Ok(())
    /// # }
    /// ```
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

        let response: reqwest::Response = client
            .post("https://dns.hetzner.com/api/v1/records")
            .header("Content-Type", "application/json")
            .header("Auth-API-Token", &self.auth_api_token)
            .json(&request_body)
            .send()
            .await?;

        let status: reqwest::StatusCode = response.status();
        let response_json: serde_json::Value = response.json().await?;
        if status.is_success() {
            Ok(response_json)
        } else {
            // Try to extract a more detailed error message, including the code and any details
            let error_message = response_json["error"]["message"]
                .as_str()
                .unwrap_or("Unknown error");
            let error_code = response_json["error"]["code"]
                .as_u64()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "Unknown".to_string());

            // If the error message contains "Unprocessable Content" and a "taken" field, include it
            let mut detailed_message = error_message.to_string();
            if let Some(details) = response_json["error"]["details"].as_object() {
                if let Some(taken) = details.get("taken").and_then(|v| v.as_str()) {
                    detailed_message = format!("{}: taken: {}", error_message, taken);
                }
            }

            match status {
                reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                    Err(format!("Error 422: {}", detailed_message).into())
                }
                reqwest::StatusCode::CONFLICT => {
                    Err(format!("Error 409 Conflict: {}", detailed_message).into())
                }
                _ => Err(format!("Error {}: {}", error_code, detailed_message).into()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env::var;
    use tokio;
    use tracing::info;

    #[tokio::test]
    async fn test_create_hetzner_test_record() {
        dotenv::dotenv().ok();

        let api_token: String =
            var("HETZNER_API_ACCESS_TOKEN").expect("HETZNER_API_ACCESS_TOKEN must be set");
        let zone_id: String =
            var("HETZNER_TESTS_ZONE_ID").expect("HETZNER_TESTS_ZONE_ID must be set");

        let client = crate::HetznerClient::new(api_token);

        let value = "127.0.0.2";
        let ttl = 3600;
        let type_ = "A";
        let name = "hetzner_test_record_create";

        let result = client
            .create_record(value, ttl, type_, name, &zone_id)
            .await;

        match result {
            Ok(response) => {
                info!("Create record response: {:#?}", response);
                // Optionally, assert that the response contains the expected record name
                let record_name = response
                    .get("record")
                    .and_then(|rec| rec.get("name"))
                    .and_then(|n| n.as_str())
                    .unwrap_or("");
                assert_eq!(record_name, name);

                // delete the record
                let record_id = response
                    .get("record")
                    .and_then(|rec| rec.get("id"))
                    .and_then(|id| id.as_str())
                    .unwrap_or("");
                let delete_result = client.delete_record(record_id).await;
                match delete_result {
                    Ok(_) => info!("Record deleted successfully"),
                    Err(e) => panic!("Failed to delete record: {:?}", e),
                }
            }
            Err(e) => panic!("Failed to create record: {:?}", e),
        }
    }
}
