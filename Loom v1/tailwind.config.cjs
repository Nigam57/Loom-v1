/** @type {import('tailwindcss').Config} */
// Design tokens live in src/styles/global.css as OKLCH "L C H" triplets; Tailwind only maps names onto them.
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
      // Named type scale: oversized steps collapse to 24px so nothing in the tool shouts.
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
      // 4-step radius scale, tighter inside and softer outside; existing 2xl/3xl snap to 10px.
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
        DEFAULT: '180ms',
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
        // Glows are out; the keys stay so old classes render nothing until they are cleaned up.
        glow: 'none',
        glass: '0 8px 24px -8px oklch(0.1 0.02 255 / 0.5)',
        float: '0 12px 32px -12px oklch(0.1 0.02 255 / 0.55)'
      }
    }
  },
  plugins: [require('tailwindcss-animate')]
};
