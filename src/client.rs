use crate::HetznerClient;

impl HetznerClient {
    /// Creates a new `HetznerClient` instance.
    ///
    /// # Arguments
    ///
    /// * `auth_api_token` - A string slice that holds the authentication API token for accessing the Hetzner DNS API.
    ///
    /// # Returns
    ///
    /// A new `HetznerClient` instance.
    ///
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
