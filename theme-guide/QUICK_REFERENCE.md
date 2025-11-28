# Quick Reference

Copy-paste code snippets for common patterns in the Ferox Theme System.

## Table of Contents

- [Setup](#setup)
- [Theme Toggle](#theme-toggle)
- [Language Toggle](#language-toggle)
- [Components](#components)
- [CSS Variables](#css-variables)
- [Common Patterns](#common-patterns)

---

## Setup

### Minimal HTML Boilerplate

```html
<!DOCTYPE html>
<html lang="ar" dir="rtl" data-theme="dark">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>My App</title>

    <!-- Fonts -->
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Sans+Arabic:wght@400;500;600;700&family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet">

    <!-- Tailwind -->
    <script src="https://cdn.tailwindcss.com"></script>

    <!-- Theme CSS -->
    <style>
        [data-theme="dark"] {
            --bg-base: #12161F;
            --bg-surface: #1A1F2B;
            --text-primary: #F0F2F5;
            --text-secondary: #9CA3AF;
            --border-default: #2D3748;
            --success: #10B981;
            --success-soft: rgba(16, 185, 129, 0.15);
            --warning: #F59E0B;
            --error: #EF4444;
            --info: #3B82F6;
        }
        [data-theme="light"] {
            --bg-base: #F4F5F7;
            --bg-surface: #FFFFFF;
            --text-primary: #111827;
            --text-secondary: #6B7280;
            --border-default: #E5E7EB;
            --success: #059669;
            --success-soft: rgba(5, 150, 105, 0.15);
            --warning: #D97706;
            --error: #DC2626;
            --info: #2563EB;
        }
        * { transition: background-color 0.3s, color 0.3s, border-color 0.3s; }
    </style>
</head>
<body style="background: var(--bg-base); color: var(--text-primary); font-family: 'IBM Plex Sans Arabic', 'Inter', sans-serif;">
    <!-- Content -->
</body>
</html>
```

---

## Theme Toggle

### Simple Toggle

```html
<button onclick="toggleTheme()" style="background: var(--bg-surface); border: 1px solid var(--border-default); padding: 0.5rem; border-radius: 0.5rem;">
    🌙
</button>

<script>
function toggleTheme() {
    const html = document.documentElement;
    const next = html.getAttribute('data-theme') === 'dark' ? 'light' : 'dark';
    html.setAttribute('data-theme', next);
    localStorage.setItem('theme', next);
    event.target.textContent = next === 'dark' ? '☀️' : '🌙';
}
</script>
```

### With Persistence

```javascript
// Add to <head> to prevent flash
(function() {
    const t = localStorage.getItem('theme');
    if (t) document.documentElement.setAttribute('data-theme', t);
})();
```

---

## Language Toggle

### Simple Toggle

```html
<button id="lang-btn" onclick="toggleLang()" style="background: var(--bg-surface); border: 1px solid var(--border-default); padding: 0.5rem 1rem; border-radius: 0.5rem;">
    EN
</button>

<script>
const translations = {
    ar: { hello: "مرحباً", welcome: "أهلاً بك" },
    en: { hello: "Hello", welcome: "Welcome" }
};

let lang = localStorage.getItem('language') || 'ar';

function toggleLang() {
    lang = lang === 'ar' ? 'en' : 'ar';
    document.documentElement.lang = lang;
    document.documentElement.dir = lang === 'ar' ? 'rtl' : 'ltr';
    document.querySelectorAll('[data-i18n]').forEach(el => {
        el.textContent = translations[lang][el.getAttribute('data-i18n')];
    });
    document.getElementById('lang-btn').textContent = lang === 'ar' ? 'EN' : 'AR';
    localStorage.setItem('language', lang);
}
</script>
```

---

## Components

### Card

```html
<div style="background: var(--bg-surface); border: 1px solid var(--border-default); border-radius: 0.75rem; padding: 1.5rem;">
    <h3 style="color: var(--text-primary); font-weight: 600;">Card Title</h3>
    <p style="color: var(--text-secondary);">Card content</p>
</div>
```

### Stats Card

```html
<div style="background: var(--bg-surface); border: 1px solid var(--border-default); border-radius: 0.75rem; padding: 1.5rem;">
    <div style="color: var(--text-secondary); font-size: 0.875rem;">Users</div>
    <div style="color: var(--text-primary); font-size: 1.875rem; font-weight: 700;">12,847</div>
    <div style="color: var(--success); font-size: 0.875rem;">↑ +12.5%</div>
</div>
```

### Button - Primary

```html
<button style="background: var(--info); color: white; padding: 0.75rem 1.5rem; border-radius: 0.5rem; font-weight: 500;">
    Button
</button>
```

### Button - Secondary

```html
<button style="background: var(--bg-surface); color: var(--text-primary); border: 1px solid var(--border-default); padding: 0.75rem 1.5rem; border-radius: 0.5rem; font-weight: 500;">
    Button
</button>
```

### Button - Success

```html
<button style="background: var(--success); color: white; padding: 0.75rem 1.5rem; border-radius: 0.5rem; font-weight: 500;">
    Success
</button>
```

### Button - Danger

```html
<button style="background: var(--error); color: white; padding: 0.75rem 1.5rem; border-radius: 0.5rem; font-weight: 500;">
    Delete
</button>
```

### Badge - Success

```html
<span style="background: var(--success-soft); color: var(--success); padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.75rem; font-weight: 500;">
    Active
</span>
```

### Badge - Warning

```html
<span style="background: var(--warning-soft); color: var(--warning); padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.75rem; font-weight: 500;">
    Pending
</span>
```

### Alert - Info

```html
<div style="background: var(--info-soft); border: 1px solid var(--info); color: var(--info); padding: 1rem; border-radius: 0.5rem;">
    ℹ️ This is an info message
</div>
```

### Alert - Success

```html
<div style="background: var(--success-soft); border: 1px solid var(--success); color: var(--success); padding: 1rem; border-radius: 0.5rem;">
    ✅ Operation successful
</div>
```

### Alert - Error

```html
<div style="background: var(--error-soft); border: 1px solid var(--error); color: var(--error); padding: 1rem; border-radius: 0.5rem;">
    ❌ Something went wrong
</div>
```

### Input

```html
<input type="text" placeholder="Enter text..."
       style="background: var(--bg-elevated); border: 1px solid var(--border-default); color: var(--text-primary); padding: 0.75rem 1rem; border-radius: 0.5rem; width: 100%;">
```

### Input with Label

```html
<div style="display: flex; flex-direction: column; gap: 0.5rem;">
    <label style="color: var(--text-secondary); font-size: 0.875rem; font-weight: 500;">Email</label>
    <input type="email" placeholder="you@example.com"
           style="background: var(--bg-elevated); border: 1px solid var(--border-default); color: var(--text-primary); padding: 0.75rem 1rem; border-radius: 0.5rem;">
</div>
```

---

## CSS Variables

### All Variables (Dark)

```css
[data-theme="dark"] {
    --bg-base: #12161F;
    --bg-surface: #1A1F2B;
    --bg-elevated: #242A38;
    --text-primary: #F0F2F5;
    --text-secondary: #9CA3AF;
    --text-muted: #6B7280;
    --border-default: #2D3748;
    --border-subtle: #1F2937;
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

### All Variables (Light)

```css
[data-theme="light"] {
    --bg-base: #F4F5F7;
    --bg-surface: #FFFFFF;
    --bg-elevated: #FFFFFF;
    --text-primary: #111827;
    --text-secondary: #6B7280;
    --text-muted: #9CA3AF;
    --border-default: #E5E7EB;
    --border-subtle: #F3F4F6;
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

## Common Patterns

### Number Display (Always LTR)

```html
<span style="font-family: 'Inter', monospace; direction: ltr; font-variant-numeric: tabular-nums;">
    12,847.50
</span>
```

### Centered Container

```html
<div style="max-width: 1200px; margin: 0 auto; padding: 0 1.5rem;">
    <!-- Content -->
</div>
```

### Sticky Header

```html
<header style="position: sticky; top: 0; z-index: 50; background: rgba(18, 22, 31, 0.8); backdrop-filter: blur(12px); border-bottom: 1px solid var(--border-default);">
    <!-- Header content -->
</header>
```

### Grid Layout

```html
<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 1.5rem;">
    <!-- Grid items -->
</div>
```

### Flex Row (Direction-Aware)

```html
<div style="display: flex; align-items: center; gap: 1rem;">
    <!-- Flex items -->
</div>
```

### Table

```html
<table style="width: 100%; border-collapse: collapse;">
    <thead>
        <tr style="border-bottom: 1px solid var(--border-default);">
            <th style="padding: 1rem; text-align: start; color: var(--text-secondary);">Header</th>
        </tr>
    </thead>
    <tbody>
        <tr style="border-bottom: 1px solid var(--border-subtle);">
            <td style="padding: 1rem; color: var(--text-primary);">Data</td>
        </tr>
    </tbody>
</table>
```

---

## Tailwind + CSS Variables

### Common Classes

```html
<!-- Card -->
<div class="p-6 rounded-xl" style="background: var(--bg-surface); border: 1px solid var(--border-default);">

<!-- Button -->
<button class="px-6 py-3 rounded-lg font-medium" style="background: var(--info); color: white;">

<!-- Badge -->
<span class="px-3 py-1 rounded-full text-xs font-medium" style="background: var(--success-soft); color: var(--success);">

<!-- Input -->
<input class="w-full px-4 py-3 rounded-lg" style="background: var(--bg-elevated); border: 1px solid var(--border-default); color: var(--text-primary);">
```

---

<div align="center">

[← Back to Documentation](README.md)

</div>
