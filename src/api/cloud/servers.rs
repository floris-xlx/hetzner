use super::enums::{ServerSort, ServerStatus};
use crate::HetznerClient;
use crate::error::Result;
use crate::types::{CloudServer, CloudServerEnvelope, CloudServersEnvelope, CreateServerResponse};
use reqwest::Method;
use serde::Serialize;

#[derive(Debug, Clone, Default)]
pub struct ListServersParams {
    pub name: Option<String>,
    pub label_selector: Option<String>,
    pub sort: Vec<ServerSort>,
    pub status: Vec<ServerStatus>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

impl ListServersParams {
    fn to_query_pairs(&self) -> Vec<(String, String)> {
        let mut query = Vec::new();

        if let Some(name) = &self.name {
            query.push(("name".to_string(), name.clone()));
        }

        if let Some(label_selector) = &self.label_selector {
            query.push(("label_selector".to_string(), label_selector.clone()));
        }

        for sort in &self.sort {
            query.push(("sort".to_string(), sort.as_str().to_string()));
        }

        for status in &self.status {
            query.push(("status".to_string(), status.as_str().to_string()));
        }

        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }

        if let Some(per_page) = self.per_page {
            query.push(("per_page".to_string(), per_page.to_string()));
        }

        query
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateServerInput {
    pub name: String,
    pub server_type: String,
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_after_create: Option<bool>,
}

#[derive(Debug, Clone, Copy)]
pub struct ServersApi<'a> {
    pub(crate) client: &'a HetznerClient,
}

impl<'a> ServersApi<'a> {
    pub async fn list(self, params: Option<&ListServersParams>) -> Result<Vec<CloudServer>> {
        let query = params.map(ListServersParams::to_query_pairs);

        let response: CloudServersEnvelope = self
            .client
            .request_cloud(Method::GET, "servers", query.as_ref(), None)
            .await?;

        Ok(response.servers)
    }

    pub async fn get(self, server_id: u64) -> Result<CloudServerEnvelope> {
        let path = format!("servers/{server_id}");
        self.client
            .request_cloud(Method::GET, &path, None::<&()>, None)
            .await
    }

    pub async fn create(self, input: &CreateServerInput) -> Result<CreateServerResponse> {
        self.client
            .request_cloud(
                Method::POST,
                "servers",
                None::<&()>,
                Some(serde_json::to_value(input)?),
            )
            .await
    }
}
