import { readFileSync, writeFileSync } from 'node:fs';

const specPath = process.argv[2] ?? 'openapi.json';
const outPath = process.argv[3] ?? 'docs/API_REFERENCE.md';

const spec = JSON.parse(readFileSync(specPath, 'utf8'));

const groups = [
  { title: 'Servers API', test: (p) => p.startsWith('/servers') },
  { title: 'Domain and DNS API', test: (p) => p.startsWith('/zones') },
  { title: 'Private Network API', test: (p) => p.startsWith('/networks') },
  { title: 'Load Balancer API', test: (p) => p.startsWith('/load_balancers') || p.startsWith('/load_balancer_types') },
  { title: 'Storage API', test: (p) => p.startsWith('/volumes') || p.startsWith('/images') || p.startsWith('/isos') },
  { title: 'Actions API', test: (p) => p.startsWith('/actions') },
];

const methods = ['get', 'post', 'put', 'patch', 'delete'];

function collectOps(test) {
  const ops = [];
  for (const [path, item] of Object.entries(spec.paths ?? {})) {
    if (!test(path)) continue;
    for (const m of methods) {
      const op = item[m];
      if (!op) continue;
      ops.push({ method: m.toUpperCase(), path, operationId: op.operationId ?? '-', summary: op.summary ?? '' });
    }
  }
  ops.sort((a, b) => a.path.localeCompare(b.path) || a.method.localeCompare(b.method));
  return ops;
}

const lines = [];
lines.push('# API Reference');
lines.push('');
lines.push('This reference is generated from `openapi.json`.');
lines.push('');

for (const group of groups) {
  const ops = collectOps(group.test);
  lines.push(`## ${group.title}`);
  lines.push('');
  lines.push(`Total operations: **${ops.length}**`);
  lines.push('');
  lines.push('| Method | Path | Operation ID | Summary |');
  lines.push('| --- | --- | --- | --- |');
  for (const op of ops) {
    const summary = op.summary.replace(/\|/g, '\\|');
    lines.push(
      '| `' +
        op.method +
        '` | `' +
        op.path +
        '` | `' +
        op.operationId +
        '` | ' +
        summary +
        ' |'
    );
  }
  lines.push('');
}

writeFileSync(outPath, lines.join('\n') + '\n');
console.log(`Wrote ${outPath}`);
