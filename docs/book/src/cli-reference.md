# CLI Command Reference

Specimen provides a unified, cross-platform CLI tool:

### `specimen init [PATH] [--name <NAME>]`
Initializes a new git-friendly design library.
- `[PATH]`: Destination directory (defaults to `.`).
- `--name`: Friendly name for the design library.

### `specimen doctor`
Inspects host OS, Chrome/Edge browser binary paths, and installed AI agent CLIs.

### `specimen capture <URL> [OPTIONS]`
Captures a website or scripted flow into the design library.
- `--mode <page|flow>`: Single-page capture or scripted interaction flow.
- `--dry-run`: Validates URL policy and simulates capture plan without touching disk.
- `--headed`: Launches visible browser window for manual interactions or sign-ins.
- `--allow-private`: Permissively allows localhost/private IP connections (SSRF warning).
- `--allow-eval`: Enables custom JavaScript execution in flow scripts.
- `--library <PATH>`: Target library path (defaults to `.`).

### `specimen crawl <URL> [OPTIONS]`
Performs a bounded breadth-first crawl across same-origin links.
- `--depth <N>`: Maximum link recursion depth (defaults to 2).
- `--max-pages <N>`: Hard cap on total pages captured (defaults to 10).
- `--dry-run`: Validates crawl boundaries and robots.txt without issuing requests.
- `--headed`: Runs in visible headed browser mode.

### `specimen record <URL> [-o flow.yaml]`
Interactive recording driver for generating repeatable flow scripts.

### `specimen serve [--port <PORT>] [--open]`
Runs the local loopback web application strictly on `127.0.0.1:<PORT>`.
- `--port <P>`: Port to listen on (defaults to 4000).
- `--open`: Opens default browser to the web app.

### `specimen mcp [--allow-capture]`
Runs the Model Context Protocol (MCP) server over `stdio` for Claude Desktop and Cursor.
- `--allow-capture`: Enables the `capture_url` tool (disabled by default for safety).

### `specimen search <QUERY> [--json]`
Fast in-memory search over library items with structured filters (`type:`, `tag:`, `source:`, `license:`, `scheme:`).

### `specimen validate [PATH] [--json]`
Validates library integrity, JSON schemas, content-hashed IDs, and license declarations.

### `specimen export [--format <FMT>] [-o <OUT>]`
Exports tokens into CSS, Tailwind v3/v4, SCSS, DTCG, or Tokens Studio format.

### `specimen migrate [PATH]`
Upgrades legacy item schemas to the current canonical version.

### `specimen completions <SHELL>`
Generates shell autocomplete scripts for bash, zsh, fish, powershell, or elvish.
