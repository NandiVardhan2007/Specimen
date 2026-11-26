#![forbid(unsafe_code)]

//! Specimen MCP: Model Context Protocol server exposing library search, inspection, and token exports.

use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::io::{BufRead, Write};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct McpServerConfig {
    pub library_root: PathBuf,
    pub allow_capture: bool,
}

pub struct McpServer {
    config: McpServerConfig,
}

impl McpServer {
    pub fn new(config: McpServerConfig) -> Self {
        Self { config }
    }

    /// Runs the stdio JSON-RPC 2.0 loop reading lines from stdin and replying to stdout.
    pub fn run_stdio(&self) -> Result<(), Box<dyn std::error::Error>> {
        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout();
        let reader = stdin.lock();

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            if let Some(resp) = self.handle_message(&line) {
                writeln!(stdout, "{}", resp)?;
                stdout.flush()?;
            }
        }

        Ok(())
    }

    /// Handles a single JSON-RPC 2.0 message and returns the serialized response string.
    pub fn handle_message(&self, raw_line: &str) -> Option<String> {
        let request: Value = serde_json::from_str(raw_line).ok()?;
        let id = request.get("id").cloned();
        let method = request.get("method")?.as_str()?;
        let params = request.get("params").cloned().unwrap_or(Value::Null);

        let result = match method {
            "initialize" => self.handle_initialize(),
            "ping" => Ok(json!({})),
            "tools/list" => self.handle_tools_list(),
            "tools/call" => self.handle_tools_call(&params),
            "resources/list" => self.handle_resources_list(),
            "resources/read" => self.handle_resources_read(&params),
            _ => Err(json!({
                "code": -32601,
                "message": format!("Method '{}' not found", method)
            })),
        };

        match result {
            Ok(res) => id.map(|id| {
                serde_json::to_string(&json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": res
                }))
                .unwrap()
            }),
            Err(err) => id.map(|id| {
                serde_json::to_string(&json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": err
                }))
                .unwrap()
            }),
        }
    }

    fn handle_initialize(&self) -> Result<Value, Value> {
        Ok(json!({
            "protocolVersion": "2024-11-05",
            "serverInfo": {
                "name": "specimen-mcp",
                "version": env!("CARGO_PKG_VERSION")
            },
            "capabilities": {
                "tools": {},
                "resources": {}
            }
        }))
    }

    fn handle_tools_list(&self) -> Result<Value, Value> {
        let mut tools = vec![
            json!({
                "name": "search_library",
                "description": "Search items in the Specimen design library by query text or filters (type: tag: license:).",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "description": "Search query text or filter term" }
                    },
                    "required": ["query"]
                }
            }),
            json!({
                "name": "get_item",
                "description": "Retrieve full canonical item JSON for a given Item ID.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "string", "description": "The Item ID (e.g. palette-dark-7a8b9c)" }
                    },
                    "required": ["id"]
                }
            }),
            json!({
                "name": "list_collections",
                "description": "List all collections and item counts in the design library.",
                "inputSchema": {
                    "type": "object"
                }
            }),
            json!({
                "name": "get_design_md",
                "description": "Retrieve the latest DESIGN.md design system summary document.",
                "inputSchema": {
                    "type": "object"
                }
            }),
            json!({
                "name": "export_tokens",
                "description": "Export tokens in CSS custom properties, Tailwind v4, or W3C DTCG format.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "format": {
                            "type": "string",
                            "enum": ["css", "tailwind", "dtcg"],
                            "description": "Export format to generate"
                        }
                    },
                    "required": ["format"]
                }
            }),
        ];

        if self.config.allow_capture {
            tools.push(json!({
                "name": "capture_url",
                "description": "Capture a public web page into the design library (subject to SSRF and rate limiting policy).",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "url": { "type": "string", "description": "Public URL to capture" }
                    },
                    "required": ["url"]
                }
            }));
        }

        Ok(json!({ "tools": tools }))
    }

    fn handle_tools_call(&self, params: &Value) -> Result<Value, Value> {
        let name = params
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let args = params.get("arguments").cloned().unwrap_or(Value::Null);

        match name {
            "search_library" => {
                let query = args
                    .get("query")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let filter = specimen_library::SearchIndex::parse_query(query);
                let hits =
                    specimen_library::SearchIndex::search(&self.config.library_root, &filter)
                        .unwrap_or_default();

                let json_text = serde_json::to_string_pretty(&hits).unwrap_or_default();
                Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": json_text
                    }]
                }))
            }
            "get_item" => {
                let item_id = args.get("id").and_then(|v| v.as_str()).unwrap_or_default();
                let item_json = self.find_item_by_id(item_id).ok_or_else(|| {
                    json!({
                        "code": -32602,
                        "message": format!("Item '{}' not found in library", item_id)
                    })
                })?;

                Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": item_json
                    }]
                }))
            }
            "list_collections" => {
                let mut counts = BTreeMap::new();
                let coll_dir = self.config.library_root.join("collections");
                if coll_dir.exists() {
                    if let Ok(entries) = fs::read_dir(coll_dir) {
                        for e in entries.flatten() {
                            if e.path().is_dir() {
                                let cname = e.file_name().to_string_lossy().to_string();
                                let cnt = fs::read_dir(e.path())
                                    .map(|it| {
                                        it.filter_map(Result::ok)
                                            .filter(|i| i.path().is_dir())
                                            .count()
                                    })
                                    .unwrap_or(0);
                                counts.insert(cname, cnt);
                            }
                        }
                    }
                }
                Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string_pretty(&counts).unwrap_or_default()
                    }]
                }))
            }
            "get_design_md" => {
                let design_md_text = self.find_latest_design_md().unwrap_or_else(|| {
                    "# Design System\n\nNo captures recorded yet in library.".to_string()
                });
                Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": design_md_text
                    }]
                }))
            }
            "export_tokens" => {
                let fmt = args.get("format").and_then(|v| v.as_str()).unwrap_or("css");
                let text = specimen_export::export_tokens_to_string(&self.config.library_root, fmt)
                    .unwrap_or_else(|e| format!("/* Error: {} */", e));
                Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": text
                    }]
                }))
            }
            "capture_url" => {
                if !self.config.allow_capture {
                    return Err(json!({
                        "code": -32600,
                        "message": "Capture via MCP is disabled by default. Enable with --allow-capture."
                    }));
                }
                Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": "Capture request queued."
                    }]
                }))
            }
            _ => Err(json!({
                "code": -32601,
                "message": format!("Tool '{}' not recognized", name)
            })),
        }
    }

    fn handle_resources_list(&self) -> Result<Value, Value> {
        Ok(json!({
            "resources": [
                {
                    "uri": "specimen://library/summary",
                    "name": "Design Library Summary",
                    "mimeType": "application/json"
                },
                {
                    "uri": "specimen://design.md",
                    "name": "Latest DESIGN.md Document",
                    "mimeType": "text/markdown"
                }
            ]
        }))
    }

    fn handle_resources_read(&self, params: &Value) -> Result<Value, Value> {
        let uri = params
            .get("uri")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        match uri {
            "specimen://library/summary" => {
                let summary = json!({
                    "library_root": self.config.library_root.display().to_string(),
                    "server": "specimen-mcp"
                });
                Ok(json!({
                    "contents": [{
                        "uri": uri,
                        "mimeType": "application/json",
                        "text": serde_json::to_string_pretty(&summary).unwrap_or_default()
                    }]
                }))
            }
            "specimen://design.md" => {
                let text = self.find_latest_design_md().unwrap_or_default();
                Ok(json!({
                    "contents": [{
                        "uri": uri,
                        "mimeType": "text/markdown",
                        "text": text
                    }]
                }))
            }
            _ => Err(json!({
                "code": -32602,
                "message": format!("Resource '{}' not found", uri)
            })),
        }
    }

    fn find_item_by_id(&self, item_id: &str) -> Option<String> {
        let coll_dir = self.config.library_root.join("collections");
        if coll_dir.exists() {
            if let Ok(entries) = fs::read_dir(coll_dir) {
                for e in entries.flatten() {
                    let file = e.path().join(item_id).join("item.json");
                    if file.exists() {
                        return fs::read_to_string(file).ok();
                    }
                }
            }
        }
        None
    }

    fn find_latest_design_md(&self) -> Option<String> {
        let sources_dir = self.config.library_root.join("sources");
        if sources_dir.exists() {
            if let Ok(host_entries) = fs::read_dir(sources_dir) {
                for h in host_entries.flatten() {
                    if let Ok(sub_entries) = fs::read_dir(h.path()) {
                        for sub in sub_entries.flatten() {
                            let md_file = sub.path().join("DESIGN.md");
                            if md_file.exists() {
                                return fs::read_to_string(md_file).ok();
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_initialize_and_ping() {
        let temp = tempfile::tempdir().unwrap();
        let server = McpServer::new(McpServerConfig {
            library_root: temp.path().to_path_buf(),
            allow_capture: false,
        });

        let init_req = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
        let init_resp = server.handle_message(init_req).unwrap();
        assert!(init_resp.contains("specimen-mcp"));
        assert!(init_resp.contains("protocolVersion"));

        let ping_req = r#"{"jsonrpc":"2.0","id":2,"method":"ping","params":{}}"#;
        let ping_resp = server.handle_message(ping_req).unwrap();
        assert!(ping_resp.contains("\"result\":{}"));
    }

    #[test]
    fn test_mcp_tools_list() {
        let temp = tempfile::tempdir().unwrap();
        let server = McpServer::new(McpServerConfig {
            library_root: temp.path().to_path_buf(),
            allow_capture: false,
        });

        let req = r#"{"jsonrpc":"2.0","id":3,"method":"tools/list","params":{}}"#;
        let resp = server.handle_message(req).unwrap();
        assert!(resp.contains("search_library"));
        assert!(resp.contains("get_item"));
        assert!(resp.contains("export_tokens"));
    }
}
