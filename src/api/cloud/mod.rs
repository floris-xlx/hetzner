use crate::HetznerClient;

pub mod actions;
pub mod domains;
pub mod enums;
pub mod generated_ops;
pub mod load_balancers_api;
pub mod private_networks;
pub mod servers;
pub mod servers_api;
pub mod storage;

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

    pub fn servers_api(self) -> servers_api::ServersFullApi<'a> {
        servers_api::ServersFullApi { cloud: self }
    }

    pub fn actions(self) -> actions::ActionsApi<'a> {
        actions::ActionsApi {
            client: self.client,
        }
    }

    pub fn domains(self) -> domains::DomainsApi<'a> {
        domains::DomainsApi { cloud: self }
    }

    pub fn private_networks(self) -> private_networks::PrivateNetworksApi<'a> {
        private_networks::PrivateNetworksApi { cloud: self }
    }

    pub fn load_balancers(self) -> load_balancers_api::LoadBalancersApi<'a> {
        load_balancers_api::LoadBalancersApi { cloud: self }
    }

    pub fn storage(self) -> storage::StorageApi<'a> {
        storage::StorageApi { cloud: self }
    }
}
