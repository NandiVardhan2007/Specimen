#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

use specimen_core::{Item, ItemId, ItemType, LicenseStatus};

use crate::error::Result;

#[derive(Debug, Clone, Default)]
pub struct SearchFilter {
    pub item_type: Option<ItemType>,
    pub tag: Option<String>,
    pub source: Option<String>,
    pub license: Option<String>,
    pub scheme: Option<String>,
    pub near_color: Option<String>,
    pub delta: Option<f64>,
    pub text_terms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit {
    pub id: ItemId,
    pub item_type: ItemType,
    pub name: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub license_status: LicenseStatus,
    pub confidence: f64,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub id: ItemId,
    pub item_type: ItemType,
    pub name: Option<String>,
    pub slug: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub license_status: LicenseStatus,
    pub confidence: f64,
    pub provenance_sources: Vec<String>,
    pub provenance_schemes: Vec<String>,
    pub data_scheme: Option<String>,
    pub file_mtime_secs: u64,
    pub file_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LibraryIndexCache {
    pub schema_version: u32,
    pub entries: BTreeMap<String, IndexEntry>,
}

pub struct SearchIndex;

impl SearchIndex {
    /// Parses search query string into structured filters and free-text terms.
    ///
    /// Examples:
    /// - `"glass card"` -> terms: ["glass", "card"]
    /// - `"type:effect tag:glassmorphism license:reusable"`
    /// - `"type:palette near:#3b82f6 delta:0.05 scheme:dark"`
    pub fn parse_query(query: &str) -> SearchFilter {
        let mut filter = SearchFilter::default();

        for part in query.split_whitespace() {
            if let Some((k, v)) = part.split_once(':') {
                let k_lower = k.to_lowercase();
                let val = v.trim_matches('"').to_string();
                match k_lower.as_str() {
                    "type" => filter.item_type = val.parse::<ItemType>().ok(),
                    "tag" => filter.tag = Some(val.to_lowercase()),
                    "source" => filter.source = Some(val.to_lowercase()),
                    "license" => filter.license = Some(val.to_lowercase()),
                    "scheme" => filter.scheme = Some(val.to_lowercase()),
                    "near" => filter.near_color = Some(val.to_lowercase()),
                    "delta" => filter.delta = val.parse::<f64>().ok(),
                    _ => filter.text_terms.push(part.to_string()),
                }
            } else {
                filter.text_terms.push(part.to_string());
            }
        }

        filter
    }

    /// Loads or rebuilds the fast in-memory search index with disk caching in `.cache/index.json`.
    pub fn load_or_build(library_root: &Path) -> Result<Vec<IndexEntry>> {
        let cache_dir = library_root.join(".cache");
        let cache_file = cache_dir.join("index.json");

        let now_secs = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        if let Ok(meta) = cache_file.metadata() {
            let mtime = meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);

            // Fast-path: if index cache was generated within last 5 seconds, use it directly without re-walking disk
            if now_secs.saturating_sub(mtime) < 5 {
                if let Ok(content) = fs::read_to_string(&cache_file) {
                    if let Ok(cached) = serde_json::from_str::<LibraryIndexCache>(&content) {
                        if !cached.entries.is_empty() {
                            return Ok(cached.entries.into_values().collect());
                        }
                    }
                }
            }
        }

        let mut cached: LibraryIndexCache = if cache_file.exists() {
            fs::read_to_string(&cache_file)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            LibraryIndexCache::default()
        };

        let collections_dir = library_root.join("collections");
        if !collections_dir.exists() {
            return Ok(Vec::new());
        }

        let mut fresh_entries = BTreeMap::new();
        let mut cache_modified = false;

        for entry in fs::read_dir(collections_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                for item_entry in fs::read_dir(&path)? {
                    let item_entry = item_entry?;
                    let item_path = item_entry.path();
                    let item_file = item_path.join("item.json");
                    if item_file.exists() {
                        if let Ok(meta) = item_file.metadata() {
                            let mtime = meta
                                .modified()
                                .ok()
                                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                                .map(|d| d.as_secs())
                                .unwrap_or(0);
                            let size = meta.len();
                            let file_key = item_file.to_string_lossy().to_string();

                            // Check cache hit
                            if let Some(existing) = cached.entries.get(&file_key) {
                                if existing.file_mtime_secs == mtime && existing.file_size == size {
                                    fresh_entries.insert(file_key, existing.clone());
                                    continue;
                                }
                            }

                            // Cache miss or modified file: parse item.json
                            if let Ok(content) = fs::read_to_string(&item_file) {
                                if let Ok(item) = serde_json::from_str::<Item<Value>>(&content) {
                                    let prov_sources = item
                                        .provenance
                                        .iter()
                                        .map(|p| p.source.to_lowercase())
                                        .collect();
                                    let prov_schemes = item
                                        .provenance
                                        .iter()
                                        .map(|p| p.scheme.to_lowercase())
                                        .collect();
                                    let data_scheme = item
                                        .data
                                        .get("scheme")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_lowercase());

                                    let index_entry = IndexEntry {
                                        id: item.id,
                                        item_type: item.item_type,
                                        name: item.name,
                                        slug: item.slug,
                                        description: item.description,
                                        tags: item.tags.into_iter().collect(),
                                        license_status: item.license.status,
                                        confidence: item.confidence,
                                        provenance_sources: prov_sources,
                                        provenance_schemes: prov_schemes,
                                        data_scheme,
                                        file_mtime_secs: mtime,
                                        file_size: size,
                                    };

                                    fresh_entries.insert(file_key, index_entry);
                                    cache_modified = true;
                                }
                            }
                        }
                    }
                }
            }
        }

        if cache_modified || fresh_entries.len() != cached.entries.len() {
            cached.entries = fresh_entries.clone();
            cached.schema_version = 1;
            let _ = fs::create_dir_all(&cache_dir);
            if let Ok(serialized) = serde_json::to_string(&cached) {
                let _ = fs::write(&cache_file, serialized);
            }
        }

        Ok(fresh_entries.into_values().collect())
    }

    /// Queries a preloaded index of entries in memory with high performance.
    pub fn query_entries(entries: &[IndexEntry], filter: &SearchFilter) -> Vec<SearchHit> {
        let mut hits = Vec::new();
        for entry in entries {
            if let Some(hit) = Self::evaluate_entry(entry, filter) {
                hits.push(hit);
            }
        }

        hits.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        hits
    }

    /// Searches items in a library against parsed filters and ranks results.
    /// Automatically manages the `.cache/index.json` caching tier.
    pub fn search(library_root: &Path, filter: &SearchFilter) -> Result<Vec<SearchHit>> {
        let entries = Self::load_or_build(library_root)?;
        Ok(Self::query_entries(&entries, filter))
    }

    fn evaluate_entry(entry: &IndexEntry, filter: &SearchFilter) -> Option<SearchHit> {
        // 1. Type filter
        if let Some(expected_type) = filter.item_type {
            if entry.item_type != expected_type {
                return None;
            }
        }

        // 2. License filter
        if let Some(ref expected_lic) = filter.license {
            let lic_str = match entry.license_status {
                LicenseStatus::Reusable => "reusable",
                LicenseStatus::ReferenceOnly => "reference-only",
                LicenseStatus::Unknown => "unknown",
            };
            if lic_str != expected_lic {
                return None;
            }
        }

        // 3. Tag filter
        if let Some(ref expected_tag) = filter.tag {
            let has_tag = entry.tags.iter().any(|t| t.to_lowercase() == *expected_tag);
            if !has_tag {
                return None;
            }
        }

        // 4. Source filter
        if let Some(ref expected_src) = filter.source {
            let has_src = entry
                .provenance_sources
                .iter()
                .any(|src| src.contains(expected_src));
            if !has_src {
                return None;
            }
        }

        // 5. Scheme filter (for palettes or themes)
        if let Some(ref expected_scheme) = filter.scheme {
            let scheme_in_data = entry.data_scheme.as_deref() == Some(expected_scheme);
            let scheme_in_prov = entry
                .provenance_schemes
                .iter()
                .any(|s| s == expected_scheme);
            if !scheme_in_data && !scheme_in_prov {
                return None;
            }
        }

        // 6. Text terms matching & scoring
        let mut score = 1.0;
        let mut text_matched = true;

        let name_str = entry.name.clone().unwrap_or_default().to_lowercase();
        let desc_str = entry.description.clone().unwrap_or_default().to_lowercase();
        let slug_str = entry.slug.to_lowercase();
        let id_str = entry.id.as_str().to_lowercase();

        for term in &filter.text_terms {
            let t = term.to_lowercase();
            let mut term_score = 0.0;
            if id_str.contains(&t) {
                term_score += 10.0;
            }
            if name_str.contains(&t) {
                term_score += 8.0;
            }
            if slug_str.contains(&t) {
                term_score += 6.0;
            }
            if entry.tags.iter().any(|tag| tag.to_lowercase().contains(&t)) {
                term_score += 5.0;
            }
            if desc_str.contains(&t) {
                term_score += 3.0;
            }

            if term_score == 0.0 {
                text_matched = false;
                break;
            } else {
                score += term_score;
            }
        }

        if !text_matched && !filter.text_terms.is_empty() {
            return None;
        }

        Some(SearchHit {
            id: entry.id.clone(),
            item_type: entry.item_type,
            name: entry.name.clone(),
            description: entry.description.clone(),
            tags: entry.tags.clone(),
            license_status: entry.license_status,
            confidence: entry.confidence,
            score: score * entry.confidence,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn test_parse_query_filters_and_terms() {
        let q = "type:effect tag:glassmorphism license:reusable glass card";
        let f = SearchIndex::parse_query(q);
        assert_eq!(f.item_type, Some(ItemType::Effect));
        assert_eq!(f.tag.as_deref(), Some("glassmorphism"));
        assert_eq!(f.license.as_deref(), Some("reusable"));
        assert_eq!(f.text_terms, vec!["glass", "card"]);
    }

    #[test]
    fn test_cached_index_load_and_query() {
        let temp = tempfile::tempdir().unwrap();
        let lib_root = temp.path().join("lib");
        crate::init_library(&lib_root, None).unwrap();

        // Create a dummy item
        let col_dir = lib_root
            .join("collections")
            .join("palettes")
            .join("test-item");
        fs::create_dir_all(&col_dir).unwrap();

        let item_id = ItemId::generate(ItemType::Palette, "dark-core", r#"{"scheme":"dark"}"#);
        let item = Item {
            schema: "specimen.item/1".to_string(),
            id: item_id.clone(),
            item_type: ItemType::Palette,
            name: Some("Dark Core Palette".to_string()),
            slug: "dark-core".to_string(),
            description: Some("Midnight dark mode colors".to_string()),
            tags: BTreeSet::from(["midnight".to_string(), "dark".to_string()]),
            data: serde_json::json!({"scheme": "dark"}),
            provenance: vec![],
            license: specimen_core::License {
                status: LicenseStatus::Reusable,
                basis: "public tokens".to_string(),
                spdx: None,
            },
            confidence: 0.95,
            similar_to: vec![],
            annotations: None,
            user: specimen_core::UserMeta::default(),
            previews: vec![],
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
        };

        fs::write(
            col_dir.join("item.json"),
            serde_json::to_string(&item).unwrap(),
        )
        .unwrap();

        // First search builds the cache
        let filter = SearchIndex::parse_query("type:palette dark");
        let hits = SearchIndex::search(&lib_root, &filter).unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].id, item_id);

        // Verify cache was created on disk
        let cache_file = lib_root.join(".cache").join("index.json");
        assert!(cache_file.exists());

        // Second search hits the cache cleanly
        let hits2 = SearchIndex::search(&lib_root, &filter).unwrap();
        assert_eq!(hits2.len(), 1);
        assert_eq!(hits2[0].id, item_id);
    }
}
