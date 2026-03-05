#!/usr/bin/env node

import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import readline from 'node:readline';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, '..', '..');

const DEFAULTS = {
  openapiPath: process.env.HETZNER_OPENAPI_PATH || path.join(rootDir, 'openapi.json'),
  baseUrl: process.env.HCLOUD_BASE_URL || 'https://api.hetzner.cloud/v1',
  token: process.env.HCLOUD_TOKEN || '',
};

function createResponseManager() {
  function formatOperationResult({ ok, status, statusText, method, url, response }) {
    return {
      ok,
      status,
      status_text: statusText,
      method,
      url,
      response,
    };
  }

  function parseResponseBody(text) {
    if (!text) return null;
    try {
      return JSON.parse(text);
    } catch {
      return text;
    }
  }

  function mcpSuccess(id, result) {
    return { jsonrpc: '2.0', id, result };
  }

  function mcpError(id, code, message, data) {
    return { jsonrpc: '2.0', id, error: { code, message, data } };
  }

  function cliSuccess(result) {
    return { ok: true, result };
  }

  function cliError(message, data = null) {
    return { ok: false, error: { message, data } };
  }

  function toolCallResult(result) {
    return {
      content: [{ type: 'text', text: JSON.stringify(result, null, 2) }],
      isError: !result.ok,
    };
  }

  return {
    formatOperationResult,
    parseResponseBody,
    mcpSuccess,
    mcpError,
    cliSuccess,
    cliError,
    toolCallResult,
  };
}

const responses = createResponseManager();

function loadSpec(specPath) {
  try {
    return JSON.parse(readFileSync(specPath, 'utf8'));
  } catch (error) {
    throw new Error(`Failed to read OpenAPI spec at ${specPath}: ${error.message}`);
  }
}

function buildOperations(spec) {
  const methods = ['get', 'post', 'put', 'patch', 'delete'];
  const operations = [];

  for (const [routePath, pathItem] of Object.entries(spec.paths || {})) {
    for (const method of methods) {
      const operation = pathItem?.[method];
      if (!operation) continue;

      const operationId = operation.operationId || `${method}_${routePath.replace(/[{}\/]/g, '_')}`;
      const summary = operation.summary || '';
      const description = operation.description || summary;
      const pathParams = [...routePath.matchAll(/\{([^}]+)\}/g)].map((m) => m[1]);

      operations.push({
        operationId,
        method: method.toUpperCase(),
        path: routePath,
        summary,
        description,
        pathParams,
      });
    }
  }

  operations.sort((a, b) => a.operationId.localeCompare(b.operationId));
  return operations;
}

function toTool(op, defaultBaseUrl) {
  const pathParamHint =
    op.pathParams.length > 0
      ? `Required path params: ${op.pathParams.join(', ')}.`
      : 'No path params.';

  return {
    name: op.operationId,
    description: `${op.method} ${op.path}${op.summary ? ` - ${op.summary}` : ''}`,
    inputSchema: {
      type: 'object',
      properties: {
        path_params: {
          type: 'object',
          description: pathParamHint,
          additionalProperties: {
            anyOf: [{ type: 'string' }, { type: 'number' }, { type: 'boolean' }],
          },
        },
        query: {
          type: 'object',
          description: 'Query string parameters. Arrays are supported and expanded as repeated params.',
          additionalProperties: {
            anyOf: [
              { type: 'string' },
              { type: 'number' },
              { type: 'boolean' },
              {
                type: 'array',
                items: {
                  anyOf: [{ type: 'string' }, { type: 'number' }, { type: 'boolean' }],
                },
              },
            ],
          },
        },
        body: {
          description: 'Optional JSON request body for POST/PUT/PATCH endpoints.',
        },
        base_url: {
          type: 'string',
          description: `Override API base URL (default: ${defaultBaseUrl}).`,
        },
        token: {
          type: 'string',
          description: 'Override bearer token for this call. Defaults to HCLOUD_TOKEN.',
        },
        extra_headers: {
          type: 'object',
          additionalProperties: { type: 'string' },
          description: 'Additional HTTP headers to merge into the request.',
        },
      },
      additionalProperties: false,
    },
  };
}

function renderPath(pathTemplate, pathParams) {
  return pathTemplate.replace(/\{([^}]+)\}/g, (_m, name) => {
    if (!(name in pathParams)) {
      throw new Error(`Missing required path parameter: ${name}`);
    }
    return encodeURIComponent(String(pathParams[name]));
  });
}

function appendQuery(url, query) {
  if (!query || typeof query !== 'object') return;

  for (const [key, value] of Object.entries(query)) {
    if (value === undefined || value === null) continue;

    if (Array.isArray(value)) {
      for (const item of value) {
        if (item === undefined || item === null) continue;
        url.searchParams.append(key, String(item));
      }
      continue;
    }

    url.searchParams.append(key, String(value));
  }
}

async function executeOperation(op, args, runtime) {
  const pathParams = args?.path_params || {};
  const query = args?.query || undefined;
  const body = Object.prototype.hasOwnProperty.call(args || {}, 'body') ? args.body : undefined;

  const baseUrl = args?.base_url || runtime.defaults.baseUrl;
  const token = args?.token || runtime.defaults.token;

  if (!token) {
    throw new Error('Missing API token. Set HCLOUD_TOKEN or pass `token` in arguments.');
  }

  const renderedPath = renderPath(op.path, pathParams);
  const url = new URL(renderedPath, `${baseUrl.replace(/\/$/, '')}/`);
  appendQuery(url, query);

  const headers = {
    Authorization: `Bearer ${token}`,
    'Content-Type': 'application/json',
    ...(args?.extra_headers || {}),
  };

  const init = { method: op.method, headers };

  if (body !== undefined && op.method !== 'GET' && op.method !== 'DELETE') {
    init.body = JSON.stringify(body);
  }

  const response = await fetch(url, init);
  const text = await response.text();
  const parsed = responses.parseResponseBody(text);

  return responses.formatOperationResult({
    ok: response.ok,
    status: response.status,
    statusText: response.statusText,
    method: op.method,
    url: url.toString(),
    response: parsed,
  });
}

function createRuntime(overrides = {}) {
  const defaults = {
    openapiPath: overrides.openapiPath || DEFAULTS.openapiPath,
    baseUrl: overrides.baseUrl || DEFAULTS.baseUrl,
    token: overrides.token || DEFAULTS.token,
  };

  const spec = loadSpec(defaults.openapiPath);
  const operations = buildOperations(spec);
  const opByName = new Map(operations.map((op) => [op.operationId, op]));

  return {
    defaults,
    spec,
    operations,
    opByName,
    serverInfo: { name: 'hetzner-mcp-server', version: '1.1.0' },
    capabilities: { tools: { listChanged: false } },
    tools: operations.map((op) => toTool(op, defaults.baseUrl)),
  };
}

function printJson(obj) {
  process.stdout.write(`${JSON.stringify(obj, null, 2)}\n`);
}

function parseJsonArg(value, flagName) {
  if (!value) return undefined;
  try {
    return JSON.parse(value);
  } catch (error) {
    throw new Error(`Invalid JSON for ${flagName}: ${error.message}`);
  }
}

function parseCliArgs(argv) {
  const args = {
    mode: 'stdio',
    openapiPath: undefined,
    baseUrl: undefined,
    token: undefined,
    operation: undefined,
    pathParams: undefined,
    query: undefined,
    body: undefined,
    extraHeaders: undefined,
  };

  for (let i = 0; i < argv.length; i += 1) {
    const arg = argv[i];
    const next = argv[i + 1];

    switch (arg) {
      case '--help':
      case '-h':
        args.mode = 'help';
        break;
      case '--stdio':
        args.mode = 'stdio';
        break;
      case '--list-tools':
        args.mode = 'list-tools';
        break;
      case '--describe':
        args.mode = 'describe';
        args.operation = next;
        i += 1;
        break;
      case '--call':
        args.mode = 'call';
        args.operation = next;
        i += 1;
        break;
      case '--spec':
        args.openapiPath = next;
        i += 1;
        break;
      case '--base-url':
        args.baseUrl = next;
        i += 1;
        break;
      case '--token':
        args.token = next;
        i += 1;
        break;
      case '--path-params':
        args.pathParams = parseJsonArg(next, '--path-params');
        i += 1;
        break;
      case '--query':
        args.query = parseJsonArg(next, '--query');
        i += 1;
        break;
      case '--body':
        args.body = parseJsonArg(next, '--body');
        i += 1;
        break;
      case '--extra-headers':
        args.extraHeaders = parseJsonArg(next, '--extra-headers');
        i += 1;
        break;
      default:
        break;
    }
  }

  return args;
}

function printHelp() {
  const lines = [
    'Hetzner MCP/CLI',
    '',
    'Usage:',
    '  node mcp/src/index.js --stdio',
    '  node mcp/src/index.js --list-tools',
    '  node mcp/src/index.js --describe <operationId>',
    '  node mcp/src/index.js --call <operationId> [--path-params JSON] [--query JSON] [--body JSON]',
    '',
    'Options:',
    '  --spec <path>            OpenAPI file (default: openapi.json)',
    '  --base-url <url>         Base URL override',
    '  --token <token>          API token override',
    '  --extra-headers JSON     Extra headers object',
    '  --help, -h               Show this message',
    '',
    'Environment:',
    '  HCLOUD_TOKEN, HCLOUD_BASE_URL, HETZNER_OPENAPI_PATH',
  ];

  process.stdout.write(`${lines.join('\n')}\n`);
}

function sendMcp(message) {
  process.stdout.write(`${JSON.stringify(message)}\n`);
}

async function handleMcpRequest(msg, runtime) {
  const { id, method, params } = msg;

  try {
    switch (method) {
      case 'initialize':
        sendMcp(
          responses.mcpSuccess(id, {
            protocolVersion: params?.protocolVersion || '2024-11-05',
            serverInfo: runtime.serverInfo,
            capabilities: runtime.capabilities,
          })
        );
        return;
      case 'notifications/initialized':
        return;
      case 'tools/list':
        sendMcp(responses.mcpSuccess(id, { tools: runtime.tools }));
        return;
      case 'tools/call': {
        const name = params?.name;
        const op = runtime.opByName.get(name);

        if (!op) {
          sendMcp(responses.mcpError(id, -32602, `Unknown tool: ${name}`));
          return;
        }

        const result = await executeOperation(op, params?.arguments || {}, runtime);
        sendMcp(responses.mcpSuccess(id, responses.toolCallResult(result)));
        return;
      }
      default:
        sendMcp(responses.mcpError(id, -32601, `Method not found: ${method}`));
    }
  } catch (error) {
    sendMcp(responses.mcpError(id, -32000, error.message, { stack: error.stack }));
  }
}

function startStdioMcp(runtime) {
  const rl = readline.createInterface({
    input: process.stdin,
    crlfDelay: Infinity,
  });

  rl.on('line', (line) => {
    const trimmed = line.trim();
    if (!trimmed) return;

    let msg;
    try {
      msg = JSON.parse(trimmed);
    } catch (error) {
      sendMcp(responses.mcpError(null, -32700, `Parse error: ${error.message}`));
      return;
    }

    if (msg.jsonrpc !== '2.0') {
      sendMcp(responses.mcpError(msg.id ?? null, -32600, 'Invalid Request: jsonrpc must be "2.0"'));
      return;
    }

    void handleMcpRequest(msg, runtime);
  });
}

async function runCliMode(cli, runtime) {
  switch (cli.mode) {
    case 'help':
      printHelp();
      return 0;
    case 'list-tools':
      printJson(responses.cliSuccess(runtime.tools.map((t) => ({ name: t.name, description: t.description }))));
      return 0;
    case 'describe': {
      const op = runtime.opByName.get(cli.operation || '');
      if (!op) {
        printJson(responses.cliError(`Unknown operation: ${cli.operation}`));
        return 1;
      }
      printJson(
        responses.cliSuccess({
          operationId: op.operationId,
          method: op.method,
          path: op.path,
          summary: op.summary,
          description: op.description,
          path_params: op.pathParams,
        })
      );
      return 0;
    }
    case 'call': {
      const op = runtime.opByName.get(cli.operation || '');
      if (!op) {
        printJson(responses.cliError(`Unknown operation: ${cli.operation}`));
        return 1;
      }

      try {
        const args = {
          path_params: cli.pathParams,
          query: cli.query,
          body: cli.body,
          extra_headers: cli.extraHeaders,
          base_url: cli.baseUrl,
          token: cli.token,
        };

        const result = await executeOperation(op, args, runtime);
        printJson(result.ok ? responses.cliSuccess(result) : responses.cliError('HTTP request failed', result));
        return result.ok ? 0 : 2;
      } catch (error) {
        printJson(responses.cliError(error.message));
        return 1;
      }
    }
    case 'stdio':
    default:
      startStdioMcp(runtime);
      return 0;
  }
}

async function main() {
  try {
    const cli = parseCliArgs(process.argv.slice(2));

    const runtime = createRuntime({
      openapiPath: cli.openapiPath,
      baseUrl: cli.baseUrl,
      token: cli.token,
    });

    const code = await runCliMode(cli, runtime);
    if (code !== 0 && cli.mode !== 'stdio') {
      process.exitCode = code;
    }
  } catch (error) {
    printJson(responses.cliError(error.message, { stack: error.stack }));
    process.exitCode = 1;
  }
}

void main();
