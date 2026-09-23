// xterm theme: surfaces come from app tokens; ANSI colours are fixed palettes harmonised per mode.
import type { ITheme } from '@xterm/xterm';

type Mode = 'dark' | 'light';
type Resolve = (cssVar: string) => string;
type AnsiPalette = Required<
  Pick<
    ITheme,
    | 'black' | 'red' | 'green' | 'yellow' | 'blue' | 'magenta' | 'cyan' | 'white'
    | 'brightBlack' | 'brightRed' | 'brightGreen' | 'brightYellow' | 'brightBlue' | 'brightMagenta' | 'brightCyan' | 'brightWhite'
  >
>;

const ANSI: Record<Mode, AnsiPalette> = {
  dark: {
    black: '#1c1f24',
    red: '#e06c64',
    green: '#7fbf8e',
    yellow: '#d9b25c',
    blue: '#6f9fd8',
    magenta: '#b58ad6',
    cyan: '#63b6b8',
    white: '#c9ced6',
    brightBlack: '#5c6370',
    brightRed: '#f08a82',
    brightGreen: '#98d4a6',
    brightYellow: '#ecc978',
    brightBlue: '#8cb6ea',
    brightMagenta: '#caa4e6',
    brightCyan: '#82cdcf',
    brightWhite: '#eef1f5'
  },
  light: {
    black: '#1f2328',
    red: '#b3413a',
    green: '#2f7d4a',
    yellow: '#8a6a12',
    blue: '#2f62b3',
    magenta: '#7d4fa8',
    cyan: '#1f7a7c',
    white: '#6b7280',
    brightBlack: '#4b5563',
    brightRed: '#c9544c',
    brightGreen: '#3b9159',
    brightYellow: '#a07c16',
    brightBlue: '#3b74cc',
    brightMagenta: '#9161bf',
    brightCyan: '#27908f',
    brightWhite: '#374151'
  }
};

export const buildTerminalTheme = (mode: Mode, resolve: Resolve): ITheme => ({
  ...ANSI[mode],
  background: resolve('--color-background'),
  foreground: resolve('--color-text'),
  cursor: resolve('--color-primary'),
  cursorAccent: resolve('--color-background'),
  selectionBackground: `${resolve('--color-primary')}55`
});

/** Resolve an OKLCH token to #rrggbb by painting one pixel (xterm does not parse oklch()). */
export const resolveTokenHex: Resolve = (cssVar) => {
  const value = getComputedStyle(document.documentElement).getPropertyValue(cssVar).trim();
  const canvas = document.createElement('canvas');
  canvas.width = 1;
  canvas.height = 1;
  const ctx = canvas.getContext('2d', { willReadFrequently: true });
  if (!ctx || !value) return '#000000';
  ctx.fillStyle = `oklch(${value})`;
  ctx.fillRect(0, 0, 1, 1);
  const [r, g, b] = ctx.getImageData(0, 0, 1, 1).data;
  return `#${[r, g, b].map((n) => n.toString(16).padStart(2, '0')).join('')}`;
};

export const currentTerminalMode = (): Mode =>
  document.documentElement.dataset.resolvedTheme === 'light' ? 'light' : 'dark';

/** Re-run `apply` whenever the app theme or accent changes. Returns a disposer. */
export const watchTerminalTheme = (apply: (theme: ITheme) => void): (() => void) => {
  const observer = new MutationObserver(() => apply(buildTerminalTheme(currentTerminalMode(), resolveTokenHex)));
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['data-resolved-theme', 'data-accent'] });
  return () => observer.disconnect();
};
