# Hetzner API Rust SDK

current version: `1.0.0`

A Rust SDK for Hetzner APIs with a domain-oriented structure and typed error handling.

## Documentation

- Full docs: `docs/COMPREHENSIVE_DOCUMENTATION.md`
- API reference: `docs/API_REFERENCE.md`
- Project-owned OpenAPI: `openapi.json`

## API Coverage

### `cloud` (openapi-backed)
- [x] Full Hetzner Cloud OpenAPI operation coverage via generated methods on `cloud()`
- [x] Hand-written typed surfaces for:
- [x] `servers.list`
- [x] `servers.get`
- [x] `servers.create` (typed response: `server`, `action`, `next_actions`, `root_password`)
- [x] `actions.list` (ID-filtered)
- [x] `actions.get`

### `dns` (legacy)
- [x] **GetAllZones**
- [x] **GetAllRecords**
- [x] **CreateRecord**
- [x] **GetRecord**
- [x] **UpdateRecord**
- [x] **DeleteRecord**
- [ ] **CreateZone**
- [ ] **GetZone**
- [ ] **UpdateZone**
- [ ] **DeleteZone**

## Deprecation Notice

The legacy DNS API surface is now deprecated.

- Deprecated style: direct DNS methods on `HetznerClient` (for example `client.get_all_zones()`).
- Preferred style: domain APIs (for example `client.dns().list_zones()`).

Compatibility wrappers remain available for now and are marked deprecated in code.

## Installation

```toml
[dependencies]
hetzner = "0.3.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Quick Start

```rust,no_run
use hetzner::HetznerClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HetznerClient::new("HETZNER_CLOUD_API_TOKEN");

    // Legacy DNS domain usage (preferred over deprecated direct methods):
    let zones = client.dns().list_zones().await?;
    println!("{zones:#?}");

    Ok(())
}
```

### Cloud Servers Example

```rust,no_run
use hetzner::{HetznerClient, ListServersParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HetznerClient::new("HETZNER_CLOUD_API_TOKEN");

    let params = ListServersParams {
        per_page: Some(25),
        page: Some(1),
        ..Default::default()
    };

    let servers = client.cloud().servers().list(Some(&params)).await?;
    println!("{servers:#?}");

    Ok(())
}
```

### Cloud Actions Example

```rust,no_run
use hetzner::{HetznerClient, ListActionsParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HetznerClient::new("HETZNER_CLOUD_API_TOKEN");

    let actions = client
        .cloud()
        .actions()
        .list(&ListActionsParams { ids: vec![42, 43] })
        .await?;
    println!("{actions:#?}");

    Ok(())
}
```

### Full OpenAPI Operation Example

All operations from `hetzner-cloud-openapi.json` are generated as methods on `CloudApi` using their `operationId`.

```rust,no_run
use hetzner::HetznerClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HetznerClient::new("HETZNER_CLOUD_API_TOKEN");

    // operationId: list_servers
    let servers = client.cloud().list_servers(None, None).await?;

    // operationId: get_server
    let server = client.cloud().get_server(42_u64, None, None).await?;

    println!("{servers:#?}");
    println!("{server:#?}");
    Ok(())
}
```

### API Facades

The SDK now exposes direct API groups for the areas you requested:

- `client.dns()` legacy DNS API (records/zones)
- `client.cloud().servers_api()` full Servers API (CRUD/actions/metrics)
- `client.cloud().domains()` Domain/Zone API (zones/rrsets)
- `client.cloud().private_networks()` Private Network API
- `client.cloud().load_balancers()` Load Balancer API
- `client.cloud().storage()` Storage API (volumes/images/isos)
- plus complete raw OpenAPI methods via `client.cloud().<operation_id>(...)`

## Cloud API Example Request

Since a new project commonly has no servers yet, this is the expected empty list shape.

### Example Request

```bash
curl -H "Authorization: Bearer HETZNER_CLOUD_API_TOKEN" \
  https://api.hetzner.cloud/v1/servers
```

### Example Response

```json
{
  "servers": [],
  "meta": {
    "pagination": {
      "page": 1,
      "per_page": 25,
      "previous_page": null,
      "next_page": null,
      "last_page": 1,
      "total_entries": 0
    }
  }
}
```

## Authentication

All requests to the Hetzner Cloud API must be authenticated with an API token.

Use this header:

```text
Authorization: Bearer HETZNER_CLOUD_API_TOKEN
```

To create a token: Hetzner Console -> Project -> Security -> API Tokens.

## Error Handling

The SDK returns typed errors (`HetznerError`) and preserves API error payload details.

Hetzner API error payload shape:

- `code`: machine-parsable error identifier
- `message`: human-readable description
- `details`: optional object with code-specific fields

### Example Error Payload

```json
{
  "error": {
    "code": "invalid_input",
    "message": "invalid input in field 'broken_field': is too long",
    "details": {
      "fields": [
        {
          "name": "broken_field",
          "messages": ["is too long"]
        }
      ]
    }
  }
}
```

### Common Error Codes

| Status | Code | Description |
| --- | --- | --- |
| 400 | `json_error` | Invalid JSON input in request |
| 401 | `unauthorized` | Invalid or unknown token |
| 401 | `token_readonly` | Token only allows GET requests |
| 403 | `forbidden` | Insufficient permissions |
| 403 | `maintenance` | Operation blocked by maintenance |
| 403 | `resource_limit_exceeded` | Account/project resource limit exceeded |
| 404 | `not_found` | Entity not found |
| 405 | `method_not_allowed` | HTTP method not allowed |
| 409 | `uniqueness_error` | Uniqueness constraint violation |
| 409 | `conflict` | Resource changed during request |
| 410 | `deprecated_api_endpoint` | Endpoint functionality removed |
| 412 | `resource_unavailable` | Requested resource unavailable |
| 422 | `invalid_input` | Input parsing/validation failed |
| 422 | `service_error` | Internal service-level error |
| 422 | `unsupported_error` | Action unsupported by resource |
| 423 | `locked` | Resource locked by running action |
| 423 | `protected` | Action protected for resource |
| 429 | `rate_limit_exceeded` | Request limit exceeded |
| 500 | `server_error` | API backend error |
| 503 | `unavailable` | Service/product unavailable |
| 504 | `timeout` | Request timeout |

## Actions

Actions are asynchronous tasks that may be returned when mutating resources.

- Wait until action state is `success` or `error`.
- Avoid aggressive polling to reduce rate limit pressure.
- Failed actions include additional error context.

## Labels

Labels are key/value pairs on resources.

- Keys: optional DNS-like prefix + required name segment (`prefix/name`).
- Values: up to 63 chars, empty or alphanumeric-bounded with `-`, `_`, `.` allowed.
- Prefix `hetzner.cloud/` is reserved.

### Example Labels

```json
{
  "labels": {
    "environment": "development",
    "service": "backend",
    "example.com/my": "label",
    "just-a-key": ""
  }
}
```

## Label Selector

Supported selectors include:

- `k==v` or `k=v`
- `k!=v`
- `k`
- `!k`
- `k in (v1,v2,v3)`
- `k notin (v1,v2,v3)`

Examples:

```text
env=production,type!=database
env in (testing,staging)
!type
```

## Pagination

List endpoints can support `page` and `per_page` (default 25, usually max 50).

Responses can include:

- `meta.pagination` in JSON
- `Link` header with `prev`, `next`, `last`

### Example Pagination Payload

```json
{
  "servers": [],
  "meta": {
    "pagination": {
      "page": 2,
      "per_page": 25,
      "previous_page": 1,
      "next_page": 3,
      "last_page": 4,
      "total_entries": 100
    }
  }
}
```

## Rate Limiting

All requests are rate limited.

Headers:

- `RateLimit-Limit`
- `RateLimit-Remaining`
- `RateLimit-Reset` (UNIX timestamp)

Default limit is `3600` requests/hour per project, with gradual refill.

## Server Metadata

In-server metadata endpoint:

- `http://169.254.169.254/hetzner/v1/metadata`

Available keys include:

- `hostname`
- `instance-id`
- `public-ipv4`
- `private-networks`
- `availability-zone`
- `region`

## Sorting

Some list endpoints support repeated `sort` query params.

Examples:

```text
https://api.hetzner.cloud/v1/actions?sort=status
https://api.hetzner.cloud/v1/actions?sort=status:asc
https://api.hetzner.cloud/v1/actions?sort=status:desc
https://api.hetzner.cloud/v1/actions?sort=status:asc&sort=command:desc
```

## Structure

Current crate structure:

- `client`: transport, auth, request lifecycle
- `error`: typed SDK errors and API error envelopes
- `types`: shared response/resource models
- `api::dns`: DNS domain operations
- `api::cloud`: Cloud domain operations (`servers` first)
- `api::cloud::enums`: OpenAPI-derived enums for server status/sort and action status
- `api::cloud::generated_ops`: full OpenAPI-generated operation methods

This layout is intended to support additional Hetzner domains without flattening everything into one module.
