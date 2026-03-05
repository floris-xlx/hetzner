import { readFileSync, writeFileSync } from 'node:fs';

const sourcePath = process.argv[2] ?? 'hetzner-cloud-openapi.json';
const outPath = process.argv[3] ?? 'openapi.json';

const source = JSON.parse(readFileSync(sourcePath, 'utf8'));
const includePrefixes = [
  '/actions',
  '/servers',
  '/networks',
  '/load_balancers',
  '/load_balancer_types',
  '/volumes',
  '/images',
  '/isos',
  '/zones',
];

const paths = {};
for (const [path, item] of Object.entries(source.paths ?? {})) {
  if (includePrefixes.some((prefix) => path === prefix || path.startsWith(`${prefix}/`))) {
    paths[path] = item;
  }
}

const custom = {
  openapi: source.openapi ?? '3.1.0',
  info: {
    title: 'Hetzner SDK Unified API',
    version: '1.0.0',
    description:
      'Curated OpenAPI document for the SDK-first API families: DNS/Domain, Servers, Private Networks, Load Balancers, Storage, and Actions.',
  },
  servers: source.servers ?? [{ url: 'https://api.hetzner.cloud/v1' }],
  tags: [
    { name: 'Actions', description: 'Asynchronous task resources.' },
    { name: 'Servers', description: 'Compute server lifecycle and operations.' },
    { name: 'Networks', description: 'Private network resources and actions.' },
    { name: 'Load Balancers', description: 'Load balancer resources, metrics, and actions.' },
    { name: 'Storage', description: 'Volumes, images, and ISOs.' },
    { name: 'Domains', description: 'Zone/domain and RRSet resources.' },
  ],
  paths,
  components: source.components ?? {},
  security: source.security ?? [{ APIToken: [] }],
};

writeFileSync(outPath, JSON.stringify(custom, null, 2) + '\n');

const pathCount = Object.keys(paths).length;
console.log(`Wrote ${outPath} with ${pathCount} paths`);
