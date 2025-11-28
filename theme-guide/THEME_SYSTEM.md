# Theme System

The Ferox Theme System is built on CSS Custom Properties (CSS Variables) providing a flexible, maintainable, and performant theming solution.

## Table of Contents

- [Architecture](#architecture)
- [Design Principles](#design-principles)
- [Theme Structure](#theme-structure)
- [Implementation](#implementation)
- [Customization](#customization)

---

## Architecture

### Core Concepts

The theme system is built on three pillars:

1. **CSS Variables** — Design tokens that define all visual properties
2. **Data Attributes** — `data-theme` for theme state, `data-i18n` for translations
3. **JavaScript Controllers** — Functions for theme switching and persistence

### How It Works

```
User Action (click toggle)
       ↓
JavaScript toggleTheme()
       ↓
Update data-theme attribute
       ↓
CSS Variables automatically update
       ↓
All styled elements reflect new theme
       ↓
Save preference to localStorage
```

---

## Design Principles

### 1. Single Source of Truth

All colors, fonts, and spacing are defined in CSS variables. Never use hardcoded values.

```css
/* Good */
.card {
    background: var(--bg-surface);
    color: var(--text-primary);
}

/* Bad */
.card {
    background: #1A1F2B;
    color: #F0F2F5;
}
```

### 2. Semantic Naming

Variables are named by purpose, not appearance:

| Variable | Purpose |
|----------|---------|
| `--bg-base` | Page background |
| `--bg-surface` | Card/container background |
| `--text-primary` | Main text color |
| `--success` | Positive actions/states |
| `--error` | Negative actions/states |

### 3. Smooth Transitions

All theme changes include smooth transitions:

```css
* {
    transition: background-color 0.3s ease,
                color 0.3s ease,
                border-color 0.3s ease;
}
```

### 4. Accessibility First

- Sufficient color contrast ratios (WCAG AA minimum)
- Focus states for keyboard navigation
- Reduced motion support

---

## Theme Structure

### Dark Theme

```css
[data-theme="dark"] {
    /* Backgrounds */
    --bg-base: #12161F;
    --bg-surface: #1A1F2B;
    --bg-elevated: #242A38;

    /* Text */
    --text-primary: #F0F2F5;
    --text-secondary: #9CA3AF;
    --text-muted: #6B7280;

    /* Borders */
    --border-default: #2D3748;
    --border-subtle: #1F2937;

    /* Semantic Colors */
    --success: #10B981;
    --success-soft: rgba(16, 185, 129, 0.15);
    --warning: #F59E0B;
    --warning-soft: rgba(245, 158, 11, 0.15);
    --error: #EF4444;
    --error-soft: rgba(239, 68, 68, 0.15);
    --info: #3B82F6;
    --info-soft: rgba(59, 130, 246, 0.15);
}
```

### Light Theme

```css
[data-theme="light"] {
    /* Backgrounds */
    --bg-base: #F4F5F7;
    --bg-surface: #FFFFFF;
    --bg-elevated: #FFFFFF;

    /* Text */
    --text-primary: #111827;
    --text-secondary: #6B7280;
    --text-muted: #9CA3AF;

    /* Borders */
    --border-default: #E5E7EB;
    --border-subtle: #F3F4F6;

    /* Semantic Colors */
    --success: #059669;
    --success-soft: rgba(5, 150, 105, 0.15);
    --warning: #D97706;
    --warning-soft: rgba(217, 119, 6, 0.15);
    --error: #DC2626;
    --error-soft: rgba(220, 38, 38, 0.15);
    --info: #2563EB;
    --info-soft: rgba(37, 99, 235, 0.15);
}
```

---

## Implementation

### HTML Setup

```html
<!DOCTYPE html>
<html lang="en" dir="ltr" data-theme="dark">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>My App</title>

    <!-- Fonts -->
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap" rel="stylesheet">

    <!-- Tailwind CSS -->
    <script src="https://cdn.tailwindcss.com"></script>

    <!-- Theme Variables -->
    <style>
        [data-theme="dark"] { /* ... */ }
        [data-theme="light"] { /* ... */ }
    </style>
</head>
<body style="background: var(--bg-base); color: var(--text-primary);">
    <!-- Content -->
</body>
</html>
```

### JavaScript Controller

```javascript
// Theme Management
const ThemeController = {
    // Initialize theme from localStorage or system preference
    init() {
        const saved = localStorage.getItem('theme');
        const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        const theme = saved || (prefersDark ? 'dark' : 'light');
        this.setTheme(theme);

        // Listen for system preference changes
        window.matchMedia('(prefers-color-scheme: dark)')
            .addEventListener('change', e => {
                if (!localStorage.getItem('theme')) {
                    this.setTheme(e.matches ? 'dark' : 'light');
                }
            });
    },

    // Set theme
    setTheme(theme) {
        document.documentElement.setAttribute('data-theme', theme);
        this.updateToggleIcon(theme);
    },

    // Toggle between themes
    toggle() {
        const current = document.documentElement.getAttribute('data-theme');
        const next = current === 'dark' ? 'light' : 'dark';
        this.setTheme(next);
        localStorage.setItem('theme', next);
    },

    // Update toggle button icon
    updateToggleIcon(theme) {
        const toggle = document.getElementById('theme-toggle');
        if (toggle) {
            toggle.innerHTML = theme === 'dark' ? '☀️' : '🌙';
        }
    }
};

// Initialize on page load
document.addEventListener('DOMContentLoaded', () => ThemeController.init());
```

### Toggle Button

```html
<button id="theme-toggle"
        onclick="ThemeController.toggle()"
        class="w-10 h-10 rounded-lg flex items-center justify-center transition-all duration-200"
        style="background: var(--bg-elevated); border: 1px solid var(--border-default);">
    🌙
</button>
```

---

## Customization

### Adding New Variables

1. Define the variable in both themes:

```css
[data-theme="dark"] {
    --my-custom-color: #8B5CF6;
}

[data-theme="light"] {
    --my-custom-color: #7C3AED;
}
```

2. Use in your styles:

```css
.custom-element {
    background: var(--my-custom-color);
}
```

### Creating a Custom Theme

```css
[data-theme="ocean"] {
    --bg-base: #0A1929;
    --bg-surface: #132F4C;
    --bg-elevated: #1E4976;
    --text-primary: #E3F2FD;
    --text-secondary: #90CAF9;
    --success: #66BB6A;
    --info: #29B6F6;
    /* ... define all variables */
}
```

### Extending the Toggle

```javascript
function cycleTheme() {
    const themes = ['dark', 'light', 'ocean'];
    const current = document.documentElement.getAttribute('data-theme');
    const currentIndex = themes.indexOf(current);
    const nextIndex = (currentIndex + 1) % themes.length;
    const next = themes[nextIndex];

    document.documentElement.setAttribute('data-theme', next);
    localStorage.setItem('theme', next);
}
```

---

## Best Practices

### Do's

- Always use CSS variables for colors
- Test both themes during development
- Ensure sufficient contrast ratios
- Use semantic variable names
- Include smooth transitions

### Don'ts

- Don't hardcode colors in components
- Don't forget to define variables for both themes
- Don't use overly long transition durations
- Don't neglect accessibility

---

## Related Documentation

- [CSS Variables Reference](CSS_VARIABLES.md)
- [Dark/Light Mode Guide](DARK_LIGHT_MODE.md)
- [Colors](COLORS.md)

---

<div align="center">

[← Back to Documentation](README.md)

</div>
