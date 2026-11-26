# Design Library Structure

Specimen stores design systems as plain, human-readable files. No proprietary databases or lock-in:

```
my-design-system/
├── specimen.toml                 # Library configuration
├── DESIGN.md                     # Markdown summary of extracted tokens
├── collections/                  # Extracted and normalized items
│   ├── palettes/                 # Color palettes (OKLCH, hex, contrast)
│   ├── typography/               # Font families, scales, weights
│   ├── components/               # Button, input, card skeletons & states
│   ├── icons/                    # Sanitized SVGs and fingerprints
│   ├── effects/                  # Elevation shadows, blurs, gradients
│   ├── animations/               # Keyframe timing and easing curves
│   └── layouts/                  # Semantic wireframe grid structures
├── sources/                      # Raw capture evidence
│   └── example.com/
│       └── 2026-01-01T00-00-00Z/
│           ├── manifest.json
│           ├── DOM.html
│           ├── report.md
│           ├── screenshots/
│           └── css/
└── .cache/                       # In-memory search index cache (gitignored)
```

## Canonical Item Format

Each extracted element is saved in its own subdirectory as `item.json`:

```json
{
  "$schema": "specimen.item/1",
  "id": "palette-dark-core-a8f219c0",
  "item_type": "palette",
  "name": "Dark Core Palette",
  "slug": "dark-core",
  "confidence": 0.95,
  "license": {
    "status": "reusable",
    "basis": "Extracted uncopyrightable hex values and functional tokens"
  },
  "provenance": [
    {
      "source": "example.com/2026-01-01T00-00-00Z",
      "url": "https://example.com/",
      "viewport": "desktop",
      "scheme": "dark",
      "captured_at": "2026-01-01T00:00:00Z"
    }
  ],
  "similar_to": [],
  "tags": ["dark", "midnight", "tokens"],
  "data": { ... }
}
```

### Determinism & Content-Hashed IDs
IDs adhere to `<type>-<slug>-<hash8>`, where `hash8` is the first 8 characters of the SHA-256 hash computed over the canonicalized JSON payload. This ensures:
- If a website's palette changes, the ID changes.
- If the palette is identical, it deduplicates automatically and appends provenance.
