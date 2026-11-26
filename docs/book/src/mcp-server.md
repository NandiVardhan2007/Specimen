# Model Context Protocol (MCP) Server

Specimen includes a native **Model Context Protocol (MCP)** stdio server (`specimen mcp`), allowing AI assistants like **Claude Desktop**, **Cursor**, and **Antigravity** to directly query your design library.

## Configuring Claude Desktop

Add Specimen to your Claude Desktop configuration (`claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "specimen": {
      "command": "specimen",
      "args": ["mcp", "--library", "/path/to/my-design-system"]
    }
  }
}
```

## Configuring Cursor

In Cursor's **Features > MCP Servers** settings, add a new stdio command:
- **Name**: `Specimen`
- **Command**: `specimen mcp --library .`

## Available Tools

- `search_library(query: string)`: Searches the design library with filters (`type:`, `tag:`, `source:`, etc.).
- `get_item(id: string)`: Retrieves the complete canonical payload of any design item.
- `list_collections()`: Lists collection categories and counts.
- `get_design_md()`: Returns the generated markdown design overview.
- `export_tokens(format: string)`: Exports tokens to CSS, Tailwind v3/v4, SCSS, or DTCG.
- `capture_url(url: string)`: Captures a website (requires `--allow-capture` flag on server launch).
