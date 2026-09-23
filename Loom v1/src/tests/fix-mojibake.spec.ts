import { describe, expect, it } from 'vitest';
// @ts-expect-error plain ESM script without type declarations
import { repairLine } from '../../scripts/fix-mojibake.mjs';

describe('repairLine', () => {
  it('repairs punctuation mojibake', () => {
    expect(repairLine("owner: 'Group Owner â€” {count}',")).toBe("owner: 'Group Owner — {count}',");
    expect(repairLine("hint: 'Enter to send â€¢ Shift+Enter'")).toBe("hint: 'Enter to send • Shift+Enter'");
    expect(repairLine('FB: front Â· FR')).toBe('FB: front · FR');
  });

  it('repairs CJK mojibake', () => {
    const broken = new TextDecoder('windows-1252').decode(new TextEncoder().encode('// 主题引擎'));
    expect(repairLine(broken)).toBe('// 主题引擎');
  });

  it('leaves correct text alone', () => {
    for (const line of ['plain ascii', 'Em dash — fine', '中文注释', 'café']) {
      expect(repairLine(line)).toBe(line);
    }
  });
});
