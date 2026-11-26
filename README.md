# Specimen

> **Capture the design. Keep the system.**

Specimen is a Rust CLI that captures the design of a website into a plain-file, git-friendly design library (icons, palettes, typography, themes, layouts, effects, animations, components), with a local web app, an MCP server and token exporters.

## Quick Start

```bash
# Check environment and setup
cargo run --bin specimen -- doctor

# Inspect CLI options
cargo run --bin specimen -- --help
```

## Architecture

- `crates/specimen-core`: Core types, IDs, safe path validation, canonical JSON, config.
- `crates/specimen-capture`: Chrome CDP integration and interaction drivers (the only impure layer).
- `crates/specimen-extract`: Pure extractors transforming evidence into design items.
- `crates/specimen-library`: Library layout, filesystem persistence, index, and normalization.
- `crates/specimen-agent`: AI agent adapters and output validators.
- `crates/specimen-export`: Token exporters (CSS variables, Tailwind, DTCG, SCSS, Tokens Studio).
- `crates/specimen-server`: Local web application (`specimen serve`).
- `crates/specimen-mcp`: Model Context Protocol server (`specimen mcp`).
- `crates/specimen-cli`: The unified binary CLI.
- `xtask`: Developer automation.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
