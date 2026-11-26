#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;

use specimen_core::to_canonical_json;

use crate::error::Result;

/// Outcome report of a library schema migration run.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MigrationReport {
    pub items_checked: usize,
    pub items_migrated: usize,
    pub errors: Vec<String>,
}

/// Migrates items in `collections/` to the current schema specification (`specimen.item/1`).
pub fn migrate_library(library_root: &Path) -> Result<MigrationReport> {
    let mut report = MigrationReport::default();
    let collections_dir = library_root.join("collections");
    if !collections_dir.exists() {
        return Ok(report);
    }

    for coll in fs::read_dir(collections_dir)? {
        let coll = coll?;
        if coll.path().is_dir() {
            for item_dir in fs::read_dir(coll.path())? {
                let item_dir = item_dir?;
                if item_dir.path().is_dir() {
                    let item_file = item_dir.path().join("item.json");
                    if item_file.exists() {
                        report.items_checked += 1;
                        if let Ok(content) = fs::read_to_string(&item_file) {
                            if let Ok(mut val) = serde_json::from_str::<Value>(&content) {
                                let mut needs_write = false;

                                // 1. Schema version check
                                if val.get("schema").and_then(|v| v.as_str())
                                    != Some("specimen.item/1")
                                {
                                    val["schema"] = Value::String("specimen.item/1".to_string());
                                    needs_write = true;
                                }

                                // 2. Ensure similar_to array exists
                                if val.get("similar_to").is_none() {
                                    val["similar_to"] = Value::Array(Vec::new());
                                    needs_write = true;
                                }

                                // 3. Ensure user metadata exists
                                if val.get("user").is_none() {
                                    val["user"] = serde_json::json!({
                                        "favorite": false,
                                        "notes": ""
                                    });
                                    needs_write = true;
                                }

                                if needs_write {
                                    if let Ok(canonical) = to_canonical_json(&val) {
                                        if fs::write(&item_file, canonical).is_ok() {
                                            report.items_migrated += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_migrate_legacy_item() {
        let temp = tempfile::tempdir().unwrap();
        let item_dir = temp
            .path()
            .join("collections")
            .join("palettes")
            .join("test-item");
        fs::create_dir_all(&item_dir).unwrap();

        let legacy = json!({
            "schema": "specimen.item/0",
            "id": "palette-test-12345678",
            "type": "palette",
            "data": {},
            "provenance": [],
            "license": { "status": "reusable", "basis": "test", "spdx": null },
            "confidence": 1.0,
            "created_at": "2026-11-26T00:00:00Z",
            "updated_at": "2026-11-26T00:00:00Z"
        });

        fs::write(item_dir.join("item.json"), legacy.to_string()).unwrap();

        let report = migrate_library(temp.path()).unwrap();
        assert_eq!(report.items_checked, 1);
        assert_eq!(report.items_migrated, 1);

        let migrated: Value =
            serde_json::from_str(&fs::read_to_string(item_dir.join("item.json")).unwrap()).unwrap();
        assert_eq!(migrated["schema"], "specimen.item/1");
        assert!(migrated["similar_to"].is_array());
    }
}
