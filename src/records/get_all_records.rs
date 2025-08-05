use crate::HetznerClient;
use reqwest::Client;
use serde::Deserialize;
use tracing::info;

/// Represents a DNS record.
#[derive(Deserialize, Debug, Clone)]
pub struct Record {
    /// The unique identifier of the record.
    id: String,
    /// The name of the record.
    name: String,
    /// The time-to-live (TTL) value of the record.
    ttl: u64,
    /// The type of the record (e.g., A, AAAA, CNAME).
    type_: String,
    /// The value of the record.
    value: String,
    /// The zone ID associated with the record.
    zone_id: String,
}

/// Represents the API response containing a list of DNS records.
#[derive(Deserialize)]
struct ApiResponse {
    /// A list of DNS records.
    records: Vec<Record>,
}

impl HetznerClient {
    /// Fetches all DNS records for a given zone ID.
    ///
    /// # Arguments
    ///
    /// * `zone_id` - A string slice that holds the ID of the zone for which to fetch records.
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    /// * `Ok` containing the JSON response with the list of records if the request is successful.
    /// * `Err` containing a `reqwest::Error` if the request fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use hetzner::HetznerClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = HetznerClient::new("your_api_token".to_string());
    ///
    /// let zone_id = "your_zone_id";
    /// let records = client.get_all_records(zone_id).await?;
    ///
    /// if let Some(records_array) = records["records"].as_array() {
    ///     for record in records_array {
    ///         println!("Record: {:?}", record);
    ///     }
    /// } else {
    ///     println!("No records found or response format is incorrect.");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_all_records(
        &self,
        zone_id: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let client: Client = Client::new();
        let url: String = format!("https://dns.hetzner.com/api/v1/records?zone_id={}", zone_id);
        let response = client
            .get(&url)
            .header("Auth-API-Token", &self.auth_api_token)
            .send()
            .await?;

        info!("Response status: {:#?}", response);

        let api_response: serde_json::Value = response.json().await?;
        Ok(api_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_all_records() {
        dotenv::dotenv().ok();

        let api_token: &str = &std::env::var("HETZNER_API_ACCESS_TOKEN")
            .expect("HETZNER_API_ACCESS_TOKEN must be set");
        let client: HetznerClient = HetznerClient::new(api_token.to_string());

        let zone_id: &String =
            &std::env::var("HETZNER_TESTS_ZONE_ID").expect("HETZNER_TESTS_ZONE_ID must be set");

        let records = client.get_all_records(zone_id).await.unwrap();

        // Check if the response contains any records
        if let Some(records_array) = records["records"].as_array() {
            assert!(
                !records_array.is_empty(),
                "Records list should not be empty"
            );
        } else {
            panic!("Response format is incorrect: 'records' field is missing or not an array");
        }
    }
}
