use crate::api::cloud::{CloudApi, generated_ops::QueryPairs};
use crate::error::Result;
use serde_json::Value;

#[derive(Debug, Clone, Copy)]
pub struct PrivateNetworksApi<'a> {
    pub(crate) cloud: CloudApi<'a>,
}

impl<'a> PrivateNetworksApi<'a> {
    pub async fn list(self, query: Option<&QueryPairs>) -> Result<Value> {
        self.cloud.list_networks(query, None).await
    }

    pub async fn create(self, body: Value) -> Result<Value> {
        self.cloud.create_network(None, Some(body)).await
    }

    pub async fn get(self, id: impl ToString) -> Result<Value> {
        self.cloud.get_network(id, None, None).await
    }

    pub async fn update(self, id: impl ToString, body: Value) -> Result<Value> {
        self.cloud.update_network(id, None, Some(body)).await
    }

    pub async fn delete(self, id: impl ToString) -> Result<Value> {
        self.cloud.delete_network(id, None, None).await
    }

    pub async fn add_subnet(self, id: impl ToString, body: Value) -> Result<Value> {
        self.cloud.add_network_subnet(id, None, Some(body)).await
    }

    pub async fn add_route(self, id: impl ToString, body: Value) -> Result<Value> {
        self.cloud.add_network_route(id, None, Some(body)).await
    }
}
