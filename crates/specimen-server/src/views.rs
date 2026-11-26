#![forbid(unsafe_code)]

use serde_json::Value;
use std::collections::BTreeMap;

pub fn render_base_layout(title: &str, content: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{title} - Specimen</title>
  <style>
    :root {{
      --bg: #0b0d12;
      --surface: #141721;
      --surface-hover: #1c202e;
      --border: #262b3d;
      --text: #f0f2f5;
      --text-muted: #8c93a8;
      --accent: #7c5cff;
      --accent-hover: #6945f8;
      --success: #10b981;
      --font: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    }}
    * {{ box-sizing: border-box; margin: 0; padding: 0; }}
    body {{
      background-color: var(--bg);
      color: var(--text);
      font-family: var(--font);
      line-height: 1.5;
      padding: 0;
      min-height: 100vh;
      display: flex;
      flex-direction: column;
    }}
    header {{
      background-color: var(--surface);
      border-bottom: 1px solid var(--border);
      padding: 16px 32px;
      display: flex;
      justify-content: space-between;
      align-items: center;
    }}
    .brand {{
      display: flex;
      align-items: center;
      gap: 12px;
      font-weight: 700;
      font-size: 18px;
      color: var(--text);
      text-decoration: none;
    }}
    .brand-pill {{
      background-color: var(--accent);
      color: white;
      padding: 2px 8px;
      border-radius: 9999px;
      font-size: 11px;
      font-weight: 600;
    }}
    nav {{ display: flex; gap: 20px; align-items: center; }}
    nav a {{
      color: var(--text-muted);
      text-decoration: none;
      font-size: 14px;
      font-weight: 500;
      transition: color 0.15s ease;
    }}
    nav a:hover, nav a.active {{ color: var(--text); }}
    main {{ flex: 1; padding: 32px; max-width: 1200px; margin: 0 auto; width: 100%; }}
    .grid-cards {{
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
      gap: 20px;
      margin-top: 24px;
    }}
    .card {{
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: 8px;
      padding: 20px;
      transition: transform 0.15s ease, border-color 0.15s ease;
      text-decoration: none;
      color: inherit;
      display: block;
    }}
    .card:hover {{
      transform: translateY(-2px);
      border-color: var(--accent);
    }}
    .badge {{
      display: inline-block;
      padding: 2px 8px;
      border-radius: 4px;
      font-size: 12px;
      font-weight: 600;
      background: var(--surface-hover);
      color: var(--accent);
    }}
    .search-bar {{
      display: flex;
      gap: 12px;
      margin-bottom: 24px;
    }}
    .input-text {{
      flex: 1;
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: 6px;
      padding: 10px 16px;
      color: var(--text);
      font-size: 14px;
    }}
    .btn {{
      background: var(--accent);
      color: white;
      border: none;
      border-radius: 6px;
      padding: 10px 20px;
      font-weight: 600;
      cursor: pointer;
      font-size: 14px;
    }}
    .btn:hover {{ background: var(--accent-hover); }}
    pre {{
      background: #050608;
      border: 1px solid var(--border);
      border-radius: 6px;
      padding: 16px;
      overflow-x: auto;
      font-family: monospace;
      font-size: 13px;
      color: #93c5fd;
    }}
    footer {{
      background: var(--surface);
      border-top: 1px solid var(--border);
      padding: 20px 32px;
      text-align: center;
      font-size: 13px;
      color: var(--text-muted);
    }}
  </style>
</head>
<body>
  <header>
    <a href="/" class="brand">
      <span>Specimen</span>
      <span class="brand-pill">Library</span>
    </a>
    <nav>
      <a href="/">Overview</a>
      <a href="/collections/palettes">Palettes</a>
      <a href="/collections/typography">Typography</a>
      <a href="/collections/components">Components</a>
      <a href="/collections/icons">Icons</a>
      <a href="/collections/effects">Effects</a>
      <a href="/collections/animations">Animations</a>
      <a href="/collections/layouts">Layouts</a>
      <a href="/sources">Sources</a>
      <a href="/search">Search</a>
      <a href="/export">Export</a>
    </nav>
  </header>
  <main>
    {content}
  </main>
  <footer>
    Specimen design library. Stored as plain files, git-friendly, loopback only.
  </footer>
</body>
</html>"#
    )
}

pub fn render_home(counts: &BTreeMap<String, usize>, sources_count: usize) -> String {
    let mut cards_html = String::new();
    let collections = [
        (
            "palettes",
            "Palettes",
            "Color roles, OKLCH values, WCAG contrast",
        ),
        (
            "typography",
            "Typography",
            "Font families, type scales, line heights",
        ),
        (
            "components",
            "Components",
            "Buttons, inputs, cards across interaction states",
        ),
        ("icons", "Icons", "Sanitized SVG vectors and fingerprints"),
        (
            "effects",
            "Effects",
            "Elevation shadows, gradients, glassmorphism",
        ),
        (
            "animations",
            "Animations",
            "Keyframe timing, easing curves, CSS/WAAPI drivers",
        ),
        (
            "layouts",
            "Layouts",
            "Semantic wireframe skeletons and grid layouts",
        ),
    ];

    for (slug, title, desc) in collections {
        let count = counts.get(slug).copied().unwrap_or(0);
        cards_html.push_str(&format!(
            r#"<a href="/collections/{slug}" class="card">
  <div style="display:flex; justify-content:space-between; align-items:center;">
    <h3>{title}</h3>
    <span class="badge">{count} items</span>
  </div>
  <p style="color:var(--text-muted); font-size:13px; margin-top:8px;">{desc}</p>
</a>"#
        ));
    }

    let content = format!(
        r#"<div>
  <h2>Design Library Overview</h2>
  <p style="color:var(--text-muted); margin-top:4px;">
    Browsing local design library collections. ({sources_count} source captures recorded).
  </p>
  <div class="grid-cards">
    {cards_html}
  </div>
</div>"#
    );

    render_base_layout("Overview", &content)
}

pub fn render_collections_view(coll_type: &str, items: &[Value]) -> String {
    let mut items_html = String::new();
    for it in items {
        let id = it["id"].as_str().unwrap_or("unknown");
        let name = it["name"].as_str().unwrap_or(id);
        let conf = it["confidence"].as_f64().unwrap_or(1.0);
        let license = it["license"]["status"].as_str().unwrap_or("reference-only");

        items_html.push_str(&format!(
            r#"<a href="/items/{id}" class="card">
  <div style="display:flex; justify-content:space-between; align-items:center;">
    <h4>{name}</h4>
    <span class="badge">{:.0}% conf</span>
  </div>
  <p style="color:var(--text-muted); font-size:12px; margin-top:6px; font-family:monospace;">{id}</p>
  <div style="margin-top:12px; display:flex; gap:8px;">
    <span class="badge" style="background:#064e3b; color:#34d399;">{license}</span>
  </div>
</a>"#
        , conf * 100.0));
    }

    if items.is_empty() {
        items_html = r#"<p style="color:var(--text-muted); margin-top:16px;">No items in this collection yet. Capture a website to populate.</p>"#.to_string();
    }

    let content = format!(
        r#"<div>
  <h2>Collection: {coll_type}</h2>
  <p style="color:var(--text-muted); margin-top:4px;">Showing {} extracted design item(s).</p>
  <div class="grid-cards">
    {items_html}
  </div>
</div>"#,
        items.len()
    );

    render_base_layout(coll_type, &content)
}

pub fn render_item_detail(item: &Value) -> String {
    let id = item["id"].as_str().unwrap_or("unknown");
    let name = item["name"].as_str().unwrap_or(id);
    let item_type = item["type"].as_str().unwrap_or("unknown");
    let desc = item["description"].as_str().unwrap_or("");
    let json_pretty = serde_json::to_string_pretty(item).unwrap_or_default();

    let content = format!(
        r#"<div>
  <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:16px;">
    <div>
      <h2>{name}</h2>
      <p style="color:var(--text-muted); font-family:monospace; font-size:13px;">{id}</p>
    </div>
    <span class="badge" style="font-size:14px;">Type: {item_type}</span>
  </div>
  <p style="margin-bottom:20px;">{desc}</p>
  <h3 style="margin-bottom:8px;">Canonical Item Payload</h3>
  <pre>{json_pretty}</pre>
</div>"#
    );

    render_base_layout(name, &content)
}

pub fn render_sources_list(sources: &[String]) -> String {
    let mut sources_html = String::new();
    for s in sources {
        sources_html.push_str(&format!(
            r#"<div class="card" style="margin-bottom:12px;">
  <h4>{s}</h4>
  <p style="color:var(--text-muted); font-size:12px; margin-top:4px;">Recorded evidence, DOM snapshots, network log, and report.md</p>
</div>"#
        ));
    }

    if sources.is_empty() {
        sources_html = "<p style='color:var(--text-muted);'>No captured sources found in sources/ directory.</p>".to_string();
    }

    let content = format!(
        r#"<div>
  <h2>Captured Sources</h2>
  <p style="color:var(--text-muted); margin-top:4px; margin-bottom:20px;">History of website captures and evidence bundles.</p>
  <div>{sources_html}</div>
</div>"#
    );

    render_base_layout("Sources", &content)
}

pub fn render_export_view(tokens_css: &str, format: &str) -> String {
    let content = format!(
        r#"<div>
  <h2>Export Design Tokens</h2>
  <p style="color:var(--text-muted); margin-top:4px; margin-bottom:16px;">
    Export your extracted design system tokens. Supported formats: CSS Variables, Tailwind v4, Tailwind v3, DTCG.
  </p>
  <div style="display:flex; gap:12px; margin-bottom:20px;">
    <a href="/export?format=css" class="btn" style="text-decoration:none;">CSS Custom Properties</a>
    <a href="/export?format=tailwind" class="btn" style="text-decoration:none; background:#0284c7;">Tailwind v4 (@theme)</a>
    <a href="/export?format=dtcg" class="btn" style="text-decoration:none; background:#059669;">W3C DTCG Tokens</a>
  </div>
  <h3>Generated {format}</h3>
  <pre style="margin-top:12px;">{tokens_css}</pre>
</div>"#
    );

    render_base_layout("Export", &content)
}

pub fn render_search_view(query: &str, results_count: usize) -> String {
    let content = format!(
        r#"<div>
  <h2>Search Library</h2>
  <p style="color:var(--text-muted); margin-top:4px; margin-bottom:20px;">
    Filter by attributes: <code>type:palette</code>, <code>tag:dark</code>, <code>license:reusable</code>.
  </p>
  <form action="/search" method="GET" class="search-bar">
    <input type="text" name="q" class="input-text" placeholder="Search tokens, components, icons..." value="{query}" />
    <button type="submit" class="btn">Search</button>
  </form>
  <p style="color:var(--text-muted);">Found {results_count} matching item(s).</p>
</div>"#
    );

    render_base_layout("Search", &content)
}
