# Working on Specimen (Agent and Contributor Guide)

Specimen is a Rust CLI that captures the design of a website into a plain-file, git-friendly design library.

## Architecture Map

- `crates/specimen-core`: types, schemas, errors, config, canonical JSON, IDs.
- `crates/specimen-capture`: browser lifecycle, CDP session, interaction drivers (ONLY crate touching browser/network).
- `crates/specimen-extract`: pure extractors (Evidence -> RawItem).
- `crates/specimen-library`: filesystem layout, writers, normalize, dedupe, index.
- `crates/specimen-agent`: agent adapters, prompt builder, output validators.
- `crates/specimen-export`: css, tailwind, dtcg, scss, tokens-studio.
- `crates/specimen-server`: local web app (axum, askama, htmx).
- `crates/specimen-mcp`: MCP server.
- `crates/specimen-cli`: the unified `specimen` binary.
- `fixtures/`: deterministic fixture sites and ground-truth manifests.
- `schemas/`: generated JSON Schemas.
- `xtask/`: development automation tasks.

## Invariants

1. **Measured values are immutable**: AI interpretation only adds annotations.
2. **Capture is the only impure stage**: Extractors are pure functions over stored evidence.
3. **Determinism**: Canonical JSON, sorted BTreeMaps, fixed rounding, content-hashed IDs.
4. **Plain files are the source of truth**: Disposable cache and indexes.
5. **No secrets in the library**: No cookies, tokens, credentials, or sessions stored.
6. **License tracking**: License status is mandatory for every item.
7. **No evasion**: No bot bypass, CAPTCHA solving, or stealth spoofing.
8. **Loopback only**: Servers bind strictly to `127.0.0.1`.
9. **Paths are validated**: Use `RelPath` with prefix checks.

## Verification Workflow

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```
