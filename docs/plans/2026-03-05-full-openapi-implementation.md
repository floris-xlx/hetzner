# Full OpenAPI Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement full Hetzner Cloud OpenAPI coverage in this SDK via generation and integration.

**Architecture:** Generate a complete Rust client from `hetzner-cloud-openapi.json` using OpenAPI Generator, keep existing hand-written DNS modules intact, and expose the generated Cloud client as first-class API from this crate.

**Tech Stack:** Rust, reqwest, OpenAPI Generator (docker image), serde.

---

### Task 1: Generate full OpenAPI client
- Files:
- Create: `src/generated/cloud/*`
- Step 1: run OpenAPI generator with Rust + reqwest library settings
- Step 2: verify generated output compiles standalone

### Task 2: Integrate generated client into crate
- Files:
- Modify: `src/lib.rs`
- Modify: `Cargo.toml`
- Step 1: add module export for generated client
- Step 2: add dependency tweaks required by generated code
- Step 3: wire easy entrypoint constructor from existing token

### Task 3: Preserve existing API + docs
- Files:
- Modify: `README.md`
- Modify: `src/main.rs`
- Step 1: document generated full-coverage cloud client usage
- Step 2: keep legacy DNS notes and compatibility methods

### Task 4: Validate
- Files:
- Modify: as needed to resolve build mismatches
- Step 1: `cargo fmt`
- Step 2: `cargo check`
- Step 3: `cargo test --no-run`
