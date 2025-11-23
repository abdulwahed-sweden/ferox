/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        // Ferox brand colors
        'ferox-green': '#00ff88',
        'ferox-green-dark': '#00cc6a',
        // Dark theme palette
        dark: {
          900: '#0a0e17',
          800: '#0f1525',
          700: '#151d30',
          600: '#1a253d',
          500: '#243049',
          400: '#2e3b56',
          300: '#3d4d6a',
        },
        // Semantic colors
        danger: '#ff3366',
        warning: '#ffaa00',
        info: '#00ccff',
        success: '#00ff88',
        // Text colors
        text: {
          primary: '#ffffff',
          secondary: '#a0aec0',
          muted: '#6b7a90',
        },
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
