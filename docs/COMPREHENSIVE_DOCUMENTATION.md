# Hetzner SDK Documentation

## Overview

This SDK provides two layers for Hetzner APIs:

- Typed facade APIs for the most common workflows.
- Full OpenAPI operation coverage via generated `operationId` methods.

## Architecture

### Layers

1. `HetznerClient`
- Auth token holder and request transport.
- Exposes `dns()` and `cloud()` domains.

2. Typed domain facades
- `client.cloud().servers()`
- `client.cloud().actions()`
- `client.cloud().servers_api()`
- `client.cloud().domains()`
- `client.cloud().private_networks()`
- `client.cloud().load_balancers()`
- `client.cloud().storage()`

3. Full generated operations
- `client.cloud().<operation_id>(query, body)` for all included operations.
- Implemented in `src/api/cloud/generated_ops.rs`.

## Authentication

### Cloud

Use bearer token authentication.

```http
Authorization: Bearer <TOKEN>
```

### DNS (legacy)

Uses `Auth-API-Token` header in DNS-specific methods.

```http
Auth-API-Token: <TOKEN>
```

## Error Model

All SDK calls return:

- `Result<T, HetznerError>`

`HetznerError` variants:

- `Http(reqwest::Error)`
- `Serialization(serde_json::Error)`
- `Api(ApiError)` with `status`, `code`, `message`, `details`
- `UnexpectedResponse(&'static str)`

## API Groups

### DNS API

- `client.dns().list_zones()`
- `client.dns().records(zone_id).list()`
- `client.dns().records(zone_id).create(...)`
- `client.dns().record(record_id).get/update/delete`

Legacy direct methods on `HetznerClient` remain available but deprecated.

### Servers API

Typed:

- `client.cloud().servers().list(...)`
- `client.cloud().servers().get(id)`
- `client.cloud().servers().create(...)`

Full facade:

- `client.cloud().servers_api().list/create/get/update/delete`
- server action helpers like `power_on`, `power_off`, `reboot`

### Domain API

- `client.cloud().domains().list/create/get/update/delete`
- `client.cloud().domains().list_rrsets/get_rrset/upsert_rrset`

### Private Network API

- `client.cloud().private_networks().list/create/get/update/delete`
- `add_subnet`, `add_route`

### Load Balancer API

- `client.cloud().load_balancers().list/create/get/update/delete`
- `metrics`

### Storage API

- Volumes: list/create/get/update/delete
- Images: list/get
- ISOs: list/get

## Full OpenAPI Coverage

`openapi.json` is the project-owned OpenAPI document for this SDK scope.

- Includes: Actions, Servers, Domains/DNS Zones, Private Networks, Load Balancers, Storage.
- `docs/API_REFERENCE.md` is generated from `openapi.json`.

## Regeneration Workflow

Rebuild curated OpenAPI document:

```bash
node scripts/build_custom_openapi.mjs hetzner-cloud-openapi.json openapi.json
```

Rebuild operation reference docs:

```bash
node scripts/generate_api_reference.mjs openapi.json docs/API_REFERENCE.md
```

Rebuild generated cloud operation methods:

```bash
node scripts/generate_cloud_ops.mjs hetzner-cloud-openapi.json src/api/cloud/generated_ops.rs
```

## Examples

### Full generated operation call

```rust,no_run
use hetzner::HetznerClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HetznerClient::new("tbzcPCFQOYdvw5udQKYpaGeeT32GvUix2Iw7T6z0ZVy22c5EnmTVCxgkYOz8b4p5");
    let response = client.cloud().list_servers(None, None).await?;
    println!("{response:#?}");
    Ok(())
}
```

### Domain facade call

```rust,no_run
use hetzner::HetznerClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HetznerClient::new("tbzcPCFQOYdvw5udQKYpaGeeT32GvUix2Iw7T6z0ZVy22c5EnmTVCxgkYOz8b4p5");
    let zones = client.cloud().domains().list(None).await?;
    println!("{zones:#?}");
    Ok(())
}
```
