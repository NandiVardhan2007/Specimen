# Introduction to Specimen

**Specimen** is a fast, deterministic CLI tool and local agent server written in pure Rust that captures the living design system of any website into a plain-file, git-friendly design library.

Unlike tools that treat web scraping as ad-hoc screenshots, Specimen operates as an analytical instrument:
1. **Measured values are immutable**: Extractors produce hard physical evidence (colors, font files, OKLCH conversions, WAAPI keyframes, SVG paths) which are stored in plain JSON and markdown.
2. **Deterministic & Git-friendly**: All JSON is formatted canonically (sorted keys, fixed decimal rounding, LF newlines), ensuring clean `git diff`s.
3. **Loopback & Secure**: All local services (`specimen serve`, `specimen mcp`) bind strictly to `127.0.0.1` with DNS rebinding defenses, strict CSP, and `#![forbid(unsafe_code)]` workspace-wide.
4. **No Secrets Stored**: Passwords, authorization headers, and session tokens are redacted automatically during capture.
