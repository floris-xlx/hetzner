use crate::api::cloud::{CloudApi, generated_ops::QueryPairs};
use crate::error::Result;
use serde_json::Value;

#[derive(Debug, Clone, Copy)]
pub struct ServersFullApi<'a> {
    pub(crate) cloud: CloudApi<'a>,
}

impl<'a> ServersFullApi<'a> {
    pub async fn list(self, query: Option<&QueryPairs>) -> Result<Value> {
        self.cloud.list_servers(query, None).await
    }

    pub async fn create(self, body: Value) -> Result<Value> {
        self.cloud.create_server(None, Some(body)).await
    }

    pub async fn get(self, id: impl ToString) -> Result<Value> {
        self.cloud.get_server(id, None, None).await
    }

    pub async fn update(self, id: impl ToString, body: Value) -> Result<Value> {
        self.cloud.update_server(id, None, Some(body)).await
    }

    pub async fn delete(self, id: impl ToString) -> Result<Value> {
        self.cloud.delete_server(id, None, None).await
    }

    pub async fn metrics(self, id: impl ToString, query: Option<&QueryPairs>) -> Result<Value> {
        self.cloud.get_server_metrics(id, query, None).await
    }

    pub async fn list_actions(
        self,
        id: impl ToString,
        query: Option<&QueryPairs>,
    ) -> Result<Value> {
        self.cloud.list_server_actions(id, query, None).await
    }

    pub async fn power_on(self, id: impl ToString) -> Result<Value> {
        self.cloud.poweron_server(id, None, None).await
    }

    pub async fn power_off(self, id: impl ToString) -> Result<Value> {
        self.cloud.poweroff_server(id, None, None).await
    }

    pub async fn reboot(self, id: impl ToString) -> Result<Value> {
        self.cloud.reboot_server(id, None, None).await
    }
}
