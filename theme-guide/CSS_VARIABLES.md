# CSS Variables Reference

Complete reference for all CSS custom properties (design tokens) in the Ferox Theme System.

## Table of Contents

- [Background Colors](#background-colors)
- [Text Colors](#text-colors)
- [Border Colors](#border-colors)
- [Semantic Colors](#semantic-colors)
- [Typography](#typography)
- [Shadows](#shadows)
- [Usage Examples](#usage-examples)

---

## Background Colors

### Core Backgrounds

| Variable | Dark Mode | Light Mode | Usage |
|----------|-----------|------------|-------|
| `--bg-base` | `#12161F` | `#F4F5F7` | Page background |
| `--bg-surface` | `#1A1F2B` | `#FFFFFF` | Cards, containers |
| `--bg-elevated` | `#242A38` | `#FFFFFF` | Dropdowns, modals |

### Usage

```css
body {
    background: var(--bg-base);
}

.card {
    background: var(--bg-surface);
}

.dropdown {
    background: var(--bg-elevated);
}
```

---

## Text Colors

### Core Text

| Variable | Dark Mode | Light Mode | Usage |
|----------|-----------|------------|-------|
| `--text-primary` | `#F0F2F5` | `#111827` | Headings, body text |
| `--text-secondary` | `#9CA3AF` | `#6B7280` | Subtitles, labels |
| `--text-muted` | `#6B7280` | `#9CA3AF` | Hints, placeholders |

### Usage

```css
h1 {
    color: var(--text-primary);
}

.subtitle {
    color: var(--text-secondary);
}

.placeholder {
    color: var(--text-muted);
}
```

---

## Border Colors

### Core Borders

| Variable | Dark Mode | Light Mode | Usage |
|----------|-----------|------------|-------|
| `--border-default` | `#2D3748` | `#E5E7EB` | Card borders, dividers |
| `--border-subtle` | `#1F2937` | `#F3F4F6` | Subtle separators |

### Usage

```css
.card {
    border: 1px solid var(--border-default);
}

.divider {
    border-top: 1px solid var(--border-subtle);
}
```

---

## Semantic Colors

### Success (Green)

| Variable | Dark Mode | Light Mode | Usage |
|----------|-----------|------------|-------|
| `--success` | `#10B981` | `#059669` | Success buttons, icons |
| `--success-soft` | `rgba(16, 185, 129, 0.15)` | `rgba(5, 150, 105, 0.15)` | Success backgrounds |

### Warning (Orange/Yellow)

| Variable | Dark Mode | Light Mode | Usage |
|----------|-----------|------------|-------|
| `--warning` | `#F59E0B` | `#D97706` | Warning buttons, icons |
| `--warning-soft` | `rgba(245, 158, 11, 0.15)` | `rgba(217, 119, 6, 0.15)` | Warning backgrounds |

### Error (Red)

| Variable | Dark Mode | Light Mode | Usage |
|----------|-----------|------------|-------|
| `--error` | `#EF4444` | `#DC2626` | Error buttons, icons |
| `--error-soft` | `rgba(239, 68, 68, 0.15)` | `rgba(220, 38, 38, 0.15)` | Error backgrounds |

### Info (Blue)

| Variable | Dark Mode | Light Mode | Usage |
|----------|-----------|------------|-------|
| `--info` | `#3B82F6` | `#2563EB` | Info buttons, links |
| `--info-soft` | `rgba(59, 130, 246, 0.15)` | `rgba(37, 99, 235, 0.15)` | Info backgrounds |

### Usage Examples

```css
/* Success button */
.btn-success {
    background: var(--success);
    color: white;
}

/* Success alert */
.alert-success {
    background: var(--success-soft);
    color: var(--success);
    border: 1px solid var(--success);
}

/* Error text */
.error-message {
    color: var(--error);
}
```

---

## Typography

### Font Families

```css
/* Arabic text */
font-family: 'IBM Plex Sans Arabic', sans-serif;

/* English text */
font-family: 'Inter', sans-serif;

/* Code/monospace */
font-family: 'JetBrains Mono', monospace;
```

### Font Sizes

| Size | Value | Usage |
|------|-------|-------|
| Base | `16px` | Body text |
| Small | `14px` | Labels, captions |
| Large | `18px` | Emphasized text |
| XL | `20px` | Section headings |
| 2XL | `24px` | Page headings |
| 3XL | `30px` | Hero text |

### Font Weights

| Weight | Value | Usage |
|--------|-------|-------|
| Light | `300` | Subtle text |
| Regular | `400` | Body text |
| Medium | `500` | Emphasis |
| Semibold | `600` | Headings |
| Bold | `700` | Strong emphasis |

---

## Shadows

### Elevation Shadows

```css
/* Card shadow */
.shadow-card {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1),
                0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

/* Elevated shadow */
.shadow-elevated {
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1),
                0 4px 6px -2px rgba(0, 0, 0, 0.05);
}

/* Modal shadow */
.shadow-modal {
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
}
```

---

## Usage Examples

### Card Component

```html
<div class="p-6 rounded-xl"
     style="background: var(--bg-surface);
            border: 1px solid var(--border-default);">
    <h3 style="color: var(--text-primary);">Card Title</h3>
    <p style="color: var(--text-secondary);">Card description</p>
</div>
```

### Button Variants

```html
<!-- Primary -->
<button style="background: var(--info); color: white;">
    Primary
</button>

<!-- Success -->
<button style="background: var(--success); color: white;">
    Success
</button>

<!-- Warning -->
<button style="background: var(--warning); color: white;">
    Warning
</button>

<!-- Danger -->
<button style="background: var(--error); color: white;">
    Danger
</button>

<!-- Ghost -->
<button style="background: transparent;
               color: var(--text-primary);
               border: 1px solid var(--border-default);">
    Ghost
</button>
```

### Alert Messages

```html
<!-- Info Alert -->
<div style="background: var(--info-soft);
            color: var(--info);
            border: 1px solid var(--info);
            padding: 1rem;
            border-radius: 0.5rem;">
    This is an info message
</div>

<!-- Success Alert -->
<div style="background: var(--success-soft);
            color: var(--success);
            border: 1px solid var(--success);
            padding: 1rem;
            border-radius: 0.5rem;">
    Operation successful!
</div>
```

### Table Styling

```html
<table style="width: 100%; border-collapse: collapse;">
    <thead>
        <tr style="border-bottom: 1px solid var(--border-default);">
            <th style="color: var(--text-secondary); padding: 1rem;">
                Header
            </th>
        </tr>
    </thead>
    <tbody>
        <tr style="border-bottom: 1px solid var(--border-subtle);">
            <td style="color: var(--text-primary); padding: 1rem;">
                Data
            </td>
        </tr>
    </tbody>
</table>
```

---

## Complete Variable List

```css
:root {
    /* Backgrounds */
    --bg-base: /* theme dependent */;
    --bg-surface: /* theme dependent */;
    --bg-elevated: /* theme dependent */;

    /* Text */
    --text-primary: /* theme dependent */;
    --text-secondary: /* theme dependent */;
    --text-muted: /* theme dependent */;

    /* Borders */
    --border-default: /* theme dependent */;
    --border-subtle: /* theme dependent */;

    /* Semantic - Solid */
    --success: /* theme dependent */;
    --warning: /* theme dependent */;
    --error: /* theme dependent */;
    --info: /* theme dependent */;

    /* Semantic - Soft */
    --success-soft: /* theme dependent */;
    --warning-soft: /* theme dependent */;
    --error-soft: /* theme dependent */;
    --info-soft: /* theme dependent */;
}
```

---

## Related Documentation

- [Theme System](THEME_SYSTEM.md)
- [Colors](COLORS.md)
- [Typography](TYPOGRAPHY.md)

---

<div align="center">

[← Back to Documentation](README.md)

</div>
