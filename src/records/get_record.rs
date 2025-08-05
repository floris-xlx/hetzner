use crate::HetznerClient;
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde_json::Value;
use tracing::{error, info};

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
    pub async fn get_record(&self, record_id: &str) -> Result<Value> {
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
                Err(anyhow!("Unauthorized: Invalid API token."))
            }
            reqwest::StatusCode::FORBIDDEN => {
                error!("Forbidden: You do not have permission to access this record.");
                Err(anyhow!(
                    "Forbidden: You do not have permission to access this record."
                ))
            }
            reqwest::StatusCode::NOT_FOUND => {
                error!("Not found: Record does not exist.");
                Err(anyhow!("Not found: Record does not exist."))
            }
            reqwest::StatusCode::NOT_ACCEPTABLE => {
                error!("Not acceptable: The request was not acceptable.");
                Err(anyhow!("Not acceptable: The request was not acceptable."))
            }
            _ => {
                let error_message = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                error!("Error fetching record: {}", error_message);
                Err(anyhow!("Error fetching record: {}", error_message))
            }
        }
    }
}
