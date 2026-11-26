# Specimen: Product Requirements Document (PRD)

| | |
|---|---|
| **Product** | Specimen |
| **Tagline** | Capture the design. Keep the system. |
| **Document status** | Draft v0.1 (planning) |
| **Date** | 2026-11-25 |
| **License (planned)** | MIT OR Apache-2.0 |
| **Related docs** | [SPEC.md](SPEC.md) · [TRD.md](TRD.md) · [APP_FLOW.md](APP_FLOW.md) · [AGENT.md](AGENT.md) · [TESTING_GUIDE.md](TESTING_GUIDE.md) · [IMPLEMENTATION-PLAN.md](IMPLEMENTATION-PLAN.md) |

---

## 1. Summary

Specimen is an open source, single-binary command line tool (written in Rust) that **captures the design of any website** and turns it into a **personal, git-friendly design library**. The library is organised into icons, color palettes, typography, themes, layouts, effects, animations and components. Every item is stored as plain files (JSON, Markdown, images) with provenance and license status.

The library can be browsed in a local web app, queried by AI coding agents over MCP, and exported as design tokens (CSS variables, Tailwind, W3C Design Tokens, SCSS, Tokens Studio for Figma).

Specimen does **not** rebuild or clone sites. It identifies, measures, organises and remembers.

Mobile app (APK) capture is planned for v2.

## 2. Problem

Developers and designers constantly study other products for reference: how a pricing page is laid out, what easing a navigation uses, which type scale a product team chose. Today that research is scattered:

- Screenshots in random folders, with no measurements.
- DevTools sessions that disappear when the tab closes.
- Browser extensions that grab colors or fonts but not motion, layout or effects, and keep nothing in a form that can be reused.
- Design galleries that show pictures, not values.
- AI agents that are asked to "make it look like X" and guess, because nobody gave them measured data.

There is no tool that **measures a live design, organises it into reusable categories, remembers the license situation of each piece, and makes the result available to humans and agents over time.**

## 3. Vision and positioning

> Every developer owns a design library built from the real world: searchable, measured, licensed, and portable.

Positioning relative to related open source projects (verified from their public READMEs, November 2026):

| Project | What it is | How Specimen differs |
|---|---|---|
| **Open Design** (nexu-io) | Agent-driven design platform built around `DESIGN.md` brand systems | Specimen *produces* measured design systems from live sources; it does not generate designs |
| **Craft Anything** (storytold) | Rust reverse-engineering engine that inspects a target and has a coding agent rebuild it | Specimen stops at inspection and organisation, and optimises for a long-lived, reusable library rather than a single rebuild |

Specimen should interoperate with that ecosystem: its `DESIGN.md` output should be easy for agent workflows to consume, and its MCP server should work with any MCP client.

## 4. Goals and non-goals

### Goals

1. **Faithful measurement.** Extract values from the rendered page (computed styles, loaded font files, real keyframes), not from guesswork.
2. **A library, not a dump.** Normalise and de-duplicate across sources so the library gets more useful with every capture.
3. **Agent-ready.** Any AI coding agent can query the library while it works.
4. **Honest about rights.** Every asset carries provenance and a license status (`reusable`, `reference-only`, `unknown`).
5. **Single binary, local-first.** No account, no cloud, no telemetry. Works offline after capture.
6. **Reproducible.** Same input evidence gives the same output items and IDs.
7. **Bring your own AI.** Use whichever agent CLI the user already has (Claude Code, Codex, Antigravity and others) for interpretation, never for inventing values.

### Non-goals

- Cloning or rebuilding a site or app.
- Bypassing logins, paywalls, CAPTCHAs, bot protection, DRM or anti-cheat.
- Hosting or redistributing captured proprietary assets.
- Generating new designs or images.
- A hosted SaaS or team cloud in v1 (a git repository is the sharing mechanism).
- Figma plugin or direct Figma API writes (Tokens Studio export covers Figma in v1).
- Mobile app capture in v1 (planned v2).
- Decomposing canvas, WebGL or video content. These are recorded and flagged, not reverse engineered.

## 5. Users and personas

**P1. Asha, solo front-end developer.** Freelances, reviews a lot of reference sites, keeps a messy screenshots folder. Wants to capture a site's palette, fonts and motion in one command, and pull them into her Tailwind project.

**P2. Marcus, design engineer on a small team.** Maintains a team design system and benchmarks competitors. Wants a shared git repository of references with clear license flags so nobody ships a proprietary icon by accident.

**P3. Priya, AI-assisted developer.** Builds with an agent in her editor. Wants the agent to query "my saved glassmorphism effects" or "my favourite dark palettes" through MCP instead of describing them from memory.

## 6. User stories and acceptance criteria

Priority: **M** = must (v1 track), **S** = should, **C** = could.

| ID | Story | Acceptance criteria | Pri |
|---|---|---|---|
| US-01 | As a developer, I run `specimen capture <url>` and get a structured folder | A source folder is created with manifest, evidence, screenshots, assets and `DESIGN.md`; exit code 0; a summary of counts is printed | M |
| US-02 | I get screenshots at several viewport sizes | Default mobile, tablet, laptop, desktop screenshots exist, full-page and above-the-fold; configurable | M |
| US-03 | I get the colors with roles | Palette item lists colors with usage weight, a role guess, and WCAG contrast for text/background pairs | M |
| US-04 | I get the exact fonts used | Font files that were actually loaded are saved with family, weight, style, format, hash, and license hints read from the font file | M |
| US-05 | I get the type scale | Typography item lists size, weight, line-height, letter-spacing, family per element role | M |
| US-06 | I get icons as clean SVG files | Inline, sprite, `<img>`, CSS and icon-font icons are extracted, de-duplicated by normalised geometry, with set detection for known open source sets | S (v0.2) |
| US-07 | I get effects | Shadows, gradients, blur, glass, borders, filters are listed with CSS and a recipe name | S (v0.2) |
| US-08 | I get layout skeletons without page content | Section skeletons (hero, nav, grid, footer) with flex/grid structure, breakpoints and an SVG wireframe | S (v0.2) |
| US-09 | I get animations with timing | Keyframes, transitions, easing, duration, trigger and a preview clip for each captured animation | M (v0.3) |
| US-10 | I can script interactions | `flow` mode replays a YAML script (hover, click, scroll, type); `specimen record` generates the script | S (v0.3) |
| US-11 | I can capture many pages politely | `crawl` mode with depth, page cap, same-origin, rate limit and robots handling | S (v0.4) |
| US-12 | I can search my library | `specimen search "glass card"` and filters by type, tag, color proximity, license, source | M |
| US-13 | I can browse visually | `specimen serve` shows collections with previews, item detail and source detail | S (v0.4) |
| US-14 | My AI agent can query the library | `specimen mcp` exposes search and get tools over MCP stdio | S (v0.4) |
| US-15 | I can export tokens | CSS variables, Tailwind (v3 and v4), W3C DTCG JSON, SCSS and Tokens Studio JSON | M (CSS, DTCG v0.2; rest by v0.5) |
| US-16 | I can use Specimen without AI | `--no-ai` yields complete deterministic output with generated names and a templated `DESIGN.md` | M |
| US-17 | I know what I may reuse | Every item shows a license status and basis; proprietary assets are kept out of git by default | M |
| US-18 | I can capture a site behind a login I own | `--headed` lets me sign in manually; no credentials or cookies are written to the library | S (v0.3) |
| US-19 | I understand what could not be captured | Each source has a limitations report (canvas, WebGL, blocked requests, partial iframes) | M |

## 7. Feature list and release mapping

| Release | Scope |
|---|---|
| **v0.1** | `init`, `doctor`, `capture` (page mode), screenshots, palette, typography, spacing/radius scale, fonts, `DESIGN.md`, library write, `--no-ai`, first agent adapter |
| **v0.2** | Icons, effects, layouts, themes (light/dark), search CLI, CSS + DTCG exports, license detection v1 |
| **v0.3** | Animations (CSS, WAAPI, traces, Lottie detection), preview clips, `flow` mode, `record`, headed login mode, components with states |
| **v0.4** | `crawl` mode, local web app, MCP server |
| **v0.5** | Tailwind, SCSS, Tokens Studio exports, cross-source de-duplication and similarity links, polish, docs site |
| **v2** | APK capture (static resources and emulator capture) |

## 8. Success metrics

Specimen is a local, no-telemetry tool, so metrics come from tests and the open source community.

**Quality (measured in CI against fixture sites; see TESTING_GUIDE.md):**
- Color, font and keyframe recall on fixtures of at least 98%.
- Zero invented values: 100% of numeric and color values in the library trace to evidence.
- Deterministic IDs: identical results across repeated captures of a fixture.

**Adoption (community signals):**
- 500 GitHub stars within six months of v0.5.
- 25 external contributors within a year.
- Third-party agents or tools using the MCP server or the `DESIGN.md` output.

**Usability:**
- First useful capture within 5 minutes of installing (install, `doctor`, `init`, `capture`).
- Typical single-page capture under 60 seconds on broadband.

## 9. Legal, ethical and compliance considerations

Specimen reads publicly served pages the way a browser does, then stores measurements and assets. That is legitimate for research, but the result can contain copyrighted or trademarked material. Product requirements:

1. **License status on every asset and item** (`reusable`, `reference-only`, `unknown`) with the basis recorded (for example "matched Lucide icon set, ISC").
2. **Restricted assets are segregated** into a directory excluded from git by default, so a shared library repo does not accidentally publish proprietary files.
3. **Respect `robots.txt` by default**, rate limit, and identify itself in the user agent. Overrides require an explicit flag and print a warning.
4. **No evasion.** No stealth plugins, CAPTCHA solving, proxy rotation or login bypass.
5. **Authenticated captures are user-driven** (manual sign-in in a headed browser). Cookies, tokens and authorization headers are never written to the library. A `--redact-text` option removes page text from stored evidence.
6. **Clear user responsibility notice** in the README and on first run: the user is responsible for complying with site terms and applicable law, and for how they reuse extracted material.
7. **Font license hints are hints.** Specimen reads embedded license metadata but does not give legal advice.
8. APK analysis (v2) only on apps the user has the right to analyse.

*Specimen's documentation is not legal advice. Before launch, have the license text, terms of use and notice wording reviewed by someone qualified.*

## 10. Assumptions and dependencies

- Users have Chrome, Chromium or Edge installed, or allow Specimen to download a pinned browser build.
- At least one agent CLI is available for the best output, but is optional.
- Target platforms: macOS, Linux, Windows.
- Specimen depends on the Chrome DevTools Protocol, so changes in Chrome can break capture. CI pins and tests a browser version.

## 11. Risks

| Risk | Impact | Likelihood | Mitigation |
|---|---|---|---|
| Sites block headless browsers | Failed or partial captures | High | Clear errors, headed mode, no evasion; document limits |
| JS-driven animation is hard to decompose | Incomplete motion data | High | Layered strategy (CSS, WAAPI, sampled traces, library detection); label confidence |
| Rust CDP crate lags Chrome | Maintenance burden | Medium | Thin internal CDP layer; fallback to own typed client |
| Agent CLIs change flags or output | Adapter breakage | High | Adapter contract tests, version detection, manual-prompt fallback |
| Legal complaints about captured assets | Reputation and project risk | Medium | License status, restricted dir, notices, no redistribution features |
| Prompt injection from captured pages | Agent misbehaviour | Medium | Data-only prompts, schema-validated output, annotation-only merge (see AGENT.md) |
| Scope creep (rebuilders, hosted features) | Delay | Medium | Non-goals list; roadmap gates |
| Name collision (crate, org, domain, trademark) | Rebrand cost | Unknown | Verify before first public release (IMPLEMENTATION-PLAN Phase 0) |

## 12. Open questions

1. Final license: MIT OR Apache-2.0 dual (recommended), or something else?
2. Default for `robots.txt` in single-page mode: respect (recommended) or warn only?
3. Preview clip format: animated WebP and GIF by default, MP4 when `ffmpeg` is present. Acceptable?
4. Should the library support Git LFS out of the box for large binaries?
5. Should Specimen publish a public, community "open-licensed" library (icons and palettes only, no proprietary assets)?
6. Which `DESIGN.md` conventions should it align with, so output works with existing agent workflows? (Review Open Design's current format before finalising.)
7. Governance: single maintainer, or a small core team with a code of conduct from day one?
