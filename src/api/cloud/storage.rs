use crate::api::cloud::{CloudApi, generated_ops::QueryPairs};
use crate::error::Result;
use serde_json::Value;

#[derive(Debug, Clone, Copy)]
pub struct StorageApi<'a> {
    pub(crate) cloud: CloudApi<'a>,
}

impl<'a> StorageApi<'a> {
    pub async fn list_volumes(self, query: Option<&QueryPairs>) -> Result<Value> {
        self.cloud.list_volumes(query, None).await
    }

    pub async fn create_volume(self, body: Value) -> Result<Value> {
        self.cloud.create_volume(None, Some(body)).await
    }

    pub async fn get_volume(self, id: impl ToString) -> Result<Value> {
        self.cloud.get_volume(id, None, None).await
    }

    pub async fn update_volume(self, id: impl ToString, body: Value) -> Result<Value> {
        self.cloud.update_volume(id, None, Some(body)).await
    }

    pub async fn delete_volume(self, id: impl ToString) -> Result<Value> {
        self.cloud.delete_volume(id, None, None).await
    }

    pub async fn list_images(self, query: Option<&QueryPairs>) -> Result<Value> {
        self.cloud.list_images(query, None).await
    }

    pub async fn get_image(self, id: impl ToString) -> Result<Value> {
        self.cloud.get_image(id, None, None).await
    }

    pub async fn list_isos(self, query: Option<&QueryPairs>) -> Result<Value> {
        self.cloud.list_isos(query, None).await
    }

    pub async fn get_iso(self, id: impl ToString) -> Result<Value> {
        self.cloud.get_iso(id, None, None).await
    }
}
