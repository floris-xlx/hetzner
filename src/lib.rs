//! # Hetzner DNS API SDK
//!
//! This SDK provides a convenient way to interact with the Hetzner DNS API. It allows you to manage DNS records and zones programmatically.
//!
//! ## Overview
//!
//! The `HetznerClient` struct is the main entry point for interacting with the Hetzner DNS API. It provides methods to create, update, delete, and fetch DNS records and zones.
//!
//! ## Usage
//!
//! To use this SDK, you need to create an instance of `HetznerClient` with your API token and base URL. Then, you can call the available methods to perform various operations on DNS records and zones.

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod client;
pub mod records;
pub mod zones;

/// Represents a client for interacting with the Hetzner DNS API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HetznerClient {
    /// The authentication API token for accessing the Hetzner DNS API.
    pub auth_api_token: String,
    /// The base URL for the Hetzner DNS API.
    pub base_url: String,

    // Optional query parameters for various API requests.
    pub name: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub search_name: Option<String>,
    pub zone_id: Option<String>,
    pub ttl: Option<u32>,
    pub value: Option<String>,
    pub type_: Option<String>,
    pub record_id: Option<String>,
}

/// Represents a DNS record.
#[derive(Debug, Clone)]
pub struct Record {
    /// The unique identifier of the record.
    pub id: String,
    /// The name of the record.
    pub name: String,
    /// The time-to-live (TTL) value of the record.
    pub ttl: u64,
    /// The type of the record (e.g., A, AAAA, CNAME).
    pub type_: String,
    /// The value of the record.
    pub value: String,
    /// The zone ID associated with the record.
    pub zone_id: String,
}

/// Enum representing the different types of DNS records.
#[derive(Debug, Clone)]
pub enum RecordType {
    A,
    AAAA,
    NS,
    MX,
    CNAME,
    RP,
    TXT,
    SOA,
    HINFO,
    SRV,
    DANE,
    TLSA,
    DS,
    CAA,
}

/// Represents a primary server for a DNS zone.
pub struct PrimaryServer {
    /// The address of the primary server.
    pub address: String,
    /// The port number of the primary server.
    pub port: u16,
    /// The zone ID associated with the primary server.
    pub zone_id: String,
}

/// Represents a DNS zone.
///
/// # Examples
///
/// ```rust,no_run
/// use hetzner::Zone;
///
/// let zone = Zone::new(
///     "2023-01-01T00:00:00Z".to_string(),
///     vec!["ns1.example.com".to_string(), "ns2.example.com".to_string()],
///     "example.com".to_string(),
///     "zone_id_123".to_string(),
///     false,
///     "legacy_dns_host".to_string(),
///     vec!["legacy_ns1".to_string(), "legacy_ns2".to_string()],
///     "2023-01-02T00:00:00Z".to_string(),
///     "owner_id".to_string(),
///     false,
///     "read".to_string(),
///     "project_id".to_string(),
///     10,
///     "registrar_name".to_string(),
///     "active".to_string(),
///     3600,
///     hetzner::TxtVerification {
///         name: "txt_name".to_string(),
///         token: "txt_token".to_string(),
///         verified: false,
///     },
///     "unverified".to_string(),
///     hetzner::ZoneType {
///         description: "master".to_string(),
///         id: "zone_type_id".to_string(),
///         name: "zone_type_name".to_string(),
///         prices: None,
///     },
/// );
/// ```
///
/// # Returns
///
/// A new `Zone` instance.
///
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Zone {
    /// The creation timestamp of the DNS zone.
    pub created: String,
    /// The zone ID associated with the DNS zone.
    pub id: String,
    /// Indicates if the zone is a secondary DNS.
    pub is_secondary_dns: bool,
    /// The legacy DNS host associated with the zone.
    pub legacy_dns_host: String,
    /// The legacy name servers associated with the zone.
    pub legacy_ns: Vec<String>,
    /// The last modified timestamp of the DNS zone.
    pub modified: String,
    /// The name of the DNS zone.
    pub name: String,
    /// The name servers associated with the zone.
    pub ns: Vec<String>,
    /// The owner of the DNS zone.
    pub owner: String,
    /// Indicates if the zone is paused.
    pub paused: bool,
    /// The permission level of the DNS zone.
    pub permission: String,
    /// The project associated with the DNS zone.
    pub project: String,
    /// The count of records in the DNS zone.
    pub records_count: i64,
    /// The registrar of the DNS zone.
    pub registrar: String,
    /// The status of the DNS zone.
    pub status: String,
    /// The time-to-live (TTL) value of the DNS zone.
    pub ttl: u32,
    /// The TXT verification details of the DNS zone.
    pub txt_verification: TxtVerification,
    /// The verification status of the DNS zone.
    pub verified: String,
    /// The zone type details of the DNS zone.
    pub zone_type: ZoneType,
}

/// Represents TXT verification details for a DNS zone.
#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TxtVerification {
    pub name: String,
    pub token: String,
}

/// Represents the type details of a DNS zone.
#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ZoneType {
    pub description: String,
    pub id: String,
    pub name: String,
    pub prices: Option<Value>,
}
