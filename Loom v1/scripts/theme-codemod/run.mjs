// Usage: node scripts/theme-codemod/run.mjs [--write]
import { readFileSync, writeFileSync, readdirSync, statSync } from 'node:fs';
import { join, extname } from 'node:path';
import { transformClasses } from './transform.mjs';

const SKIP = [/emoji-data\.ts$/, /[\\/]i18n[\\/]/, /[\\/]tests[\\/]/];
const write = process.argv.includes('--write');

const walk = (dir) =>
  readdirSync(dir).flatMap((name) => {
    const path = join(dir, name);
    return statSync(path).isDirectory() ? walk(path) : [path];
  });

let changed = 0;
for (const file of walk('src').filter((f) => ['.vue', '.ts'].includes(extname(f)) && !SKIP.some((r) => r.test(f)))) {
  const before = readFileSync(file, 'utf8');
  const after = transformClasses(before);
  if (after !== before) {
    changed += 1;
    console.log(file);
    if (write) writeFileSync(file, after);
  }
}
console.log(`${changed} file(s) ${write ? 'rewritten' : 'would change'}`);
