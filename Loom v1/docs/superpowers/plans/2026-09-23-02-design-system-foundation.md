# Design System Foundation Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans (or superpowers:subagent-driven-development) to implement this plan task-by-task. Design rules come from the Taste Skill (`~/.agents/skills/redesign-existing-projects`, `design-taste-frontend`) and Hallmark (usehallmark.com); read both before starting.

**Goal:** Replace Loom's generic glass/sky-blue/Be Vietnam Pro look with one token-driven system — Geist + Geist Mono, an OKLCH neutral ramp with a single amber accent, a 4-step radius scale, a named type scale, restrained motion, visible focus — at full dark/light parity, without rewriting components.

**Architecture:** Change the look from the outside in: (1) redefine CSS variables and the Tailwind theme so every existing class snaps to the new system at once (e.g. `rounded-3xl` becomes 10px, `shadow-glow` becomes nothing); (2) run a tested codemod that swaps hardcoded `white/*`, arbitrary `text-[Npx]`, and `font-bold/light` for semantic tokens; (3) delete the 50-line `!important` light-mode override block that the codemod makes redundant. Layout/IA changes are **not** in this plan — they are Plan 03.

**Tech Stack:** Vue 3, Tailwind CSS **3.4** (v3 syntax only — do not use v4 `@theme`), Vite 6, vitest 2, xterm 6, `@fontsource-variable/geist`, `@fontsource-variable/geist-mono`, `tailwindcss-animate`.

**Master plan:** `2026-09-23-00-master-plan.md` (Phase U1).

---

## Audit findings this plan fixes (from the 2026-09-23 taste audit)

| # | Finding | Evidence | Fixed in |
|---|---|---|---|
| 1 | One font (Be Vietnam Pro) doing every job, weights 300–700; JetBrains Mono requested by xterm but not bundled | `global.css:7-45`, `TerminalPane.vue:2056` | Task 2 |
| 2 | Sky-400 accent (~93% saturation) that changes hue in light mode; three gray families mixed | `global.css:347,370`, `WorkspaceSelection.vue:43-46`, `MessagesList.vue:45` | Task 1, 4 |
| 3 | Glass + neon: 41 `backdrop-blur`, 16 `shadow-glow`, blur on every message hover | `MessagesList.vue:25`, `SidebarNav.vue:47`, `ChatInput.vue:112` | Task 1, 5 |
| 4 | 460 arbitrary values (`text-[11px]` ×73 …, `text-[10.5px]`); no type scale | repo-wide | Task 4 |
| 5 | Radius chaos (`rounded-full` ×81, `2xl` ×47, `[14px]/[18px]`) | repo-wide | Task 1 |
| 6 | Light mode = ~35 `!important` overrides; anything not listed leaks white-on-white | `global.css:429-570` | Task 4 |
| 7 | 27 `animate-in …` classes do nothing (`tailwindcss-animate` not installed); no reduced-motion | `package.json` | Task 6 |
| 8 | Focus outlines stripped; only 7 `focus-visible` | `global.css:78-87`, `MemberRow.vue:14` | Task 7 |
| 9 | xterm theme has 3 colors, hardcoded `#0b0f14`, never follows light mode | `TerminalPane.vue:2059-2063` | Task 8 |
| 10 | 72 component gradients, incl. AI purple→indigo avatar for Antigravity | `terminalCatalog.ts:32-38`, `avatars.ts:17-75`, `ChatHeader.vue:6,29` | Task 5 |

---

### Task 0: Baseline screenshots

Run `pnpm install` then `pnpm run dev:tauri` from `D:\Loom\Loom v1`. Capture dark and light screenshots of: workspace selection, chat (with a long agent reply), members sidebar, terminal workspace, skill store, settings. Save them to `docs/superpowers/assets/2026-09-23-before/`. They are the comparison for every later task.

---

### Task 1: Tokens — OKLCH ramp, one accent, radius/type/motion/z scales

**Files:**
- Modify: `src/styles/global.css` (the `:root`, `[data-theme='light']` and `prefers-color-scheme` blocks at ~336-400)
- Modify: `tailwind.config.cjs`

**Step 1: Replace the variable blocks.** Values are `L C H` triplets consumed as `oklch(var(--x) / a)`. `--color-border` and `--color-overlay` stay "ink" colors because the codebase uses them with low alpha (`border-border/10`); solid hairlines use the new `line` tokens.

```css
/* Theme tokens: OKLCH "L C H" triplets, consumed as oklch(var(--token) / alpha). One hue family (255), one accent (amber 70). */
:root {
  --color-background: 0.155 0.006 255;
  --color-surface: 0.185 0.007 255;
  --color-surface-2: 0.215 0.008 255;
  --color-panel: 0.185 0.007 255;
  --color-panel-strong: 0.215 0.008 255;
  --color-panel-soft: 0.25 0.009 255;
  --color-hover: 0.25 0.009 255;
  --color-line: 0.30 0.010 255;
  --color-line-strong: 0.38 0.012 255;
  --color-text: 0.93 0.005 255;
  --color-text-muted: 0.71 0.010 255;
  --color-text-faint: 0.56 0.012 255;
  --color-border: 0.93 0.005 255;
  --color-overlay: 1 0 0;
  --color-primary: 0.80 0.13 70;
  --color-primary-hover: 0.76 0.13 70;
  --color-on-primary: 0.22 0.03 70;
  --color-secondary: 0.71 0.010 255;
  --color-success: 0.74 0.12 155;
  --color-warning: 0.82 0.14 90;
  --color-danger: 0.66 0.17 25;
  --scrollbar-thumb: 0.30 0.010 255;
  --scrollbar-thumb-hover: 0.38 0.012 255;
  --shadow-modal: 0 16px 40px -12px oklch(0.1 0.02 255 / 0.55);
  --ease-out: cubic-bezier(0.16, 1, 0.3, 1);
  --window-glow: none;
  --window-glow-inactive: none;
  --z-context-menu: 60;
  color-scheme: dark;
}

:root[data-theme='light'] {
  --color-background: 0.985 0.003 255;
  --color-surface: 0.97 0.004 255;
  --color-surface-2: 1 0 0;
  --color-panel: 0.97 0.004 255;
  --color-panel-strong: 1 0 0;
  --color-panel-soft: 0.945 0.005 255;
  --color-hover: 0.945 0.005 255;
  --color-line: 0.90 0.006 255;
  --color-line-strong: 0.82 0.008 255;
  --color-text: 0.22 0.010 255;
  --color-text-muted: 0.46 0.012 255;
  --color-text-faint: 0.58 0.012 255;
  --color-border: 0.22 0.010 255;
  --color-overlay: 0.22 0.010 255;
  --color-primary: 0.60 0.14 60;
  --color-primary-hover: 0.55 0.14 60;
  --color-on-primary: 0.99 0 0;
  --color-secondary: 0.46 0.012 255;
  --color-success: 0.55 0.13 155;
  --color-warning: 0.62 0.14 85;
  --color-danger: 0.55 0.19 25;
  --scrollbar-thumb: 0.82 0.008 255;
  --scrollbar-thumb-hover: 0.70 0.010 255;
  --shadow-modal: 0 16px 40px -12px oklch(0.3 0.02 255 / 0.18);
  color-scheme: light;
}
```

Duplicate the light block inside the existing `@media (prefers-color-scheme: light) { :root[data-theme='system'] { … } }` wrapper (replace its old values). Then, repo-wide in CSS and `<style>` blocks (including `index.html`), replace `rgb(var(--color-` → `oklch(var(--color-` and `rgb(var(--scrollbar-` → `oklch(var(--scrollbar-`:

```powershell
Get-ChildItem -Recurse -Include *.css,*.vue,*.html -Path src,index.html | ForEach-Object {
  $c = Get-Content -Raw -Encoding utf8 $_.FullName
  $n = $c -replace 'rgb\(var\(--(color|scrollbar)-', 'oklch(var(--$1-'
  if ($n -ne $c) { Set-Content -NoNewline -Encoding utf8 $_.FullName $n; $_.FullName }
}
```

Then `Select-String -Path src\**\*.vue,src\**\*.ts,src\**\*.css -Pattern 'rgb\(var\(--'` → Expected: no matches.

**Step 2: Rewrite `tailwind.config.cjs`:**

```js
/** @type {import('tailwindcss').Config} */
const token = (name) => `oklch(var(--color-${name}) / <alpha-value>)`;

module.exports = {
  content: ['./index.html', './src/**/*.{vue,ts,tsx,js,jsx}'],
  darkMode: 'class',
  theme: {
    extend: {
      fontFamily: {
        sans: ['"Geist Variable"', 'ui-sans-serif', 'system-ui', 'sans-serif'],
        mono: ['"Geist Mono Variable"', 'ui-monospace', 'Consolas', 'monospace']
      },
      // Named type scale (Hallmark: display/body/label ladder). Oversized steps collapse to 24px: no screaming headings in a tool.
      fontSize: {
        '2xs': ['11px', '16px'],
        xs: ['12px', '16px'],
        sm: ['13px', '19px'],
        base: ['14px', '20px'],
        lg: ['16px', '22px'],
        xl: ['20px', '26px'],
        '2xl': ['24px', '30px'],
        '3xl': ['24px', '30px'],
        '4xl': ['24px', '30px']
      },
      colors: {
        background: token('background'),
        surface: token('surface'),
        'surface-2': token('surface-2'),
        panel: token('panel'),
        'panel-strong': token('panel-strong'),
        'panel-soft': token('panel-soft'),
        hover: token('hover'),
        line: token('line'),
        'line-strong': token('line-strong'),
        overlay: token('overlay'),
        primary: token('primary'),
        'primary-hover': token('primary-hover'),
        'on-primary': token('on-primary'),
        secondary: token('secondary'),
        text: token('text'),
        muted: token('text-muted'),
        faint: token('text-faint'),
        border: token('border'),
        success: token('success'),
        warning: token('warning'),
        danger: token('danger')
      },
      // 4-step radius scale: tighter inside, softer outside. Existing rounded-2xl/3xl snap to 10px.
      borderRadius: {
        sm: '3px',
        DEFAULT: '5px',
        md: '5px',
        lg: '7px',
        xl: '7px',
        '2xl': '10px',
        '3xl': '10px'
      },
      transitionTimingFunction: {
        DEFAULT: 'cubic-bezier(0.16, 1, 0.3, 1)',
        out: 'cubic-bezier(0.16, 1, 0.3, 1)'
      },
      transitionDuration: {
        DEFAULT: '200ms',
        fast: '120ms',
        slow: '280ms'
      },
      zIndex: {
        sticky: '10',
        dropdown: '20',
        overlay: '30',
        modal: '40',
        toast: '50',
        menu: '60'
      },
      boxShadow: {
        // Glows are banned (Taste: "NO neon/outer glows"); kept as keys so existing classes render nothing until the codemod removes them.
        glow: 'none',
        glass: '0 8px 24px -8px oklch(0.1 0.02 255 / 0.5)',
        float: '0 12px 32px -12px oklch(0.1 0.02 255 / 0.55)'
      }
    }
  },
  plugins: [require('tailwindcss-animate')]
};
```

**Step 3: Install the animate plugin** (makes the 27 existing `animate-in` classes work): `pnpm add -D tailwindcss-animate@1.0.7`.

**Step 4: Verify** `pnpm build` → succeeds. `pnpm run dev:tauri` → the whole app is now charcoal with amber accents; corners are tighter; glows are gone. Compare against Task 0 screenshots. Light mode will still have leaks — Task 4 fixes them.

**Step 5: Commit** `git commit -am "feat(ui): OKLCH token ramp, single accent, radius/type/motion scales"`

---

### Task 2: Fonts — Geist (UI) + Geist Mono (display, labels, code, terminal)

**Files:**
- Modify: `package.json` (deps), `src/main.ts`, `src/styles/global.css:7-45`, `index.html` (`<body>` class)
- Delete: `src/assets/fonts/BeVietnamPro-*.woff2`
- Modify: `src/features/chat/modals/InviteAssistantModal.vue:312`, `src/features/Settings.vue:2213` (stray Rajdhani/Orbitron stacks → `font-mono`)

**Step 1:** `pnpm add @fontsource-variable/geist @fontsource-variable/geist-mono`

**Step 2:** In `src/main.ts`, before `import './styles/global.css';`:

```ts
import '@fontsource-variable/geist';
import '@fontsource-variable/geist-mono';
```

**Step 3:** Delete the five `@font-face { font-family: "Be Vietnam Pro" … }` blocks from `global.css` (keep Material Symbols) and the font files. Add, right after the `@tailwind` lines:

```css
@layer base {
  html {
    font-family: theme('fontFamily.sans');
    font-size: 13px;
    line-height: 1.45;
    font-feature-settings: 'ss01', 'cv11';
    -webkit-font-smoothing: antialiased;
  }
  /* Numbers line up in lists, timestamps, counters. */
  time, .tabular, [data-tabular] { font-variant-numeric: tabular-nums; }
  code, kbd, pre, samp { font-family: theme('fontFamily.mono'); }
}
```

**Step 4:** `index.html` body: `class="text-white h-screen …"` → `class="bg-background text-text font-sans h-screen overflow-hidden selection:bg-primary/30"`.

**Step 5: Verify** `rg -n "Be Vietnam|Rajdhani|Orbitron|JetBrains" src index.html` → Expected: only the xterm line (fixed in Task 8). Build and look: UI text is Geist; nothing falls back to a serif.

**Step 6: Commit** `git commit -am "feat(ui): Geist + Geist Mono, drop Be Vietnam Pro"`

---

### Task 3: Theme codemod — tested transform

**Files:**
- Create: `scripts/theme-codemod/transform.mjs`
- Create: `scripts/theme-codemod/run.mjs`
- Test: `src/tests/theme-codemod.spec.ts`

**Step 1: Write the failing test** — `src/tests/theme-codemod.spec.ts`:

```ts
import { describe, expect, it } from 'vitest';
// @ts-expect-error — plain ESM script without types
import { transformClasses } from '../../scripts/theme-codemod/transform.mjs';

describe('transformClasses', () => {
  it('maps hardcoded white utilities to theme tokens, keeping variants and alpha', () => {
    expect(transformClasses('text-white hover:text-white/70 bg-white/5 border-white/10 divide-white/5')).toBe(
      'text-text hover:text-text/70 bg-overlay/5 border-border/10 divide-border/5'
    );
  });

  it('inverts solid white/black surfaces', () => {
    expect(transformClasses('bg-white text-black')).toBe('bg-text text-background');
  });

  it('snaps arbitrary pixel font sizes to the named scale', () => {
    expect(transformClasses('text-[10.5px] text-[11px] text-[12px] text-[13px] text-[15px] text-[18px] text-[22px] text-[32px]')).toBe(
      'text-2xs text-2xs text-xs text-sm text-base text-lg text-xl text-2xl'
    );
  });

  it('collapses the weight ladder to 400/500/600', () => {
    expect(transformClasses('font-light font-bold font-extrabold font-medium')).toBe(
      'font-normal font-semibold font-semibold font-medium'
    );
  });

  it('removes blur and glow utilities and tidies spaces', () => {
    expect(transformClasses('px-2 backdrop-blur-md hover:backdrop-blur-[6px] shadow-glow rounded')).toBe('px-2 rounded');
  });

  it('leaves unrelated words alone', () => {
    const source = "const context = 'text-whiteboard'; // background-white";
    expect(transformClasses(source)).toBe(source);
  });
});
```

**Step 2: Run** `pnpm test src/tests/theme-codemod.spec.ts` → Expected: FAIL (module missing).

**Step 3: Implement** `scripts/theme-codemod/transform.mjs`:

```js
// Theme codemod: rewrites Tailwind utilities inside source text to Loom's semantic tokens.
const WHITE_TARGET = { text: 'text-text', bg: 'bg-overlay', border: 'border-border', divide: 'divide-border', ring: 'ring-border' };

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

// A utility starts after whitespace, a quote, a backtick or a variant colon.
const START = '(?<=^|[\\s"\'`:])';
const END = '(?=$|[\\s"\'`])';

export const transformClasses = (source) =>
  source
    .replace(new RegExp(`${START}bg-white${END}`, 'g'), 'bg-text')
    .replace(new RegExp(`${START}text-black${END}`, 'g'), 'text-background')
    .replace(
      new RegExp(`${START}(text|bg|border|divide|ring)-white(\\/[\\w.\\[\\]]+)?${END}`, 'g'),
      (_, kind, alpha = '') => `${WHITE_TARGET[kind]}${alpha}`
    )
    .replace(new RegExp(`${START}text-\\[(\\d+(?:\\.\\d+)?)px\\]${END}`, 'g'), (_, px) => `text-${sizeFor(Number(px))}`)
    .replace(new RegExp(`${START}font-(light|thin|extralight|bold|extrabold|black)${END}`, 'g'), (_, w) => `font-${WEIGHTS[w]}`)
    .replace(new RegExp(`${START}(?:[\\w-]+:)*(?:backdrop-blur(?:-[\\w]+|-\\[[^\\]]+\\])?|shadow-glow)${END}`, 'g'), '')
    .replace(/(?<=\S)[ \t]{2,}(?=\S)/g, ' ');
```

Only runs of spaces *between* two tokens are collapsed; indentation and string literals like `' '` are untouched. A removal at the start or end of a class string can leave one stray space inside the quotes — harmless, and visible in review.

`scripts/theme-codemod/run.mjs`:

```js
// Usage: node scripts/theme-codemod/run.mjs [--write]
import { readFileSync, writeFileSync, readdirSync, statSync } from 'node:fs';
import { join, extname } from 'node:path';
import { transformClasses } from './transform.mjs';

const SKIP = [/emoji-data\.ts$/, /[\\/]i18n[\\/]/];
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
```

**Step 4: Run** `pnpm test src/tests/theme-codemod.spec.ts` → PASS.

**Step 5: Commit** `git commit -am "chore(ui): theme codemod with tests"`

---

### Task 4: Apply the codemod and delete the light-mode override block

**Step 1:** Dry run `node scripts/theme-codemod/run.mjs` → review the file list. Then `node scripts/theme-codemod/run.mjs --write` and read `git diff --stat` plus a sample of diffs in `MessagesList.vue`, `SidebarNav.vue`, `Settings.vue`.

**Step 2:** Delete the light-theme utility override block in `global.css` (the section starting with the comment "light theme overrides several utility colors" at ~429 through ~570). Then `rg -n "!important" src/styles/global.css` → only the xterm helper rules remain.

**Step 3: Manual sweep of what the codemod can't decide.** Run and fix each hit by hand:

```powershell
rg -n "(text|bg|border)-(gray|slate|zinc|neutral|stone)-\d{2,3}" src   # → text-muted / text-faint / bg-surface-2 / border-line
rg -n "rgb\(\s*\d+\s*,\s*\d+\s*,\s*\d+" src                          # e.g. unread badge rgb(209,101,91) → bg-danger
rg -n "#0b0f14|#[0-9a-fA-F]{6}" src --glob "*.vue"                   # hardcoded colors in templates/styles
rg -n "tracking-\[0\.[1-9]" src                                       # wide-tracked uppercase labels → sentence case, font-mono text-2xs
```

**Step 4: Verify** both themes against Task 0 screenshots on all six screens. Nothing should be white-on-white or black-on-black. `pnpm lint` and `pnpm test` pass.

**Step 5: Commit** `git commit -am "refactor(ui): semantic tokens everywhere, remove light-mode !important overrides"`

---

### Task 5: Surfaces — no glass, no gradients, flat agent identity

**Files:**
- Modify: `index.html` (delete the duplicate `.glass-panel`, `.glass-modal`, `.bg-glass-*`, `.text-shadow` rules in `<style>`)
- Modify: `src/styles/global.css` (`.glass-panel`, `.glass-modal`, `.titlebar`)
- Modify: `src/shared/constants/terminalCatalog.ts:32-38`, `src/shared/constants/avatars.ts:17-75`
- Modify: every file from `rg -l "bg-gradient|from-|via-|to-transparent" src`

**Step 1:** Keep the class names (templates use them) but make them solid:

```css
.glass-panel {
  background-color: oklch(var(--color-surface));
  border: 1px solid oklch(var(--color-line));
}

.glass-modal {
  background-color: oklch(var(--color-surface-2));
  border: 1px solid oklch(var(--color-line-strong));
  box-shadow: var(--shadow-modal);
}
```

In `.titlebar`, remove `backdrop-filter` and set `background: oklch(var(--color-background)); border-bottom: 1px solid oklch(var(--color-line));`. In `.titlebar__title` remove `text-transform: uppercase` and `letter-spacing`, and set `font-family: theme('fontFamily.mono'); font-weight: 500;`.

**Step 2: Agent identity = one flat swatch per agent type** (Taste: max one accent; Hallmark: no gradients). In `terminalCatalog.ts`, replace each `gradient: 'from-… to-…'` with `swatch`, and update consumers to `bg-[oklch(var(--swatch))]`-free usage via inline style:

```ts
// Low-chroma swatches (C≈0.08) so the amber accent stays the only loud color.
export const AGENT_SWATCHES: Record<string, string> = {
  claude: 'oklch(0.70 0.08 45)',
  codex: 'oklch(0.70 0.06 160)',
  gemini: 'oklch(0.70 0.08 250)',
  opencode: 'oklch(0.70 0.05 200)',
  qwen: 'oklch(0.70 0.08 290)',
  'antigravity-cli': 'oklch(0.72 0.08 230)',
  shell: 'oklch(0.66 0.01 255)'
};
```

Render agent avatars as 5px-radius squares (`rounded`) filled with the swatch and a mono two-letter monogram in `text-on-primary`-like dark ink (`oklch(0.2 0.02 255)`), instead of gradient circles. Human avatars keep images, same square shape.

**Step 3: Remove remaining gradients.** For each file from the `rg` above: replace `bg-gradient-to-* from-X to-Y` with `bg-surface-2` (containers) or `bg-primary text-on-primary` (the single primary action on that screen). Gradient hairline dividers in `Settings.vue` (`:258,308,343,674,720,778`) become `border-t border-line`.

**Step 4: Verify** `rg -n "gradient|backdrop-blur|shadow-glow" src index.html` → Expected: no matches except comments. Screenshots both themes.

**Step 5: Commit** `git commit -am "refactor(ui): solid surfaces, flat agent swatches, no gradients"`

---

### Task 6: Motion — tokens, reduced motion, no `transition-all`

**Files:** `src/styles/global.css`, repo-wide `transition-all`

**Step 1:** Append to `global.css`:

```css
/* Motion: exponential ease-out, transform/opacity only; honor reduced motion. */
@media (prefers-reduced-motion: reduce) {
  *,
  *::before,
  *::after {
    animation-duration: 1ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 1ms !important;
    scroll-behavior: auto !important;
  }
}

.press:active,
button:not(:disabled):active {
  transform: translateY(1px);
}
```

**Step 2:** Replace `transition-all` with the narrowest property: `transition-colors` for hover color changes, `transition-transform` for movement, `transition-opacity` for fades. `rg -n "transition-all" src` → review each; zero should remain.

**Step 3:** Replace the fake typewriter streaming effect (`MessagesList.vue:161-162`) with plain text growth — real deltas now stream from the backend (Plan 01).

**Step 4: Verify** with Windows "Animation effects" off (Settings → Accessibility → Visual effects): modals and toasts appear without motion. **Commit** `git commit -am "feat(ui): motion tokens and reduced-motion support"`

---

### Task 7: Focus that you can see

**Files:** `src/styles/global.css:78-87`, `src/features/chat/components/MemberRow.vue:14`

**Step 1:** Delete the `.member-avatar-button:focus, :focus-visible` outline removal (keep the reset for `:focus` only), remove `outline-none` on `MemberRow.vue:14`, and add:

```css
:focus-visible {
  outline: 2px solid oklch(var(--color-primary));
  outline-offset: 2px;
}

:focus:not(:focus-visible) {
  outline: none;
}
```

**Step 2: Verify** by tabbing through: nav, conversation list, message input, member rows, settings controls — each shows the amber ring. **Commit** `git commit -am "fix(a11y): visible focus rings"`

---

### Task 8: Terminal theme that follows the app

**Files:**
- Create: `src/features/terminal/terminalTheme.ts`
- Test: `src/tests/terminal-theme.spec.ts`
- Modify: `src/features/terminal/TerminalPane.vue:2052-2063` (and the hardcoded `#0b0f14` at `:4`, `TerminalWorkspace.vue:189`)

**Step 1: Failing test:**

```ts
import { describe, expect, it } from 'vitest';
import { buildTerminalTheme } from '@/features/terminal/terminalTheme';

describe('buildTerminalTheme', () => {
  const resolve = (token: string) => `#${token.length.toString(16).padStart(6, '0')}`;

  it('takes surfaces from tokens and a full 16-color ANSI palette per mode', () => {
    const dark = buildTerminalTheme('dark', resolve);
    expect(dark.background).toBe(resolve('--color-background'));
    expect(dark.foreground).toBe(resolve('--color-text'));
    expect(dark.cursor).toBe(resolve('--color-primary'));
    expect(dark.brightWhite).toBe('#eef1f5');
    expect(buildTerminalTheme('light', resolve).black).toBe('#1f2328');
  });
});
```

**Step 2: Run** → FAIL. **Step 3: Implement** `terminalTheme.ts`:

```ts
// xterm theme: surfaces come from app tokens; ANSI colors are fixed, harmonized palettes per mode.
import type { ITheme } from '@xterm/xterm';

type Mode = 'dark' | 'light';
type Resolve = (cssVar: string) => string;

const ANSI: Record<Mode, Omit<ITheme, 'background' | 'foreground' | 'cursor' | 'cursorAccent' | 'selectionBackground'>> = {
  dark: {
    black: '#1c1f24', red: '#e06c64', green: '#7fbf8e', yellow: '#d9b25c',
    blue: '#6f9fd8', magenta: '#b58ad6', cyan: '#63b6b8', white: '#c9ced6',
    brightBlack: '#5c6370', brightRed: '#f08a82', brightGreen: '#98d4a6', brightYellow: '#ecc978',
    brightBlue: '#8cb6ea', brightMagenta: '#caa4e6', brightCyan: '#82cdcf', brightWhite: '#eef1f5'
  },
  light: {
    black: '#1f2328', red: '#b3413a', green: '#2f7d4a', yellow: '#8a6a12',
    blue: '#2f62b3', magenta: '#7d4fa8', cyan: '#1f7a7c', white: '#6b7280',
    brightBlack: '#4b5563', brightRed: '#c9544c', brightGreen: '#3b9159', brightYellow: '#a07c16',
    brightBlue: '#3b74cc', brightMagenta: '#9161bf', brightCyan: '#27908f', brightWhite: '#374151'
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
  canvas.width = canvas.height = 1;
  const ctx = canvas.getContext('2d', { willReadFrequently: true });
  if (!ctx) return '#000000';
  ctx.fillStyle = `oklch(${value})`;
  ctx.fillRect(0, 0, 1, 1);
  const [r, g, b] = ctx.getImageData(0, 0, 1, 1).data;
  return `#${[r, g, b].map((n) => n.toString(16).padStart(2, '0')).join('')}`;
};

export const currentMode = (): Mode =>
  document.documentElement.dataset.resolvedTheme === 'light' ? 'light' : 'dark';
```

**Step 4: Wire it** in `TerminalPane.vue`:

```ts
import { buildTerminalTheme, currentMode, resolveTokenHex } from './terminalTheme';
// …
  terminal = new Terminal({
    cursorBlink: true,
    allowProposedApi: true,
    fontFamily: "'Geist Mono Variable', ui-monospace, Consolas, monospace",
    fontSize: 13,
    lineHeight: 1.2,
    scrollback: 5000,
    theme: buildTerminalTheme(currentMode(), resolveTokenHex)
  });
  themeObserver = new MutationObserver(() => {
    if (terminal) terminal.options.theme = buildTerminalTheme(currentMode(), resolveTokenHex);
  });
  themeObserver.observe(document.documentElement, { attributes: true, attributeFilter: ['data-resolved-theme'] });
```

Declare `let themeObserver: MutationObserver | null = null;` next to the other handles and call `themeObserver?.disconnect()` in the existing unmount cleanup. Replace `#0b0f14` in `TerminalPane.vue:4` and `TerminalWorkspace.vue:189` with `bg-background`.

**Step 5: Run** `pnpm test src/tests/terminal-theme.spec.ts` → PASS. Manually toggle the theme in Settings with a terminal open: colors switch live; `ls` output and `git status` colors are readable in both modes.

**Step 6: Commit** `git commit -am "feat(terminal): token-driven xterm theme with ANSI palettes"`

---

### Task 10: Accent colour picker in Settings (owner request)

**Files:**
- Create: `src/features/global/accents.ts`
- Test: `src/tests/accents.spec.ts`
- Modify: `src/features/global/settingsStore.ts` (persist `accent`), `src/features/global/theme.ts` (apply it), `src/features/Settings.vue` (appearance section), `index.html` (pre-paint script), `src/i18n/locales/en-US.ts` + `zh-CN.ts`

**Design:** a fixed set of single-hue accents. Each has an OKLCH triplet per mode, so contrast holds in both themes and the "one accent" rule still holds. Applying one sets `data-accent` on `<html>`; CSS maps that to `--color-primary`, `--color-primary-hover` and `--color-on-primary`. The pre-paint script in `index.html` reads `localStorage['loom-accent']` the same way it already reads `loom-theme`, so there's no flash on start. The xterm cursor and selection follow automatically (Task 8 resolves tokens).

**Step 1: Failing test** (`src/tests/accents.spec.ts`):

```ts
import { describe, expect, it } from 'vitest';
import { ACCENTS, DEFAULT_ACCENT, accentCss, isAccentId } from '@/features/global/accents';

describe('accents', () => {
  it('defaults to amber and validates ids', () => {
    expect(DEFAULT_ACCENT).toBe('amber');
    expect(isAccentId('teal')).toBe(true);
    expect(isAccentId('rainbow')).toBe(false);
  });

  it('every accent defines dark and light triplets', () => {
    for (const accent of ACCENTS) {
      expect(accent.dark.primary).toMatch(/^0\.\d+ 0\.\d+ \d+$/);
      expect(accent.light.primary).toMatch(/^0\.\d+ 0\.\d+ \d+$/);
    }
  });

  it('generates one CSS rule per accent and mode', () => {
    const css = accentCss();
    expect(css).toContain(":root[data-accent='teal']");
    expect(css).toContain(":root[data-theme='light'][data-accent='teal']");
  });
});
```

**Step 2: Implement** `src/features/global/accents.ts`:

```ts
// Accent palette: one low-chroma hue per option, tuned per mode so text on the accent stays readable.
export type AccentTriplets = { primary: string; hover: string; onPrimary: string };
export type Accent = { id: string; labelKey: string; dark: AccentTriplets; light: AccentTriplets };

export const ACCENTS: Accent[] = [
  { id: 'amber', labelKey: 'settings.accent.amber', dark: { primary: '0.80 0.13 70', hover: '0.76 0.13 70', onPrimary: '0.22 0.03 70' }, light: { primary: '0.60 0.14 60', hover: '0.55 0.14 60', onPrimary: '0.99 0 0' } },
  { id: 'teal', labelKey: 'settings.accent.teal', dark: { primary: '0.78 0.10 185', hover: '0.74 0.10 185', onPrimary: '0.20 0.03 185' }, light: { primary: '0.55 0.10 185', hover: '0.50 0.10 185', onPrimary: '0.99 0 0' } },
  { id: 'blue', labelKey: 'settings.accent.blue', dark: { primary: '0.74 0.11 250', hover: '0.70 0.11 250', onPrimary: '0.20 0.03 250' }, light: { primary: '0.55 0.14 255', hover: '0.50 0.14 255', onPrimary: '0.99 0 0' } },
  { id: 'rose', labelKey: 'settings.accent.rose', dark: { primary: '0.74 0.12 15', hover: '0.70 0.12 15', onPrimary: '0.20 0.03 15' }, light: { primary: '0.56 0.16 15', hover: '0.51 0.16 15', onPrimary: '0.99 0 0' } },
  { id: 'green', labelKey: 'settings.accent.green', dark: { primary: '0.78 0.12 145', hover: '0.74 0.12 145', onPrimary: '0.20 0.03 145' }, light: { primary: '0.52 0.12 145', hover: '0.47 0.12 145', onPrimary: '0.99 0 0' } },
  { id: 'mono', labelKey: 'settings.accent.mono', dark: { primary: '0.90 0.005 255', hover: '0.84 0.005 255', onPrimary: '0.18 0.006 255' }, light: { primary: '0.30 0.010 255', hover: '0.24 0.010 255', onPrimary: '0.99 0 0' } }
];

export const DEFAULT_ACCENT = 'amber';

export const isAccentId = (value: unknown): value is string =>
  typeof value === 'string' && ACCENTS.some((accent) => accent.id === value);

const vars = (t: AccentTriplets) =>
  `--color-primary: ${t.primary}; --color-primary-hover: ${t.hover}; --color-on-primary: ${t.onPrimary};`;

/** CSS for every accent in both modes; injected once at startup. */
export const accentCss = (): string =>
  ACCENTS.map(
    (accent) =>
      `:root[data-accent='${accent.id}'] { ${vars(accent.dark)} }\n` +
      `:root[data-theme='light'][data-accent='${accent.id}'] { ${vars(accent.light)} }\n` +
      `@media (prefers-color-scheme: light) { :root[data-theme='system'][data-accent='${accent.id}'] { ${vars(accent.light)} } }`
  ).join('\n');

export const applyAccent = (id: string) => {
  const accent = isAccentId(id) ? id : DEFAULT_ACCENT;
  document.documentElement.dataset.accent = accent;
  window.localStorage.setItem('loom-accent', accent);
};
```

**Step 3: Wire it up.**
- In `theme.ts` startup, inject `accentCss()` once into a `<style id="loom-accents">`.
- Add `accent: string` (default `DEFAULT_ACCENT`) to `settingsStore.ts`'s persisted settings, following the pattern the theme setting already uses. Call `applyAccent` when it hydrates and when it changes.
- In `index.html`'s pre-paint script, add `root.dataset.accent = localStorage.getItem('loom-accent') || 'amber';`.
- In Settings → Appearance, show a row of swatches (a 20px square with 5px radius, each filled with its own `oklch(primary)`, the selected one ringed with `outline-2 outline-text`). Use `role="radiogroup"` with arrow-key navigation, the i18n label as `aria-label`, and a sentence-case section title: "Accent colour".

**Step 4: Run** `pnpm test src/tests/accents.spec.ts` → PASS. Then manually switch between all six accents in both themes: focus rings, the primary buttons, the active nav marker and the terminal cursor all follow, and the choice survives a restart with no flash.

**Step 5: Commit** `git commit -am "feat(settings): user-selectable accent colour"`

---

### Task 9: Pre-flight (Taste "Hard pre-flight check" + Hallmark audit)

Do not mark this plan done until every box honestly passes. Record the results in the PR.

- [ ] `rg -n "gradient|backdrop-blur|shadow-glow|Be Vietnam|text-\[\d" src index.html` → no matches
- [ ] One accent: amber appears only on focus, the active nav marker, the primary action per screen, and running-agent state (≤5% of any screenshot)
- [ ] One gray family: `rg -n "(gray|slate|zinc|neutral|stone)-\d" src` → no matches
- [ ] Weight ladder 400/500/600 only: `rg -n "font-(bold|light|thin|black|extrabold)" src` → no matches
- [ ] Dark/light parity: six screens × two themes screenshots in `docs/superpowers/assets/2026-09-23-after/`, no contrast failures (check body text ≥ 4.5:1 with the devtools contrast picker)
- [ ] Focus ring visible on every interactive element reached by Tab
- [ ] Reduced motion honored
- [ ] Terminal follows the theme
- [ ] `pnpm lint`, `pnpm test`, `pnpm build` pass
