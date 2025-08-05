use crate::HetznerClient;
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde_json::Value;
use tracing::error;

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
        let url: String = format!("https://dns.hetzner.com/api/v1/records/{}", record_id);

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

#[cfg(test)]
mod tests {
    use std::env::var;
    use tokio;
    use tracing::info;

    #[tokio::test]
    async fn test_create_get_delete_record() {
        dotenv::dotenv().ok();

        let api_token: String =
            var("HETZNER_API_ACCESS_TOKEN").expect("HETZNER_API_ACCESS_TOKEN must be set");
        let zone_id: String =
            var("HETZNER_TESTS_ZONE_ID").expect("HETZNER_TESTS_ZONE_ID must be set");

        let client = crate::HetznerClient::new(api_token);

        // Create a record
        let value = "127.0.0.6";
        let ttl = 3600;
        let type_ = "A";
        let name = "hetzner_get_test_record_get";

        let create_result = client
            .create_record(value, ttl, type_, name, &zone_id)
            .await;

        let record_id = match create_result {
            Ok(response) => {
                info!("Create record response: {:#?}", response);
                response
                    .get("record")
                    .and_then(|rec| rec.get("id"))
                    .and_then(|id| id.as_str())
                    .unwrap_or("")
                    .to_string()
            }
            Err(e) => panic!("Failed to create record for get test: {:?}", e),
        };

        assert!(!record_id.is_empty(), "Record ID should not be empty");

        // Get the record
        let get_result = client.get_record(&record_id).await;
        match get_result {
            Ok(record) => {
                info!("Fetched record: {:#?}", record);
                let fetched_name = record
                    .get("record")
                    .and_then(|rec| rec.get("name"))
                    .and_then(|n| n.as_str())
                    .unwrap_or("");
                assert_eq!(fetched_name, name, "Fetched record name should match");
            }
            Err(e) => panic!("Failed to fetch record: {:?}", e),
        }

        // Delete the record
        let delete_result = client.delete_record(&record_id).await;
        match delete_result {
            Ok(_) => info!("Record deleted successfully"),
            Err(e) => panic!("Failed to delete record: {:?}", e),
        }
    }
}
