use crate::HetznerClient;
use reqwest::{Client, Response};
use tracing::{error, info};

impl HetznerClient {
    /// Deletes a DNS record.
    ///
    /// # Arguments
    ///
    /// * `record_id` - A string slice that holds the ID of the record to be deleted.
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    /// * `Ok(())` if the record is deleted successfully.
    /// * `Err` containing an error message if the deletion fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use hetzner::HetznerClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = HetznerClient::new("your_api_token".to_string());
    ///
    /// match client.delete_record("record_id").await {
    ///     Ok(_) => println!("Record deleted successfully."),
    ///     Err(e) => eprintln!("Error deleting record: {}", e),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_record(&self, record_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client: Client = Client::new();
        let url: String = format!("https://dns.hetzner.com/api/v1/records/{}", record_id);

        let response: Response = client
            .delete(&url)
            .header("Auth-API-Token", &self.auth_api_token)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                info!("Record deleted successfully.");
                Ok(())
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                error!("Unauthorized: Invalid API token.");
                Err("Unauthorized: Invalid API token.".into())
            }
            reqwest::StatusCode::FORBIDDEN => {
                error!("Forbidden: You do not have permission to delete this record.");
                Err("Forbidden: You do not have permission to delete this record.".into())
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
                let error_message: String = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                error!("Error deleting record: {}", error_message);
                Err(format!("Error deleting record: {}", error_message).into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::var;
    use tokio;
    use tracing::info;

    #[tokio::test]
    async fn test_delete_hetzner_test_record() {
        dotenv::dotenv().ok();

        let api_token: String =
            var("HETZNER_API_ACCESS_TOKEN").expect("HETZNER_API_ACCESS_TOKEN must be set");
        let zone_id: String =
            var("HETZNER_TESTS_ZONE_ID").expect("HETZNER_TESTS_ZONE_ID must be set");

        let client: HetznerClient = crate::HetznerClient::new(api_token);

        // First, create a record to delete
        let value = "127.0.0.14";
        let ttl = 3600;
        let type_ = "A";
        let name = "hetzner_delete_test_record_delete";

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
            Err(e) => panic!("Failed to create record for deletion test: {:?}", e),
        };

        assert!(!record_id.is_empty(), "Record ID should not be empty");

        // Now, delete the record
        let delete_result = client.delete_record(&record_id).await;
        match delete_result {
            Ok(_) => info!("Record deleted successfully"),
            Err(e) => panic!("Failed to delete record: {:?}", e),
        }
    }
}
