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
pub struct Zone {
    /// The unique identifier of the zone.
    pub zone_id: Option<String>,
    /// The name of the zone.
    pub name: String,
    /// The time-to-live (TTL) value of the zone.
    pub ttl: u64,
}
