# Specimen: Application Flow (APP_FLOW)

| | |
|---|---|
| **Status** | Draft v0.1 |
| **Date** | 2026-11-25 |
| **Purpose** | Describe how users and the system move through Specimen: first run, capture, interpretation, browsing, MCP, export, and failure paths |
| **Related** | [PRD.md](PRD.md) · [SPEC.md](SPEC.md) · [TRD.md](TRD.md) · [AGENT.md](AGENT.md) · [TESTING_GUIDE.md](TESTING_GUIDE.md) · [IMPLEMENTATION-PLAN.md](IMPLEMENTATION-PLAN.md) |

Diagrams use Mermaid.

---

## 1. Map of all flows

| # | Flow | Actor | Entry point |
|---|---|---|---|
| 1 | First run and setup | Developer | `specimen init`, `specimen doctor` |
| 2 | Page capture | Developer | `specimen capture <url>` |
| 3 | Flow capture (scripted or recorded) | Developer | `specimen record`, `specimen capture --mode flow` |
| 4 | Crawl capture | Developer | `specimen capture --mode crawl` |
| 5 | Interpret stage (AI) | System, agent | automatic or `specimen interpret` |
| 6 | Normalize and de-duplicate | System | automatic or `specimen normalize` |
| 7 | Search and inspect | Developer | `specimen search`, `list`, `show` |
| 8 | Browse in the web app | Developer | `specimen serve` |
| 9 | Agent queries via MCP | AI agent | `specimen mcp` |
| 10 | Export tokens | Developer | `specimen export` |
| 11 | Share the library with a team | Developer | git |
| 12 | Failure and recovery paths | All | various |
| 13 | APK capture (v2) | Developer | `specimen capture app.apk` |

## 2. Flow 1: First run and setup

```mermaid
flowchart TD
  A[Install Specimen] --> B[specimen doctor]
  B --> C{Browser found?}
  C -- no --> D[Offer: specimen doctor --install-browser<br/>or set --browser path]
  D --> B
  C -- yes --> E{Agent CLI found?}
  E -- none --> F[Note: AI optional, use --no-ai<br/>or install an agent CLI]
  E -- one or more --> G[List agents with versions]
  F --> H[specimen init my-library]
  G --> H
  H --> I[Creates specimen.toml, .gitignore, folders, README]
  I --> J[Prints legal and ethics notice once]
  J --> K[Ready: specimen capture URL]
```

Behaviour notes:

- `doctor` is non-destructive and exits 0 even when optional parts are missing. It exits non-zero only when no usable browser exists.
- `init` is idempotent. Running it in an existing library reports "already initialised" and changes nothing.
- On first run, the notice states: respect site terms, the user is responsible for reuse, and Specimen is not legal advice. The notice is printed once and recorded in `specimen.toml` (`notice_acknowledged = true`).

## 3. Flow 2: Page capture (the core loop)

### 3.1 Sequence

```mermaid
sequenceDiagram
  actor Dev as Developer
  participant CLI as specimen-cli
  participant Cap as capture
  participant Br as Browser (CDP)
  participant Ext as extract
  participant Lib as library
  participant Ag as agent adapter

  Dev->>CLI: specimen capture https://example.com
  CLI->>CLI: validate URL, load config
  CLI->>Cap: start(job)
  Cap->>Cap: robots.txt check, URL policy
  Cap->>Br: launch (temp profile)
  loop each viewport x scheme
    Cap->>Br: emulate, navigate, settle
    Cap->>Br: scroll through page
    Cap->>Br: snapshot DOM and styles
    Cap->>Br: screenshots
  end
  Cap->>Br: probe hover/focus/active states
  Cap->>Br: observe animations, sample traces
  Cap->>Lib: write evidence, assets, manifest (.partial)
  Cap->>Br: close and clean up
  CLI->>Ext: run extractors on evidence
  Ext->>Lib: raw items
  CLI->>Ag: interpret (unless --no-ai)
  Ag-->>Lib: annotations (validated)
  CLI->>Lib: normalize into collections
  CLI->>Lib: write DESIGN.md, report.md, rename .partial to final
  CLI-->>Dev: summary and paths
```

### 3.2 Capture job state machine

```mermaid
stateDiagram-v2
  [*] --> Validating
  Validating --> Failed: bad URL / policy violation
  Validating --> RobotsCheck
  RobotsCheck --> Failed: disallowed (exit 5)
  RobotsCheck --> LaunchingBrowser
  LaunchingBrowser --> Failed: no browser (exit 3)
  LaunchingBrowser --> Capturing
  Capturing --> Capturing: next viewport/scheme
  Capturing --> Probing
  Capturing --> Failed: navigation error (exit 4)
  Probing --> WritingEvidence
  WritingEvidence --> Extracting
  Extracting --> Interpreting: AI enabled
  Extracting --> Normalizing: --no-ai
  Interpreting --> Normalizing: success or skipped with warning
  Interpreting --> Failed: --require-ai and agent failed (exit 6)
  Normalizing --> Finalizing
  Finalizing --> CompletedWithWarnings: warnings exist (exit 8)
  Finalizing --> Completed: clean (exit 0)
  Completed --> [*]
  CompletedWithWarnings --> [*]
  Failed --> [*]
```

### 3.3 What the user sees (example terminal output)

```
$ specimen capture https://example.com
 ✓ Policy         robots.txt allows, rate limit 1 rps
 ✓ Browser        Chrome 1xx (headless)
 ✓ Capture        4 viewports × 2 schemes, 11 screenshots
 ✓ Assets         9 fonts files, 42 images, 31 svg (6 restricted)
 ✓ Extract        palette 2 · typography 1 · icons 31 · effects 14 · layouts 6 · animations 9 · components 18
 ✓ Interpret      claude-code (annotated 80 items)
 ✓ Normalize      61 new items, 12 merged into existing, 4 similar links
 ! Warnings       2 (see report.md)

 Source:  sources/example.com/2026-11-25T101500Z
 Design:  sources/example.com/2026-11-25T101500Z/DESIGN.md
```

Progress is shown with a spinner and a stage list on TTY; on non-TTY or `--json`, progress is emitted as newline-delimited JSON events.

## 4. Flow 3: Flow capture (scripted and recorded)

```mermaid
flowchart TD
  A[Developer wants states that need interaction] --> B{Have a flow file?}
  B -- no --> C[specimen record URL -o flow.yaml]
  C --> D[Headed browser opens]
  D --> E[Developer clicks, hovers, scrolls, types]
  E --> F[Press Ctrl+C or close window]
  F --> G[Script written. Password input text never recorded]
  G --> H[Edit file if needed]
  B -- yes --> H
  H --> I[specimen capture URL --mode flow --flow flow.yaml]
  I --> J[Run steps sequentially]
  J --> K{Step failed?}
  K -- optional --> L[Warn and continue]
  K -- required --> M[Abort flow, keep evidence so far, exit 8]
  K -- no --> N[Next step]
  L --> N
  N --> O{More steps?}
  O -- yes --> J
  O -- no --> P[Extract, interpret, normalize as in Flow 2]
```

Behaviour notes:

- `capture:` steps inside the flow create additional evidence snapshots labelled with the step label, so a "menu open" state can yield its own components, layouts and screenshots.
- `eval:` steps are rejected unless `--allow-eval` is passed. The CLI prints the script before running.

## 5. Flow 4: Crawl capture

```mermaid
flowchart TD
  A[Start URL] --> B[Queue: depth 0]
  B --> C[Pop URL]
  C --> D{Allowed by robots, include/exclude, same-origin, page cap?}
  D -- no --> C
  D -- yes --> E[Wait for rate limiter]
  E --> F[Capture page as Flow 2 steps 1-8]
  F --> G[Compute DOM template signature]
  G --> H{Template already captured and dedupe on?}
  H -- yes --> I[Record URL as same-template, skip heavy capture]
  H -- no --> J[Store page evidence]
  I --> K[Collect links]
  J --> K
  K --> L[Add new links within depth limit]
  L --> M{Queue empty or page cap reached?}
  M -- no --> C
  M -- yes --> N[Normalize across all pages together]
  N --> O[Write group DESIGN.md and per-page notes]
```

User controls: `Ctrl+C` stops gracefully, finalising whatever has been captured and marking the source `partial: true` in the manifest.

## 6. Flow 5: Interpret stage (AI annotation)

```mermaid
flowchart TD
  A[Raw items from extractors] --> B[Group into tasks:<br/>name palette, label components, tag effects,<br/>describe animations, write DESIGN.md prose]
  B --> C{Agent available?}
  C -- none --> D[Use deterministic names and templated DESIGN.md]
  C -- yes --> E[Build task files in scratch dir:<br/>evidence slice JSON, screenshot paths, output schema]
  E --> F[Spawn agent CLI with timeout]
  F --> G{Valid JSON matching schema?}
  G -- no --> H{Retries left?}
  H -- yes --> F
  H -- no --> I[Warn W-AGENT-FAILED, fall back to deterministic names for this task]
  G -- yes --> J[Validate: IDs known, no new numbers or colors]
  J -- fail --> H
  J -- pass --> K[Merge annotation fields only]
  K --> L[Record agent, time, prompt hash]
  D --> M[Continue to normalize]
  I --> M
  L --> M
```

See AGENT.md Part B for prompts, schemas and guardrails.

## 7. Flow 6: Normalize and de-duplicate

```mermaid
flowchart TD
  A[Raw items + annotations] --> B[Canonicalise data]
  B --> C[Compute id = type-slug-hash8]
  C --> D{Item exists in collection?}
  D -- yes --> E[Append provenance, keep user and annotations fields]
  D -- no --> F[Write new item folder: item.json, previews, files]
  E --> G[Near-duplicate check]
  F --> G
  G --> H{Within similarity threshold of other items?}
  H -- yes --> I[Add similar_to links both ways]
  H -- no --> J[Done]
  I --> J
  J --> K[Update README index and cache]
```

## 8. Flow 7: Search and inspect from the CLI

```
specimen search "glass card"
specimen search "type:palette near:#3b82f6 delta:0.05 scheme:dark license:reusable"
specimen list animations --trigger hover --duration "<400ms"
specimen show effect-frosted-glass-card-9f3a1c2e
```

- `search` prints a ranked table (ID, type, name, source, license, confidence).
- `show` prints the item's key data, a CSS snippet where relevant, provenance and the path to the preview image. With `--open` it opens the preview in the default viewer.
- `--json` returns structured output for scripting.
- No match prints a hint listing available types and tags, and exits 0.

## 9. Flow 8: Browse in the web app

```mermaid
flowchart TD
  S[specimen serve] --> H[Home: counts, recent sources, search]
  H --> C1[Collection grid]
  H --> SR[Source detail]
  H --> Q[Search results]
  C1 --> F[Filters: tag, license, scheme, source, confidence]
  F --> C1
  C1 --> ID[Item detail]
  Q --> ID
  SR --> ID
  SR --> SG[Screenshots gallery]
  SR --> DM[DESIGN.md rendered]
  ID --> CP[Copy CSS or token]
  ID --> SM[Similar items]
  ID --> FV[Favorite, note, tag, set license override]
  ID --> EX[Export dialog]
  C1 --> CMP[Compare two items or sources]
  EX --> DL[Preview output and download]
```

Page-specific behaviour:

| Page | Key interactions |
|---|---|
| Palette | Click swatch to copy value; "find near this color" runs a proximity search; contrast matrix toggles light and dark |
| Typography | Type specimen renders with the actual font file; toggle weights; scale table with copy |
| Animation | Loops the preview clip; timing curve drawn from easing; trigger and targets listed; speed control |
| Layout | Wireframe with breakpoint switcher; hover a block to see its flex or grid rules |
| Icons | Grid with size slider and color preview (for `currentColor` icons); license badge; copy SVG |
| Effects | Live-rendered sample using the CSS; toggle light and dark background |
| Component | State tabs (default, hover, focus, active, disabled) with screenshot and styles |

Write actions (favorite, notes, tags, license override) save into the item's `user` object. The UI shows a confirmation and the file changed, so the user understands the change appears in git diffs.

## 10. Flow 9: Agent queries via MCP

```mermaid
sequenceDiagram
  actor U as Developer
  participant A as AI coding agent
  participant M as specimen mcp
  participant L as Library

  U->>A: "Build a pricing section using my saved dark glass cards"
  A->>M: search_library("type:effect tag:glassmorphism scheme:dark")
  M->>L: query index
  L-->>M: item summaries
  M-->>A: results (bounded size)
  A->>M: get_item(effect-frosted-glass-card-9f3a1c2e)
  M-->>A: full item JSON with CSS and license
  A->>M: get_palette(query: "dark brand")
  M-->>A: palette with roles and contrast
  A-->>U: Code using returned tokens, with license note for reference-only items
```

Behaviour notes:

- The MCP server is read-only by default and never returns paths outside the library.
- Items with `reference-only` or `unknown` license include a `license_warning` field so the agent can mention it to the user.
- `capture_url` is absent from the tool list unless enabled in config.

## 11. Flow 10: Export tokens

```mermaid
flowchart TD
  A[specimen export --format F --scope S] --> B[Select items by scope]
  B --> C[Filter: confidence >= min, scheme]
  C --> D{Any non-reusable items?}
  D -- yes --> E[Print warning listing them]
  D -- no --> F[Continue]
  E --> F
  F --> G[Map to tokens, apply export.names overrides]
  G --> H[Render format: css / tailwind / dtcg / scss / tokens-studio]
  H --> I[Write to exports/ or --out, print path]
```

Example:

```
specimen export --format tailwind --tailwind-version 4 --scope source:example.com/2026-11-25T101500Z --scheme both
specimen export --format dtcg --scope items:palette-brand-dark-3b1c9a02,theme-dark-77ac10e4
```

## 12. Flow 11: Share the library with a team

1. The library is a git repository (`git init` inside the library folder).
2. The generated `.gitignore` excludes `.cache/` and `assets-restricted/`, so proprietary files stay local.
3. Teammates clone and run `specimen validate` to confirm schema health, then `specimen serve` or `specimen mcp`.
4. Merge conflicts are rare because items are separate files with content-hash IDs; `specimen normalize` can rebuild collections from sources if needed.
5. For large binaries (video, big fonts), enable Git LFS patterns in `.gitattributes` (documented, not automatic).

## 13. Flow 12: Failure and recovery paths

| Situation | Detection | User sees | Recovery |
|---|---|---|---|
| No browser | Discovery fails | Exit 3 with install hints | `specimen doctor --install-browser` or `--browser <path>` |
| Disallowed by robots.txt | Policy check | Exit 5, rule shown | Respect it, or run `--robots ignore` knowingly (prints warning) |
| Navigation timeout or HTTP error | CDP events | Exit 4 with status and last URL | Retry with larger `--timeout`, check connectivity |
| Bot protection or interstitial | Status codes and interstitial markers | `E-BLOCKED` with explanation | Try `--headed` and complete the check manually if the site allows; no evasion is attempted |
| Page needs login | Redirect to login detected | Hint about `--headed` | Sign in manually in the headed browser |
| Cross-origin iframes | Frame attach limits | `W-IFRAME` warning | Capture the iframe URL separately |
| Canvas or WebGL content | Detection | `W-CANVAS`, clip recorded | None; documented limitation |
| Agent not installed or not authenticated | `detect()` | `W-AGENT-SKIPPED`, deterministic output | Install or log in to an agent, then `specimen interpret <source>` |
| Agent output invalid | Schema and value validation | `W-AGENT-FAILED` for that task | Automatic retry, then fallback; re-run `interpret` later |
| Interrupted capture (Ctrl+C, crash) | `.partial` folder | Message with resume hint | Folder is not promoted; rerun capture, or `specimen sources rm` the partial |
| Disk full or permission denied | I/O error | Clear error and path | Free space, fix permissions; partial writes are atomic so the library stays valid |
| Library schema older than binary | Version check | Prompt to migrate | `specimen migrate` (backup made first) |
| Validation errors in library | `specimen validate` | List of files and problems | Fix manually or re-run normalize |

## 14. Flow 13: APK capture (v2 outline)

```mermaid
flowchart TD
  A[specimen capture app.apk] --> B[Check ownership/entitlement notice]
  B --> C[Static decode: resources, fonts, drawables, anim, layouts]
  C --> D{Emulator available?}
  D -- yes --> E[Install and launch, take screenshots, dump UI hierarchy, record transitions]
  D -- no --> F[Static results only, warn]
  E --> G[Framework detection: native, Flutter, React Native, game engine]
  F --> G
  G --> H[Extractors for Android sources, same item types]
  H --> I[Interpret and normalize as in Flows 5 and 6]
  I --> J[Source folder with DESIGN.md and limitations report]
```

## 15. Cross-cutting UX rules

1. **Never surprise the user with network access.** Capture states which host it will contact before starting (`--dry-run` shows the full plan).
2. **Always show what was not captured.** Every run ends with a count of warnings and the path to `report.md`.
3. **Every item answers three questions:** where did this come from, may I reuse it, and how sure are you?
4. **Everything is a file.** If a user can see it in the UI, they can find it on disk.
5. **AI is optional and visible.** Output always says whether AI annotated it, and by which agent.
6. **Safe defaults, explicit overrides.** Risky options (`--robots ignore`, `--allow-private`, `--allow-eval`, enabling MCP capture) require a flag and print a one-line warning.
