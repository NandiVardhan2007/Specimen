# Specimen: Functional Specification (SPEC)

| | |
|---|---|
| **Status** | Draft v0.1 |
| **Date** | 2026-11-25 |
| **Scope** | Behaviour, formats and interfaces of Specimen v0.1 to v0.5 (APK in v2 is outlined in section 14) |
| **Related** | [PRD.md](PRD.md) (why) · [TRD.md](TRD.md) (how it is built) · [APP_FLOW.md](APP_FLOW.md) · [AGENT.md](AGENT.md) · [TESTING_GUIDE.md](TESTING_GUIDE.md) · [IMPLEMENTATION-PLAN.md](IMPLEMENTATION-PLAN.md) |

Key words MUST, SHOULD and MAY follow RFC 2119.

---

## 1. Glossary

| Term | Meaning |
|---|---|
| **Library** | A folder (usually a git repo) that holds all Specimen data. |
| **Source** | One capture of one site at one time. Immutable once written. |
| **Capture** | The act of visiting a URL and recording evidence. |
| **Evidence** | Raw recorded data: DOM snapshots, stylesheets, computed styles, network log, animation records. |
| **Item** | One normalised, reusable thing (a palette, an icon, an effect...). |
| **Collection** | All items of one type, across all sources. |
| **Annotation** | Human or AI supplied metadata on an item (name, tags, description). Never changes measured values. |
| **Interpret stage** | The step where an AI agent adds annotations. Optional. |
| **Normalize stage** | The step that converts evidence into de-duplicated items. |
| **Provenance** | Where an item came from (source, URL, selector, viewport, scheme, time). |
| **License status** | `reusable`, `reference-only` or `unknown`. |

## 2. System overview

```
URL ──► CAPTURE ──► evidence/ ──► EXTRACT ──► raw items ──► INTERPRET (AI, optional)
                                                                 │
                       collections/ ◄── NORMALIZE ◄──────────────┘
                            │
        ┌───────────┬───────┴───────┬────────────┐
      CLI search   Web app       MCP server    Exporters
```

Rules that hold throughout:

1. **Capture is the only stage that uses the browser or the network.** Everything after it reads evidence from disk and can be re-run offline.
2. **Measured values are immutable.** AI may add names, tags, roles and descriptions only.
3. **Everything is plain files.** The search index is a disposable cache.

## 3. Library layout

```
design-library/
├── specimen.toml                   # library config (section 11)
├── README.md                       # auto-generated index of sources and counts
├── .gitignore                      # generated: .cache/, assets-restricted/, *.tmp
├── sources/
│   └── example.com/
│       └── 2026-11-25T101500Z/     # one capture
│           ├── manifest.json       # capture parameters, versions, timings, warnings
│           ├── DESIGN.md           # human and agent readable design system summary
│           ├── report.md           # limitations and warnings
│           ├── evidence/
│           │   ├── dom/            # DOM snapshots per viewport and scheme
│           │   ├── css/            # stylesheets as loaded (original and parsed)
│           │   ├── computed/       # computed style records
│           │   ├── animations/     # raw animation records and sampled traces
│           │   └── network.json    # request list (no bodies, no secrets)
│           ├── screenshots/
│           │   ├── desktop/ tablet/ mobile/
│           │   └── states/         # hover, focus, menu-open, scrolled
│           ├── assets/             # open-licensed assets (git friendly)
│           │   ├── fonts/ images/ svg/ video/ lottie/
│           └── assets-restricted/  # reference-only and unknown assets (gitignored)
├── collections/
│   ├── icons/ palettes/ typography/ themes/
│   ├── layouts/ effects/ animations/ components/
│   │   └── <item-id>/
│   │       ├── item.json
│   │       ├── preview.png          # and/or preview.webp, preview.svg
│   │       └── (type specific files, e.g. icon.svg)
├── exports/                        # generated token files
└── .cache/                         # search index and thumbnails (gitignored)
```

Rules:

- Source folder names use UTC time `YYYY-MM-DDTHHMMSSZ`. The host is lower-cased and punycode-decoded for the folder name; ports appear as `host_8080`.
- Files in `sources/` MUST NOT be modified after capture completes, except `DESIGN.md` and annotation files when `interpret` is re-run.
- Items in `collections/` are rebuilt from `sources/` by `specimen normalize`. Manual edits to `item.json` are allowed only inside the `annotations` and `user` objects, which normalisation preserves.
- JSON is written with stable key order, two-space indent, UTF-8, trailing newline.

## 4. Capture modes

### 4.1 `page` (default)

Visits one URL and records evidence. Steps:

1. Resolve and validate the URL (scheme `http` or `https`; private network addresses require `--allow-private`).
2. Check `robots.txt` (unless `--robots ignore`).
3. Launch the browser with a clean temporary profile.
4. For each viewport × color scheme: load, wait for settle, scroll through the page to trigger lazy content and scroll animations, record evidence, take screenshots.
5. Probe interactive elements for states (hover, focus, active) unless `--no-states`.
6. Observe animations during load, scroll and probing.
7. Write evidence, assets and manifest.
8. Run extractors → items → normalise.
9. Run the interpret stage unless `--no-ai`.
10. Write `DESIGN.md` and `report.md`.

**Settle rule:** the page is settled when no network requests are in flight for 750 ms and no layout shifts have been observed for 500 ms, with a hard timeout (default 60 s).

**Default viewports** (configurable):

| Name | Width × Height | DPR |
|---|---|---|
| mobile | 390 × 844 | 2 |
| tablet | 768 × 1024 | 2 |
| laptop | 1280 × 800 | 1 |
| desktop | 1920 × 1080 | 1 |

**Color schemes:** `light` and `dark`, using emulated `prefers-color-scheme`. If the page renders identically in both, only one scheme is stored and the report says so. Sites that switch theme by a class or attribute toggle MAY be probed by `flow` mode.

### 4.2 `flow`

Runs a YAML script before and during capture so that menus, modals, tabs and in-page states can be recorded. See section 5.

### 4.3 `crawl`

Follows links from the start URL.

| Flag | Default | Meaning |
|---|---|---|
| `--depth` | 1 | Maximum link depth from the start URL |
| `--max-pages` | 20 | Hard page cap |
| `--same-origin` | on | Only follow same-origin links |
| `--include` / `--exclude` | none | Glob patterns on URL path |
| `--rate-limit` | 1 request/s | Maximum request rate per host |
| `--dedupe-templates` | on | Skip pages whose DOM structure matches an already captured template (for example product detail pages) |

Each page is stored as a separate source under a shared crawl group (`sources/<host>/<time>/pages/<slug>/`). Normalisation de-duplicates across the group.

## 5. Flow script format

```yaml
# flow.yaml
version: 1
name: "Open mobile menu and pricing toggle"
start: https://example.com
viewport: mobile            # name from config, or WxH
scheme: dark                # light | dark | both
steps:
  - wait: { selector: "header", timeout_ms: 5000 }
  - screenshot: { name: "home-initial" }
  - hover: { selector: "nav a:first-child" }
  - screenshot: { name: "nav-hover" }
  - click: { selector: "button[aria-label='Menu']" }
  - wait: { ms: 400 }
  - screenshot: { name: "menu-open", full_page: false }
  - scroll: { to: "bottom", step_px: 400, pause_ms: 150 }
  - type: { selector: "input[type=email]", text: "name@example.com" }
  - press: { key: "Tab" }
  - click: { text: "Annual" }
  - capture: { label: "pricing-annual" }   # records DOM, styles and animations at this moment
```

Supported actions: `wait`, `hover`, `click`, `type`, `press`, `scroll`, `screenshot`, `capture`, `set_scheme`, `resize`, `eval` (disabled unless `--allow-eval`).

Rules:

- Steps run sequentially; a failing step aborts the flow unless `optional: true`.
- `type` MUST NOT be used with real credentials. The recorder warns when it sees a password field.
- `specimen record <url> -o flow.yaml` opens a headed browser and writes a script of the user's clicks, hovers, scrolls and key presses. Text typed into password fields is never recorded.

## 6. Extraction specifications

All extractors read evidence only. Each emits **raw items** with `data`, `provenance`, `confidence` and a first-pass `license`.

### 6.1 Palette (`palettes`)

- Collect every used color from computed styles of visible elements: `color`, `background-color`, `border-*-color`, `outline-color`, `fill`, `stroke`, `box-shadow` colors, gradient stops, and `caret-color`.
- Weight each color by rendered area and element count. Ignore fully transparent values.
- Convert to OKLCH. Merge colors within ΔE (OKLab distance) of 0.01 into one entry, keeping the most frequent original value.
- Assign **roles** by heuristics: `background` (largest area behind text), `surface` (card-like containers), `text`, `text-muted`, `border`, `accent` or `primary` (colors on interactive elements such as links and buttons), `success`, `warning`, `danger` (hue and class/ARIA hints). Each role has a confidence.
- Compute WCAG 2.x contrast ratios for each foreground/background pair actually observed (text on its effective background). Record AA/AAA pass flags.
- Include CSS custom properties found on `:root` that resolve to colors, with their names.

`data` shape:

```json
{
  "scheme": "dark",
  "colors": [
    {
      "role": "background",
      "name": null,
      "hex": "#0b0d12",
      "oklch": [0.17, 0.012, 265.0],
      "alpha": 1,
      "usage": { "area_pct": 41.2, "elements": 128 },
      "css_variables": ["--bg"]
    }
  ],
  "pairs": [
    { "fg": "#e8eaf0", "bg": "#0b0d12", "wcag_ratio": 15.1, "aa": true, "aaa": true }
  ]
}
```

### 6.2 Typography (`typography`)

- Find fonts actually used via `CSS.getPlatformFontsForNode` and the network log (resource type `Font`), and parse `@font-face` rules (family, weight, style, `unicode-range`, `font-display`, source URLs).
- Save loaded font files (WOFF, WOFF2, TTF, OTF), hash them (SHA-256) and read metadata: family, subfamily, version, copyright (name ID 0), manufacturer (8), designer (9), license description (13), license URL (14), embedding permissions (`OS/2.fsType`), and variable font axes (`fvar`).
- Record how the font was delivered (`self-hosted`, `google-fonts`, `adobe-fonts`, `other-cdn`, `system`).
- Build the **type scale** from computed styles of headings, body, small text, buttons and links: size, weight, line-height, letter-spacing, text-transform, family.
- Record system and fallback stacks as written in CSS.

`data` shape (abbreviated):

```json
{
  "families": [{
    "name": "Inter",
    "stack": ["Inter", "system-ui", "sans-serif"],
    "delivery": "self-hosted",
    "files": [{
      "file": "assets/fonts/inter-var.woff2", "format": "woff2", "sha256": "…",
      "weight": "100 900", "style": "normal", "variable_axes": [{"tag":"wght","min":100,"max":900}]
    }],
    "license_hints": { "name_table_license": "SIL Open Font License 1.1", "license_url": "…", "fs_type": 0 }
  }],
  "scale": [
    { "role": "h1", "family": "Inter", "size_px": 56, "weight": 700, "line_height": 1.1, "letter_spacing_em": -0.02 }
  ]
}
```

### 6.3 Spacing, radii, borders (stored in the `themes` item as tokens)

- Histogram of `margin`, `padding` and `gap` values; infer a base unit (for example 4 px or 8 px) and the observed scale.
- Histogram of `border-radius` and `border-width`.
- Breakpoints from `@media` conditions and from layout changes seen across viewports.

### 6.4 Themes (`themes`)

- One item per scheme (`light`, `dark`).
- Token groups: `color`, `typography`, `spacing`, `radius`, `border`, `shadow`, `breakpoint`, `motion` (durations and easings seen most often).
- CSS custom properties on `:root`, `html`, `body`, `[data-theme]` and `.dark` style selectors, with resolved values and the selector that scopes them.

### 6.5 Icons (`icons`)

Sources of icons:

| Kind | Detection |
|---|---|
| Inline SVG | `<svg>` elements below a size threshold or inside buttons, links, nav |
| Sprite | `<symbol>` referenced by `<use>` |
| Image | `<img>` or `<image>` pointing to `.svg` |
| CSS | `background-image` or `mask-image` using SVG or data URI |
| Icon font | Glyphs from fonts whose glyphs sit in the private-use area or whose family is a known icon font; glyph outlines are converted to SVG paths |

Processing:

1. Parse and normalise SVG (remove ids and scripts, convert shapes to paths, round coordinates to a fixed precision, normalise `viewBox`, detect `currentColor` use).
2. Hash the normalised geometry. Identical hash means the same icon.
3. Match against a bundled **fingerprint database** of open source icon sets (name, license, version). A match sets `set.name`, `set.license` and `license.status = reusable`.
4. Unmatched icons get `license.status = unknown` (or `reference-only` if the page or file declares restrictions).
5. Save `icon.svg` in the item folder and a rasterised preview.

### 6.6 Effects (`effects`)

Categories and the properties they come from:

| Category | Source properties |
|---|---|
| `shadow` | `box-shadow`, `text-shadow`, `drop-shadow()` |
| `gradient` | linear, radial, conic gradients in backgrounds, borders, text fills |
| `blur` / `glass` | `backdrop-filter`, `filter: blur()` |
| `border` | multi-layer borders, outlines, `border-image`, gradient borders |
| `filter` | `filter` functions other than blur |
| `mask` / `clip` | `mask-*`, `clip-path` |
| `blend` | `mix-blend-mode`, `background-blend-mode` |
| `transform` | static 3D transforms, perspective |

Rule-based **recipes** are applied before any AI step, for example:

- `glassmorphism`: `backdrop-filter: blur(≥8px)` + translucent background + 1 px light border.
- `neumorphism`: paired light and dark shadows on a surface close to its parent color.
- `elevation-scale`: ordered set of shadows used on stacked surfaces.
- `gradient-text`: gradient background clipped to text.

Each effect stores the exact CSS values, the element roles it was applied to, and a rendered preview.

### 6.7 Layouts (`layouts`)

- Walk the DOM from landmarks and large blocks to find **sections** (nav, hero, feature grid, pricing, testimonial, footer, and so on).
- For each section, record a **content-free skeleton**: tag or role, `display`, flex or grid properties, gap, alignment, sizing, relative positions, repeating-child patterns. Text is replaced with placeholders.
- Record behaviour across breakpoints (columns changing, stack order, hidden elements).
- Generate a wireframe SVG preview.

### 6.8 Animations (`animations`)

Layered strategy (details in TRD §8):

| Layer | Captures |
|---|---|
| CSS | `@keyframes` rules, `animation-*` and `transition-*` declarations, scroll-driven timelines (`animation-timeline`) |
| Runtime | `document.getAnimations()` and the DevTools Animation domain: running CSS animations, transitions and Web Animations API calls, with resolved keyframes |
| Traces | For motion not exposed above (for example GSAP or other JS libraries writing styles each frame), sampled computed values of `transform`, `opacity`, `clip-path` and similar per frame |
| Assets | Lottie JSON files and animated media (recorded, not decomposed) |
| Library hints | Detection of GSAP, Framer Motion, Lottie, Anime.js, three.js and so on, recorded in `driver` |

`data` shape:

```json
{
  "kind": "css-keyframes",
  "name": "fade-up",
  "driver": "css",
  "keyframes": [
    { "offset": 0, "props": { "opacity": "0", "transform": "translateY(16px)" }, "easing": "ease-out" },
    { "offset": 1, "props": { "opacity": "1", "transform": "none" } }
  ],
  "timing": { "duration_ms": 600, "delay_ms": 0, "easing": "cubic-bezier(0.16,1,0.3,1)",
              "iterations": 1, "direction": "normal", "fill": "both" },
  "trigger": { "type": "scroll-into-view", "selector": ".feature-card", "threshold": 0.2 },
  "targets": [{ "selector": ".feature-card", "role": "card" }],
  "stagger_ms": 80,
  "preview": { "clip": "preview.webp", "fps": 30, "duration_ms": 1200 }
}
```

Trigger types: `load`, `hover`, `focus`, `click`, `scroll-into-view`, `scroll-linked`, `timer`, `media`, `unknown`.

Canvas, WebGL and video content are recorded as clips and flagged `kind: "canvas-recording"` with `decomposed: false`.

### 6.9 Components (`components`)

- Cluster visible elements by a **style and structure signature** (display, size class, padding, radius, colors, typography, child pattern).
- Assign a role from tag, ARIA role and cluster traits: `button`, `link`, `input`, `select`, `checkbox`, `card`, `badge`, `tabs`, `modal`, `nav`, `table`, `tooltip`, `toast`, and so on.
- For interactive roles, record state styles (`default`, `hover`, `focus`, `active`, `disabled`) by forcing pseudo-states through the browser.
- Store a sanitised HTML skeleton (text replaced with placeholders, scripts and tracking attributes removed), computed style subset per state, and screenshots per state.
- Group variants (for example primary, secondary, ghost buttons).

## 7. Item format

Every item is stored as `collections/<type>/<id>/item.json`:

```json
{
  "schema": "specimen.item/1",
  "id": "effect-frosted-glass-card-9f3a1c2e",
  "type": "effect",
  "name": "Frosted glass card",
  "slug": "frosted-glass-card",
  "description": "Translucent surface with 16px backdrop blur and a hairline light border.",
  "tags": ["glassmorphism", "card", "dark"],
  "data": { },
  "provenance": [
    {
      "source": "example.com/2026-11-25T101500Z",
      "url": "https://example.com/",
      "selector": ".hero .card",
      "viewport": "desktop",
      "scheme": "dark",
      "captured_at": "2026-11-25T10:15:42Z"
    }
  ],
  "license": { "status": "reference-only", "basis": "proprietary site, no license declared", "spdx": null },
  "confidence": 0.95,
  "similar_to": [],
  "annotations": { "by": "claude-code", "at": "2026-11-25T10:18:03Z", "prompt_hash": "sha256:…" },
  "user": { "favorite": false, "notes": "" },
  "previews": ["preview.png"],
  "created_at": "2026-11-25T10:18:05Z",
  "updated_at": "2026-11-25T10:18:05Z"
}
```

### 7.1 IDs and de-duplication

- `id = <type>-<slug>-<hash8>` where `hash8` is the first 8 hex characters of a SHA-256 over the **canonical form** of `data` (stable key order, fixed rounding).
- The slug comes from the annotated name, or a deterministic fallback (`palette-1`, `glass-blur-16`) when no AI is used. Changing the slug does not change `hash8`.
- Same canonical `data` means the same item; its provenance list is extended, not duplicated.
- Near duplicates (colors within ΔE 0.02, icons with different normalised geometry but high raster similarity, effects differing by small numeric deltas) are **linked** through `similar_to`, never merged automatically.

### 7.2 Confidence

`confidence` is extraction confidence from 0 to 1, not AI confidence.

| Value | Meaning |
|---|---|
| 1.0 | Directly measured (computed value, loaded file) |
| 0.8 to 0.99 | Measured but classified by rules (for example effect recipe) |
| 0.5 to 0.79 | Inferred by heuristic (for example color roles, component clusters) |
| below 0.5 | Weak guess; shown with a warning, excluded from exports by default |

### 7.3 License status

| Status | When it is set |
|---|---|
| `reusable` | Matched a bundled open source set, or a font whose embedded license allows reuse, or a user override |
| `reference-only` | Proprietary or declared restricted; use as inspiration only |
| `unknown` | No information. Treated like `reference-only` for gitignore and export warnings |

Plain values such as individual colors, spacing numbers and CSS property values are measurements, not files. Their items are marked `reusable` with the basis `"plain value, not a distributed asset (not legal advice)"`. Specimen does not decide whether a combination of values amounts to a protected design. Complete visual designs, logos, trademarks, illustrations and fonts remain subject to their owners' rights, and the docs and UI MUST carry the "not legal advice" caveat wherever license status is shown.

## 8. `DESIGN.md` format

Generated per source. YAML front matter holds machine-readable summary; body is for humans and agents.

```markdown
---
specimen: 1
source: example.com
captured_at: 2026-11-25T10:15:42Z
url: https://example.com/
schemes: [light, dark]
viewports: [mobile, tablet, laptop, desktop]
---

# Design: example.com

## Overview
Short description (AI generated, or a template when --no-ai).

## Color
| Role | Value | Usage | Contrast notes |
…

## Typography
Font stacks, files, scale table.

## Spacing, radius, borders
Base unit, scale, radii.

## Layout and breakpoints
Section list, grid behaviour, breakpoints.

## Components
Buttons, inputs, cards... with state notes.

## Effects
Shadows, gradients, glass, and so on, with CSS values.

## Motion
Durations, easings, notable animations with triggers.

## Iconography
Style, sizes, sets detected.

## Imagery and media
Observed treatment (rounded, masked, overlays).

## Accessibility observations
Contrast failures, focus styles, reduced-motion handling.

## Limitations
What could not be captured.

## Item index
Links to items in collections/.
```

Rules:

- Every number and color in `DESIGN.md` MUST come from evidence or items. The `--no-ai` template contains no prose beyond headings and tables.
- AI-written prose is marked in front matter: `annotated_by: claude-code`.

## 9. CLI specification

Binary: `specimen`. Global flags: `--library <path>` (default: current directory or `$SPECIMEN_LIBRARY`), `--log-level`, `--log-json`, `--no-color`, `-q/--quiet`.

| Command | Purpose |
|---|---|
| `specimen init [path]` | Create a library (config, `.gitignore`, folders) |
| `specimen doctor` | Check browser, agent CLIs, write access, versions |
| `specimen capture <url>` | Capture and process a page (see below) |
| `specimen record <url> -o flow.yaml` | Record a flow in a headed browser |
| `specimen interpret <source>` | Re-run the AI interpret stage on an existing source |
| `specimen normalize [--source S]` | Rebuild collections from sources |
| `specimen list <type> [filters]` | List items |
| `specimen show <id>` | Print an item and its preview path |
| `specimen search <query>` | Search the library |
| `specimen export` | Export tokens |
| `specimen serve` | Local web app |
| `specimen mcp` | MCP server over stdio |
| `specimen sources list \| rm <source>` | Manage sources |
| `specimen validate` | Validate all files against schemas |
| `specimen migrate` | Migrate library to the current schema version |
| `specimen config get \| set` | Read and write `specimen.toml` |
| `specimen completions <shell>` | Shell completions |

### 9.1 `capture` flags

| Flag | Default | Description |
|---|---|---|
| `--mode page\|flow\|crawl` | `page` | Capture mode |
| `--flow <file>` | | Flow script (flow mode) |
| `--viewports a,b,c` | config | Viewport names or `WxH` |
| `--schemes light,dark` | config | Color schemes |
| `--states hover,focus,active` | all | Interactive states to probe; `--no-states` disables |
| `--depth`, `--max-pages`, `--include`, `--exclude`, `--same-origin` | see 4.3 | Crawl options |
| `--no-ai` | off | Skip interpret stage |
| `--agent <id>\|auto` | `auto` | Agent adapter (`claude`, `codex`, `antigravity`, `gemini`, `copilot`) |
| `--require-ai` | off | Fail if no agent is usable |
| `--headed` | off | Visible browser (for manual sign-in) |
| `--robots respect\|ignore` | `respect` | `robots.txt` policy |
| `--rate-limit <rps>` | 1 | Request rate |
| `--timeout <s>` | 60 | Per-page timeout |
| `--allow-private` | off | Allow localhost and private network addresses |
| `--redact-text` | off | Remove page text from stored evidence |
| `--user-agent` | default | Override UA string |
| `--browser <path>` | auto | Browser executable |
| `--dry-run` | off | Print the plan without capturing |
| `--json` | off | Machine-readable result on stdout |

### 9.2 Search syntax

Free text plus filters:

```
specimen search "glass card"
specimen search "type:effect tag:glassmorphism license:reusable"
specimen search "type:palette near:#3b82f6 delta:0.05 scheme:dark"
specimen search "type:animation duration:<400ms trigger:hover"
specimen search "source:example.com type:icon"
```

Filters: `type`, `tag`, `source`, `license`, `scheme`, `near` and `delta` (color proximity in OKLab), `duration`, `trigger`, `confidence`, `favorite`. Results are ranked by text relevance, then confidence.

### 9.3 Exit codes

| Code | Meaning |
|---|---|
| 0 | Success |
| 1 | Unspecified error |
| 2 | Usage error |
| 3 | Browser not found or failed to start |
| 4 | Navigation failed, blocked, or timed out |
| 5 | Disallowed by `robots.txt` |
| 6 | No usable AI agent (only when `--require-ai`) |
| 7 | Validation failure |
| 8 | Completed with warnings (partial capture) |

### 9.4 Output

Human output is a short summary (counts per collection, warnings, paths). `--json` prints the full result object, including the source path and item IDs.

## 10. Exporters

`specimen export --format <fmt> [--scope library|source:<id>|items:<id,id>] [--scheme light|dark|both] [--out path]`

Exports only include items with confidence ≥ 0.5 (override with `--min-confidence`) and warn when exporting items that are not `reusable`.

| Format | Output |
|---|---|
| `css` | `:root` custom properties; dark values under `@media (prefers-color-scheme: dark)` and `[data-theme="dark"]` |
| `tailwind` | Tailwind v4 CSS (`@theme` block) with `--tailwind-version 4` (default) or a v3 `tailwind.config.js` preset with `--tailwind-version 3` |
| `dtcg` | W3C Design Tokens Community Group JSON (`$value`, `$type`, `$description`), targeting the latest stable spec version at release time |
| `scss` | Variables and maps |
| `tokens-studio` | JSON for the Tokens Studio Figma plugin (token sets and `$themes`) |

Token mapping: colors → `color`; spacing, radii, sizes → `dimension`; font families → `fontFamily`; weights → `fontWeight`; durations → `duration`; easings → `cubicBezier`; shadows → `shadow`; gradients → `gradient`; type styles → `typography`; transitions → `transition`.

Token names are derived deterministically (for example `color.background.base`, `space.4`, `radius.md`) and can be overridden in `specimen.toml` under `[export.names]`.

## 11. Configuration (`specimen.toml`)

```toml
[library]
schema_version = 1
name = "My design library"

[capture]
viewports = [
  { name = "mobile",  width = 390,  height = 844, dpr = 2 },
  { name = "tablet",  width = 768,  height = 1024, dpr = 2 },
  { name = "laptop",  width = 1280, height = 800,  dpr = 1 },
  { name = "desktop", width = 1920, height = 1080, dpr = 1 },
]
schemes = ["light", "dark"]
robots = "respect"          # respect | ignore
rate_limit_rps = 1.0
timeout_s = 60
user_agent = "default"      # default appends " Specimen/<version>" to the browser UA

[ai]
agent = "auto"
require = false
timeout_s = 300
max_items_per_call = 40

[assets]
store = "all"               # all | open-licensed | none
restricted_dir = "assets-restricted"

[privacy]
redact_text = false
redact_headers = ["authorization", "cookie", "set-cookie", "x-api-key"]

[web]
host = "127.0.0.1"
port = 4747

[mcp]
allow_capture = false

[export.names]
# "color.primary" = "color.brand.500"
```

## 12. Local web app (`specimen serve`)

Binds to `127.0.0.1` only. Pages:

| Page | Content |
|---|---|
| Home | Counts per collection, recent sources, search box |
| Collection | Grid of previews with filters (tag, license, scheme, source, confidence) |
| Item detail | Preview, data, CSS snippet with copy button, provenance, license badge, similar items, export button |
| Palette view | Swatches with roles and contrast pairs; "find near this color" |
| Typography view | Live specimen of each font with the scale rendered |
| Animation view | Looping preview and a timing curve graph |
| Layout view | Wireframe with breakpoint switcher |
| Source detail | Manifest, `DESIGN.md` rendered, screenshots gallery, report |
| Compare | Side by side comparison of two sources or two items |
| Export | Choose scope and format, preview output, download |
| Settings | Read-only view of config; link to edit file |

The web app is read-mostly. Writes are limited to favorites, notes, tags and license overrides, saved into the `user` object in `item.json`.

## 13. MCP server (`specimen mcp`)

Transport: stdio (HTTP optional later). Read-only by default.

| Tool | Input | Output |
|---|---|---|
| `search_library` | `query`, `limit` | Item summaries |
| `list_collections` | none | Types and counts |
| `get_item` | `id` | Full item JSON |
| `get_palette` | `id` or `query` | Palette with roles and contrast |
| `get_typography` | `id` or `query` | Families, scale, license hints |
| `get_animation` | `id` | Keyframes, timing, trigger |
| `get_layout` | `id` | Skeleton JSON |
| `get_design_md` | `source` | `DESIGN.md` text |
| `export_tokens` | `format`, `scope` | Token file content |
| `list_sources` | none | Sources and capture dates |
| `capture_url` | `url`, `mode` | Disabled unless `[mcp] allow_capture = true` |

Resources: `specimen://item/<id>`, `specimen://source/<id>/design-md`.

Responses are size-bounded (default 64 KB); larger content returns a summary and a path.

## 14. APK capture (v2 outline)

Input: `.apk` path or a package installed on a connected emulator or device.

- **Static:** decode resources (colors, styles, themes, dimens, fonts, drawables, vector drawables converted to SVG, animation XML, Lottie assets, layout XML).
- **Dynamic:** run on an emulator, take screenshots, dump the UI hierarchy, record screen video of transitions.
- **Framework-aware limits:** Flutter and React Native apps keep most UI in compiled bundles; Specimen records assets, fonts and screenshots and reports that layout and style extraction is partial.
- Output uses the same library structure and item types.
- Only for apps the user is entitled to analyse.

## 15. Errors and warnings

Errors are structured (`code`, `message`, `hint`, `source`), printed as one line plus a hint, and included in `manifest.json`. Warnings never abort a capture; they set exit code 8 when any item or asset could not be captured.

Common warnings: `W-BLOCKED` (resource blocked), `W-CANVAS` (canvas content not decomposed), `W-IFRAME` (cross-origin frame partially captured), `W-FONT-LICENSE` (font license unknown), `W-AGENT-SKIPPED`, `W-LOW-CONFIDENCE`.

## 16. Versioning and compatibility

- Library schema version in `specimen.toml` and in each file (`specimen.item/1`).
- Breaking schema changes bump the major schema number and ship a `specimen migrate` step. Old libraries remain readable for at least one minor release.
- CLI follows semantic versioning after 1.0. Before 1.0, minor versions may break the CLI with release notes.

## 17. Non-functional requirements (summary)

| Area | Requirement |
|---|---|
| Performance | Typical page capture under 60 s; library of 10,000 items searched in under 100 ms (in-memory index) |
| Footprint | Single binary under 40 MB (without browser); no runtime besides a Chromium-family browser |
| Privacy | No telemetry; no network calls except the captured site, optional browser download and the user's chosen agent |
| Security | See TRD §11 and AGENT.md §B.6 |
| Portability | macOS, Linux, Windows; paths and line endings handled per platform |
| Accessibility | Web app meets WCAG 2.2 AA for its own UI |
| Determinism | Same evidence gives byte-identical items and IDs |
