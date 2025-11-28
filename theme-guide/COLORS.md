# Colors

Complete color palette reference for the Ferox Theme System, including semantic colors, backgrounds, and usage guidelines.

## Table of Contents

- [Color Philosophy](#color-philosophy)
- [Background Colors](#background-colors)
- [Text Colors](#text-colors)
- [Semantic Colors](#semantic-colors)
- [Border Colors](#border-colors)
- [Color Palette Display](#color-palette-display)
- [Accessibility](#accessibility)

---

## Color Philosophy

The Ferox color system is built on these principles:

1. **Semantic Meaning** — Colors convey meaning (success = green, error = red)
2. **Sufficient Contrast** — All text meets WCAG AA standards
3. **Theme Consistency** — Colors work harmoniously in both themes
4. **Soft Variants** — Each semantic color has a soft/background variant

---

## Background Colors

### Dark Theme

| Variable | Hex | Preview | Usage |
|----------|-----|---------|-------|
| `--bg-base` | `#12161F` | ![#12161F](https://via.placeholder.com/20/12161F/12161F) | Page background |
| `--bg-surface` | `#1A1F2B` | ![#1A1F2B](https://via.placeholder.com/20/1A1F2B/1A1F2B) | Cards, containers |
| `--bg-elevated` | `#242A38` | ![#242A38](https://via.placeholder.com/20/242A38/242A38) | Dropdowns, inputs |

### Light Theme

| Variable | Hex | Preview | Usage |
|----------|-----|---------|-------|
| `--bg-base` | `#F4F5F7` | ![#F4F5F7](https://via.placeholder.com/20/F4F5F7/F4F5F7) | Page background |
| `--bg-surface` | `#FFFFFF` | ![#FFFFFF](https://via.placeholder.com/20/FFFFFF/FFFFFF) | Cards, containers |
| `--bg-elevated` | `#FFFFFF` | ![#FFFFFF](https://via.placeholder.com/20/FFFFFF/FFFFFF) | Dropdowns, inputs |

### Usage

```css
body {
    background: var(--bg-base);
}

.card {
    background: var(--bg-surface);
}

.dropdown, .input {
    background: var(--bg-elevated);
}
```

---

## Text Colors

### Dark Theme

| Variable | Hex | Preview | Usage |
|----------|-----|---------|-------|
| `--text-primary` | `#F0F2F5` | ![#F0F2F5](https://via.placeholder.com/20/F0F2F5/F0F2F5) | Headings, body |
| `--text-secondary` | `#9CA3AF` | ![#9CA3AF](https://via.placeholder.com/20/9CA3AF/9CA3AF) | Labels, subtitles |
| `--text-muted` | `#6B7280` | ![#6B7280](https://via.placeholder.com/20/6B7280/6B7280) | Placeholders, hints |

### Light Theme

| Variable | Hex | Preview | Usage |
|----------|-----|---------|-------|
| `--text-primary` | `#111827` | ![#111827](https://via.placeholder.com/20/111827/111827) | Headings, body |
| `--text-secondary` | `#6B7280` | ![#6B7280](https://via.placeholder.com/20/6B7280/6B7280) | Labels, subtitles |
| `--text-muted` | `#9CA3AF` | ![#9CA3AF](https://via.placeholder.com/20/9CA3AF/9CA3AF) | Placeholders, hints |

### Usage

```css
h1, p {
    color: var(--text-primary);
}

label, .subtitle {
    color: var(--text-secondary);
}

::placeholder, .hint {
    color: var(--text-muted);
}
```

---

## Semantic Colors

### Success (Green)

| Theme | Solid | Soft |
|-------|-------|------|
| Dark | `#10B981` | `rgba(16, 185, 129, 0.15)` |
| Light | `#059669` | `rgba(5, 150, 105, 0.15)` |

**Usage:** Positive actions, confirmations, active states

```html
<!-- Success button -->
<button style="background: var(--success); color: white;">
    Confirm
</button>

<!-- Success badge -->
<span style="background: var(--success-soft); color: var(--success);">
    Active
</span>

<!-- Success alert -->
<div style="background: var(--success-soft); border: 1px solid var(--success); color: var(--success);">
    Operation successful!
</div>
```

### Warning (Orange/Amber)

| Theme | Solid | Soft |
|-------|-------|------|
| Dark | `#F59E0B` | `rgba(245, 158, 11, 0.15)` |
| Light | `#D97706` | `rgba(217, 119, 6, 0.15)` |

**Usage:** Caution states, pending actions, alerts

```html
<!-- Warning button -->
<button style="background: var(--warning); color: white;">
    Proceed with Caution
</button>

<!-- Warning badge -->
<span style="background: var(--warning-soft); color: var(--warning);">
    Pending
</span>
```

### Error (Red)

| Theme | Solid | Soft |
|-------|-------|------|
| Dark | `#EF4444` | `rgba(239, 68, 68, 0.15)` |
| Light | `#DC2626` | `rgba(220, 38, 38, 0.15)` |

**Usage:** Destructive actions, errors, failures

```html
<!-- Danger button -->
<button style="background: var(--error); color: white;">
    Delete
</button>

<!-- Error message -->
<span style="color: var(--error);">
    This field is required
</span>
```

### Info (Blue)

| Theme | Solid | Soft |
|-------|-------|------|
| Dark | `#3B82F6` | `rgba(59, 130, 246, 0.15)` |
| Light | `#2563EB` | `rgba(37, 99, 235, 0.15)` |

**Usage:** Primary actions, links, informational content

```html
<!-- Primary button -->
<button style="background: var(--info); color: white;">
    Submit
</button>

<!-- Link -->
<a style="color: var(--info);">Learn more</a>
```

---

## Border Colors

### Dark Theme

| Variable | Hex | Usage |
|----------|-----|-------|
| `--border-default` | `#2D3748` | Card borders, dividers |
| `--border-subtle` | `#1F2937` | Subtle separators |

### Light Theme

| Variable | Hex | Usage |
|----------|-----|-------|
| `--border-default` | `#E5E7EB` | Card borders, dividers |
| `--border-subtle` | `#F3F4F6` | Subtle separators |

### Usage

```css
.card {
    border: 1px solid var(--border-default);
}

.table-row {
    border-bottom: 1px solid var(--border-subtle);
}

.divider {
    border-top: 1px solid var(--border-default);
}
```

---

## Color Palette Display

### Complete Palette Grid

```html
<div class="grid grid-cols-4 gap-4">
    <!-- Backgrounds -->
    <div class="p-4 rounded" style="background: var(--bg-base);">
        <span style="color: var(--text-primary);">bg-base</span>
    </div>
    <div class="p-4 rounded" style="background: var(--bg-surface);">
        <span style="color: var(--text-primary);">bg-surface</span>
    </div>
    <div class="p-4 rounded" style="background: var(--bg-elevated);">
        <span style="color: var(--text-primary);">bg-elevated</span>
    </div>

    <!-- Semantic -->
    <div class="p-4 rounded" style="background: var(--success);">
        <span style="color: white;">success</span>
    </div>
    <div class="p-4 rounded" style="background: var(--warning);">
        <span style="color: white;">warning</span>
    </div>
    <div class="p-4 rounded" style="background: var(--error);">
        <span style="color: white;">error</span>
    </div>
    <div class="p-4 rounded" style="background: var(--info);">
        <span style="color: white;">info</span>
    </div>

    <!-- Soft variants -->
    <div class="p-4 rounded" style="background: var(--success-soft);">
        <span style="color: var(--success);">success-soft</span>
    </div>
    <div class="p-4 rounded" style="background: var(--warning-soft);">
        <span style="color: var(--warning);">warning-soft</span>
    </div>
    <div class="p-4 rounded" style="background: var(--error-soft);">
        <span style="color: var(--error);">error-soft</span>
    </div>
    <div class="p-4 rounded" style="background: var(--info-soft);">
        <span style="color: var(--info);">info-soft</span>
    </div>
</div>
```

---

## Accessibility

### Contrast Ratios

All text colors meet WCAG AA standards:

| Combination | Contrast | Rating |
|-------------|----------|--------|
| `--text-primary` on `--bg-base` | 15.2:1 (dark) / 16.1:1 (light) | AAA |
| `--text-secondary` on `--bg-base` | 6.8:1 (dark) / 4.6:1 (light) | AA |
| `--text-primary` on `--bg-surface` | 12.4:1 (dark) / 21:1 (light) | AAA |
| `--success` on `--success-soft` | 4.7:1 | AA |
| `--error` on `--error-soft` | 4.5:1 | AA |

### Testing Contrast

Use these tools to verify contrast:

1. [WebAIM Contrast Checker](https://webaim.org/resources/contrastchecker/)
2. [Contrast Ratio](https://contrast-ratio.com/)
3. Browser DevTools Accessibility panel

### Color Blindness Considerations

- Don't rely solely on color to convey information
- Use icons alongside colored badges
- Provide text labels for status indicators

```html
<!-- Good: Icon + Color + Text -->
<span style="background: var(--success-soft); color: var(--success);">
    ✓ Active
</span>

<!-- Bad: Color only -->
<span style="background: var(--success-soft); color: var(--success);">
    •
</span>
```

---

## Related Documentation

- [CSS Variables](CSS_VARIABLES.md)
- [Theme System](THEME_SYSTEM.md)
- [Dark/Light Mode](DARK_LIGHT_MODE.md)

---

<div align="center">

[← Back to Documentation](README.md)

</div>
