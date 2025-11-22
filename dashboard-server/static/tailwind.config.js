/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Ferox brand colors
        ferox: {
          green: '#00ff88',
          'green-dark': '#00cc6a',
          'green-glow': 'rgba(0, 255, 136, 0.5)',
        },
        // Status colors
        danger: '#ff3366',
        warning: '#ffaa00',
        info: '#00ccff',
        // Dark theme backgrounds
        dark: {
          900: '#0a0e27',
          800: '#0f1535',
          700: '#1a1f3a',
          600: '#252b4a',
          500: '#2d3561',
          400: '#3d4578',
          300: '#5a6490',
        },
        // Text colors
        text: {
          primary: '#ffffff',
          secondary: '#8892b0',
          muted: '#5a6490',
        },
      },
      fontFamily: {
        mono: ['JetBrains Mono', 'Fira Code', 'Consolas', 'monospace'],
        sans: ['Inter', 'system-ui', 'sans-serif'],
      },
      boxShadow: {
        'glow-green': '0 0 20px rgba(0, 255, 136, 0.3)',
        'glow-red': '0 0 20px rgba(255, 51, 102, 0.3)',
        'glow-blue': '0 0 20px rgba(0, 204, 255, 0.3)',
      },
      animation: {
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'glow': 'glow 2s ease-in-out infinite alternate',
      },
      keyframes: {
        glow: {
          '0%': { boxShadow: '0 0 5px rgba(0, 255, 136, 0.2)' },
          '100%': { boxShadow: '0 0 20px rgba(0, 255, 136, 0.6)' },
        },
      },
    },
  },
  plugins: [],
}
