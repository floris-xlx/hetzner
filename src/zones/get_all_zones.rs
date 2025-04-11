use crate::{HetznerClient, PrimaryServer, Record, RecordType};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::info;

/// Represents a DNS zone with its details.
#[derive(Deserialize, Debug, Clone)]
pub struct Zone {
    /// The creation timestamp of the DNS zone.
    created: String,
    /// The last modified timestamp of the DNS zone.
    modified: String,
    /// The legacy DNS host associated with the zone.
    legacy_dns_host: String,
    /// The legacy name servers associated with the zone.
    legacy_ns: Vec<String>,
    /// The name of the DNS zone.
    name: String,
    /// The name servers associated with the zone.
    ns: Vec<String>,
    /// The owner of the DNS zone.
    owner: String,
    /// Indicates if the zone is paused.
    paused: bool,
    /// The permission level of the DNS zone.
    permission: String,
    /// The project associated with the DNS zone.
    project: String,
    /// The registrar of the DNS zone.
    registrar: String,
    /// The status of the DNS zone.
    status: String,
    /// The time-to-live (TTL) value of the DNS zone.
    ttl: u32,
    /// The verification status of the DNS zone.
    verified: String,
    /// The count of records in the DNS zone.
    records_count: u32,
    /// Indicates if the zone is a secondary DNS.
    is_secondary_dns: bool,
    /// The TXT verification details of the DNS zone.
    txt_verification: serde_json::Value,
}

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
    /// A `Result` which is:
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
