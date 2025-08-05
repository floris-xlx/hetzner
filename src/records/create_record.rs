use crate::HetznerClient;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::info;

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

/// Represents an error response from the API.
#[derive(Deserialize, Debug)]
struct ErrorResponse {
    /// Details of the error.
    error: ErrorDetails,
}

/// Contains detailed information about an error.
#[derive(Deserialize, Debug)]
struct ErrorDetails {
    /// The error code.
    code: u16,
    /// The error message.
    message: String,
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

        info!("Creating record with request body: {:#?}", request_body);

        let response = client
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
            let error_message: &str = response_json["error"]["message"]
                .as_str()
                .unwrap_or("Unknown error");
            let error_code: u64 = response_json["error"]["code"].as_u64().unwrap_or(0);
            Err(format!("Error {}: {}", error_code, error_message).into())
        }
    }
}
