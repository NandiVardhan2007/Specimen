#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;

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

    /// Searches items in a library against parsed filters and ranks results.
    pub fn search(library_root: &Path, filter: &SearchFilter) -> Result<Vec<SearchHit>> {
        let mut hits = Vec::new();
        let collections_dir = library_root.join("collections");
        if !collections_dir.exists() {
            return Ok(hits);
        }

        for entry in fs::read_dir(collections_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                for item_entry in fs::read_dir(&path)? {
                    let item_entry = item_entry?;
                    let item_file = item_entry.path().join("item.json");
                    if item_file.exists() {
                        if let Ok(content) = fs::read_to_string(&item_file) {
                            if let Ok(item) = serde_json::from_str::<Item<Value>>(&content) {
                                if let Some(hit) = Self::evaluate_item(&item, filter) {
                                    hits.push(hit);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Rank by score descending
        hits.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Ok(hits)
    }

    fn evaluate_item(item: &Item<Value>, filter: &SearchFilter) -> Option<SearchHit> {
        // 1. Type filter
        if let Some(expected_type) = filter.item_type {
            if item.item_type != expected_type {
                return None;
            }
        }

        // 2. License filter
        if let Some(ref expected_lic) = filter.license {
            let lic_str = match item.license.status {
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
            let has_tag = item.tags.iter().any(|t| t.to_lowercase() == *expected_tag);
            if !has_tag {
                return None;
            }
        }

        // 4. Source filter
        if let Some(ref expected_src) = filter.source {
            let has_src = item
                .provenance
                .iter()
                .any(|p| p.source.to_lowercase().contains(expected_src));
            if !has_src {
                return None;
            }
        }

        // 5. Scheme filter (for palettes or themes)
        if let Some(ref expected_scheme) = filter.scheme {
            let scheme_in_data = item
                .data
                .get("scheme")
                .and_then(|v| v.as_str())
                .map(|s| s.to_lowercase());
            let scheme_in_prov = item
                .provenance
                .iter()
                .any(|p| p.scheme.to_lowercase() == *expected_scheme);
            if scheme_in_data.as_deref() != Some(expected_scheme) && !scheme_in_prov {
                return None;
            }
        }

        // 6. Text terms matching & scoring
        let mut score = 1.0;
        let mut text_matched = true;

        let name_str = item.name.clone().unwrap_or_default().to_lowercase();
        let desc_str = item.description.clone().unwrap_or_default().to_lowercase();
        let slug_str = item.slug.to_lowercase();
        let id_str = item.id.as_str().to_lowercase();

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
            if item.tags.iter().any(|tag| tag.to_lowercase().contains(&t)) {
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
            id: item.id.clone(),
            item_type: item.item_type,
            name: item.name.clone(),
            description: item.description.clone(),
            tags: item.tags.iter().cloned().collect(),
            license_status: item.license.status,
            confidence: item.confidence,
            score: score * item.confidence,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_query_filters_and_terms() {
        let q = "type:effect tag:glassmorphism license:reusable glass card";
        let f = SearchIndex::parse_query(q);
        assert_eq!(f.item_type, Some(ItemType::Effect));
        assert_eq!(f.tag.as_deref(), Some("glassmorphism"));
        assert_eq!(f.license.as_deref(), Some("reusable"));
        assert_eq!(f.text_terms, vec!["glass", "card"]);
    }
}
