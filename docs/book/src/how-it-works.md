# How Specimen Works

Specimen is designed around a three-stage functional pipeline:

```
┌──────────────┐      ┌───────────────┐      ┌─────────────────┐
│ 1. Capture   │ ───> │ 2. Extract    │ ───> │ 3. Library      │
│  (Impure)    │      │  (Pure Fn)    │      │  (Determinism)  │
└──────────────┘      └───────────────┘      └─────────────────┘
   Browser CDP           Raw Evidence          Canonical JSON
  HTML, CSS, Net         Token Structs         Sorted, Rounded
```

## 1. Capture Stage (Impure)
The only impure stage in Specimen is `specimen-capture`. It controls a local Chrome/Edge browser instance via the Chrome DevTools Protocol (CDP) over a loopback WebSocket. It executes interactions, simulates media queries (dark mode, mobile viewports), records network activity, and saves raw evidence into `sources/<host>/<timestamp>/`.

## 2. Extraction Stage (Pure)
`specimen-extract` contains pure functions that consume raw evidence and produce strongly-typed design token items:
- **Palette**: Colors, OKLCH conversion, WCAG APCA/2.1 contrast ratios.
- **Typography**: Font files, type scale inference, line heights.
- **Components**: Semantic roles (button, input, card, badge), interaction pseudo-states (:hover, :focus, :active, :disabled).
- **Icons**: SVG sanitization, malicious tag stripping, SHA-256 geometry fingerprinting.
- **Motion**: CSS `@keyframes`, WAAPI `.animate()`, transitions, timing functions.
- **Layouts**: Responsive grid systems and layout skeletons.

## 3. Normalization & Indexing (Pure)
`specimen-library` hashes canonical payloads into `<type>-<slug>-<hash8>` identifiers, computes cross-source similarity links, manages `.cache/index.json` caching, and generates `DESIGN.md`.
