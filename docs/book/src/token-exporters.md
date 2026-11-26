# Exporting Design Tokens

Specimen exports extracted tokens into standard production formats:

## 1. CSS Custom Properties (`--format css`)
Generates standardized CSS variables organized into `:root` and `@media (prefers-color-scheme: dark)` blocks:

```css
:root {
  --color-background: #0b0d12;
  --color-accent: #7c5cff;
  --font-inter: 'Inter', sans-serif;
  --text-h1: 32px;
  --spacing-1: 8px;
  --radius-md: 8px;
}
```

## 2. Tailwind CSS v4 (`--format tailwind`)
Generates modern Tailwind v4 `@theme` blocks:

```css
@theme {
  --color-background: #0b0d12;
  --color-accent: #7c5cff;
  --font-inter: 'Inter', sans-serif;
  --text-h1: 32px;
  --spacing-1: 8px;
  --radius-md: 8px;
}
```

## 3. Tailwind CSS v3 (`--format tailwind-v3`)
Generates legacy Tailwind v3 JavaScript configuration presets (`module.exports = { theme: { extend: { ... } } }`).

## 4. SCSS Variables (`--format scss`)
Generates Sass variable definitions:

```scss
$color-background: #0b0d12;
$color-accent: #7c5cff;
$font-inter: 'Inter', sans-serif;
$text-h1: 32px;
$radius-md: 8px;
```

## 5. W3C Design Tokens Community Group (`--format dtcg`)
Generates standard DTCG compliant JSON token dictionaries:

```json
{
  "color": {
    "background": { "$value": "#0b0d12", "$type": "color" }
  }
}
```

## 6. Tokens Studio (`--format tokens-studio`)
Generates Figma Tokens / Tokens Studio compatible JSON with nested token groupings.
