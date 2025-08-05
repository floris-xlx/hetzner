use crate::{TxtVerification, Zone, ZoneType};

pub mod get_all_zones;
pub mod zone_types;

impl Zone {
    /// `new` creates a new `Zone` instance.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the zone.
    /// * `id` - The ID of the zone.
    /// * `is_secondary_dns` - Indicates if the zone is a secondary DNS.
    /// * `legacy_dns_host` - The legacy DNS host associated with the zone.
    /// * `legacy_ns` - The legacy name servers associated with the zone.
    /// * `modified` - The last modified timestamp of the DNS zone.
    /// * `owner` - The owner of the DNS zone.
    /// * `paused` - Indicates if the zone is paused.
    /// * `permission` - The permission level of the DNS zone.
    /// * `project` - The project associated with the DNS zone.
    /// * `records_count` - The count of records in the DNS zone.
    /// * `registrar` - The registrar of the DNS zone.
    /// * `status` - The status of the DNS zone.
    /// * `ttl` - The TTL of the DNS zone.
    /// * `txt_verification` - The TXT verification details of the DNS zone.
    /// * `verified` - The verification status of the DNS zone.
    /// * `zone_type` - The type details of the DNS zone.
    ///
    /// # Returns
    ///
    /// A new `Zone` instance.
    pub fn new(
        created: String,
        ns: Vec<String>,
        name: String,
        id: String,
        is_secondary_dns: bool,
        legacy_dns_host: String,
        legacy_ns: Vec<String>,
        modified: String,
        owner: String,
        paused: bool,
        permission: String,
        project: String,
        records_count: i64,
        registrar: String,
        status: String,
        ttl: u32,
        txt_verification: TxtVerification,
        verified: String,
        zone_type: ZoneType,
    ) -> Self {
        Zone {
            created,
            ns,
            name,
            id,
            is_secondary_dns,
            legacy_dns_host,
            legacy_ns,
            modified,
            owner,
            paused,
            permission,
            project,
            records_count,
            registrar,
            status,
            ttl,
            txt_verification,
            verified,
            zone_type,
        }
    }
}
