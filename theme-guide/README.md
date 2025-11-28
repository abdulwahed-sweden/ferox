# Ferox Theme System Documentation

Welcome to the Ferox Theme System documentation. This comprehensive guide covers everything you need to know to use, customize, and extend the theme system in your projects.

## Table of Contents

### Getting Started
- [Quick Start Guide](#quick-start)
- [Installation](#installation)
- [Basic Usage](#basic-usage)

### Core Concepts
- [Theme System](THEME_SYSTEM.md) — Architecture and design principles
- [CSS Variables](CSS_VARIABLES.md) — Complete design tokens reference
- [Dark/Light Mode](DARK_LIGHT_MODE.md) — Theme switching implementation

### Design System
- [Colors](COLORS.md) — Color palette and semantic colors
- [Typography](TYPOGRAPHY.md) — Font families, sizes, and weights
- [Components](COMPONENTS.md) — UI component library

### Advanced
- [Internationalization](I18N.md) — RTL/LTR and translation system
- [Quick Reference](QUICK_REFERENCE.md) — Copy-paste code snippets

---

## Quick Start

### 1. Include Required Fonts

```html
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Sans+Arabic:wght@300;400;500;600;700&family=Inter:wght@300;400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
```

### 2. Include Tailwind CSS

```html
<script src="https://cdn.tailwindcss.com"></script>
```

### 3. Add CSS Variables

```css
[data-theme="dark"] {
    --bg-base: #12161F;
    --bg-surface: #1A1F2B;
    --bg-elevated: #242A38;
    --text-primary: #F0F2F5;
    --text-secondary: #9CA3AF;
    --border-default: #2D3748;
    --success: #10B981;
    --warning: #F59E0B;
    --error: #EF4444;
    --info: #3B82F6;
}

[data-theme="light"] {
    --bg-base: #F4F5F7;
    --bg-surface: #FFFFFF;
    --bg-elevated: #FFFFFF;
    --text-primary: #111827;
    --text-secondary: #6B7280;
    --border-default: #E5E7EB;
    --success: #059669;
    --warning: #D97706;
    --error: #DC2626;
    --info: #2563EB;
}
```

### 4. Set Initial Theme

```html
<html lang="en" dir="ltr" data-theme="dark">
```

### 5. Add Theme Toggle

```javascript
function toggleTheme() {
    const html = document.documentElement;
    const currentTheme = html.getAttribute('data-theme');
    const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
    html.setAttribute('data-theme', newTheme);
    localStorage.setItem('theme', newTheme);
}

// Load saved theme on page load
document.addEventListener('DOMContentLoaded', () => {
    const savedTheme = localStorage.getItem('theme') || 'dark';
    document.documentElement.setAttribute('data-theme', savedTheme);
});
```

---

## Installation

### Option 1: Copy from Template

Copy the complete template from `src/templates/mod.rs` into your project. This includes all CSS variables, components, and JavaScript functions.

### Option 2: Manual Setup

1. Add the CSS variables to your stylesheet
2. Include the required fonts
3. Add the JavaScript for theme/language switching
4. Use the component classes in your HTML

---

## Basic Usage

### Using CSS Variables

```css
.my-component {
    background: var(--bg-surface);
    color: var(--text-primary);
    border: 1px solid var(--border-default);
    border-radius: 12px;
}
```

### Using Tailwind with CSS Variables

```html
<div class="p-6 rounded-xl" style="background: var(--bg-surface); color: var(--text-primary);">
    Your content here
</div>
```

### Theme-Aware Components

All components automatically respond to theme changes:

```html
<!-- This button works in both dark and light modes -->
<button class="px-6 py-3 rounded-lg font-medium transition-all duration-200"
        style="background: var(--success); color: white;">
    Click Me
</button>
```

---

## Architecture Overview

```
Ferox Theme System
├── CSS Variables (Design Tokens)
│   ├── Colors (backgrounds, text, semantic)
│   ├── Typography (fonts, sizes, weights)
│   └── Spacing (consistent padding/margins)
├── Theme Switching
│   ├── data-theme attribute
│   ├── localStorage persistence
│   └── Smooth transitions
├── Internationalization
│   ├── RTL/LTR support
│   ├── data-i18n attributes
│   └── Translation system
└── Components
    ├── Cards
    ├── Tables
    ├── Buttons
    ├── Badges
    ├── Alerts
    └── Form Inputs
```

---

## Browser Support

| Browser | Version | Support |
|---------|---------|---------|
| Chrome | 88+ | Full |
| Firefox | 78+ | Full |
| Safari | 14+ | Full |
| Edge | 88+ | Full |

---

## Next Steps

1. **[Theme System](THEME_SYSTEM.md)** — Understand the architecture
2. **[CSS Variables](CSS_VARIABLES.md)** — Learn all available tokens
3. **[Components](COMPONENTS.md)** — Explore the component library
4. **[Quick Reference](QUICK_REFERENCE.md)** — Get code snippets

---

<div align="center">

[Back to Main README](../README.md)

</div>
