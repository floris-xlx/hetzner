use crate::HetznerClient;

impl HetznerClient {
    pub fn new(auth_api_token: String) -> Self {
        HetznerClient {
            auth_api_token,
            base_url: String::from("https://dns.hetzner.com/api/v1"),
            name: None,
            page: None,
            per_page: None,
            search_name: None,
            zone_id: None,
            ttl: None,
            value: None,
            type_: None,
            record_id: None,
        }
    }
}
