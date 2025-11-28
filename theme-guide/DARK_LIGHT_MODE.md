# Dark/Light Mode

Complete guide to implementing and customizing theme switching in the Ferox Theme System.

## Table of Contents

- [How It Works](#how-it-works)
- [Implementation](#implementation)
- [Toggle Button](#toggle-button)
- [Persistence](#persistence)
- [System Preference Detection](#system-preference-detection)
- [Transitions](#transitions)
- [Advanced Usage](#advanced-usage)

---

## How It Works

The theme system uses the `data-theme` attribute on the `<html>` element to control the active theme. CSS variables automatically update based on this attribute.

```
User clicks toggle → JavaScript updates data-theme → CSS variables change → UI updates
```

### Theme Flow

```javascript
// 1. User clicks toggle
toggleButton.addEventListener('click', toggleTheme);

// 2. JavaScript updates attribute
document.documentElement.setAttribute('data-theme', 'light');

// 3. CSS variables automatically respond
[data-theme="light"] {
    --bg-base: #F4F5F7;  /* Now active */
}

// 4. All styled elements update instantly
body { background: var(--bg-base); }  /* Now #F4F5F7 */
```

---

## Implementation

### HTML Structure

```html
<!DOCTYPE html>
<html lang="en" dir="ltr" data-theme="dark">
<head>
    <style>
        /* Dark theme variables */
        [data-theme="dark"] {
            --bg-base: #12161F;
            --bg-surface: #1A1F2B;
            --text-primary: #F0F2F5;
            --text-secondary: #9CA3AF;
            --border-default: #2D3748;
            --success: #10B981;
            --warning: #F59E0B;
            --error: #EF4444;
            --info: #3B82F6;
        }

        /* Light theme variables */
        [data-theme="light"] {
            --bg-base: #F4F5F7;
            --bg-surface: #FFFFFF;
            --text-primary: #111827;
            --text-secondary: #6B7280;
            --border-default: #E5E7EB;
            --success: #059669;
            --warning: #D97706;
            --error: #DC2626;
            --info: #2563EB;
        }

        /* Global transition for smooth switching */
        * {
            transition: background-color 0.3s ease,
                        color 0.3s ease,
                        border-color 0.3s ease;
        }
    </style>
</head>
<body style="background: var(--bg-base); color: var(--text-primary);">
    <!-- Content -->
</body>
</html>
```

### JavaScript Toggle

```javascript
function toggleTheme() {
    const html = document.documentElement;
    const currentTheme = html.getAttribute('data-theme');
    const newTheme = currentTheme === 'dark' ? 'light' : 'dark';

    // Update theme
    html.setAttribute('data-theme', newTheme);

    // Save preference
    localStorage.setItem('theme', newTheme);

    // Update toggle button icon
    updateThemeIcon(newTheme);
}

function updateThemeIcon(theme) {
    const toggle = document.getElementById('theme-toggle');
    if (toggle) {
        toggle.innerHTML = theme === 'dark' ? '☀️' : '🌙';
    }
}
```

---

## Toggle Button

### Basic Toggle

```html
<button id="theme-toggle"
        onclick="toggleTheme()"
        class="w-10 h-10 rounded-lg flex items-center justify-center transition-all duration-200"
        style="background: var(--bg-elevated);
               border: 1px solid var(--border-default);
               color: var(--text-primary);"
        aria-label="Toggle theme">
    🌙
</button>
```

### Animated Toggle with Icons

```html
<button id="theme-toggle"
        onclick="toggleTheme()"
        class="relative w-12 h-12 rounded-xl overflow-hidden transition-all duration-300"
        style="background: var(--bg-elevated);
               border: 1px solid var(--border-default);">
    <!-- Sun icon (shown in dark mode) -->
    <span class="sun-icon absolute inset-0 flex items-center justify-center transition-transform duration-300">
        ☀️
    </span>
    <!-- Moon icon (shown in light mode) -->
    <span class="moon-icon absolute inset-0 flex items-center justify-center transition-transform duration-300">
        🌙
    </span>
</button>

<style>
    [data-theme="dark"] .sun-icon { transform: translateY(0); }
    [data-theme="dark"] .moon-icon { transform: translateY(100%); }
    [data-theme="light"] .sun-icon { transform: translateY(-100%); }
    [data-theme="light"] .moon-icon { transform: translateY(0); }
</style>
```

### Toggle with Tooltip

```html
<div class="relative group">
    <button id="theme-toggle"
            onclick="toggleTheme()"
            class="w-10 h-10 rounded-lg flex items-center justify-center"
            style="background: var(--bg-elevated); border: 1px solid var(--border-default);">
        🌙
    </button>
    <span class="absolute -bottom-8 left-1/2 -translate-x-1/2 px-2 py-1 rounded text-xs
                 opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap"
          style="background: var(--bg-surface); color: var(--text-secondary);">
        Toggle theme
    </span>
</div>
```

---

## Persistence

### localStorage

```javascript
// Save theme preference
function saveTheme(theme) {
    localStorage.setItem('theme', theme);
}

// Load theme on page load
function loadSavedTheme() {
    const saved = localStorage.getItem('theme');
    if (saved) {
        document.documentElement.setAttribute('data-theme', saved);
        updateThemeIcon(saved);
    }
}

// Call on page load
document.addEventListener('DOMContentLoaded', loadSavedTheme);
```

### Prevent Flash of Wrong Theme

Add this script in `<head>` before any CSS:

```html
<head>
    <script>
        // Immediately set theme to prevent flash
        (function() {
            const saved = localStorage.getItem('theme');
            if (saved) {
                document.documentElement.setAttribute('data-theme', saved);
            }
        })();
    </script>
    <!-- CSS comes after -->
    <style>/* ... */</style>
</head>
```

---

## System Preference Detection

### Detect System Theme

```javascript
function getSystemTheme() {
    return window.matchMedia('(prefers-color-scheme: dark)').matches
        ? 'dark'
        : 'light';
}

// Use system preference if no saved preference
function initTheme() {
    const saved = localStorage.getItem('theme');
    const theme = saved || getSystemTheme();
    document.documentElement.setAttribute('data-theme', theme);
    updateThemeIcon(theme);
}
```

### Listen for System Changes

```javascript
function watchSystemTheme() {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

    mediaQuery.addEventListener('change', (e) => {
        // Only update if user hasn't manually set a preference
        if (!localStorage.getItem('theme')) {
            const newTheme = e.matches ? 'dark' : 'light';
            document.documentElement.setAttribute('data-theme', newTheme);
            updateThemeIcon(newTheme);
        }
    });
}

// Initialize
document.addEventListener('DOMContentLoaded', () => {
    initTheme();
    watchSystemTheme();
});
```

### Three-State Toggle (Auto/Dark/Light)

```javascript
const themes = ['auto', 'dark', 'light'];
let currentIndex = 0;

function cycleTheme() {
    currentIndex = (currentIndex + 1) % themes.length;
    const theme = themes[currentIndex];

    if (theme === 'auto') {
        localStorage.removeItem('theme');
        const systemTheme = getSystemTheme();
        document.documentElement.setAttribute('data-theme', systemTheme);
    } else {
        localStorage.setItem('theme', theme);
        document.documentElement.setAttribute('data-theme', theme);
    }

    updateToggleLabel(theme);
}

function updateToggleLabel(theme) {
    const icons = { auto: '🔄', dark: '🌙', light: '☀️' };
    document.getElementById('theme-toggle').innerHTML = icons[theme];
}
```

---

## Transitions

### Global Transition

```css
/* Apply to all elements */
* {
    transition: background-color 0.3s ease,
                color 0.3s ease,
                border-color 0.3s ease,
                box-shadow 0.3s ease;
}

/* Disable for initial load */
.no-transition * {
    transition: none !important;
}
```

### Prevent Transition on Page Load

```javascript
// Add class to disable transitions during load
document.documentElement.classList.add('no-transition');

// Remove after page loads
window.addEventListener('load', () => {
    requestAnimationFrame(() => {
        document.documentElement.classList.remove('no-transition');
    });
});
```

### Custom Transition Duration

```css
/* Faster transition for buttons */
button {
    transition: background-color 0.15s ease,
                transform 0.15s ease;
}

/* Slower transition for backgrounds */
.card, .modal {
    transition: background-color 0.5s ease;
}
```

---

## Advanced Usage

### Theme-Specific Images

```html
<picture>
    <source srcset="logo-light.png" media="(prefers-color-scheme: light)">
    <img src="logo-dark.png" alt="Logo">
</picture>
```

Or with CSS:

```css
.logo {
    background-image: url('logo-dark.png');
}

[data-theme="light"] .logo {
    background-image: url('logo-light.png');
}
```

### Theme-Aware Charts

```javascript
function getChartColors() {
    const theme = document.documentElement.getAttribute('data-theme');

    return {
        background: getComputedStyle(document.documentElement)
            .getPropertyValue('--bg-surface').trim(),
        text: getComputedStyle(document.documentElement)
            .getPropertyValue('--text-primary').trim(),
        grid: getComputedStyle(document.documentElement)
            .getPropertyValue('--border-subtle').trim(),
    };
}

// Update chart when theme changes
function onThemeChange() {
    const colors = getChartColors();
    chart.updateOptions({ colors });
}
```

### Theme Event Dispatch

```javascript
function toggleTheme() {
    const html = document.documentElement;
    const newTheme = html.getAttribute('data-theme') === 'dark' ? 'light' : 'dark';

    html.setAttribute('data-theme', newTheme);
    localStorage.setItem('theme', newTheme);

    // Dispatch custom event
    window.dispatchEvent(new CustomEvent('themechange', {
        detail: { theme: newTheme }
    }));
}

// Listen for theme changes
window.addEventListener('themechange', (e) => {
    console.log('Theme changed to:', e.detail.theme);
    // Update charts, maps, third-party components, etc.
});
```

---

## Complete Example

```html
<!DOCTYPE html>
<html lang="en" dir="ltr" data-theme="dark">
<head>
    <script>
        // Prevent flash
        (function() {
            const saved = localStorage.getItem('theme');
            const system = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
            document.documentElement.setAttribute('data-theme', saved || system);
        })();
    </script>

    <style>
        [data-theme="dark"] { /* dark variables */ }
        [data-theme="light"] { /* light variables */ }
        * { transition: background-color 0.3s, color 0.3s, border-color 0.3s; }
    </style>
</head>
<body style="background: var(--bg-base); color: var(--text-primary);">
    <button id="theme-toggle" onclick="toggleTheme()">🌙</button>

    <script>
        function toggleTheme() {
            const html = document.documentElement;
            const current = html.getAttribute('data-theme');
            const next = current === 'dark' ? 'light' : 'dark';
            html.setAttribute('data-theme', next);
            localStorage.setItem('theme', next);
            document.getElementById('theme-toggle').innerHTML = next === 'dark' ? '☀️' : '🌙';
        }
    </script>
</body>
</html>
```

---

## Related Documentation

- [Theme System](THEME_SYSTEM.md)
- [CSS Variables](CSS_VARIABLES.md)
- [Colors](COLORS.md)

---

<div align="center">

[← Back to Documentation](README.md)

</div>
