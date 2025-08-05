use crate::{HetznerClient, Zone};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Deserialize)]
struct Meta {
    pagination: Pagination,
}

#[derive(Deserialize)]
struct Pagination {
    page: u32,
    per_page: u32,
    last_page: u32,
    total_entries: u32,
}

#[derive(Deserialize)]
pub struct ApiResponse {
    zones: Vec<Zone>,
}

impl HetznerClient {
    /// Fetches all DNS zones.
    ///
    /// # Returns
    ///
    /// A `Result` which is
    /// * `Ok` containing a vector of `Zone` if the zones are fetched successfully.
    /// * `Err` containing a `reqwest::Error` if the fetch fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::HetznerClient;
    /// # async fn example() -> Result<(), reqwest::Error> {
    /// let client = HetznerClient::new("your_api_token".to_string());
    ///
    /// match client.get_all_zones().await {
    ///     Ok(zones) => {
    ///         for zone in zones {
    ///             println!("{:?}", zone);
    ///         }
    ///     }
    ///     Err(e) => eprintln!("Error fetching zones: {}", e),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_all_zones(&self) -> Result<Vec<Zone>, reqwest::Error> {
        let client: Client = Client::new();
        let response: reqwest::Response = client
            .get("https://dns.hetzner.com/api/v1/zones")
            .header("Auth-API-Token", &self.auth_api_token)
            .send()
            .await?;

        info!("Response status: {:#?}", response);

        let api_response: ApiResponse = response.json().await?;
        Ok(api_response.zones)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_all_zones() {
        dotenv::dotenv().ok();

        let api_token: &str = &std::env::var("HETZNER_API_ACCESS_TOKEN")
            .expect("HETZNER_API_ACCESS_TOKEN must be set");
        let client: HetznerClient = HetznerClient::new(api_token.to_string());

        match client.get_all_zones().await {
            Ok(zones) => {
                // Check if the response contains zones
                assert!(!zones.is_empty(), "Zones list should not be empty");
                // Optionally, print the zones for manual inspection
                for zone in zones {
                    println!("{:?}", zone);
                }
            }
            Err(e) => {
                panic!("Failed to fetch zones: {:?}", e);
            }
        }
    }
}
