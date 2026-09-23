import { describe, expect, it } from 'vitest';
import { ACCENTS, DEFAULT_ACCENT, accentCss, isAccentId } from '@/features/global/accents';

describe('accents', () => {
  it('defaults to amber and validates ids', () => {
    expect(DEFAULT_ACCENT).toBe('amber');
    expect(isAccentId('teal')).toBe(true);
    expect(isAccentId('rainbow')).toBe(false);
    expect(isAccentId(undefined)).toBe(false);
  });

  it('every accent defines dark and light OKLCH triplets', () => {
    for (const accent of ACCENTS) {
      for (const mode of [accent.dark, accent.light]) {
        for (const value of [mode.primary, mode.hover, mode.onPrimary]) {
          expect(value).toMatch(/^[\d.]+ [\d.]+ \d+$/);
        }
      }
    }
  });

  it('generates a rule per accent for dark, light and system-light', () => {
    const css = accentCss();
    expect(css).toContain(":root[data-accent='teal']");
    expect(css).toContain(":root[data-theme='light'][data-accent='teal']");
    expect(css).toContain(":root[data-theme='system'][data-accent='teal']");
  });
});
