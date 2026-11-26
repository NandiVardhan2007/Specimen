# Contributing to Specimen

Thank you for contributing to Specimen!

## Invariants to Preserve

1. **Measured values are immutable.** The AI stage only provides annotations (names, tags, roles).
2. **Capture is the only impure stage.** Extractors and normalizers operate on evidence files.
3. **Determinism.** Output IDs and `item.json` representations must be byte-reproducible.
4. **Plain files are the source of truth.** Indexes can be recreated at any time.
5. **Security and privacy.** Never commit credentials, cookies, or secrets to the library.

## Developer Workflow

```bash
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```
