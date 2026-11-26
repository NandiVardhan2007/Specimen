#![forbid(unsafe_code)]

use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::header::{HeaderValue, CONTENT_TYPE};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Json, Response};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path as StdPath, PathBuf};

use crate::state::AppState;
use crate::views::{
    render_collections_view, render_compare_view, render_export_view, render_home,
    render_item_detail, render_search_view, render_sources_list,
};

pub fn count_collections(library_root: &StdPath) -> (BTreeMap<String, usize>, usize) {
    let mut counts = BTreeMap::new();
    let collections_dir = library_root.join("collections");
    if collections_dir.exists() {
        if let Ok(entries) = fs::read_dir(&collections_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let count = fs::read_dir(entry.path())
                        .map(|iter| {
                            iter.filter_map(std::result::Result::ok)
                                .filter(|e| e.path().is_dir())
                                .count()
                        })
                        .unwrap_or(0);
                    counts.insert(name, count);
                }
            }
        }
    }

    let mut sources_count = 0;
    let sources_dir = library_root.join("sources");
    if sources_dir.exists() {
        if let Ok(entries) = fs::read_dir(&sources_dir) {
            sources_count = entries
                .filter_map(std::result::Result::ok)
                .filter(|e| e.path().is_dir())
                .count();
        }
    }

    (counts, sources_count)
}

pub fn find_item_by_id(library_root: &StdPath, item_id: &str) -> Option<Value> {
    let collections_dir = library_root.join("collections");
    if collections_dir.exists() {
        if let Ok(coll_entries) = fs::read_dir(&collections_dir) {
            for coll_entry in coll_entries.flatten() {
                let candidate_file = coll_entry.path().join(item_id).join("item.json");
                if candidate_file.exists() {
                    if let Ok(content) = fs::read_to_string(&candidate_file) {
                        if let Ok(val) = serde_json::from_str::<Value>(&content) {
                            return Some(val);
                        }
                    }
                }
            }
        }
    }
    None
}

pub fn get_collection_items(library_root: &StdPath, coll_type: &str) -> Vec<Value> {
    let coll_dir = library_root.join("collections").join(coll_type);
    let mut items = Vec::new();
    if coll_dir.exists() {
        if let Ok(entries) = fs::read_dir(&coll_dir) {
            for entry in entries.flatten() {
                let item_file = entry.path().join("item.json");
                if item_file.exists() {
                    if let Ok(content) = fs::read_to_string(&item_file) {
                        if let Ok(val) = serde_json::from_str::<Value>(&content) {
                            items.push(val);
                        }
                    }
                }
            }
        }
    }
    items
}

// ==========================================
// REST API Handlers (Real Library Data)
// ==========================================

pub async fn handle_api_status(State(state): State<AppState>) -> impl IntoResponse {
    let (counts, sources_count) = count_collections(&state.library_root);
    Json(json!({
        "status": "online",
        "library_root": state.library_root.display().to_string(),
        "version": env!("CARGO_PKG_VERSION"),
        "counts": counts,
        "sources_count": sources_count
    }))
}

pub async fn handle_api_collection(
    State(state): State<AppState>,
    Path(coll_type): Path<String>,
) -> impl IntoResponse {
    let items = get_collection_items(&state.library_root, &coll_type);
    Json(items)
}

pub async fn handle_api_item(
    State(state): State<AppState>,
    Path(item_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    if let Some(val) = find_item_by_id(&state.library_root, &item_id) {
        Ok(Json(val))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn handle_api_sources(State(state): State<AppState>) -> impl IntoResponse {
    let mut sources = Vec::new();
    let sources_dir = state.library_root.join("sources");

    if sources_dir.exists() {
        if let Ok(entries) = fs::read_dir(&sources_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let host_name = entry.file_name().to_string_lossy().to_string();
                    if let Ok(sub_entries) = fs::read_dir(entry.path()) {
                        for sub in sub_entries.flatten() {
                            if sub.path().is_dir() {
                                let ts = sub.file_name().to_string_lossy().to_string();
                                let manifest_file = sub.path().join("manifest.json");
                                let has_manifest = manifest_file.exists();
                                sources.push(json!({
                                    "host": host_name,
                                    "timestamp": ts,
                                    "path": format!("{}/{}", host_name, ts),
                                    "has_manifest": has_manifest
                                }));
                            }
                        }
                    }
                }
            }
        }
    }

    Json(sources)
}

pub async fn handle_api_search(
    State(state): State<AppState>,
    Query(params): Query<BTreeMap<String, String>>,
) -> impl IntoResponse {
    let query_str = params.get("q").cloned().unwrap_or_default();
    if query_str.trim().is_empty() {
        return Json(json!([]));
    }
    let filter = specimen_library::SearchIndex::parse_query(&query_str);
    let hits =
        specimen_library::SearchIndex::search(&state.library_root, &filter).unwrap_or_default();
    Json(json!(hits))
}

pub async fn handle_api_export(
    State(state): State<AppState>,
    Query(params): Query<BTreeMap<String, String>>,
) -> impl IntoResponse {
    let format = params.get("format").map(|s| s.as_str()).unwrap_or("css");
    let content = specimen_export::export_tokens_to_string(&state.library_root, format)
        .unwrap_or_else(|e| format!("/* Export error: {} */", e));

    Json(json!({
        "format": format,
        "content": content
    }))
}

pub async fn handle_api_compare(
    State(state): State<AppState>,
    Query(params): Query<BTreeMap<String, String>>,
) -> impl IntoResponse {
    let id_a = params.get("a").map(|s| s.as_str()).unwrap_or_default();
    let id_b = params.get("b").map(|s| s.as_str()).unwrap_or_default();

    let item_a = if !id_a.is_empty() {
        find_item_by_id(&state.library_root, id_a)
    } else {
        None
    };

    let item_b = if !id_b.is_empty() {
        find_item_by_id(&state.library_root, id_b)
    } else {
        None
    };

    Json(json!({
        "id_a": id_a,
        "item_a": item_a,
        "id_b": id_b,
        "item_b": item_b
    }))
}

// ==========================================
// Static Assets & Web Frontend Servicing
// ==========================================

fn get_frontend_dist() -> PathBuf {
    let candidates = [
        PathBuf::from("frontend/dist"),
        PathBuf::from("../frontend/dist"),
        PathBuf::from("../../frontend/dist"),
    ];
    for c in candidates {
        if c.exists() && c.join("index.html").exists() {
            return c;
        }
    }
    PathBuf::from("frontend/dist")
}

pub async fn handle_index(State(state): State<AppState>) -> Response {
    let dist_index = get_frontend_dist().join("index.html");
    if dist_index.exists() {
        if let Ok(html) = fs::read_to_string(&dist_index) {
            let mut res = Html(html).into_response();
            res.headers_mut().insert(
                CONTENT_TYPE,
                HeaderValue::from_static("text/html; charset=utf-8"),
            );
            return res;
        }
    }

    let (counts, sources_count) = count_collections(&state.library_root);
    Html(render_home(&counts, sources_count)).into_response()
}

pub async fn handle_static_asset(Path(path): Path<String>) -> Response {
    let dist = get_frontend_dist();
    let asset_path = dist.join("assets").join(&path);

    if asset_path.exists() && asset_path.is_file() {
        if let Ok(bytes) = fs::read(&asset_path) {
            let mime = if path.ends_with(".js") {
                "application/javascript; charset=utf-8"
            } else if path.ends_with(".css") {
                "text/css; charset=utf-8"
            } else if path.ends_with(".svg") {
                "image/svg+xml"
            } else if path.ends_with(".png") {
                "image/png"
            } else if path.ends_with(".woff2") {
                "font/woff2"
            } else {
                "application/octet-stream"
            };

            let mut res = Response::new(Body::from(bytes));
            res.headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static(mime));
            return res;
        }
    }

    StatusCode::NOT_FOUND.into_response()
}

pub async fn handle_fallback(State(state): State<AppState>) -> Response {
    let dist_index = get_frontend_dist().join("index.html");
    if dist_index.exists() {
        if let Ok(html) = fs::read_to_string(&dist_index) {
            let mut res = Html(html).into_response();
            res.headers_mut().insert(
                CONTENT_TYPE,
                HeaderValue::from_static("text/html; charset=utf-8"),
            );
            return res;
        }
    }

    let (counts, sources_count) = count_collections(&state.library_root);
    Html(render_home(&counts, sources_count)).into_response()
}

// Server-rendered views (backwards compatibility / fallback)
pub async fn handle_home(State(state): State<AppState>) -> impl IntoResponse {
    let (counts, sources_count) = count_collections(&state.library_root);
    Html(render_home(&counts, sources_count))
}

pub async fn handle_collection(
    State(state): State<AppState>,
    Path(coll_type): Path<String>,
) -> impl IntoResponse {
    let items = get_collection_items(&state.library_root, &coll_type);
    Html(render_collections_view(&coll_type, &items))
}

pub async fn handle_item_detail(
    State(state): State<AppState>,
    Path(item_id): Path<String>,
) -> Result<Html<String>, StatusCode> {
    if let Some(val) = find_item_by_id(&state.library_root, &item_id) {
        Ok(Html(render_item_detail(&val)))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn handle_sources(State(state): State<AppState>) -> impl IntoResponse {
    let mut sources = Vec::new();
    let sources_dir = state.library_root.join("sources");

    if sources_dir.exists() {
        if let Ok(entries) = fs::read_dir(&sources_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let host_name = entry.file_name().to_string_lossy().to_string();
                    if let Ok(sub_entries) = fs::read_dir(entry.path()) {
                        for sub in sub_entries.flatten() {
                            if sub.path().is_dir() {
                                let ts = sub.file_name().to_string_lossy().to_string();
                                sources.push(format!("{}/{}", host_name, ts));
                            }
                        }
                    }
                }
            }
        }
    }

    Html(render_sources_list(&sources))
}

pub async fn handle_search(
    State(state): State<AppState>,
    Query(params): Query<BTreeMap<String, String>>,
) -> impl IntoResponse {
    let query_str = params.get("q").cloned().unwrap_or_default();
    let hits_count = if query_str.trim().is_empty() {
        0
    } else {
        let filter = specimen_library::SearchIndex::parse_query(&query_str);
        specimen_library::SearchIndex::search(&state.library_root, &filter)
            .map(|h| h.len())
            .unwrap_or(0)
    };

    Html(render_search_view(&query_str, hits_count))
}

pub async fn handle_export(
    State(state): State<AppState>,
    Query(params): Query<BTreeMap<String, String>>,
) -> impl IntoResponse {
    let format = params.get("format").map(|s| s.as_str()).unwrap_or("css");
    let export_text = specimen_export::export_tokens_to_string(&state.library_root, format)
        .unwrap_or_else(|e| format!("/* Export error: {} */", e));

    Html(render_export_view(&export_text, format))
}

pub async fn handle_compare(
    State(state): State<AppState>,
    Query(params): Query<BTreeMap<String, String>>,
) -> impl IntoResponse {
    let id_a = params.get("a").map(|s| s.as_str()).unwrap_or_default();
    let id_b = params.get("b").map(|s| s.as_str()).unwrap_or_default();

    let item_a = if !id_a.is_empty() {
        find_item_by_id(&state.library_root, id_a)
    } else {
        None
    };

    let item_b = if !id_b.is_empty() {
        find_item_by_id(&state.library_root, id_b)
    } else {
        None
    };

    Html(render_compare_view(
        item_a.as_ref(),
        item_b.as_ref(),
        id_a,
        id_b,
    ))
}
