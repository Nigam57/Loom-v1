import { describe, expect, it } from 'vitest';
// @ts-expect-error plain ESM script without type declarations
import { transformClasses } from '../../scripts/theme-codemod/transform.mjs';

describe('transformClasses', () => {
  it('maps hardcoded white utilities to theme tokens, keeping variants and alpha', () => {
    expect(transformClasses('text-white hover:text-white/70 bg-white/5 border-white/10 divide-white/5 bg-white/[0.02]')).toBe(
      'text-text hover:text-text/70 bg-overlay/5 border-border/10 divide-border/5 bg-overlay/[0.02]'
    );
  });

  it('inverts solid white/black surfaces', () => {
    expect(transformClasses('bg-white text-black')).toBe('bg-text text-background');
  });

  it('snaps arbitrary pixel font sizes to the named scale', () => {
    expect(
      transformClasses('text-[10.5px] text-[11px] text-[12px] text-[13px] text-[15px] text-[18px] text-[22px] text-[32px]')
    ).toBe('text-2xs text-2xs text-xs text-sm text-base text-lg text-xl text-2xl');
  });

  it('collapses the weight ladder to 400/500/600', () => {
    expect(transformClasses('font-light font-bold font-extrabold font-medium')).toBe(
      'font-normal font-semibold font-semibold font-medium'
    );
  });

  it('removes blur and glow utilities', () => {
    expect(transformClasses('px-2 backdrop-blur-md hover:backdrop-blur-[6px] shadow-glow rounded')).toBe('px-2 rounded');
  });

  it('maps named palette colours to semantic tokens, keeping variants and alpha', () => {
    expect(transformClasses('text-red-400 hover:bg-red-500/20 bg-emerald-500/10 text-amber-400')).toBe(
      'text-danger hover:bg-danger/20 bg-success/10 text-warning'
    );
    expect(transformClasses('text-gray-300 text-gray-400 text-slate-500 text-slate-900 placeholder-gray-500')).toBe(
      'text-text/80 text-muted text-faint text-background placeholder-faint'
    );
    expect(transformClasses('bg-[rgb(209,101,91)] border-[rgb(209,101,91)]')).toBe('bg-danger border-danger');
  });

  it('replaces black drop shadows and shouting section labels', () => {
    expect(transformClasses('p-2 shadow-2xl hover:shadow-lg').trim()).toBe('p-2 shadow-float');
    expect(transformClasses('text-2xs font-semibold text-text/40 uppercase tracking-wider')).toBe('text-2xs font-medium text-faint');
  });

  it('leaves unrelated words and string literals alone', () => {
    const source = "const context = 'text-whiteboard'; const gap = ' '; // background-white";
    expect(transformClasses(source)).toBe(source);
  });
});
