import { describe, expect, it } from 'vitest';
import { buildTerminalTheme } from '@/features/terminal/terminalTheme';

describe('buildTerminalTheme', () => {
  const resolve = (token: string) => (token === '--color-primary' ? '#e0a040' : token === '--color-text' ? '#eeeeee' : '#111111');

  it('takes surfaces from tokens', () => {
    const theme = buildTerminalTheme('dark', resolve);
    expect(theme.background).toBe('#111111');
    expect(theme.foreground).toBe('#eeeeee');
    expect(theme.cursor).toBe('#e0a040');
    expect(theme.selectionBackground).toBe('#e0a04055');
  });

  it('ships a full 16-colour ANSI palette for each mode', () => {
    for (const mode of ['dark', 'light'] as const) {
      const theme = buildTerminalTheme(mode, resolve);
      const keys = ['black', 'red', 'green', 'yellow', 'blue', 'magenta', 'cyan', 'white'] as const;
      for (const key of keys) {
        expect(theme[key]).toMatch(/^#[0-9a-f]{6}$/);
        const bright = `bright${key[0].toUpperCase()}${key.slice(1)}` as keyof typeof theme;
        expect(theme[bright]).toMatch(/^#[0-9a-f]{6}$/);
      }
    }
    expect(buildTerminalTheme('light', resolve).black).toBe('#1f2328');
  });
});
