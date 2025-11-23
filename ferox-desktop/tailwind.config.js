/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        // Ferox brand colors (CSS variables)
        'ferox-green': 'var(--color-ferox-green)',
        'ferox-green-dark': 'var(--color-ferox-green-dim)',
        // Dark theme palette (CSS variables for theme support)
        dark: {
          900: 'var(--dark-900)',
          800: 'var(--dark-800)',
          700: 'var(--dark-700)',
          600: 'var(--dark-600)',
          500: 'var(--dark-500)',
          400: 'var(--dark-400)',
          300: 'var(--dark-300)',
        },
        // Semantic colors (CSS variables)
        danger: 'var(--color-error)',
        warning: 'var(--color-warning)',
        info: 'var(--color-info)',
        success: 'var(--color-success)',
        // Text colors (CSS variables)
        text: {
          primary: 'var(--text-primary)',
          secondary: 'var(--text-secondary)',
          muted: 'var(--text-muted)',
        },
        // Additional theme-aware colors
        primary: 'var(--color-primary)',
        accent: 'var(--color-accent)',
      },
      backgroundColor: {
        // Theme-aware background colors
        'theme-primary': 'var(--bg-primary)',
        'theme-secondary': 'var(--bg-secondary)',
        'theme-panel': 'var(--bg-panel)',
        'theme-hover': 'var(--bg-hover)',
        'theme-active': 'var(--bg-active)',
        'theme-input': 'var(--bg-input)',
      },
      borderColor: {
        // Theme-aware border colors
        'theme-primary': 'var(--border-primary)',
        'theme-secondary': 'var(--border-secondary)',
        'theme-focus': 'var(--border-focus)',
      },
      boxShadow: {
        // Theme-aware shadows
        'theme-sm': 'var(--shadow-sm)',
        'theme-md': 'var(--shadow-md)',
        'theme-lg': 'var(--shadow-lg)',
        'theme-xl': 'var(--shadow-xl)',
      },
      fontFamily: {
        mono: ['JetBrains Mono', 'Fira Code', 'Consolas', 'monospace'],
        sans: ['Inter', 'system-ui', 'sans-serif'],
      },
      animation: {
        'fade-in': 'fadeIn 0.2s ease-out',
        'slide-in': 'slideIn 0.2s ease-out',
        'pulse-glow': 'pulseGlow 2s ease-in-out infinite',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideIn: {
          '0%': { opacity: '0', transform: 'translateY(-10px)' },
          '100%': { opacity: '1', transform: 'translateY(0)' },
        },
        pulseGlow: {
          '0%, 100%': { boxShadow: '0 0 5px rgba(0, 255, 136, 0.3)' },
          '50%': { boxShadow: '0 0 20px rgba(0, 255, 136, 0.6)' },
        },
      },
    },
  },
  plugins: [],
};
