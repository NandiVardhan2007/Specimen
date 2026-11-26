# Specimen

> **Capture the design. Keep the system.**

Specimen is a high-performance Rust CLI and local web workbench that captures the design of any website into a plain-file, git-friendly design library (icons, palettes, typography, themes, layouts, effects, animations, components), with a rich local web app, an MCP server, and zero-loss token exporters.

---

## Highlights

- **Pure Rust Extraction Engine**: 10 dedicated crates built with `#![forbid(unsafe_code)]` for rock-solid stability and speed.
- **Git-Friendly Plain Files**: Every captured element is stored as deterministic, canonical JSON with content-hashed IDs.
- **Craft Anything-Inspired Workbench**: High-density dark interface (`#0b0d12` canvas, `#7c5cff` electric purple accent) for visual inspection, token exports, and design comparison.
- **Zero Mock Data**: UI connects directly to live local libraries via the embedded Axum REST API.
- **Strict Security Guarantees**: Bound strictly to loopback (`127.0.0.1`), DNS rebinding protection, SSRF guards, private IP blocking, and SVG sanitization.
- **Model Context Protocol (MCP)**: Native stdio-based MCP server (`specimen mcp`) allowing AI assistants (Claude, Cursor, Antigravity) to query and capture design systems.

---

## Quick Start

### 1. Build and Verify Environment

```bash
# Verify environment and available browser instances
cargo run --bin specimen -- doctor

# Inspect CLI commands
cargo run --bin specimen -- --help
```

### 2. Capture a Website Design

```bash
# Capture a single page in headless mode (Chrome/Edge/Chromium)
cargo run --bin specimen -- capture https://example.com

# Capture in visible headed mode for manual sign-in / auth
cargo run --bin specimen -- capture https://dashboard.example.com --headed

# Crawl an entire website (bounded by domain and max depth)
cargo run --bin specimen -- crawl https://example.com --max-pages 10
```

### 3. Launch the Web Workbench

```bash
# Starts local web app on http://127.0.0.1:4000
cargo run --bin specimen -- serve --port 4000 --open
```

You can also run the frontend in standalone Vite dev mode during UI development:

```bash
cd frontend
npm install
npm run dev
```

---

## Workbench Features

The Specimen Workbench provides a dense, interactive control room for your captured design library:

- **Palettes**: OKLCH perceived lightness and chroma metrics, WCAG 2.1 AA/AAA contrast ratios against light/dark surfaces, and one-click hex copy.
- **Typography**: Modular font size scales, loaded web font family inspectors, and live editable specimen text sandbox.
- **Components**: Isolated HTML skeleton previews with interactive pseudo-state toggle (`:default`, `:hover`, `:focus`, `:active`, `:disabled`).
- **Icons**: Sanitized vector SVG gallery with light/dark backdrop toggle, geometry hash, and raw SVG copy/download.
- **Effects**: Visual elevation shadow stacks and glassmorphism backdrop blur recipe cards.
- **Animations**: CSS transition curves and `@keyframes` timeline playback visualizer.
- **Layouts**: Wireframe section breakdowns and responsive multi-device viewport switcher (Mobile, Tablet, Desktop).
- **Exporter**: Live export to CSS custom properties, Tailwind CSS (v4 & v3), SCSS variables, W3C DTCG tokens, and Tokens Studio for Figma.
- **Diff / Compare**: Side-by-side visual and property comparison of any two captured items.
- **Sources**: Complete provenance audit tracking domains, timestamps, and page manifests.

---

## Architecture

Specimen is organized into modular workspace crates:

- `crates/specimen-core`: Core schemas, item models, canonical JSON serializer, content IDs, and safe path traversal guards.
- `crates/specimen-capture`: Chrome CDP integration, interaction drivers, and rate limiting (the only network-impure crate).
- `crates/specimen-extract`: Pure extractors converting browser evidence into design items.
- `crates/specimen-library`: Library layout, filesystem persistence, full-text search index, and normalization.
- `crates/specimen-agent`: AI agent adapters, structured prompts, and injection validators.
- `crates/specimen-export`: Multi-format token generators (CSS, Tailwind v4/v3, SCSS, DTCG, Tokens Studio).
- `crates/specimen-server`: Axum-powered local HTTP API and embedded SPA static asset server.
- `crates/specimen-mcp`: Stdio-based Model Context Protocol (MCP) server for AI assistants.
- `crates/specimen-cli`: Unified CLI binary (`specimen`).
- `frontend/`: Production React 18, TypeScript, and Tailwind CSS web workbench.
- `xtask/`: Development automation, benchmarks, and accuracy verification harness.

---

## Development & Verification

Run the full automated check suite across all crates:

```bash
cargo run --bin xtask -- check
cargo run --bin xtask -- accuracy
cargo run --bin xtask -- bench
```

---

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
