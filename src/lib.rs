
use serde::{Deserialize, Serialize};

pub mod client;
pub mod zones;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HetznerClient {
    pub auth_api_token: String,
    pub base_url: String,

    // query params
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


#[derive(Debug, Clone)]
pub struct Record {
    pub id: String,
    pub name: String,
    pub ttl: u64,
    pub type_: String,
    pub value: String,
    pub zone_id: String,
}


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
    CAA
}


pub struct PrimaryServer {
    pub address: String,
    pub port: u16,
    pub zone_id: String,
}

pub struct Zone {
    pub zone_id: Option<String>,
    pub name: String,
    pub ttl: u64,
}