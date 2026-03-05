use crate::api::cloud::{CloudApi, generated_ops::QueryPairs};
use crate::error::Result;
use serde_json::Value;

#[derive(Debug, Clone, Copy)]
pub struct LoadBalancersApi<'a> {
    pub(crate) cloud: CloudApi<'a>,
}

impl<'a> LoadBalancersApi<'a> {
    pub async fn list(self, query: Option<&QueryPairs>) -> Result<Value> {
        self.cloud.list_load_balancers(query, None).await
    }

    pub async fn create(self, body: Value) -> Result<Value> {
        self.cloud.create_load_balancer(None, Some(body)).await
    }

    pub async fn get(self, id: impl ToString) -> Result<Value> {
        self.cloud.get_load_balancer(id, None, None).await
    }

    pub async fn update(self, id: impl ToString, body: Value) -> Result<Value> {
        self.cloud.update_load_balancer(id, None, Some(body)).await
    }

    pub async fn delete(self, id: impl ToString) -> Result<Value> {
        self.cloud.delete_load_balancer(id, None, None).await
    }

    pub async fn metrics(self, id: impl ToString, query: Option<&QueryPairs>) -> Result<Value> {
        self.cloud.get_load_balancer_metrics(id, query, None).await
    }
}
