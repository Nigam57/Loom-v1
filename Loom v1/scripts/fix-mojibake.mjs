// Repairs UTF-8 text that was decoded as Windows-1252 and re-saved ("â€”" -> "—", "ä¸­" -> "中").
// Works line by line and only rewrites a line when the reversal yields valid UTF-8, so correct lines are untouched.
// Usage: node scripts/fix-mojibake.mjs [--write] <files...>
import { readFileSync, writeFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';

const CP1252 = {
  0x20ac: 0x80, 0x201a: 0x82, 0x0192: 0x83, 0x201e: 0x84, 0x2026: 0x85, 0x2020: 0x86, 0x2021: 0x87,
  0x02c6: 0x88, 0x2030: 0x89, 0x0160: 0x8a, 0x2039: 0x8b, 0x0152: 0x8c, 0x017d: 0x8e, 0x2018: 0x91,
  0x2019: 0x92, 0x201c: 0x93, 0x201d: 0x94, 0x2022: 0x95, 0x2013: 0x96, 0x2014: 0x97, 0x02dc: 0x98,
  0x2122: 0x99, 0x0161: 0x9a, 0x203a: 0x9b, 0x0153: 0x9c, 0x017e: 0x9e, 0x0178: 0x9f
};

const SUSPECT = /[Â-ô][\u0080-¿ŒœŠšŸŽžƒˆ˜–-›€™]/;
const decoder = new TextDecoder('utf-8', { fatal: true });

export const repairLine = (line) => {
  if (!SUSPECT.test(line)) return line;
  const bytes = [];
  for (const char of line) {
    const code = char.codePointAt(0);
    if (code < 0x100) bytes.push(code);
    else if (CP1252[code] !== undefined) bytes.push(CP1252[code]);
    else return line; // a real non-Latin character: this line is not mojibake
  }
  try {
    const repaired = decoder.decode(Uint8Array.from(bytes));
    return repaired === line ? line : repairLine(repaired);
  } catch {
    return line;
  }
};

export const repairText = (text) => text.split('\n').map(repairLine).join('\n');

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  const write = process.argv.includes('--write');
  for (const file of process.argv.slice(2).filter((arg) => arg !== '--write')) {
    const before = readFileSync(file, 'utf8');
    const after = repairText(before);
    const changed = before.split('\n').filter((line, i) => line !== after.split('\n')[i]).length;
    console.log(`${changed} line(s) ${write ? 'repaired' : 'to repair'}: ${file}`);
    if (write && changed) writeFileSync(file, after);
  }
}
