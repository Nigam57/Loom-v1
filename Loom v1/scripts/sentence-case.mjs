// One-off copy pass: short Title Case UI labels -> sentence case ("Save Changes" -> "Save changes").
// Only touches 2–5 word values where every word is capitalised; proper nouns stay as they are.
// Usage: node scripts/sentence-case.mjs [--write] <locale file>
import { readFileSync, writeFileSync } from 'node:fs';

const KEEP = new Set([
  'CLI', 'Claude', 'Code', 'Gemini', 'Qwen', 'Codex', 'OpenCode', 'Antigravity', 'Loom', 'Notion', 'Evernote',
  'MCP', 'GitHub', 'PowerShell', 'UTC', 'Enter', 'Shift', 'Ctrl', 'Alt', 'AI', 'API', 'URL', 'ID', 'OS', 'PNG', 'JPG'
]);

export const toSentenceCase = (value) => {
  const words = value.split(' ');
  if (words.length < 2 || words.length > 5) return value;
  if (!words.every((word) => /^[A-Z][A-Za-z]*$/.test(word) || KEEP.has(word))) return value;
  if (!words.slice(1).some((word) => !KEEP.has(word))) return value;
  return [words[0], ...words.slice(1).map((word) => (KEEP.has(word) ? word : word.toLowerCase()))].join(' ');
};

const [, , ...args] = process.argv;
const write = args.includes('--write');
const file = args.find((arg) => arg !== '--write');
if (file) {
  const source = readFileSync(file, 'utf8');
  let count = 0;
  const next = source.replace(/^(\s+[\w]+: ')([^'{}\n]+)(',?)$/gm, (line, head, value, tail) => {
    // Agent/product names and time zones keep their casing.
    if (/memberOptions|timeZones/.test(head)) return line;
    const cased = toSentenceCase(value);
    if (cased !== value) {
      count += 1;
      console.log(`${value}  ->  ${cased}`);
    }
    return `${head}${cased}${tail}`;
  });
  console.log(`${count} label(s) ${write ? 'rewritten' : 'would change'}`);
  if (write) writeFileSync(file, next);
}
