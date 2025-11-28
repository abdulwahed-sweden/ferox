# Components

A comprehensive library of UI components built with the Ferox Theme System. All components are theme-aware and support RTL/LTR layouts.

## Table of Contents

- [Statistics Cards](#statistics-cards)
- [Data Tables](#data-tables)
- [Buttons](#buttons)
- [Badges](#badges)
- [Alerts](#alerts)
- [Form Inputs](#form-inputs)
- [Navigation](#navigation)

---

## Statistics Cards

### Basic Stats Card

```html
<div class="p-6 rounded-xl transition-all duration-300"
     style="background: var(--bg-surface); border: 1px solid var(--border-default);">

    <!-- Header -->
    <div class="flex items-center justify-between mb-4">
        <span class="text-sm font-medium" style="color: var(--text-secondary);">
            Total Users
        </span>
        <span class="w-10 h-10 rounded-lg flex items-center justify-center"
              style="background: var(--info-soft);">
            👥
        </span>
    </div>

    <!-- Value -->
    <div class="text-3xl font-bold mb-2"
         style="color: var(--text-primary); font-variant-numeric: tabular-nums;">
        12,847
    </div>

    <!-- Trend -->
    <div class="flex items-center gap-2">
        <span class="text-sm font-medium" style="color: var(--success);">
            ↑ +12.5%
        </span>
        <span class="text-sm" style="color: var(--text-muted);">
            vs last month
        </span>
    </div>
</div>
```

### Stats Card with Icon Variants

```html
<!-- Revenue Card -->
<div class="p-6 rounded-xl" style="background: var(--bg-surface); border: 1px solid var(--border-default);">
    <div class="flex items-center justify-between mb-4">
        <span style="color: var(--text-secondary);">Revenue</span>
        <span class="w-10 h-10 rounded-lg flex items-center justify-center"
              style="background: var(--success-soft);">💰</span>
    </div>
    <div class="text-3xl font-bold" style="color: var(--text-primary);">$48,290</div>
    <div style="color: var(--success);">↑ +8.2%</div>
</div>

<!-- Orders Card -->
<div class="p-6 rounded-xl" style="background: var(--bg-surface); border: 1px solid var(--border-default);">
    <div class="flex items-center justify-between mb-4">
        <span style="color: var(--text-secondary);">Orders</span>
        <span class="w-10 h-10 rounded-lg flex items-center justify-center"
              style="background: var(--warning-soft);">📦</span>
    </div>
    <div class="text-3xl font-bold" style="color: var(--text-primary);">1,429</div>
    <div style="color: var(--warning);">↓ -3.1%</div>
</div>
```

---

## Data Tables

### Basic Table

```html
<div class="rounded-xl overflow-hidden"
     style="background: var(--bg-surface); border: 1px solid var(--border-default);">

    <table class="w-full">
        <thead>
            <tr style="border-bottom: 1px solid var(--border-default);">
                <th class="px-6 py-4 text-left text-sm font-semibold"
                    style="color: var(--text-secondary);">
                    Name
                </th>
                <th class="px-6 py-4 text-left text-sm font-semibold"
                    style="color: var(--text-secondary);">
                    Status
                </th>
                <th class="px-6 py-4 text-right text-sm font-semibold"
                    style="color: var(--text-secondary);">
                    Amount
                </th>
            </tr>
        </thead>
        <tbody>
            <tr class="transition-colors duration-200 hover-row"
                style="border-bottom: 1px solid var(--border-subtle);">
                <td class="px-6 py-4" style="color: var(--text-primary);">
                    John Doe
                </td>
                <td class="px-6 py-4">
                    <span class="px-3 py-1 rounded-full text-xs font-medium"
                          style="background: var(--success-soft); color: var(--success);">
                        Active
                    </span>
                </td>
                <td class="px-6 py-4 text-right font-mono"
                    style="color: var(--text-primary);">
                    $1,250.00
                </td>
            </tr>
        </tbody>
    </table>
</div>
```

### RTL-Aware Table

For RTL languages, swap `text-left` with `text-right`:

```html
<!-- RTL Table Header -->
<th class="px-6 py-4 text-right">Name</th>  <!-- Right-aligned in RTL -->

<!-- LTR Table Header -->
<th class="px-6 py-4 text-left">Name</th>   <!-- Left-aligned in LTR -->
```

---

## Buttons

### Button Variants

```html
<!-- Primary Button -->
<button class="px-6 py-3 rounded-lg font-medium transition-all duration-200"
        style="background: var(--info); color: white;">
    Primary
</button>

<!-- Secondary Button -->
<button class="px-6 py-3 rounded-lg font-medium transition-all duration-200"
        style="background: var(--bg-elevated); color: var(--text-primary);
               border: 1px solid var(--border-default);">
    Secondary
</button>

<!-- Success Button -->
<button class="px-6 py-3 rounded-lg font-medium transition-all duration-200"
        style="background: var(--success); color: white;">
    Success
</button>

<!-- Warning Button -->
<button class="px-6 py-3 rounded-lg font-medium transition-all duration-200"
        style="background: var(--warning); color: white;">
    Warning
</button>

<!-- Danger Button -->
<button class="px-6 py-3 rounded-lg font-medium transition-all duration-200"
        style="background: var(--error); color: white;">
    Danger
</button>

<!-- Ghost Button -->
<button class="px-6 py-3 rounded-lg font-medium transition-all duration-200"
        style="background: transparent; color: var(--text-primary);
               border: 1px solid var(--border-default);">
    Ghost
</button>
```

### Button Sizes

```html
<!-- Small -->
<button class="px-4 py-2 text-sm rounded-md">Small</button>

<!-- Medium (default) -->
<button class="px-6 py-3 rounded-lg">Medium</button>

<!-- Large -->
<button class="px-8 py-4 text-lg rounded-xl">Large</button>
```

### Button with Icon

```html
<button class="px-6 py-3 rounded-lg font-medium flex items-center gap-2"
        style="background: var(--info); color: white;">
    <span>📥</span>
    <span>Download</span>
</button>
```

---

## Badges

### Status Badges

```html
<!-- Success Badge -->
<span class="px-3 py-1 rounded-full text-xs font-medium"
      style="background: var(--success-soft); color: var(--success);">
    Active
</span>

<!-- Warning Badge -->
<span class="px-3 py-1 rounded-full text-xs font-medium"
      style="background: var(--warning-soft); color: var(--warning);">
    Pending
</span>

<!-- Error Badge -->
<span class="px-3 py-1 rounded-full text-xs font-medium"
      style="background: var(--error-soft); color: var(--error);">
    Inactive
</span>

<!-- Info Badge -->
<span class="px-3 py-1 rounded-full text-xs font-medium"
      style="background: var(--info-soft); color: var(--info);">
    New
</span>
```

### Badge Sizes

```html
<!-- Small Badge -->
<span class="px-2 py-0.5 rounded text-xs">Small</span>

<!-- Medium Badge (default) -->
<span class="px-3 py-1 rounded-full text-xs">Medium</span>

<!-- Large Badge -->
<span class="px-4 py-1.5 rounded-full text-sm">Large</span>
```

---

## Alerts

### Alert Variants

```html
<!-- Info Alert -->
<div class="p-4 rounded-lg flex items-start gap-3"
     style="background: var(--info-soft); border: 1px solid var(--info);">
    <span>ℹ️</span>
    <div>
        <div class="font-medium" style="color: var(--info);">Information</div>
        <div class="text-sm" style="color: var(--info);">
            This is an informational message.
        </div>
    </div>
</div>

<!-- Success Alert -->
<div class="p-4 rounded-lg flex items-start gap-3"
     style="background: var(--success-soft); border: 1px solid var(--success);">
    <span>✅</span>
    <div>
        <div class="font-medium" style="color: var(--success);">Success</div>
        <div class="text-sm" style="color: var(--success);">
            Operation completed successfully.
        </div>
    </div>
</div>

<!-- Warning Alert -->
<div class="p-4 rounded-lg flex items-start gap-3"
     style="background: var(--warning-soft); border: 1px solid var(--warning);">
    <span>⚠️</span>
    <div>
        <div class="font-medium" style="color: var(--warning);">Warning</div>
        <div class="text-sm" style="color: var(--warning);">
            Please review before proceeding.
        </div>
    </div>
</div>

<!-- Error Alert -->
<div class="p-4 rounded-lg flex items-start gap-3"
     style="background: var(--error-soft); border: 1px solid var(--error);">
    <span>❌</span>
    <div>
        <div class="font-medium" style="color: var(--error);">Error</div>
        <div class="text-sm" style="color: var(--error);">
            Something went wrong. Please try again.
        </div>
    </div>
</div>
```

### Dismissible Alert

```html
<div class="p-4 rounded-lg flex items-start justify-between gap-3"
     style="background: var(--info-soft); border: 1px solid var(--info);">
    <div class="flex items-start gap-3">
        <span>ℹ️</span>
        <span style="color: var(--info);">This alert can be dismissed.</span>
    </div>
    <button onclick="this.parentElement.remove()"
            style="color: var(--info);">✕</button>
</div>
```

---

## Form Inputs

### Text Input

```html
<div class="space-y-2">
    <label class="block text-sm font-medium" style="color: var(--text-secondary);">
        Email Address
    </label>
    <input type="email"
           placeholder="you@example.com"
           class="w-full px-4 py-3 rounded-lg text-base transition-all duration-200"
           style="background: var(--bg-elevated);
                  border: 1px solid var(--border-default);
                  color: var(--text-primary);">
    <p class="text-sm" style="color: var(--text-muted);">
        We'll never share your email.
    </p>
</div>
```

### Input with Validation States

```html
<!-- Valid Input -->
<input type="text"
       class="w-full px-4 py-3 rounded-lg"
       style="background: var(--bg-elevated);
              border: 2px solid var(--success);
              color: var(--text-primary);">

<!-- Invalid Input -->
<input type="text"
       class="w-full px-4 py-3 rounded-lg"
       style="background: var(--bg-elevated);
              border: 2px solid var(--error);
              color: var(--text-primary);">
```

### Select Input

```html
<select class="w-full px-4 py-3 rounded-lg"
        style="background: var(--bg-elevated);
               border: 1px solid var(--border-default);
               color: var(--text-primary);">
    <option>Option 1</option>
    <option>Option 2</option>
    <option>Option 3</option>
</select>
```

### Textarea

```html
<textarea rows="4"
          placeholder="Enter your message..."
          class="w-full px-4 py-3 rounded-lg resize-none"
          style="background: var(--bg-elevated);
                 border: 1px solid var(--border-default);
                 color: var(--text-primary);"></textarea>
```

---

## Navigation

### Header Navigation

```html
<header class="sticky top-0 z-50 backdrop-blur-xl"
        style="background: rgba(18, 22, 31, 0.8);
               border-bottom: 1px solid var(--border-default);">
    <div class="container mx-auto px-6 py-4">
        <div class="flex items-center justify-between">
            <!-- Logo -->
            <div class="text-xl font-bold" style="color: var(--text-primary);">
                Logo
            </div>

            <!-- Nav Links -->
            <nav class="flex items-center gap-6">
                <a href="#" class="text-sm font-medium"
                   style="color: var(--text-primary);">Home</a>
                <a href="#" class="text-sm font-medium"
                   style="color: var(--text-secondary);">Features</a>
                <a href="#" class="text-sm font-medium"
                   style="color: var(--text-secondary);">Docs</a>
            </nav>

            <!-- Actions -->
            <div class="flex items-center gap-3">
                <button id="lang-toggle">AR/EN</button>
                <button id="theme-toggle">🌙</button>
            </div>
        </div>
    </div>
</header>
```

### Footer

```html
<footer style="background: var(--bg-surface);
               border-top: 1px solid var(--border-default);">
    <div class="container mx-auto px-6 py-8">
        <div class="flex items-center justify-between">
            <span style="color: var(--text-secondary);">
                © 2024 Your Company
            </span>
            <div class="flex items-center gap-4">
                <a href="#" style="color: var(--text-secondary);">Privacy</a>
                <a href="#" style="color: var(--text-secondary);">Terms</a>
            </div>
        </div>
    </div>
</footer>
```

---

## Related Documentation

- [CSS Variables](CSS_VARIABLES.md)
- [Colors](COLORS.md)
- [Typography](TYPOGRAPHY.md)

---

<div align="center">

[← Back to Documentation](README.md)

</div>
