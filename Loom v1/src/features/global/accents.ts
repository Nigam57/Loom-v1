// Accent palette: one low-chroma hue per option, tuned per mode so text on the accent stays readable.
export type AccentTriplets = { primary: string; hover: string; onPrimary: string };
export type Accent = { id: string; labelKey: string; dark: AccentTriplets; light: AccentTriplets };

export const ACCENTS: Accent[] = [
  {
    id: 'amber',
    labelKey: 'settings.accent.amber',
    dark: { primary: '0.80 0.13 70', hover: '0.76 0.13 70', onPrimary: '0.22 0.03 70' },
    light: { primary: '0.60 0.14 60', hover: '0.55 0.14 60', onPrimary: '0.99 0 0' }
  },
  {
    id: 'teal',
    labelKey: 'settings.accent.teal',
    dark: { primary: '0.78 0.10 185', hover: '0.74 0.10 185', onPrimary: '0.20 0.03 185' },
    light: { primary: '0.55 0.10 185', hover: '0.50 0.10 185', onPrimary: '0.99 0 0' }
  },
  {
    id: 'blue',
    labelKey: 'settings.accent.blue',
    dark: { primary: '0.74 0.11 250', hover: '0.70 0.11 250', onPrimary: '0.20 0.03 250' },
    light: { primary: '0.55 0.14 255', hover: '0.50 0.14 255', onPrimary: '0.99 0 0' }
  },
  {
    id: 'rose',
    labelKey: 'settings.accent.rose',
    dark: { primary: '0.74 0.12 15', hover: '0.70 0.12 15', onPrimary: '0.20 0.03 15' },
    light: { primary: '0.56 0.16 15', hover: '0.51 0.16 15', onPrimary: '0.99 0 0' }
  },
  {
    id: 'green',
    labelKey: 'settings.accent.green',
    dark: { primary: '0.78 0.12 145', hover: '0.74 0.12 145', onPrimary: '0.20 0.03 145' },
    light: { primary: '0.52 0.12 145', hover: '0.47 0.12 145', onPrimary: '0.99 0 0' }
  },
  {
    id: 'violet',
    labelKey: 'settings.accent.violet',
    dark: { primary: '0.74 0.10 295', hover: '0.70 0.10 295', onPrimary: '0.20 0.03 295' },
    light: { primary: '0.52 0.13 295', hover: '0.47 0.13 295', onPrimary: '0.99 0 0' }
  },
  {
    id: 'mono',
    labelKey: 'settings.accent.mono',
    dark: { primary: '0.90 0.005 255', hover: '0.84 0.005 255', onPrimary: '0.18 0.006 255' },
    light: { primary: '0.30 0.010 255', hover: '0.24 0.010 255', onPrimary: '0.99 0 0' }
  }
];

export const DEFAULT_ACCENT = 'amber';
export const ACCENT_STORAGE_KEY = 'loom-accent';

export const isAccentId = (value: unknown): value is string =>
  typeof value === 'string' && ACCENTS.some((accent) => accent.id === value);

const vars = (triplets: AccentTriplets) =>
  `--color-primary: ${triplets.primary}; --color-primary-hover: ${triplets.hover}; --color-on-primary: ${triplets.onPrimary};`;

/** CSS for every accent in both modes; injected once at startup. */
export const accentCss = (): string =>
  ACCENTS.map(
    (accent) =>
      `:root[data-accent='${accent.id}'] { ${vars(accent.dark)} }\n` +
      `:root[data-theme='light'][data-accent='${accent.id}'] { ${vars(accent.light)} }\n` +
      `@media (prefers-color-scheme: light) { :root[data-theme='system'][data-accent='${accent.id}'] { ${vars(accent.light)} } }`
  ).join('\n');

export const injectAccentStyles = () => {
  if (typeof document === 'undefined' || document.getElementById('loom-accents')) return;
  const style = document.createElement('style');
  style.id = 'loom-accents';
  style.textContent = accentCss();
  document.head.appendChild(style);
};

export const applyAccent = (id: string) => {
  const accent = isAccentId(id) ? id : DEFAULT_ACCENT;
  if (typeof document !== 'undefined') {
    document.documentElement.dataset.accent = accent;
  }
  try {
    window.localStorage.setItem(ACCENT_STORAGE_KEY, accent);
  } catch {
    // Storage can be unavailable; the attribute alone still applies the accent.
  }
};
