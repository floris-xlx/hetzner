use crate::api::cloud::enums::{ActionStatus, ServerStatus};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Meta {
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pagination {
    pub page: u32,
    pub per_page: u32,
    pub previous_page: Option<u32>,
    pub next_page: Option<u32>,
    pub last_page: Option<u32>,
    pub total_entries: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Record {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub ttl: u64,
    #[serde(rename = "type")]
    pub record_type: String,
    pub value: String,
    pub zone_id: String,
    pub created: String,
    pub modified: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Zone {
    pub created: String,
    pub id: String,
    pub is_secondary_dns: bool,
    pub legacy_dns_host: String,
    pub legacy_ns: Vec<String>,
    pub modified: String,
    pub name: String,
    pub ns: Vec<String>,
    pub owner: String,
    pub paused: bool,
    pub permission: String,
    pub project: String,
    pub records_count: i64,
    pub registrar: String,
    pub status: String,
    pub ttl: u32,
    pub txt_verification: TxtVerification,
    pub verified: String,
    pub zone_type: ZoneType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TxtVerification {
    pub name: String,
    pub token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ZoneType {
    pub description: String,
    pub id: String,
    pub name: String,
    pub prices: Option<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreatedRecord {
    pub record: Record,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordEnvelope {
    pub record: Record,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordsEnvelope {
    pub records: Vec<Record>,
    #[serde(default)]
    pub meta: Option<Meta>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ZonesEnvelope {
    pub zones: Vec<Zone>,
    #[serde(default)]
    pub meta: Option<Meta>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CloudServersEnvelope {
    pub servers: Vec<CloudServer>,
    #[serde(default)]
    pub meta: Option<Meta>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CloudServerEnvelope {
    pub server: CloudServer,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CloudServer {
    pub id: u64,
    pub name: String,
    pub status: ServerStatus,
    pub created: String,
    #[serde(default)]
    pub public_net: Option<Value>,
    #[serde(default)]
    pub private_net: Vec<Value>,
    #[serde(default)]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub server_type: Option<Value>,
    #[serde(default)]
    pub datacenter: Option<Value>,
    #[serde(default)]
    pub image: Option<Value>,
    #[serde(default)]
    pub iso: Option<Value>,
    #[serde(default)]
    pub protection: Option<Value>,
    #[serde(default)]
    pub volumes: Vec<u64>,
    #[serde(default)]
    pub load_balancers: Vec<u64>,
    #[serde(default)]
    pub placement_group: Option<Value>,
    #[serde(default)]
    pub outgoing_traffic: Option<u64>,
    #[serde(default)]
    pub ingoing_traffic: Option<u64>,
    #[serde(default)]
    pub included_traffic: Option<u64>,
    #[serde(default)]
    pub backup_window: Option<String>,
    #[serde(default)]
    pub rescue_enabled: Option<bool>,
    #[serde(default)]
    pub locked: Option<bool>,
    #[serde(default)]
    pub primary_disk_size: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActionResource {
    pub id: u64,
    #[serde(rename = "type")]
    pub resource_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActionError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Action {
    pub id: u64,
    pub command: String,
    pub status: ActionStatus,
    pub started: String,
    pub finished: Option<String>,
    pub progress: i32,
    pub resources: Vec<ActionResource>,
    pub error: Option<ActionError>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActionEnvelope {
    pub action: Action,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActionsEnvelope {
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateServerResponse {
    pub server: CloudServer,
    pub action: Action,
    pub next_actions: Vec<Action>,
    pub root_password: Option<String>,
}
