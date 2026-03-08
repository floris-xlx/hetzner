# Copilot Instructions

## Project Overview

This repository is a **Rust SDK for Hetzner APIs** (Cloud and DNS). It provides a domain-oriented client with typed error handling, hand-written facades for common operations, and auto-generated methods from the Hetzner Cloud OpenAPI specification.

The repository also includes an `/mcp` sub-project: a Node.js **Model Context Protocol (MCP) server** that exposes the full SDK as callable tools.

Current crate version: `1.0.0`

## Repository Structure

```
src/
  lib.rs                    # Library entry point – public API exports
  main.rs                   # Runnable example (server listing)
  client.rs                 # Core HTTP client (auth, request lifecycle)
  error.rs                  # HetznerError and ApiError types
  types.rs                  # Shared response/resource models
  api/
    mod.rs                  # Domain routing (cloud, dns)
    cloud/
      mod.rs                # CloudApi facade (.servers(), .actions(), …)
      servers.rs            # Hand-written servers facade
      actions.rs            # Hand-written actions facade
      generated_ops.rs      # Auto-generated OpenAPI operations
      domains.rs            # Domain/Zone API
      private_networks.rs   # Private Network API
      load_balancers_api.rs # Load Balancer API
      storage.rs            # Storage API (volumes/images/isos)
      enums.rs              # OpenAPI-derived enums
    dns/
      mod.rs                # DnsApi facade
      zones.rs              # Zone operations
      records.rs            # Record CRUD operations
tests/
  cloud_facades_test.rs
  cloud_generated_ops_full_test.rs
  dns_full_api_test.rs
scripts/                    # Node.js code-generation scripts
  build_custom_openapi.mjs
  generate_cloud_ops.mjs
  generate_cloud_ops_tests.mjs
  generate_api_reference.mjs
mcp/                        # MCP server (Node.js)
  src/index.js
  tests/
  package.json
docs/
  COMPREHENSIVE_DOCUMENTATION.md
  API_REFERENCE.md
```

## Build, Lint, and Test

### Rust (main crate)

```bash
# Build
cargo build

# Run the example binary
cargo run

# Run all tests
cargo test

# Lint / format
cargo clippy
cargo fmt --check
```

Tests live in `/tests` and use `httpmock` to mock HTTP responses. No live API token is required to run the test suite.

### Code generation (Node.js scripts)

```bash
node scripts/build_custom_openapi.mjs      # Rebuild custom OpenAPI spec
node scripts/generate_cloud_ops.mjs        # Regenerate src/api/cloud/generated_ops.rs
node scripts/generate_cloud_ops_tests.mjs  # Regenerate cloud generated-ops tests
node scripts/generate_api_reference.mjs    # Regenerate docs/API_REFERENCE.md
```

Run these scripts after modifying `hetzner-cloud-openapi.json`.

### MCP server (`/mcp`)

```bash
cd mcp
npm install
npm start              # Start the MCP server
npm run test:e2e       # Run end-to-end tests
node --check src/index.js  # Syntax check
```

## Coding Conventions

### Rust style

- **Edition**: 2024.
- Use `async`/`await` throughout; the async runtime is `tokio` with `features = ["full"]`.
- HTTP is done via `reqwest` with JSON support. Do not add another HTTP client.
- Serialization uses `serde` with `#[derive(Serialize, Deserialize)]`.
- Logging uses `tracing` macros (`info!`, `debug!`, `warn!`, `error!`). Do not use `println!` in library code.
- Errors: return `Result<T, HetznerError>`. Preserve API error details (`code`, `message`, `details`) using the existing `ApiError` type in `src/error.rs`.
- Keep hand-written facades in their own files (e.g. `servers.rs`, `actions.rs`). Place auto-generated code only in `generated_ops.rs`.
- Prefer the domain-API calling style (`client.cloud().servers().list(…)`) over flat methods on `HetznerClient`.
- Mark deprecated methods with `#[deprecated]` and a `note` pointing to the preferred alternative.

### General

- Do not commit API tokens or secrets. Use environment variables (loaded via `dotenv` in development).
- Generated files (`generated_ops.rs`, `docs/API_REFERENCE.md`) should be regenerated via the scripts above rather than edited by hand.
- Keep `Cargo.toml` dependency versions pinned to what is already specified; bump only when there is a concrete reason.

## Key Concepts

### Client usage

```rust
use hetzner::HetznerClient;

let client = HetznerClient::new("HETZNER_API_TOKEN");

// Cloud API
let servers = client.cloud().servers().list(None).await?;
let action  = client.cloud().actions().get(42).await?;

// Full OpenAPI-generated operations
let result = client.cloud().list_servers(None, None).await?;

// DNS API (preferred domain style)
let zones = client.dns().list_zones().await?;
```

### Error handling

```rust
use hetzner::error::HetznerError;

match client.cloud().servers().get(99).await {
    Ok(server)                   => { /* … */ }
    Err(HetznerError::Api(e))    => eprintln!("API error {}: {}", e.code, e.message),
    Err(HetznerError::Http(e))   => eprintln!("HTTP error: {e}"),
    Err(e)                       => eprintln!("Other error: {e}"),
}
```

### Adding a new API operation

1. If it is part of the OpenAPI spec, update `hetzner-cloud-openapi.json` and re-run `node scripts/generate_cloud_ops.mjs`.
2. If it is a hand-written typed surface, add a new method (or file) under `src/api/cloud/` or `src/api/dns/`, following the patterns in `servers.rs` or `records.rs`.
3. Add or regenerate matching integration tests under `/tests`.
4. Export any new public types through `src/lib.rs`.
