# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.5.0] - 2026-11-26

### Added
- **Multi-Format Design Token Exporters**:
  - SCSS variables (`specimen export --format scss`)
  - Tokens Studio JSON (`specimen export --format tokens-studio`)
  - Tailwind CSS v3 presets (`specimen export --format tailwind-v3`)
- **Cross-Source Similarity & Deduplication**:
  - Delta-E (CIE76/2000) palette overlap calculation
  - Content similarity links (`similar_to`) in canonical `item.json`
  - Merge review & side-by-side comparison view in web app (`/compare?a=<id>&b=<id>`)
- **Performance & Caching**:
  - In-memory index caching in `.cache/index.json`
  - Blazing fast search latency (< 2 ms over 1,000+ items, 840+ QPS)
  - `xtask bench` performance benchmark suite
- **CLI & UX Polish**:
  - `--dry-run` flag for `specimen capture` and `specimen crawl`
  - Multi-stage progress indicators (`[1/5] ... [5/5]`)
  - `specimen migrate` command for schema upgrades
  - Shell autocomplete completions (`specimen completions <shell>`)
- **Documentation**:
  - Comprehensive mdBook documentation site (`docs/book/`)
  - Threat model and security policy (`docs/SECURITY.md`)
  - Upgrade and schema migration guide (`docs/UPGRADE.md`)
- **Release Engineering**:
  - GitHub Actions CI workflow for Ubuntu, macOS, and Windows
  - GitHub Actions multi-target release packaging workflow

## [v0.4.0] - 2026-11-26

### Added
- **Crawl Driver (`specimen crawl`)**:
  - Breadth-first crawl with same-origin host isolation
  - Structural template signature deduplication
  - Crawl groups and consolidated master reports
- **Web App (`specimen serve`)**:
  - Zero-dependency HTML5 system UI powered by Axum
  - Invariant 8 loopback-only binding (`127.0.0.1:<port>`)
  - DNS rebinding mitigation and strict CSP headers
  - Collection browsers, item views, sources list, and live export sandbox
- **Model Context Protocol Server (`specimen mcp`)**:
  - Stdio JSON-RPC 2.0 protocol implementation
  - MCP tools: `search_library`, `get_item`, `list_collections`, `get_design_md`, `export_tokens`, `capture_url`
  - MCP resources: `design://library/overview`, `design://library/items/{id}`

## [v0.3.0] - 2026-11-26

### Added
- **Motion & Animation Extraction**:
  - Keyframe timing, easing curves, CSS transitions, and WAAPI records
- **Component State Extraction**:
  - Role classification (button, input, card, badge)
  - Interactive pseudo-state styles (:hover, :focus, :active, :disabled)
- **Interactive Flow Mode (`specimen capture --mode flow`)**:
  - Declarative YAML flow scripts
  - Interactive flow recording (`specimen record`)
  - Password masking and credential redaction in logs
- **8-Metric Extraction Accuracy Benchmark**:
  - 100% pass across all 8 ground truth checks in `xtask accuracy`

## [v0.2.0] - 2026-11-26

### Added
- **Effect & Elevation Extraction**:
  - Elevation drop shadows, blurs, and glassmorphism classification
- **Layout & Semantic Skeleton Extraction**:
  - Semantic sectioning (header, nav, main, footer, section)
  - Flexbox and CSS Grid layout heuristics
- **SVG Icon Sanitization & Fingerprinting**:
  - Script and event handler stripping
  - Geometry-normalized SHA-256 fingerprinting

## [v0.1.0] - 2026-11-26

### Added
- Initial release of Specimen CLI
- Single-page browser capture driver via CDP
- Palette extractor (WCAG contrast, OKLCH, roles)
- Typography extractor (font families, type scales)
- Canonical JSON serialization and content-hashed IDs
- CSS, Tailwind v4, and DTCG token exporters
- Invariant 8 loopback and URL/SSRF policy enforcement
