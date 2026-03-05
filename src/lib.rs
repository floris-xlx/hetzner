//! Hetzner API SDK for Rust.
//!
//! This crate currently ships DNS support and is structured for additional
//! Hetzner APIs over time.

pub mod api;
pub mod client;
pub mod error;
pub mod types;

pub use api::cloud::{
    actions::ListActionsParams,
    domains::DomainsApi,
    enums::{ActionStatus, ServerSort, ServerStatus},
    generated_ops::QueryPairs,
    load_balancers_api::LoadBalancersApi,
    private_networks::PrivateNetworksApi,
    servers::{CreateServerInput, ListServersParams},
    servers_api::ServersFullApi,
    storage::StorageApi,
};
pub use client::HetznerClient;
pub use error::{ApiError, HetznerError, Result};
pub use types::{
    Action, ActionEnvelope, ActionError, ActionResource, ActionsEnvelope, CloudServer,
    CloudServerEnvelope, CloudServersEnvelope, CreateServerResponse, CreatedRecord, Meta,
    Pagination, Record, RecordEnvelope, RecordsEnvelope, TxtVerification, Zone, ZoneType,
    ZonesEnvelope,
};
