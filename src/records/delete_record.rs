use crate::HetznerClient;
use reqwest::Client;
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
    /// # use crate::HetznerClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = HetznerClient::new("your_api_token", "https://dns.hetzner.com/api/v1");
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
        let url = format!("https://dns.hetzner.com/api/v1/records/{}", record_id);

        info!("Deleting record with ID: {}", record_id);

        let response = client
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
                let error_message = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                error!("Error deleting record: {}", error_message);
                Err(format!("Error deleting record: {}", error_message).into())
            }
        }
    }
}
