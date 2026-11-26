#![forbid(unsafe_code)]

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Json};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;

use crate::state::AppState;
use crate::views::{
    render_collections_view, render_compare_view, render_export_view, render_home,
    render_item_detail, render_search_view, render_sources_list,
};

pub async fn handle_home(State(state): State<AppState>) -> impl IntoResponse {
    let mut counts = BTreeMap::new();
    let collections_dir = state.library_root.join("collections");
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
    let sources_dir = state.library_root.join("sources");
    if sources_dir.exists() {
        if let Ok(entries) = fs::read_dir(&sources_dir) {
            sources_count = entries
                .filter_map(std::result::Result::ok)
                .filter(|e| e.path().is_dir())
                .count();
        }
    }

    Html(render_home(&counts, sources_count))
}

pub async fn handle_collection(
    State(state): State<AppState>,
    Path(coll_type): Path<String>,
) -> impl IntoResponse {
    let coll_dir = state.library_root.join("collections").join(&coll_type);
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

    Html(render_collections_view(&coll_type, &items))
}

pub fn find_item_by_id(library_root: &std::path::Path, item_id: &str) -> Option<Value> {
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

pub async fn handle_api_status(State(state): State<AppState>) -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "library_root": state.library_root.display().to_string(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}
