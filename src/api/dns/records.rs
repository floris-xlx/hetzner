use crate::HetznerClient;
use crate::error::Result;
use crate::types::{CreatedRecord, RecordEnvelope, RecordsEnvelope};
use reqwest::Method;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Clone, Serialize)]
pub struct CreateRecordInput {
    pub value: String,
    pub ttl: u64,
    #[serde(rename = "type")]
    pub record_type: String,
    pub name: String,
    pub zone_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateRecordInput {
    pub zone_id: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub name: String,
    pub value: String,
    pub ttl: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct RecordsApi<'a> {
    pub(crate) client: &'a HetznerClient,
    pub(crate) zone_id: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct RecordApi<'a> {
    pub(crate) client: &'a HetznerClient,
    pub(crate) record_id: &'a str,
}

impl<'a> RecordsApi<'a> {
    pub async fn list(self) -> Result<Vec<crate::types::Record>> {
        let path = format!("records?zone_id={}", self.zone_id);
        let response: RecordsEnvelope = self.client.request_dns(Method::GET, &path, None).await?;
        Ok(response.records)
    }

    pub async fn create(
        self,
        name: impl Into<String>,
        record_type: impl Into<String>,
        value: impl Into<String>,
        ttl: u64,
    ) -> Result<CreatedRecord> {
        let payload = CreateRecordInput {
            value: value.into(),
            ttl,
            record_type: record_type.into(),
            name: name.into(),
            zone_id: self.zone_id.to_string(),
        };

        self.client
            .request_dns(Method::POST, "records", Some(json!(payload)))
            .await
    }
}

impl<'a> RecordApi<'a> {
    pub async fn get(self) -> Result<RecordEnvelope> {
        let path = format!("records/{}", self.record_id);
        self.client.request_dns(Method::GET, &path, None).await
    }

    pub async fn update(self, input: UpdateRecordInput) -> Result<RecordEnvelope> {
        let path = format!("records/{}", self.record_id);
        self.client
            .request_dns(Method::PUT, &path, Some(json!(input)))
            .await
    }

    pub async fn delete(self) -> Result<()> {
        let path = format!("records/{}", self.record_id);
        self.client
            .request_dns_unit(Method::DELETE, &path, None)
            .await
    }
}
