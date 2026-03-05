# Hetzner MCP Server

MCP server that exposes the full API in `../openapi.json` as tools.

- One MCP tool per OpenAPI `operationId`
- Dynamic path/query/body handling
- Auth via `Authorization: Bearer <token>`

## Requirements

- Node.js 20+

## Environment

- `HCLOUD_TOKEN`: default API token used for tool calls
- `HCLOUD_BASE_URL`: optional base URL override (default `https://api.hetzner.cloud/v1`)
- `HETZNER_OPENAPI_PATH`: optional spec path override (default `../openapi.json`)

## Run

```bash
cd mcp
npm run start
```

## CLI

The server can also run as a direct CLI utility.

```bash
cd mcp
npm run cli -- --list-tools
npm run cli -- --describe list_servers
npm run cli -- --call get_server --path-params '{"id":42}'
```

To run in MCP stdio mode explicitly:

```bash
npm run cli -- --stdio
```

## MCP Client Configuration Example

```json
{
  "mcpServers": {
    "hetzner": {
      "command": "node",
      "args": ["/Users/floris/Documents/GitHub/hetzner/mcp/src/index.js"],
      "env": {
        "HCLOUD_TOKEN": "tbzcPCFQOYdvw5udQKYpaGeeT32GvUix2Iw7T6z0ZVy22c5EnmTVCxgkYOz8b4p5"
      }
    }
  }
}
```

## Tool Input Contract

Every generated tool accepts this schema:

- `path_params` object: values for `{...}` segments in endpoint path
- `query` object: query parameters (array values expand as repeated params)
- `body`: JSON body for write operations
- `token` (optional): per-call token override
- `base_url` (optional): per-call base URL override
- `extra_headers` (optional): additional headers

CLI flags map directly to these fields:

- `--path-params <json>`
- `--query <json>`
- `--body <json>`
- `--extra-headers <json>`
- `--token <value>`
- `--base-url <value>`

## Response

The server returns structured JSON in text form:

- `ok`, `status`, `status_text`
- `method`, `url`
- parsed `response` body (JSON or text)

## Scope

This MCP server exposes all operations included in the project-owned OpenAPI file:

- Servers API
- Domain/DNS API
- Private Network API
- Load Balancer API
- Storage API
- Actions API
