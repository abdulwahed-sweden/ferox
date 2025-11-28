# Typography

Complete guide to the typography system in Ferox Theme System, including font families, sizes, weights, and best practices for multilingual text.

## Table of Contents

- [Font Families](#font-families)
- [Font Sizes](#font-sizes)
- [Font Weights](#font-weights)
- [Line Heights](#line-heights)
- [Number Display](#number-display)
- [RTL Typography](#rtl-typography)
- [Usage Examples](#usage-examples)

---

## Font Families

### Primary Fonts

| Font | Usage | Languages |
|------|-------|-----------|
| **IBM Plex Sans Arabic** | Body text, headings | Arabic |
| **Inter** | Body text, headings | English, Latin |
| **JetBrains Mono** | Code, numbers | All |

### Font Stack

```css
/* Arabic text */
.font-arabic {
    font-family: 'IBM Plex Sans Arabic', sans-serif;
}

/* English/Latin text */
.font-english {
    font-family: 'Inter', sans-serif;
}

/* Monospace/Code */
.font-mono {
    font-family: 'JetBrains Mono', monospace;
}

/* Default stack (auto-selects based on content) */
body {
    font-family: 'IBM Plex Sans Arabic', 'Inter', sans-serif;
}
```

### Loading Fonts

```html
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Sans+Arabic:wght@300;400;500;600;700&family=Inter:wght@300;400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
```

---

## Font Sizes

### Size Scale

| Name | Size | Tailwind | Usage |
|------|------|----------|-------|
| xs | 12px | `text-xs` | Badges, labels |
| sm | 14px | `text-sm` | Helper text, captions |
| base | 16px | `text-base` | Body text |
| lg | 18px | `text-lg` | Emphasized text |
| xl | 20px | `text-xl` | Section headings |
| 2xl | 24px | `text-2xl` | Page headings |
| 3xl | 30px | `text-3xl` | Hero text |
| 4xl | 36px | `text-4xl` | Large displays |
| 5xl | 48px | `text-5xl` | Extra large displays |

### Base Font Size

```css
html {
    font-size: 16px;  /* 1rem = 16px */
}
```

### Usage Examples

```html
<!-- Page heading -->
<h1 class="text-3xl font-bold">Page Title</h1>

<!-- Section heading -->
<h2 class="text-xl font-semibold">Section Title</h2>

<!-- Body text -->
<p class="text-base">Regular paragraph text.</p>

<!-- Small text -->
<span class="text-sm">Helper or caption text.</span>

<!-- Badge text -->
<span class="text-xs">Badge label</span>
```

---

## Font Weights

### Weight Scale

| Name | Value | Tailwind | Usage |
|------|-------|----------|-------|
| Light | 300 | `font-light` | Subtle text, large headings |
| Regular | 400 | `font-normal` | Body text |
| Medium | 500 | `font-medium` | Emphasis, buttons |
| Semibold | 600 | `font-semibold` | Headings, labels |
| Bold | 700 | `font-bold` | Strong emphasis |

### Usage Guidelines

```html
<!-- Light weight - large display text -->
<h1 class="text-5xl font-light">Welcome</h1>

<!-- Regular weight - body text -->
<p class="font-normal">This is regular body text.</p>

<!-- Medium weight - buttons, links -->
<button class="font-medium">Click Me</button>

<!-- Semibold weight - section headings -->
<h2 class="font-semibold">Section Title</h2>

<!-- Bold weight - important emphasis -->
<strong class="font-bold">Important!</strong>
```

---

## Line Heights

### Line Height Scale

| Name | Value | Tailwind | Usage |
|------|-------|----------|-------|
| None | 1 | `leading-none` | Single-line headings |
| Tight | 1.25 | `leading-tight` | Headings |
| Snug | 1.375 | `leading-snug` | Subheadings |
| Normal | 1.5 | `leading-normal` | Body text |
| Relaxed | 1.625 | `leading-relaxed` | Long-form content |
| Loose | 2 | `leading-loose` | Very open spacing |

### Usage Examples

```html
<!-- Heading with tight line height -->
<h1 class="text-4xl leading-tight">
    Multi-line<br>Heading Text
</h1>

<!-- Body text with normal line height -->
<p class="leading-normal">
    This paragraph has comfortable line spacing for easy reading.
</p>

<!-- Long-form content with relaxed line height -->
<article class="leading-relaxed">
    Extended content benefits from more generous line spacing...
</article>
```

---

## Number Display

### Tabular Numbers

For consistent number alignment in tables and statistics:

```css
.number-display {
    font-family: 'Inter', 'JetBrains Mono', monospace;
    font-variant-numeric: tabular-nums;
    direction: ltr;  /* Numbers always LTR, even in RTL layouts */
}
```

### Usage

```html
<!-- Statistics display -->
<div class="number-display text-3xl font-bold">
    12,847
</div>

<!-- Price display -->
<span class="number-display">
    $1,250.00
</span>

<!-- Table with numeric data -->
<td class="number-display text-right">
    99.99%
</td>
```

### Currency Formatting

```html
<!-- Arabic currency (RTL context) -->
<span class="number-display">1,250.00 ر.س</span>

<!-- English currency (LTR context) -->
<span class="number-display">$1,250.00</span>
```

---

## RTL Typography

### Direction-Aware Text

```css
/* RTL layout */
[dir="rtl"] {
    text-align: right;
}

/* LTR layout */
[dir="ltr"] {
    text-align: left;
}
```

### Switching Direction

```javascript
function setDirection(lang) {
    const html = document.documentElement;
    if (lang === 'ar') {
        html.setAttribute('dir', 'rtl');
        html.setAttribute('lang', 'ar');
    } else {
        html.setAttribute('dir', 'ltr');
        html.setAttribute('lang', 'en');
    }
}
```

### RTL-Specific Styles

```css
/* Mirror icons in RTL */
[dir="rtl"] .icon-arrow {
    transform: scaleX(-1);
}

/* Adjust spacing in RTL */
[dir="rtl"] .mr-4 {
    margin-right: 0;
    margin-left: 1rem;
}

/* Keep numbers LTR in RTL context */
[dir="rtl"] .number-display {
    direction: ltr;
    unicode-bidi: embed;
}
```

---

## Usage Examples

### Complete Heading Hierarchy

```html
<h1 class="text-3xl font-bold leading-tight"
    style="color: var(--text-primary);">
    Page Title
</h1>

<h2 class="text-xl font-semibold leading-snug"
    style="color: var(--text-primary);">
    Section Heading
</h2>

<h3 class="text-lg font-semibold"
    style="color: var(--text-primary);">
    Subsection Heading
</h3>

<h4 class="text-base font-medium"
    style="color: var(--text-secondary);">
    Minor Heading
</h4>
```

### Body Text Styles

```html
<!-- Primary body text -->
<p class="text-base font-normal leading-relaxed"
   style="color: var(--text-primary);">
    Main content paragraph.
</p>

<!-- Secondary/muted text -->
<p class="text-sm font-normal"
   style="color: var(--text-secondary);">
    Supporting information.
</p>

<!-- Helper text -->
<span class="text-sm"
      style="color: var(--text-muted);">
    Hint or additional context.
</span>
```

### Code and Monospace

```html
<!-- Inline code -->
<code class="px-2 py-1 rounded text-sm font-mono"
      style="background: var(--bg-elevated); color: var(--info);">
    const theme = 'dark';
</code>

<!-- Code block -->
<pre class="p-4 rounded-lg overflow-x-auto font-mono text-sm"
     style="background: var(--bg-elevated); color: var(--text-primary);">
function toggleTheme() {
    // ...
}
</pre>
```

### Mixed Language Content

```html
<!-- Arabic with English terms -->
<p dir="rtl" lang="ar" class="text-base">
    استخدم <code class="font-mono" dir="ltr">toggleTheme()</code> لتبديل السمة.
</p>

<!-- Numbers in RTL context -->
<p dir="rtl" lang="ar">
    إجمالي المبيعات: <span class="number-display">$12,847.50</span>
</p>
```

---

## Best Practices

### Do's

- Use semantic heading levels (h1 → h6)
- Apply consistent line heights
- Use `tabular-nums` for numeric data
- Keep numbers in LTR even in RTL layouts
- Use appropriate weights for hierarchy

### Don'ts

- Don't use too many font sizes on one page
- Don't mix font families unnecessarily
- Don't use very light weights for small text
- Don't ignore line height in long-form content
- Don't forget to load all required font weights

---

## Related Documentation

- [CSS Variables](CSS_VARIABLES.md)
- [Components](COMPONENTS.md)
- [I18N](I18N.md)

---

<div align="center">

[← Back to Documentation](README.md)

</div>
