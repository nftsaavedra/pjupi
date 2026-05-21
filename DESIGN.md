---
name: PJUPI
colors:
  primary: "#3b82f6"
  primary-dark: "#1e40af"
  primary-light: "#dbeafe"
  secondary: "#10b981"
  danger: "#ef4444"
  warning: "#f59e0b"
  surface-default: "#f8fafc"
  surface-card: "#ffffff"
  surface-primary-subtle: "#eff6ff"
  surface-primary-soft: "#dbeafe"
  surface-success-subtle: "#ecfdf5"
  surface-warning-subtle: "#fffbeb"
  surface-danger-subtle: "#fef2f2"
  surface-sidebar: "#0f172a"
  surface-sidebar-elevated: "#13233d"
  text-primary: "#1f2937"
  text-secondary: "#556274"
  text-inverted: "#ffffff"
  text-sidebar: "rgba(226, 232, 240, 0.82)"
  text-sidebar-muted: "rgba(226, 232, 240, 0.92)"
  text-sidebar-subtle: "rgba(219, 234, 254, 0.95)"
  text-blue-strong: "#1d4ed8"
  text-blue-deep: "#1e40af"
  text-blue-dark: "#1e3a8a"
  text-success: "#065f46"
  text-warning: "#92400e"
  text-danger-strong: "#b91c1c"
  text-danger-deep: "#991b1b"
  text-muted: "#64748b"
  text-subtle: "#475569"
  text-amber-dark: "#9a3412"
  text-amber: "#d97706"
  text-emerald: "#059669"
  border-default: "#e5e7eb"
  border-primary-light: "#bfdbfe"
  border-primary-soft: "#dbeafe"
  border-primary-medium: "#93c5fd"
  border-primary-strong: "#60a5fa"
  border-success: "#a7f3d0"
  border-warning: "#fde68a"
  border-warning-medium: "#fdba74"
  border-danger: "#fecaca"
  border-neutral: "#cbd5e1"
  border-sidebar: "rgba(148, 163, 184, 0.18)"
  overlay: "rgba(0, 0, 0, 0.5)"
  skeleton-start: "#e5edf7"
  skeleton-mid: "#f8fbff"

typography:
  body:
    fontFamily: "-apple-system, BlinkMacSystemFont, Segoe UI, Roboto, Oxygen, Ubuntu, Cantarell, sans-serif"
    fontSize: 16px
    fontWeight: 400
    lineHeight: 1.6
  app-title:
    fontFamily: "-apple-system, BlinkMacSystemFont, Segoe UI, Roboto, Oxygen, Ubuntu, Cantarell, sans-serif"
    fontSize: 2rem
    fontWeight: 700
    lineHeight: 1.1
    letterSpacing: -0.5px
  heading-h2:
    fontFamily: "-apple-system, BlinkMacSystemFont, Segoe UI, Roboto, Oxygen, Ubuntu, Cantarell, sans-serif"
    fontSize: 1.25rem
    fontWeight: 700
    lineHeight: 1.25
  heading-h3:
    fontFamily: "-apple-system, BlinkMacSystemFont, Segoe UI, Roboto, Oxygen, Ubuntu, Cantarell, sans-serif"
    fontSize: 1.14rem
    fontWeight: 700
    lineHeight: 1.25
  kpi-value:
    fontSize: 1.75rem
    fontWeight: 700
    lineHeight: 1
  form-label:
    fontSize: 0.9rem
    fontWeight: 600
    lineHeight: 1.35
  body-small:
    fontSize: 0.875rem
    fontWeight: 400
    lineHeight: 1.5
  button-primary:
    fontSize: 0.95rem
    fontWeight: 600
    lineHeight: 1.35
  button-small:
    fontSize: 0.85rem
    fontWeight: 500
    lineHeight: 1.35
  table-cell:
    fontSize: 0.95rem
    fontWeight: 400
    lineHeight: 1.5
  tooltip-sm:
    fontSize: 0.8rem
    fontWeight: 600
    lineHeight: 1.2
  sidebar-title:
    fontSize: 1.55rem
    fontWeight: 700
    lineHeight: 1.1
  module-title:
    fontSize: 1.36rem
    fontWeight: 700
    lineHeight: 1.1
  overline:
    fontSize: 0.72rem
    fontWeight: 700
    letterSpacing: 0.18em
    textTransform: uppercase
  caption:
    fontSize: 0.8rem
    fontWeight: 500
    lineHeight: 1.4
  status-chip:
    fontSize: 0.82rem
    fontWeight: 700
    lineHeight: 1.2

rounded:
  none: 0
  xs: 4px
  sm: 6px
  md: 8px
  lg: 10px
  xl: 12px
  "2xl": 14px
  "3xl": 16px
  "4xl": 18px
  "5xl": 20px
  full: 999px
  circle: "50%"
spacing:
  "0": 0
  "0_5": 0.125rem
  "1": 0.25rem
  "2": 0.5rem
  "3": 0.75rem
  "4": 1rem
  "5": 1.25rem
  "6": 1.5rem
  "8": 2rem
  "12": 3rem
components:
  card:
    backgroundColor: "{colors.surface-card}"
    rounded: "{rounded.xl}"
    shadow: "{shadows.md}"
    border: "1px solid {colors.border-default}"
    padding: 1.5rem
  button-primary:
    background: "linear-gradient(135deg, {colors.primary} 0%, {colors.primary-dark} 100%)"
    textColor: "{colors.text-inverted}"
    rounded: "{rounded.md}"
    padding: "0.75rem 1.5rem"
    fontWeight: 600
    fontSize: 0.95rem
    minHeight: 44px
    shadow: "{shadows.md}"
  button-secondary:
    backgroundColor: "{colors.border-default}"
    textColor: "{colors.text-primary}"
    rounded: "{rounded.md}"
    padding: "0.75rem 1.5rem"
    fontWeight: 600
    fontSize: 0.95rem
    minHeight: 44px
  button-edit:
    backgroundColor: "{colors.primary-light}"
    textColor: "{colors.primary}"
    rounded: "{rounded.sm}"
    padding: "0.5rem 1rem"
    fontWeight: 600
    fontSize: 0.9rem
  button-delete:
    backgroundColor: "#fee2e2"
    textColor: "{colors.danger}"
    rounded: "{rounded.sm}"
    padding: "0.5rem 1rem"
    fontWeight: 600
    fontSize: 0.9rem
  input:
    padding: "0.66rem 0.9rem"
    border: "1px solid {colors.border-default}"
    rounded: "{rounded.md}"
    fontSize: 0.92rem
    backgroundColor: "#ffffff"
  table:
    backgroundColor: "{colors.surface-card}"
    rounded: "{rounded.xl}"
    padding: 2rem
    shadow: "{shadows.md}"
    border: "1px solid {colors.border-default}"
  table-header:
    backgroundColor: "{colors.primary-light}"
    borderBottom: "2px solid {colors.primary}"
  modal-content:
    backgroundColor: "{colors.surface-card}"
    rounded: "{rounded.xl}"
    shadow: "{shadows.xl}"
  modal-header-form:
    background: "linear-gradient(135deg, {colors.surface-primary-subtle} 0%, #ffffff 100%)"
  tab-button:
    rounded: "{rounded['3xl']}"
    padding: "0.95rem 1rem"
    fontWeight: 600
    fontSize: 0.95rem
  tab-button-active:
    textColor: "#ffffff"
    background: "linear-gradient(135deg, rgba(59,130,246,0.28), rgba(29,78,216,0.2))"
  toast:
    rounded: "{rounded.lg}"
    padding: "0.75rem 0.9rem"
    shadow: "{shadows.lg}"
  skeleton:
    background: "linear-gradient(90deg, {colors.skeleton-start} 20%, {colors.skeleton-mid} 50%, {colors.skeleton-start} 80%)"
    rounded: "{rounded.xl}"
    animation: "shimmer 1.35s linear infinite"
  floating-tooltip:
    backgroundColor: "rgba(15, 23, 42, 0.96)"
    textColor: "#ffffff"
    shadow: "0 20px 40px -28px rgba(15, 23, 42, 0.9)"
  catalogo-card:
    backgroundColor: "{colors.surface-card}"
    rounded: "{rounded['3xl']}"
    padding: "1.25rem 1.35rem"
    border: "1px solid {colors.border-default}"
shadows:
  xs: "0 1px 2px 0 rgba(0, 0, 0, 0.05)"
  sm: "0 1px 2px 0 rgba(0, 0, 0, 0.05)"
  md: "0 4px 6px -1px rgba(0, 0, 0, 0.1)"
  lg: "0 10px 15px -3px rgba(0, 0, 0, 0.1)"
  xl: "0 20px 25px -5px rgba(0, 0, 0, 0.1)"
  sidebar: "18px 0 40px -32px rgba(15, 23, 42, 0.85)"
  tooltip: "0 20px 40px -28px rgba(15, 23, 42, 0.9)"
  brand-glow: "0 12px 26px -18px rgba(96, 165, 250, 0.9)"
  focus-ring: "0 0 0 4px rgba(96, 165, 250, 0.22)"
  focus-ring-light: "0 0 0 4px rgba(59, 130, 246, 0.16)"
  focus-ring-subtle: "0 0 0 3px rgba(59, 130, 246, 0.1)"
transitions:
  default: "all 0.3s cubic-bezier(0.4, 0, 0.2, 1)"
  fast: "all 0.18s ease"
  card: "all 0.2s ease"
  tab: "background 180ms ease, border-color 180ms ease, color 180ms ease, transform 180ms ease"
  modal: "0.22s cubic-bezier(0.22, 1, 0.36, 1)"
  fade: "0.2s ease-in"
  layout: "220ms cubic-bezier(0.22, 1, 0.36, 1)"
mediaQueries:
  mobile: 768px
  tablet: 1024px
  desktop-small: 1360px
  reduced-motion: prefers-reduced-motion
---

## Overview

PJUPI is a Tauri v2 + React desktop application for university research management
(UNF/UPIC). The design system is implemented in a single CSS file
(`src/App.css`, 4118 lines) using CSS custom properties on `:root` plus component
classes. No CSS-in-JS, no preprocessor -- all styles are vanilla CSS with one level
of variables.

The visual language is a **soft blue corporate palette** with high contrast between
a light content area and a dark sidebar. Cards use subtle gradients, generous
border-radius, and consistent box-shadows to create depth.

### Design pillars

- **Light workspace / dark navigation**: The sidebar uses `#0f172a` (slate-900) as
  base with gradient overlays; the content area uses `#f8fafc` (slate-50) as base.
- **Blue accent throughout**: Primary actions, focus rings, table headers, and
  interactive states all use the blue-500 family.
- **Cards as primary container**: Forms, charts, tables, KPIs, and settings panels
  all use the shared `.card` pattern (white bg, `border-radius: 12px`,
  `box-shadow: var(--shadow-md)`, `border: 1px solid var(--border-color)`).
- **Pill-shaped chips**: Tags, status badges, filter pills, and selected items
  use `border-radius: 999px` with soft blue backgrounds.
- **Hover feedback**: Interactive elements lift on hover (`translateY(-2px)` or
  `-4px`) with increased shadow depth.

---

## Colors

### CSS Variables (defined in `:root`)

| Token | Value | Usage |
|-------|-------|-------|
| `--primary-color` | `#3b82f6` (blue-500) | Primary actions, links, focus rings, icons |
| `--primary-dark` | `#1e40af` (blue-900) | Header gradient, auth card headings, KPI value color |
| `--primary-light` | `#dbeafe` (blue-100) | Table headers, hover states, badge backgrounds |
| `--secondary-color` | `#10b981` (emerald-500) | Available, minimal current usage |
| `--danger-color` | `#ef4444` (red-500) | Delete buttons, error states, required field marks |
| `--warning-color` | `#f59e0b` (amber-500) | Available, minimal current usage |
| `--bg-color` | `#f8fafc` (slate-50) | Page background, sidebar negative space area |
| `--card-bg` | `#ffffff` | All card/container surfaces |
| `--text-primary` | `#1f2937` (gray-800) | Body text, headings, form labels |
| `--text-secondary` | `#556274` (custom) | Supporting text, descriptions, placeholders |
| `--border-color` | `#e5e7eb` (gray-200) | Card borders, table borders, input borders |
| `--shadow-sm` | `0 1px 2px 0 rgba(0,0,0,0.05)` | Cards at rest, subtle elevation |
| `--shadow-md` | `0 4px 6px -1px rgba(0,0,0,0.1)` | Default card shadow, table container |
| `--shadow-lg` | `0 10px 15px -3px rgba(0,0,0,0.1)` | Hovered cards, modals, toasts, header |
| `--shadow-xl` | `0 20px 25px -5px rgba(0,0,0,0.1)` | Auth card, modal content |
| `--transition` | `all 0.3s cubic-bezier(0.4,0,0.2,1)` | Standard transition for interactive elements |
| `--border-radius` | `12px` | Default border-radius for cards and containers |

### Tokens Used But NOT Defined (BROKEN)

| Token | Where Used | Lines |
|-------|-----------|-------|
| `--primary` | CatalogosPanel CSS | 3938, 3940, 3978, 3996, 4041, 4065, 4102 |
| `--shadow-xs` | CatalogosPanel CSS | 3926, 3967 |

These cause silent fallback to browser defaults. `--primary` should resolve to
`#3b82f6` (same as `--primary-color`). `--shadow-xs` should be defined.

### Hardcoded Colors Not Using CSS Variables

The following hex values appear frequently but are not abstracted to variables:

| Hex | Frequency | Used For | Suggested Variable Name |
|-----|-----------|----------|------------------------|
| `#bfdbfe` | 30+ | Soft blue borders everywhere | `--border-primary-light` |
| `#1d4ed8` | 20+ | Blue text (active states, chips) | `--text-blue-strong` |
| `#eff6ff` | 15+ | Pale blue backgrounds | `--surface-primary-subtle` |
| `#1e40af` | 10+ | Same as `--primary-dark` | Replace with `var(--primary-dark)` |
| `#f8fafc` | 10+ | Same as `--bg-color` | Replace with `var(--bg-color)` |
| `#dbeafe` | 8+ | Same as `--primary-light` | Replace with `var(--primary-light)` |
| `#93c5fd` | 8+ | Hover/active border colors | `--border-primary-medium` |
| `#60a5fa` | 8+ | Brand mark gradient, accent borders | `--border-primary-strong` |
| `#1e3a8a` | 5+ | Deep blue text emphasis | `--text-blue-dark` |
| `#64748b` | 4+ | Muted labels, diff old text | `--text-muted` |
| `#475569` | 3+ | Subtle metadata text | `--text-subtle` |
| `#f3f4f6` | 3+ | Hover backgrounds | `--surface-subtle` |

### Status/Semantic Colors (Hardcoded)

| Context | Background | Text | Border |
|---------|-----------|------|--------|
| Success | `#d1fae5` / `#ecfdf5` | `#065f46` / `#059669` | `#a7f3d0` |
| Warning | `#fef3c7` / `#fffbeb` | `#92400e` / `#d97706` | `#fde68a` / `#fdba74` |
| Error/Danger | `#fef2f2` / `#fee2e2` | `#b91c1c` / `#991b1b` | `#fecaca` |

Note: Two different error text colors (`#b91c1c` vs `#991b1b`) are used inconsistently
between form validation errors and toast errors.

---

## Typography

### Font Stack

The application uses the **system font stack** -- no web fonts are loaded via
`index.html` or CSS `@font-face`:

```
-apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu',
'Cantarell', sans-serif
```

### Font Size Scale

All sizes are in `rem` units (base 16px):

| Size | Px equiv | Used Where |
|------|----------|------------|
| `0.66rem` | 10.6px | Brand marks (mini badge on icon) |
| `0.7rem` | 11.2px | Stat labels (catalogos) |
| `0.72rem` | 11.5px | Overlines/uppercase kickers (`.sidebar-kicker`) |
| `0.74rem` | 11.8px | Docente chip metadata |
| `0.75rem` | 12px | Small buttons (`.btn-small`), tags (`.linea-tag`) |
| `0.76rem` | 12.2px | Module aside kicker |
| `0.78rem` | 12.5px | Pill buttons, diff labels, count badges |
| `0.8rem` | 12.8px | Tooltips, captions, hints, descriptions |
| `0.82rem` | 13.1px | Sidebar user info, status chips, toast, pills |
| `0.83rem` | 13.3px | Form inline hints |
| `0.84rem` | 13.4px | Rich tooltip content |
| `0.85rem` | 13.6px | Small labels, session user name |
| `0.86rem` | 13.8px | Sidebar user name |
| `0.875rem` | 14px | Form fields, related entities, grupo cards |
| `0.88rem` | 14.1px | Modal descriptions, DNI labels |
| `0.9rem` | 14.4px | Form labels, KPI labels, badge labels |
| `0.92rem` | 14.7px | Sidebar subtitle, form inputs, toast messages |
| `0.95rem` | 15.2px | Tab buttons, table cells, button labels |
| `0.98rem` | 15.7px | Content module strong, settings nav label |
| `1rem` | 16px | Body text, confirm messages |
| `1.1rem` | 17.6px | Grupo card title, loading text |
| `1.14rem` | 18.2px | Table container h2 heading |
| `1.25rem` | 20px | Chart container h2, catalog header h2 |
| `1.35rem` | 21.6px | Catalog stat value |
| `1.36rem` | 21.8px | Module title (hero section) |
| `1.5rem` | 24px | Form card h2 |
| `1.55rem` | 24.8px | Sidebar title |
| `1.75rem` | 28px | KPI value numbers |
| `2rem` | 32px | App title, dashboard insight numbers, module aside |

### Font Weight Usage

| Weight | Usage Pattern |
|--------|--------------|
| `400` | Body text, subtitle descriptions, table cell body |
| `500` | Table data text, captions, sub-labels, inline hints |
| `600` | Form labels, buttons, badges, chips, tab buttons, section labels |
| `700` | All headings, KPI values, active states, status chips, overlines |
| `800` | Brand mark, sidebar avatar, diff change labels |

---

## Layout

### Shell Structure

```
+-- app-shell (CSS Grid: sidebar + workspace)
|   +-- app-sidebar (280px, dark theme, sticky)
|   +-- app-workspace (flex column)
|       +-- content-header (sticky, blurred backdrop)
|       +-- main-content (max-width 1400px, centered)
|           +-- tab-panel (fade-in animation)
```

### Key Grid Values

| Zone | Grid Columns | Gap |
|------|-------------|-----|
| App shell | `280px minmax(0, 1fr)` | -- |
| Collapsed shell | `104px minmax(0, 1fr)` | -- |
| Settings layout | `minmax(250px, 320px) minmax(0, 1fr)` | `1.5rem` |
| Docentes layout | `minmax(320px, 0.88fr) minmax(0, 1.42fr)` | `1.5rem` |
| Proyectos layout | `minmax(0, 1.5fr) minmax(260px, 0.7fr)` | `1.5rem` |
| Dashboard main | `minmax(0, 1.8fr) minmax(280px, 0.9fr)` | `1.5rem` |
| KPI grid | `repeat(auto-fit, minmax(250px, 1fr))` | `1.5rem` |
| Catalog grid | `repeat(auto-fill, minmax(300px, 1fr))` | `1rem` |
| Grupo grid | `repeat(auto-fill, minmax(350px, 1fr))` | `1.5rem` |

### Sidebar (Dark Theme Island)

- **Width**: `280px` (collapsed: `104px` for icon-only)
- **Height**: `100vh`, `position: sticky`
- **Background**: Radial gradient overlay + linear gradient
  `linear-gradient(180deg, #0f172a 0%, #13233d 52%, #10192d 100%)`
- **Border-right**: `1px solid rgba(148, 163, 184, 0.16)`
- **Shadow**: `18px 0 40px -32px rgba(15, 23, 42, 0.85)`
- **Padding**: `1.5rem 1.2rem`
- **Gap**: `1.5rem` between brand/nav/footer sections

### Content Area

- **Background**: Radial + linear gradient (`#f8fafc` to `#eef4fb`)
- **Max width**: `1400px` (shell mode: none, full width)
- **Padding**: `2rem` (tablet: `1.5rem`, mobile: `1rem`)

### Content Header

- **Position**: `sticky; top: 0; z-index: 40`
- **Padding**: `1.25rem 2rem 1rem`
- **Background**: `rgba(248, 250, 252, 0.78)` + `backdrop-filter: blur(14px)`
- **Bottom border**: `1px solid rgba(203, 213, 225, 0.75)`

### Spacing Scale in Use

Actual values from the CSS (frequencies in parentheses):

| Value | Pixels | Frequency | Recommendation |
|-------|--------|-----------|---------------|
| `0.1rem` | 1.6px | 1 | Non-standard |
| `0.12rem` | 1.9px | 3 | Non-standard |
| `0.15rem` | 2.4px | 1 | Non-standard |
| `0.18rem` | 2.9px | 1 | Non-standard |
| `0.2rem` | 3.2px | 3 | Non-standard |
| `0.22rem` | 3.5px | 3 | Non-standard |
| `0.25rem` | 4px | 5 | OK |
| `0.3rem` | 4.8px | 1 | Non-standard |
| `0.32rem` | 5.1px | 1 | Should be 0.25rem |
| `0.35rem` | 5.6px | 4 | Should be 0.25rem or 0.5rem |
| `0.38rem` | 6.1px | 2 | Should be 0.5rem |
| `0.4rem` | 6.4px | 3 | Non-standard |
| `0.42rem` | 6.7px | 1 | Non-standard |
| `0.45rem` | 7.2px | 7 | Should be 0.5rem |
| `0.5rem` | 8px | 17 | OK |
| `0.55rem` | 8.8px | 5 | Should be 0.5rem |
| `0.6rem` | 9.6px | 4 | Non-standard |
| `0.65rem` | 10.4px | 4 | Non-standard |
| `0.66rem` | 10.6px | 1 | Non-standard |
| `0.68rem` | 10.9px | 1 | Non-standard |
| `0.7rem` | 11.2px | 3 | Non-standard |
| `0.75rem` | 12px | 14 | OK |
| `0.8rem` | 12.8px | 5 | Should be 0.75rem |
| `0.85rem` | 13.6px | 12 | Should be 0.75rem or 1rem |
| `0.9rem` | 14.4px | 4 | Should be 1rem |
| `0.95rem` | 15.2px | 3 | Should be 1rem |
| `1rem` | 16px | 18 | OK |
| `1.1rem` | 17.6px | 1 | Non-standard |
| `1.25rem` | 20px | 7 | OK |
| `1.35rem` | 21.6px | 3 | Should be 1.5rem |
| `1.5rem` | 24px | 15 | OK |
| `2rem` | 32px | 11 | OK |
| `3rem` | 48px | 2 | OK |

---

## Elevation & Depth

### Shadow Scale

| Token | Value | Applied To |
|-------|-------|-----------|
| `--shadow-sm` | `0 1px 2px 0 rgba(0,0,0,0.05)` | Module hero, aside cards, settings header, grupo cards at rest |
| `--shadow-md` | `0 4px 6px -1px rgba(0,0,0,0.1)` | Default card, chart container, table container, form card, KPI card |
| `--shadow-lg` | `0 10px 15px -3px rgba(0,0,0,0.1)` | Hovered KPI cards, app header, toast items, buttons on hover |
| `--shadow-xl` | `0 20px 25px -5px rgba(0,0,0,0.1)` | Auth card, modal content |

### Custom Shadows (Not Tokenized)

| Description | Value | Lines |
|-------------|-------|-------|
| Sidebar right edge | `18px 0 40px -32px rgba(15, 23, 42, 0.85)` | 202 |
| Floating tooltip | `0 20px 40px -28px rgba(15, 23, 42, 0.9)` | 929 |
| Brand mark glow | `0 12px 26px -18px rgba(96, 165, 250, 0.9)` | 233 |
| Active role matrix card | `0 18px 34px -28px rgba(37, 99, 235, 0.55)` | 1542 |
| Active settings nav | `0 10px 24px -18px rgba(59, 130, 246, 0.9)` | 2707 |
| Active catalog pill | `0 4px 12px -6px rgba(59, 130, 246, 0.45)` | 3941 |
| Catalog card hover | `0 12px 28px -14px rgba(0, 0, 0, 0.1)` | 3973 |
| Field help trigger | `0 4px 12px -10px rgba(37, 99, 235, 0.9)` | 1914 |
| Global focus-visible | `0 0 0 4px rgba(96, 165, 250, 0.22)` | 82 |
| Input focus ring | `0 0 0 4px rgba(59, 130, 246, 0.16)` | 2322 |
| Form input focus | `0 0 0 3px rgba(59, 130, 246, 0.1)` | 3069 |
| Sidebar brand focus | `0 0 0 3px rgba(96, 165, 250, 0.14)` | 219 |
| Table row hover inset | `inset 0 0 0 1px var(--primary-color)` | 747 |
| Tab button active | `inset 0 1px 0 rgba(255, 255, 255, 0.08)` | 875 |
| Renacyt icon button | `inset 0 1px 0 rgba(255, 255, 255, 0.7)` | 2182 |

### Hover Lift Effect

| Element | Transform | Shadow Change |
|---------|-----------|---------------|
| KPI card | `translateY(-4px)` | `--shadow-md` -> `--shadow-lg`, blue border |
| Primary button | `translateY(-2px)` | `--shadow-md` -> `--shadow-lg` |
| Settings nav | `translateY(-2px)` | `--shadow-sm` -> `--shadow-md` |
| Catalog card | `translateY(-3px)` | `--shadow-xs` -> custom hover shadow |
| Table action button | `translateY(-1px)` | none -> `--shadow-sm` |
| Sidebar toggle | `translateY(-1px)` | none -> blue border |
| Grupo card | none | `--shadow-sm` -> `--shadow-md`, blue border |

---

## Shapes

### Border Radius Scale

| Level | Value | Usage Examples |
|-------|-------|---------------|
| `xs` | `4px` | Scrollbar thumb, small tags (`.linea-tag`), list items (`.linea-item`) |
| `sm` | `6px` | Small buttons (`btn-edit`, `btn-delete`, `btn-view`, `btn-small`), checkboxes, form inputs in related-entities |
| `md` | `8px` | Primary buttons (`btn-primary`, `btn-secondary`), form inputs (`.form-input`), related entity items |
| `lg` | `10px` | Table action buttons, form inline status, tooltip-sm, toast items, auth-note, renacyt detail items |
| `xl` | `12px` | **Default via `--border-radius`**: cards, modals, charts, tables, forms, KPI cards, catalogo detail view |
| `2xl` | `14px` | Rich tooltips, renacyt summary cards, sidebar toggle buttons |
| `3xl` | `16px` | Tab buttons (expanded), settings nav buttons, user avatars, catalogo summary cards |
| `4xl` | `18px` | Brand mark icon, collapsed tabs, skeleton charts, project diff cards |
| `5xl` | `20px` | Module hero, aside cards, insight cards, badges, sidebar brand, sidebar user card |
| `full` | `999px` | Pill shapes: status chips, content module chip, docente chips, category pills, refresh hint |
| `circle` | `50%` | Project number circles |

### Key Shape Rules

- **Cards**: `12px` (xl) radius, white background, `1px solid var(--border-color)` border
- **Inputs**: `8px` (md) radius, white background
- **Pills**: `999px` (full) radius for any status indicator, chip, or filter pill
- **Tab buttons**: `16px` (3xl) expanded, `18px` (4xl) collapsed
- **Modals**: `12px` (xl) radius, gradient header backgrounds
- **Sidebar elements**: `18-20px` (4xl-5xl) radius for brand mark, user card, tab buttons

---

## Components

### Buttons

| Class | Background | Text | Padding | Radius | Weight | Min H |
|-------|-----------|------|---------|--------|--------|-------|
| `.btn-primary` | `linear-gradient(135deg, blue-500, blue-900)` | white | `0.75rem 1.5rem` | `8px` | 600 (0.95rem) | 44px |
| `.btn-secondary` | `var(--border-color)` | `var(--text-primary)` | `0.75rem 1.5rem` | `8px` | 600 (0.95rem) | 44px |
| `.btn-edit` | `var(--primary-light)` | `var(--primary-color)` | `0.5rem 1rem` | `6px` | 600 (0.9rem) | 40px |
| `.btn-delete` | `#fee2e2` | `var(--danger-color)` | `0.5rem 1rem` | `6px` | 600 (0.9rem) | 40px |
| `.btn-view` | `linear-gradient(135deg, blue-500, blue-900)` | white | `0.5rem 1rem` | `6px` | 500 (0.85rem) | 40px |
| `.btn-small` | `var(--card-bg)` | `var(--text-primary)` | `0.375rem 0.75rem` | `6px` | 400 (0.75rem) | -- |
| `.btn-export` | Same as primary | Same | `1rem` (full width) | `8px` | 400 (1rem) | -- |

### Form Inputs

- **Base**: `.form-input` and native `select`
- **Padding**: `0.66rem 0.9rem`
- **Border**: `1px solid var(--border-color)`
- **Radius**: `8px` (md)
- **Font**: `0.92rem`, inherited font-family
- **Height**: `min-height: 40px`
- **Transition**: `var(--transition)`
- **Focus**: `outline: none`, border becomes `var(--primary-color)`, box-shadow `0 0 0 4px rgba(59, 130, 246, 0.16)`

### Form Groups

- **Label to input gap**: `0.32rem` (line 1806) -- but also overridden to `margin-bottom: 0` (line 1894) with separate rules
- **Label style**: `font-weight: 600`, `font-size: 0.9rem`, `color: var(--text-primary)`
- **Help text**: `font-size: 0.85rem`, `color: var(--text-secondary)`, top margin `0.25rem`
- **Required mark**: `color: var(--danger-color)`, `margin-left: 0.25rem`

### Modal / Dialog

- **Overlay**: Fixed full viewport, `rgba(0,0,0,0.5)`, z-index 1000, `fadeIn` animation
- **Content**: White bg, `12px` radius, `--shadow-xl`, max-width 600px, 90% width
- **Default border**: `1px solid #fde68a` (amber, for confirm dialogs)
- **Form modal border**: `#bfdbfe` (blue, for form modals)
- **Header gradient (confirm)**: `#fffbeb` -> `#fef3c7` (amber)
- **Header gradient (form)**: `#eff6ff` -> `#ffffff` (blue)
- **Header h2 (confirm)**: `color: #92400e`
- **Header h2 (form)**: `color: var(--primary-dark)`
- **Close button**: `2.25rem` x `2.25rem`, circular (`999px`), hover rotates blue
- **Body**: `padding: 1.5rem`, `overflow-y: auto`, custom scrollbar
- **Footer**: `padding: 1.5rem`, `border-top: 2px solid var(--border-color)`, right-aligned
- **Animation**: `modalPopIn 0.22s cubic-bezier(0.22, 1, 0.36, 1)`

### Toast Notifications

- **Container**: Fixed bottom-right, z-index 1200, `0.65rem` gap, max 420px width
- **Toast item**: Flex row, `10px` radius, `0.75rem 0.9rem` padding, `--shadow-lg`
- **Animation**: `toastIn 0.25s ease`
- **Type colors**:

| Type | Background | Text | Border |
|------|-----------|------|--------|
| Success | `#ecfdf5` | `#065f46` | `#a7f3d0` |
| Error | `#fef2f2` | `#991b1b` | `#fecaca` |
| Warning | `#fffbeb` | `#92400e` | `#fde68a` |
| Info | `#eff6ff` | `#1e40af` | `#bfdbfe` |

### Floating Tooltip (via @floating-ui/react)

- **Background**: `rgba(15, 23, 42, 0.96)` (near-opaque slate-900)
- **Text**: white
- **Shadow**: `0 20px 40px -28px rgba(15, 23, 42, 0.9)`
- **Arrow**: SVG filled with same background color
- **Sizes**:

| Size | Max-Width | Radius | Padding | Font |
|------|-----------|--------|---------|------|
| `sm` | 220px | `10px` | `0.45rem 0.68rem` | `0.8rem/600/1.2` |
| `md` | 260px | `12px` | `0.65rem 0.8rem` | `0.82rem/400/1.35` |
| `rich` | 280px | `14px` | `0.75rem 0.85rem` | `0.84rem/400/1.5` |

- **Behavior**: 90ms open delay, 60ms close delay, `safePolygon` hover, dismiss on blur

### Table

- **Container (`.table-container`)**: White bg, `12px` radius, `2rem` padding, `--shadow-md`, overflow-x auto
- **Table (`.table`)**: Full width, collapsed borders
- **Header (`.table thead`)**: `--primary-light` bg, `2px solid var(--primary-color)` bottom border
- **Header cells (`.table th`)**: `1rem` padding, `0.95rem/700`, `var(--primary-color)` text
- **Body cells (`.table td`)**: `1rem` padding, `0.95rem/400`
- **Row hover**: `--bg-color` background
- **Row focus-within**: `#eff6ff` background
- **Interactive variant (`.table-interactive`)**: Cursor pointer on rows, blue inset highlight on hover
- **Actions column**: `gap: 0.25rem`, nowrap, right-aligned

### Table Action Button

- **Size**: `2.5rem` x `2.5rem` square
- **Radius**: `10px` (lg)
- **Hover**: `translateY(-1px)` + `--shadow-sm`
- **Disabled**: No transform, no shadow
- **Icon**: Default 16px, via `AppIcon` wrapper
- **Tooltip**: Wrapped in `FloatingTooltip` with `size="sm"`, `placement="top"`

### Sidebar Tab Navigation

- **Container**: Column flex, `0.45rem` gap
- **Tab button (`.tab-button`)**: Full width, `0.95rem 1rem` padding, `16px` radius, `0.95rem/600`
- **Inactive text**: `rgba(226, 232, 240, 0.82)`
- **Hover**: White text, `rgba(255,255,255,0.08)` background, subtle blue border
- **Active**: White text, blue gradient bg (`rgba(59,130,246,0.28)` to `rgba(29,78,216,0.2)`), `rgba(147,197,253,0.28)` border, inset highlight
- **Collapsed**: `56px` square, centered icon only, `18px` radius, tooltip on hover via `FloatingTooltip`
- **Tab icon**: lucide-react, size 18

### Skeleton Loading

- **Base `.skeleton`**: `12px` radius, `overflow: hidden`
- **Shimmer gradient**: `linear-gradient(90deg, #e5edf7 20%, #f8fbff 50%, #e5edf7 80%)`
- **Animation**: `shimmer 1.35s linear infinite` (200% background sweep)
- **Variants**:

| Class | Dimensions | Notes |
|-------|-----------|-------|
| `.skeleton-line` | `0.95rem` height, 100% width | Standard text line |
| `.skeleton-line-strong` | `1.4rem` height, 55% width | Title placeholder |
| `.skeleton-line-soft` | 78% width | Secondary text |
| `.skeleton-title-sm/md/lg` | 28%/42%/56% width | Section headers |
| `.skeleton-circle` | `60px` x `60px`, `999px` radius | KPI/avatar placeholder |
| `.skeleton-input` | `46px` height, 100% width | Form input placeholder |
| `.skeleton-button` | `46px` height, 100% width | Button placeholder |
| `.skeleton-chart` | Various, `18px` radius | Chart placeholder (sm:180/md:240/lg:320px) |

### Status Chips & Badges

- **Status chip**: `999px` radius, `0.3rem 0.65rem` padding, `0.82rem/700`
  - Total: `#f3f4f6` bg / `#374151` text / `#d1d5db` border
  - Success: `#d1fae5` bg / `#065f46` text / `#a7f3d0` border
  - Warning: `#fef3c7` bg / `#92400e` text / `#fde68a` border
- **Badge**: `20px` radius, `0.25rem 0.75rem` padding, `0.85rem/600`
  - Default: `--primary-light` bg / `--primary-color` text
  - Success: `#d1fae5` bg / `#059669` text
  - Warning: `#fef3c7` bg / `#d97706` text
  - Info: `#dbeafe` bg / `#1d4ed8` text

### KPI Cards

- **Container**: `.kpi-card` -- white bg, `12px` radius, `1.5rem` padding, `--shadow-md`
- **Hover**: `translateY(-4px)`, `--shadow-lg`, blue border
- **Icon area**: `60px` min-width, `--primary-color`
- **Value**: `1.75rem/700`, `--primary-color`, `line-height: 1`
- **Label**: `0.9rem/500`, `--text-secondary`

### Catalogo Summary Cards

- **Card**: `16px` radius, `1.25rem 1.35rem` padding, white bg
- **Hover**: `translateY(-3px)`, custom shadow, blue border
- **Icon box**: `2.6rem` square, `12px` radius, blue gradient
- **Title**: `0.98rem/700`, `--text-primary`
- **Description**: `0.8rem/400`, `--text-secondary`
- **Stats bar**: `10px` radius, `#f8fafc` bg, `#f1f5f9` border
- **Stat value**: `1.35rem/700`, blue (or muted gray for inactive)
- **Stat label**: `0.7rem/500`, uppercase, `letter-spacing: 0.03em`
- **Hint text**: `0.78rem/600`, blue, slides in on hover (`translateX(-4px)` -> `translateX(0)`)

### Category Pills

- **Pill**: `100px` radius, `0.45rem 0.95rem` padding, `0.82rem/500`
- **Default**: White bg, `--text-secondary` text, `--border-color` border
- **Hover**: Primary text, blue border, `translateY(-1px)`, `--shadow-sm`
- **Active**: **USES BROKEN `--primary` variable** for bg/border, white text, custom shadow

---

## Motion

### Transition Overrides (beyond `var(--transition)`)

While most elements use the standard `--transition` (all 0.3s Material Design curve),
several components have their own transition overrides:

| Element | Properties | Duration | Easing |
|---------|-----------|----------|--------|
| App shell layout | `grid-template-columns` | 220ms | `cubic-bezier(0.22, 1, 0.36, 1)` |
| Sidebar | `padding, gap` | 220ms | `cubic-bezier(0.22, 1, 0.36, 1)` |
| Sidebar brand | `padding, gap, transform` | 220/180ms | mixed |
| Tab button | `background, border-color, color, transform` | 180ms | `ease` |
| Tab button copy | `opacity, transform` | 160/180ms | `ease` |
| Sidebar toggle | `transform, border-color, background, box-shadow` | 180ms | `ease` |
| Skip link | `top` | 160ms | `ease` |
| Content shell | `opacity, filter` | 200ms | `ease` |
| Catalog pills | `all` | 180ms | `ease` |
| Catalog cards | `all` | 200ms | `ease` |

### Keyframe Animations

| Name | Duration | Easing | Iteration | Effect |
|------|----------|--------|-----------|--------|
| `fadeIn` | 0.2s (overlay) / 0.3s (tab-panel) | `ease-in` | once | opacity 0->1, translateY 10px->0 |
| `modalPopIn` | 0.22s | `cubic-bezier(0.22,1,0.36,1)` | once | opacity + scale(0.98->1) + translateY |
| `toastIn` | 0.25s | `ease` | once | translateY(10px)->0 + translateX(10px)->0 |
| `shimmer` | 1.35s | `linear` | infinite | background-position sweep for skeletons |
| `pulseDot` | 1.2s | `ease-in-out` | infinite | opacity 0.45<->1, scale 0.92<->1 (refresh indicator) |

### Reduced Motion

```css
@media (prefers-reduced-motion: reduce) {
  .app-shell, .app-sidebar, .app-workspace,
  .sidebar-brand, .tab-button, .tab-button-copy,
  .content-sidebar-toggle, .sidebar-toggle {
    transition: none;
  }
}
```

---

## Do's and Don'ts

### Do

- Use `var(--primary-color)` for all blue accent elements (links, focus rings, icons)
- Use `var(--border-color)` for all structural borders between elements
- Use `var(--card-bg)` for all elevated container backgrounds
- Use `var(--text-primary)` for all body text and headings
- Use `var(--text-secondary)` for all supporting/description text
- Use `12px` border-radius for cards and containers (via `var(--border-radius)`)
- Use `999px` border-radius for pill-shaped elements (chips, status badges)
- Use `8px` border-radius for form inputs and primary buttons
- Use the system font stack (no custom fonts to load)
- Wrap icons in `<AppIcon>` with explicit `size` prop for consistency
- Use the `--transition` variable for interactive element transitions
- Provide `min-height` on buttons (44px primary, 40px secondary)

### Don't

- Do NOT hardcode hex colors -- use CSS variables from `:root` whenever possible
- Do NOT use `--primary` or `--shadow-xs` -- these variables are NOT defined (BUG)
- Do NOT use `--primary-dark` for text on dark backgrounds (it has insufficient contrast)
- Do NOT use different border-radius values for the same type of element
- Do NOT use `6px` and `8px` border-radius interchangeably for inputs -- pick one
- Do NOT duplicate `.form-group`, `.chart-container`, `.empty-state`, or `.btn-view` rules
- Do NOT use `#991b1b` and `#b91c1c` interchangeably for error text -- pick one
- Do NOT use raw `#1e40af` when `--primary-dark` is already available
- Do NOT use raw `#f8fafc` when `--bg-color` is already available
- Do NOT use raw `#dbeafe` when `--primary-light` is already available
- Do NOT forget to set `min-height` on interactive targets (44px recommended)

---

## Inconsistencies to Fix

### CRITICAL BUGS

1. **`--primary` variable not defined**: Used 7 times in catalogos section
   (lines 3938, 3940, 3978, 3996, 4041, 4065, 4102). Causes silent fallback
   to browser default. Fix: Add `--primary: #3b82f6;` to `:root` (alias for
   `--primary-color`).

2. **`--shadow-xs` variable not defined**: Used 2 times in catalogos section
   (lines 3926, 3967). Fix: Add to `:root` with value
   `0 1px 2px 0 rgba(0, 0, 0, 0.05)`.

### DUPLICATE CSS RULES

3. **`.form-group` defined twice** (lines 1803 and 3037):
   - First: `gap: 0.32rem`, label inline flex
   - Second: `margin-bottom: 1rem`, label display block with `margin-bottom: 0.5rem`
   - Fix: Merge into one rule. Use `gap: 0.5rem` consistently.

4. **`.chart-container` defined twice** (lines 1318 and 2436):
   - First: h2 `font-size: 1.3rem`
   - Second: h2 `font-size: 1.25rem`
   - Fix: Keep one. Use `font-size: 1.25rem` for consistency.

5. **`.empty-state` defined twice** (lines 2621 and 3257):
   - First: `padding: 2rem`, `font-style: italic`
   - Second: `padding: 3rem`, no italic
   - Fix: Merge into single rule.

6. **`.btn-view` defined twice** (lines 808 and 2419):
   - First: Full definition (gradient, padding, font, transition)
   - Second: Only `min-height: 40px`
   - Fix: Merge. Add `min-height` to first declaration.

### COLOR INCONSISTENCIES

7. **Two different error reds**: `#b91c1c` (red-700, form-inline-status-error,
   toast-error uses `#991b1b` (red-800).
   - Fix: Unify to `#b91c1c` and create `--text-danger` variable.

8. **Hardcoded `#1e40af` instead of `--primary-dark`**: Used in
   `form-inline-status-idle`, `form-inline-preview strong`, `renacyt-detail-label`,
   `field-help-trigger:hover`.
   - Fix: Replace all with `var(--primary-dark)`.

9. **Hardcoded `#f8fafc` instead of `--bg-color`**: Used in
   `docentes-selector-empty`, `form-modal-footer`, `catalogo-back-btn:hover`,
   various gradient stops.
   - Fix: Replace with `var(--bg-color)`.

10. **Hardcoded `#dbeafe` instead of `--primary-light`**: Used in
    `docente-chip:hover`, `project-docentes-trigger:hover`,
    `renacyt-inline-icon-button:hover`, `brand-mark-renacyt`, `settings-nav-icon`.
    - Fix: Replace with `var(--primary-light)`.

11. **Toast error color mismatch**: Toast `.toast-error` uses `#991b1b` (red-800)
    as text color while form validation errors use `#b91c1c` (red-700).
    - Fix: Unify to one error text color.

### SPACING INCONSISTENCIES

12. **Non-standard spacing values**: Many fractional `rem` values don't follow
    a clean 4px/8px grid:
    - `0.1rem`, `0.12rem`, `0.15rem`, `0.18rem`, `0.2rem`, `0.22rem`
    - `0.32rem` (should be `0.25rem`)
    - `0.35rem` (should be `0.25rem` or `0.5rem`)
    - `0.38rem` (should be `0.5rem`)
    - `0.42rem` (should be `0.5rem`)
    - `0.45rem` (should be `0.5rem`)
    - `0.55rem` (should be `0.5rem`)
    - `0.58rem` (should be `0.5rem`)
    - `0.66rem` (should be `0.5rem` or `0.75rem`)
    - `0.8rem` (should be `0.75rem`)
    - `0.85rem` (should be `0.75rem` or `1rem`)
    - `0.9rem` (should be `1rem`)
    - `0.95rem` (should be `1rem`)
    - `1.35rem` (should be `1.5rem`)
    - **Fix**: Standardize on `0.25rem / 0.5rem / 0.75rem / 1rem / 1.25rem / 1.5rem / 2rem / 3rem`.

### BORDER-RADIUS INCONSISTENCIES

13. **Input border-radius mismatch**: Main `.form-input` uses `8px` (md) but
    `.related-entity-form` inputs use `6px` (sm) and `.form-group input` uses `6px`.
    - Fix: All inputs should use `8px`.

14. **Badge vs status-chip shapes**: `.badge` uses `20px` (5xl) while
    `.status-chip` uses `999px` (full) -- both are pill-like but different.
    - Fix: Use `999px` for all pill/tag/chip elements.

### MISSING DESIGN VARIABLES (Recommended Additions)

15. **No semantic color variables for status states**: All success/warning/error
    colors are hardcoded hex values. Consider adding:
    ```css
    --status-success-bg: #d1fae5;
    --status-success-text: #065f46;
    --status-success-border: #a7f3d0;
    --status-warning-bg: #fef3c7;
    --status-warning-text: #92400e;
    --status-warning-border: #fde68a;
    --status-danger-bg: #fef2f2;
    --status-danger-text: #b91c1c;
    --status-danger-border: #fecaca;
    ```

16. **No sidebar text variables**: Sidebar text uses hardcoded `rgba()` values.
    For future dark mode support, consider:
    ```css
    --sidebar-bg: #0f172a;
    --sidebar-text: rgba(226, 232, 240, 0.82);
    --sidebar-text-muted: rgba(219, 234, 254, 0.95);
    ```

17. **No fast-transition variable**: Catalog components use hardcoded
    `all 0.18s ease` and `all 0.2s ease`. Consider:
    ```css
    --transition-fast: all 0.18s ease;
    --transition-card: all 0.2s ease;
    ```

---

## Responsive Behavior

| Breakpoint | Effect |
|------------|--------|
| `768px` | Root font-size -> 14px. All multi-column grids collapse to 1 column. Header padding reduced. Tab descriptions hidden. Chart labels 11px. Modal footer stacks vertically. |
| `<=900px` or `<=820px height` | Docente form grids, renacyt grids collapse to 1 column. |
| `1024px` | App shell collapses (sidebar on top, horizontal scroll). Content padding reduced. |
| `<=1360px and >=1025px` | Dashboard KPI grid 4-col -> 2-col. Settings layout collapses. |
| `720px` | Form input action groups stack vertically. |
| `860px` | Catalog grid collapses to 1 column. |
| `prefers-reduced-motion` | Shell, sidebar, workspace, tab transitions disabled. |

---

## Notes

- This DESIGN.md reflects the **current state** (v0.1.0) of PJUPI's CSS.
  All token values come directly from `src/App.css` (4118 lines).
- The project uses a single CSS file with no preprocessor, no CSS modules,
  and no CSS-in-JS. All styles are vanilla CSS.
- The YAML frontmatter documents both defined CSS variables and hardcoded
  hex values that recur frequently enough to warrant tokenization.
- `@floating-ui/react` is used for tooltips (renderless, CSS classes applied
  to its portal-rendered DOM).
- lucide-react icons are the sole icon source, used through the `AppIcon`
  wrapper component with default `size={18}` and `strokeWidth={2}`.
- No inline `style` props are used in shared components (only one occurrence
  in `ReportesTab.tsx` for minor layout tweaks and the `--skeleton-columns`
  custom property on `SkeletonTable`).
