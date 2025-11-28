/** @type {import('tailwindcss').Config} */
export default {
  darkMode: ['class', '[data-theme="dark"]'],
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        background: "hsl(var(--background))",
        foreground: "hsl(var(--foreground))",
        card: {
          DEFAULT: "hsl(var(--card))",
          foreground: "hsl(var(--card-foreground))",
        },
        popover: {
          DEFAULT: "hsl(var(--popover))",
          foreground: "hsl(var(--popover-foreground))",
        },
        primary: {
          DEFAULT: "hsl(var(--primary))",
          foreground: "hsl(var(--primary-foreground))",
        },
        secondary: {
          DEFAULT: "hsl(var(--secondary))",
          foreground: "hsl(var(--secondary-foreground))",
        },
        muted: {
          DEFAULT: "hsl(var(--muted))",
          foreground: "hsl(var(--muted-foreground))",
        },
        accent: {
          DEFAULT: "hsl(var(--accent))",
          foreground: "hsl(var(--accent-foreground))",
        },
        destructive: {
          DEFAULT: "hsl(var(--destructive))",
          foreground: "hsl(var(--destructive-foreground))",
        },
        border: "hsl(var(--border))",
        input: "hsl(var(--input))",
        ring: "hsl(var(--ring))",
        chart: {
          1: "hsl(var(--chart-1))",
          2: "hsl(var(--chart-2))",
          3: "hsl(var(--chart-3))",
          4: "hsl(var(--chart-4))",
          5: "hsl(var(--chart-5))",
          6: "hsl(var(--chart-6))",
          7: "hsl(var(--chart-7))",
          8: "hsl(var(--chart-8))",
        },
        sidebar: {
          DEFAULT: "hsl(var(--sidebar-background))",
          foreground: "hsl(var(--sidebar-foreground))",
          primary: "hsl(var(--sidebar-primary))",
          "primary-foreground": "hsl(var(--sidebar-primary-foreground))",
          accent: "hsl(var(--sidebar-accent))",
          "accent-foreground": "hsl(var(--sidebar-accent-foreground))",
          border: "hsl(var(--sidebar-border))",
          ring: "hsl(var(--sidebar-ring))",
        },
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

        // ==========================================
        // NEW FEROX THEME - Surface & Content System
        // ==========================================

        // Surface colors (backgrounds)
        surface: {
          base: 'var(--surface-base)',
          DEFAULT: 'var(--surface-default)',
          elevated: 'var(--surface-elevated)',
          overlay: 'var(--surface-overlay)',
          sidebar: 'var(--surface-sidebar)',
          input: 'var(--surface-input)',
          hover: 'var(--surface-hover)',
          active: 'var(--surface-active)',
        },

        // Content colors (text)
        content: {
          primary: 'var(--content-primary)',
          secondary: 'var(--content-secondary)',
          tertiary: 'var(--content-tertiary)',
          disabled: 'var(--content-disabled)',
          inverse: 'var(--content-inverse)',
          number: 'var(--content-number)',
          'number-secondary': 'var(--content-number-secondary)',
          data: 'var(--content-data)',
          label: 'var(--content-label)',
        },

        // New border colors
        'border-new': {
          subtle: 'var(--border-subtle)',
          DEFAULT: 'var(--border-default-new)',
          strong: 'var(--border-strong-new)',
          focus: 'var(--border-focus-new)',
        },

        // New primary colors
        'primary-new': {
          DEFAULT: 'var(--primary-new)',
          hover: 'var(--primary-hover-new)',
          active: 'var(--primary-active-new)',
          soft: 'var(--primary-soft)',
          muted: 'var(--primary-muted)',
          text: 'var(--primary-text)',
        },

        // New success colors
        'success-new': {
          DEFAULT: 'var(--success-new)',
          hover: 'var(--success-hover-new)',
          soft: 'var(--success-soft)',
          muted: 'var(--success-muted)',
          text: 'var(--success-text)',
          border: 'var(--success-border)',
        },

        // New warning colors
        'warning-new': {
          DEFAULT: 'var(--warning-new)',
          hover: 'var(--warning-hover-new)',
          soft: 'var(--warning-soft)',
          muted: 'var(--warning-muted)',
          text: 'var(--warning-text)',
          border: 'var(--warning-border)',
        },

        // New danger colors
        'danger-new': {
          DEFAULT: 'var(--danger-new)',
          hover: 'var(--danger-hover-new)',
          soft: 'var(--danger-soft)',
          muted: 'var(--danger-muted)',
          text: 'var(--danger-text)',
          border: 'var(--danger-border)',
        },

        // New info colors
        'info-new': {
          DEFAULT: 'var(--info-new)',
          hover: 'var(--info-hover-new)',
          soft: 'var(--info-soft)',
          muted: 'var(--info-muted)',
          text: 'var(--info-text)',
          border: 'var(--info-border)',
        },

        // New purple colors
        'purple-new': {
          DEFAULT: 'var(--purple-new)',
          hover: 'var(--purple-hover-new)',
          soft: 'var(--purple-soft)',
          muted: 'var(--purple-muted)',
          text: 'var(--purple-text)',
          border: 'var(--purple-border)',
        },

        // Data colors
        data: {
          positive: 'var(--data-positive)',
          negative: 'var(--data-negative)',
          neutral: 'var(--data-neutral)',
          highlight: 'var(--data-highlight)',
        },

        // Toast colors
        toast: {
          bg: 'var(--toast-bg)',
          border: 'var(--toast-border)',
          text: 'var(--toast-text)',
        },
      },
      fontSize: {
        xs: 'var(--text-xs)',
        sm: 'var(--text-sm)',
        base: 'var(--text-base)',
        lg: 'var(--text-lg)',
        xl: 'var(--text-xl)',
        '2xl': 'var(--text-2xl)',
      },
      lineHeight: {
        tight: 'var(--leading-tight)',
        normal: 'var(--leading-normal)',
        relaxed: 'var(--leading-relaxed)',
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
        'card': 'var(--shadow-card)',
        // Glow shadows
        'glow-primary': 'var(--shadow-glow-primary)',
        'glow-success': 'var(--shadow-glow-success)',
        'glow-danger': 'var(--shadow-glow-danger)',
      },
      borderRadius: {
        lg: "var(--radius)",
        md: "calc(var(--radius) - 2px)",
        sm: "calc(var(--radius) - 4px)",
        xs: 'var(--radius-xs)',
        'radius-sm': 'var(--radius-sm)',
        'radius-md': 'var(--radius-md)',
        'radius-lg': 'var(--radius-lg)',
        xl: 'var(--radius-xl)',
        '2xl': 'var(--radius-2xl)',
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
