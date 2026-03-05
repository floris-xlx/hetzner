use crate::api::cloud::{CloudApi, generated_ops::QueryPairs};
use crate::error::Result;
use serde_json::Value;

#[derive(Debug, Clone, Copy)]
pub struct DomainsApi<'a> {
    pub(crate) cloud: CloudApi<'a>,
}

impl<'a> DomainsApi<'a> {
    pub async fn list(self, query: Option<&QueryPairs>) -> Result<Value> {
        self.cloud.list_zones(query, None).await
    }

    pub async fn create(self, body: Value) -> Result<Value> {
        self.cloud.create_zone(None, Some(body)).await
    }

    pub async fn get(self, id_or_name: impl ToString, query: Option<&QueryPairs>) -> Result<Value> {
        self.cloud.get_zone(id_or_name, query, None).await
    }

    pub async fn update(self, id_or_name: impl ToString, body: Value) -> Result<Value> {
        self.cloud.update_zone(id_or_name, None, Some(body)).await
    }

    pub async fn delete(self, id_or_name: impl ToString) -> Result<Value> {
        self.cloud.delete_zone(id_or_name, None, None).await
    }

    pub async fn list_rrsets(
        self,
        id_or_name: impl ToString,
        query: Option<&QueryPairs>,
    ) -> Result<Value> {
        self.cloud.list_zone_rrsets(id_or_name, query, None).await
    }

    pub async fn get_rrset(
        self,
        id_or_name: impl ToString,
        rr_name: impl ToString,
        rr_type: impl ToString,
    ) -> Result<Value> {
        self.cloud
            .get_zone_rrset(id_or_name, rr_name, rr_type, None, None)
            .await
    }

    pub async fn upsert_rrset(
        self,
        id_or_name: impl ToString,
        rr_name: impl ToString,
        rr_type: impl ToString,
        body: Value,
    ) -> Result<Value> {
        self.cloud
            .update_zone_rrset(id_or_name, rr_name, rr_type, None, Some(body))
            .await
    }
}
