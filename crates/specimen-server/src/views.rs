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
      <a href="/compare">Compare</a>
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
    let item_type = item["item_type"]
        .as_str()
        .or_else(|| item["type"].as_str())
        .unwrap_or("unknown");
    let desc = item["description"].as_str().unwrap_or("");
    let json_pretty = serde_json::to_string_pretty(item).unwrap_or_default();

    let mut similar_html = String::new();
    if let Some(similar_ids) = item.get("similar_to").and_then(|v| v.as_array()) {
        if !similar_ids.is_empty() {
            let mut list_items = String::new();
            for sim_val in similar_ids {
                if let Some(sim_id) = sim_val.as_str() {
                    list_items.push_str(&format!(
                        r#"<div style="display:flex; justify-content:space-between; align-items:center; padding:8px 0; border-bottom:1px solid var(--border);">
  <a href="/items/{sim_id}" style="color:var(--accent); font-family:monospace; text-decoration:none;">{sim_id}</a>
  <a href="/compare?a={id}&b={sim_id}" class="badge" style="text-decoration:none; background:var(--surface-hover);">Compare side-by-side &rarr;</a>
</div>"#
                    ));
                }
            }
            similar_html = format!(
                r#"<div style="margin-top:24px; padding:16px; background:var(--surface); border:1px solid var(--border); border-radius:8px;">
  <h3 style="margin-bottom:8px;">Similar Items Detected ({})</h3>
  <p style="color:var(--text-muted); font-size:13px; margin-bottom:12px;">Items identified with matching color distributions, typography, or roles:</p>
  {list_items}
</div>"#,
                similar_ids.len()
            );
        }
    }

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
  {similar_html}
  <h3 style="margin-top:24px; margin-bottom:8px;">Canonical Item Payload</h3>
  <pre>{json_pretty}</pre>
</div>"#
    );

    render_base_layout(name, &content)
}

pub fn render_compare_view(
    item_a: Option<&Value>,
    item_b: Option<&Value>,
    id_a: &str,
    id_b: &str,
) -> String {
    let col_a = match item_a {
        Some(a) => {
            let name = a["name"].as_str().unwrap_or(id_a);
            let itype = a["item_type"]
                .as_str()
                .or_else(|| a["type"].as_str())
                .unwrap_or("unknown");
            let conf = a["confidence"].as_f64().unwrap_or(1.0);
            let lic = a["license"]["status"].as_str().unwrap_or("unknown");
            let pretty = serde_json::to_string_pretty(a).unwrap_or_default();
            format!(
                r#"<div style="flex:1; min-width:320px; background:var(--surface); border:1px solid var(--border); border-radius:8px; padding:20px;">
  <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:12px;">
    <h3>{name}</h3>
    <span class="badge">{itype}</span>
  </div>
  <p style="font-family:monospace; font-size:12px; color:var(--text-muted); margin-bottom:12px;">{id_a}</p>
  <div style="display:flex; gap:8px; margin-bottom:16px;">
    <span class="badge" style="background:#064e3b; color:#34d399;">{lic}</span>
    <span class="badge">{:.0}% conf</span>
  </div>
  <pre>{pretty}</pre>
</div>"#,
                conf * 100.0
            )
        }
        None => format!(
            r#"<div style="flex:1; min-width:320px; background:var(--surface); border:1px dashed var(--border); border-radius:8px; padding:20px; text-align:center;">
  <p style="color:var(--text-muted);">Item A ({id_a}) not found or not specified.</p>
</div>"#
        ),
    };

    let col_b = match item_b {
        Some(b) => {
            let name = b["name"].as_str().unwrap_or(id_b);
            let itype = b["item_type"]
                .as_str()
                .or_else(|| b["type"].as_str())
                .unwrap_or("unknown");
            let conf = b["confidence"].as_f64().unwrap_or(1.0);
            let lic = b["license"]["status"].as_str().unwrap_or("unknown");
            let pretty = serde_json::to_string_pretty(b).unwrap_or_default();
            format!(
                r#"<div style="flex:1; min-width:320px; background:var(--surface); border:1px solid var(--border); border-radius:8px; padding:20px;">
  <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:12px;">
    <h3>{name}</h3>
    <span class="badge">{itype}</span>
  </div>
  <p style="font-family:monospace; font-size:12px; color:var(--text-muted); margin-bottom:12px;">{id_b}</p>
  <div style="display:flex; gap:8px; margin-bottom:16px;">
    <span class="badge" style="background:#064e3b; color:#34d399;">{lic}</span>
    <span class="badge">{:.0}% conf</span>
  </div>
  <pre>{pretty}</pre>
</div>"#,
                conf * 100.0
            )
        }
        None => format!(
            r#"<div style="flex:1; min-width:320px; background:var(--surface); border:1px dashed var(--border); border-radius:8px; padding:20px; text-align:center;">
  <p style="color:var(--text-muted);">Item B ({id_b}) not found or not specified.</p>
</div>"#
        ),
    };

    let content = format!(
        r#"<div>
  <h2>Item Comparison & Merge Review</h2>
  <p style="color:var(--text-muted); margin-top:4px; margin-bottom:20px;">
    Compare two design system items side-by-side to review differences or verify deduplication candidates.
  </p>
  <form action="/compare" method="GET" class="search-bar" style="margin-bottom:24px;">
    <input type="text" name="a" class="input-text" placeholder="Item A ID..." value="{id_a}" style="flex:1;" />
    <input type="text" name="b" class="input-text" placeholder="Item B ID..." value="{id_b}" style="flex:1;" />
    <button type="submit" class="btn">Compare</button>
  </form>
  <div style="display:flex; gap:20px; flex-wrap:wrap;">
    {col_a}
    {col_b}
  </div>
</div>"#
    );

    render_base_layout("Compare", &content)
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
    Export your extracted design system tokens. Supported formats: CSS Variables, Tailwind v4, Tailwind v3, DTCG, SCSS, Tokens Studio.
  </p>
  <div style="display:flex; gap:12px; margin-bottom:20px; flex-wrap:wrap;">
    <a href="/export?format=css" class="btn" style="text-decoration:none;">CSS Custom Properties</a>
    <a href="/export?format=tailwind" class="btn" style="text-decoration:none; background:#0284c7;">Tailwind v4 (@theme)</a>
    <a href="/export?format=tailwind-v3" class="btn" style="text-decoration:none; background:#0369a1;">Tailwind v3 Config</a>
    <a href="/export?format=dtcg" class="btn" style="text-decoration:none; background:#059669;">W3C DTCG Tokens</a>
    <a href="/export?format=scss" class="btn" style="text-decoration:none; background:#d97706;">SCSS Variables</a>
    <a href="/export?format=tokens-studio" class="btn" style="text-decoration:none; background:#7c3aed;">Tokens Studio</a>
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
