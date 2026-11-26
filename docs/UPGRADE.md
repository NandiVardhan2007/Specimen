# Upgrade & Migration Guide

This guide describes how to upgrade existing Specimen libraries across version boundaries.

## Upgrading to v0.5.0 / v1.0.0

### Schema Version 1 (`specimen.item/1`)
All extracted items now adhere to schema `specimen.item/1`. Key additions:
- Mandatory `license` object with `status` (`reusable`, `reference-only`, `unknown`) and `basis`.
- `similar_to` vector containing content similarity links.
- `provenance` array tracking capture source, viewport, scheme, and timestamps.

### Automated Migration Tool

To migrate an existing design library created with earlier versions:

```bash
specimen migrate /path/to/library
```

The migration command:
1. Validates each `collections/*/*/item.json` file.
2. Backs up the original item file as `item.json.bak`.
3. Adds default schema version headers, standard license blocks, and canonicalizes formatting.
4. Updates `.cache/index.json`.

After migration, run `specimen validate` to verify library integrity.
