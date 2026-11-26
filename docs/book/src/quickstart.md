# Quickstart Guide

Get up and running with Specimen in less than 2 minutes.

## 1. Verify Environment

Run `specimen doctor` to verify that your system has Google Chrome or Microsoft Edge installed and detect available agent CLIs:

```bash
specimen doctor
```

## 2. Initialize a Design Library

Initialize an empty Specimen library in the current directory or a new folder:

```bash
specimen init my-design-system
cd my-design-system
```

This creates the standard library skeleton:
- `collections/` (extracted design tokens: palettes, typography, components, etc.)
- `sources/` (raw capture evidence, DOM snapshots, network logs)
- `specimen.toml` (configuration)
- `DESIGN.md` (human-readable executive summary)

## 3. Capture a Website

Capture a public URL into the library:

```bash
specimen capture https://example.com
```

To preview the planned operations without issuing network requests or launching a browser, pass `--dry-run`:

```bash
specimen capture https://example.com --dry-run
```

## 4. Browse with the Web UI

Launch the loopback web UI on `http://127.0.0.1:4000`:

```bash
specimen serve --open
```

## 5. Export Tokens

Export extracted colors, fonts, and radii to your framework of choice:

```bash
# CSS Custom Properties
specimen export --format css

# Tailwind CSS v4 (@theme block)
specimen export --format tailwind

# Tailwind CSS v3 (module.exports)
specimen export --format tailwind-v3

# SCSS Variables
specimen export --format scss

# W3C Design Tokens Community Group (DTCG) JSON
specimen export --format dtcg

# Tokens Studio JSON
specimen export --format tokens-studio
```
