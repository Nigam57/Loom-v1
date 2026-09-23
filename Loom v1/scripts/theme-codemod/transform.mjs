// Theme codemod: rewrites Tailwind utilities inside source text to Loom's semantic tokens.
const WHITE_TARGET = { text: 'text-text', bg: 'bg-overlay', border: 'border-border', divide: 'divide-border', ring: 'ring-border', placeholder: 'placeholder-text' };

const SIZE_STEPS = [
  [11, '2xs'],
  [12, 'xs'],
  [13, 'sm'],
  [15, 'base'],
  [18, 'lg'],
  [22, 'xl'],
  [Infinity, '2xl']
];

const sizeFor = (px) => SIZE_STEPS.find(([max]) => px <= max)[1];

const WEIGHTS = { light: 'normal', thin: 'normal', extralight: 'normal', bold: 'semibold', extrabold: 'semibold', black: 'semibold' };

// A utility starts after whitespace, a quote, a backtick or a variant colon, and ends before whitespace or a quote.
const START = '(?<=^|[\\s"\'`:])';
const END = '(?=$|[\\s"\'`])';

// Named Tailwind palette utilities -> semantic tokens (one gray family, semantic status colours).
const PALETTE = [
  [/(text|bg|border|ring)-(red|rose)-(\d{2,3})/g, (_, kind) => `${kind}-danger`],
  [/(text|bg|border|ring)-(green|emerald)-(\d{2,3})/g, (_, kind) => `${kind}-success`],
  [/(text|bg|border|ring)-(amber|yellow)-(\d{2,3})/g, (_, kind) => `${kind}-warning`],
  [/text-(gray|slate|zinc|neutral)-(50|100|200|300)/g, () => 'text-text/80'],
  [/text-(gray|slate|zinc|neutral)-400/g, () => 'text-muted'],
  [/text-(gray|slate|zinc|neutral)-(500|600)/g, () => 'text-faint'],
  [/text-(gray|slate|zinc|neutral)-(700|800|900|950)/g, () => 'text-background'],
  [/placeholder-(gray|slate|zinc|neutral)-\d{3}/g, () => 'placeholder-faint'],
  [/bg-\[rgb\(209,101,91\)\]/g, () => 'bg-danger'],
  [/border-\[rgb\(209,101,91\)\]/g, () => 'border-danger']
];

const applyPalette = (source) =>
  PALETTE.reduce(
    (text, [pattern, replace]) =>
      text.replace(new RegExp(`${START}((?:[\\w-]+:)*)${pattern.source}(\\/[\\w.\\[\\]]+)?${END}`, 'g'), (...match) => {
        const variants = match[1];
        const alpha = match[match.length - 3] ?? '';
        const core = match[0].slice(variants.length, match[0].length - alpha.length);
        return `${variants}${core.replace(new RegExp(pattern.source), replace)}${alpha}`;
      }),
    source
  );

export const transformClasses = (source) =>
  applyPalette(source)
    .replace(new RegExp(`${START}shadow-(?:2xl|xl)${END}`, 'g'), 'shadow-float')
    .replace(new RegExp(`${START}(?:[\\w-]+:)*shadow-(?:lg|md)${END}`, 'g'), '')
    .replace(
      new RegExp(`${START}(text-2xs|text-xs) font-semibold text-(?:text|muted)(?:\\/\\d+)? uppercase tracking-(?:wider|widest|wide|\\[[\\d.]+em\\])${END}`, 'g'),
      '$1 font-medium text-faint'
    )
    .replace(new RegExp(`${START}bg-white${END}`, 'g'), 'bg-text')
    .replace(new RegExp(`${START}text-black${END}`, 'g'), 'text-background')
    .replace(
      new RegExp(`${START}(text|bg|border|divide|ring|placeholder)-white(\\/[\\w.\\[\\]]+)?${END}`, 'g'),
      (_, kind, alpha = '') => `${WHITE_TARGET[kind]}${alpha}`
    )
    .replace(new RegExp(`${START}text-\\[(\\d+(?:\\.\\d+)?)px\\]${END}`, 'g'), (_, px) => `text-${sizeFor(Number(px))}`)
    .replace(new RegExp(`${START}font-(light|thin|extralight|bold|extrabold|black)${END}`, 'g'), (_, w) => `font-${WEIGHTS[w]}`)
    .replace(new RegExp(`${START}(?:[\\w-]+:)*(?:backdrop-blur(?:-[\\w]+|-\\[[^\\]]+\\])?|shadow-glow)${END}`, 'g'), '')
    .replace(/(?<=\S)[ \t]{2,}(?=\S)/g, ' ');
