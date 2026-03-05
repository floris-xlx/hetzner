use crate::HetznerClient;

pub mod actions;
pub mod enums;
pub mod generated_ops;
pub mod servers;

#[derive(Debug, Clone, Copy)]
pub struct CloudApi<'a> {
    pub(crate) client: &'a HetznerClient,
}

impl<'a> CloudApi<'a> {
    pub fn servers(self) -> servers::ServersApi<'a> {
        servers::ServersApi {
            client: self.client,
        }
    }

    pub fn actions(self) -> actions::ActionsApi<'a> {
        actions::ActionsApi {
            client: self.client,
        }
    }
}
