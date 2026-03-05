use crate::HetznerClient;
use crate::error::Result;
use crate::types::{Zone, ZonesEnvelope};
use reqwest::Method;

pub async fn list_zones(client: &HetznerClient) -> Result<Vec<Zone>> {
    let response: ZonesEnvelope = client.request_dns(Method::GET, "zones", None).await?;
    Ok(response.zones)
}
