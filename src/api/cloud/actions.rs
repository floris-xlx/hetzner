use crate::HetznerClient;
use crate::error::Result;
use crate::types::{Action, ActionEnvelope, ActionsEnvelope};
use reqwest::Method;

#[derive(Debug, Clone, Default)]
pub struct ListActionsParams {
    pub ids: Vec<u64>,
}

impl ListActionsParams {
    fn to_query_pairs(&self) -> Vec<(String, String)> {
        self.ids
            .iter()
            .map(|id| ("id".to_string(), id.to_string()))
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ActionsApi<'a> {
    pub(crate) client: &'a HetznerClient,
}

impl<'a> ActionsApi<'a> {
    pub async fn list(self, params: &ListActionsParams) -> Result<Vec<Action>> {
        let query = params.to_query_pairs();
        let response: ActionsEnvelope = self
            .client
            .request_cloud(Method::GET, "actions", Some(&query), None)
            .await?;

        Ok(response.actions)
    }

    pub async fn get(self, action_id: u64) -> Result<ActionEnvelope> {
        let path = format!("actions/{action_id}");
        self.client
            .request_cloud(Method::GET, &path, None::<&()>, None)
            .await
    }
}
