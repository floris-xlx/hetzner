use crate::HetznerClient;

pub mod records;
pub mod zones;

#[derive(Debug, Clone, Copy)]
pub struct DnsApi<'a> {
    pub(crate) client: &'a HetznerClient,
}

impl<'a> DnsApi<'a> {
    pub fn records(self, zone_id: &'a str) -> records::RecordsApi<'a> {
        records::RecordsApi {
            client: self.client,
            zone_id,
        }
    }

    pub fn record(self, record_id: &'a str) -> records::RecordApi<'a> {
        records::RecordApi {
            client: self.client,
            record_id,
        }
    }

    pub async fn list_zones(self) -> crate::error::Result<Vec<crate::types::Zone>> {
        zones::list_zones(self.client).await
    }
}
