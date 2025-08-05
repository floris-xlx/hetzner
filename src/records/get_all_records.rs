use crate::{HetznerClient, Record};
use anyhow::{Result, anyhow};
use reqwest::{Client, Response};
use serde_json::Value;

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
    /// use hetzner::{HetznerClient, Record};
    /// use std::env::var;
    /// use dotenv::dotenv;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// dotenv().ok();
    ///
    /// let api_token: &str =
    /// &var("HETZNER_API_ACCESS_TOKEN").expect("HETZNER_API_ACCESS_TOKEN must be set");
    /// let client: HetznerClient = HetznerClient::new(api_token.to_string());
    ///
    /// let zone_id: &String =
    /// &var("HETZNER_TESTS_ZONE_ID").expect("HETZNER_TESTS_ZONE_ID must be set");
    /// let records: Vec<Record> = client.get_all_records(zone_id.as_str()).await?;
    ///
    /// println!("Records: {:#?}", records);
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub async fn get_all_records(&self, zone_id: &str) -> Result<Vec<Record>> {
        let client: Client = Client::new();
        let url: String = format!("https://dns.hetzner.com/api/v1/records?zone_id={}", zone_id);
        let response: Response = client
            .get(&url)
            .header("Auth-API-Token", &self.auth_api_token)
            .send()
            .await?;

        let api_response: Value = response.json().await?;

        // Extract the "records" array from the API response
        let records_value: Value = api_response
            .get("records")
            .ok_or_else(|| anyhow!("Missing 'records' field in response"))?
            .clone();

        // The Hetzner API sometimes omits the "ttl" field for some record types.
        // We'll map each record to a Value, insert a default ttl if missing, then deserialize.
        let records_array: Vec<Value> = records_value.as_array().unwrap().to_vec();

        let records: Vec<Record> = records_array
            .iter()
            .map(|rec| {
                let mut rec_map = rec.as_object().cloned().unwrap_or_default();
                // If "ttl" is missing, insert a default value (e.g., 0)
                if !rec_map.contains_key("ttl") {
                    rec_map.insert("ttl".to_string(), Value::Number(0.into()));
                }
                serde_json::from_value(Value::Object(rec_map))
                    .map_err(|e| anyhow!("Failed to deserialize record: {}", e))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::var;
    use tokio;
    use tracing::info;

    #[tokio::test]
    async fn test_get_all_records() {
        dotenv::dotenv().ok();

        let api_token: &str =
            &var("HETZNER_API_ACCESS_TOKEN").expect("HETZNER_API_ACCESS_TOKEN must be set");
        let client: HetznerClient = HetznerClient::new(api_token.to_string());

        let zone_id: &String =
            &var("HETZNER_TESTS_ZONE_ID").expect("HETZNER_TESTS_ZONE_ID must be set");

        let records: Vec<Record> = client.get_all_records(zone_id).await.unwrap();

        info!("records: {:#?}", records);
    }
}
