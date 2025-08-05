use crate::HetznerClient;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use tracing::{error, info};

/// Represents a DNS record with its details.
#[derive(Deserialize, Debug, Clone)]
pub struct Record {
    /// The type of the DNS record (e.g., A, AAAA, CNAME).
    r#type_: String,
    /// The unique identifier of the DNS record.
    id: String,
    /// The creation timestamp of the DNS record.
    created: String,
    /// The last modified timestamp of the DNS record.
    modified: String,
    /// The zone ID associated with the DNS record.
    zone_id: String,
    /// The name of the DNS record.
    name: String,
    /// The value of the DNS record.
    value: String,
    /// The time-to-live (TTL) value of the DNS record.
    ttl: u64,
}

impl HetznerClient {
    /// Fetches a DNS record by its ID.
    ///
    /// # Arguments
    ///
    /// * `record_id` - A string slice that holds the ID of the record to be fetched.
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    /// * `Ok` containing the JSON response if the record is fetched successfully.
    /// * `Err` containing an error message if the fetch fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use hetzner::HetznerClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = HetznerClient::new("your_api_token".to_string());
    ///
    /// match client.get_record("record_id").await {
    ///     Ok(record) => println!("Record fetched: {:?}", record),
    ///     Err(e) => eprintln!("Error fetching record: {}", e),
    /// }
    /// # Ok(())
    /// # }
    /// ```
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
