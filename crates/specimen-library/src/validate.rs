#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;

use specimen_core::{
    to_canonical_json, Config, EffectData, IconData, Item, ItemId, ItemType, LayoutData,
    PaletteData, ThemeData, TypographyData,
};

use crate::error::Result;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ValidationReport {
    pub items_checked: usize,
    pub items_valid: usize,
    pub sources_checked: usize,
    pub sources_valid: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationReport {
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}

/// Validates a Specimen design library folder against schemas and invariants.
pub fn validate_library(library_root: &Path) -> Result<ValidationReport> {
    let mut report = ValidationReport::default();

    // 1. Validate specimen.toml if present
    let config_path = library_root.join("specimen.toml");
    if config_path.exists() {
        if let Err(e) = Config::load_from_path(&config_path) {
            report
                .errors
                .push(format!("specimen.toml syntax error: {}", e));
        }
    }

    // 2. Validate items in collections/
    let collections_dir = library_root.join("collections");
    if collections_dir.exists() && collections_dir.is_dir() {
        validate_collections_dir(&collections_dir, &mut report)?;
    }

    // 3. Validate sources in sources/
    let sources_dir = library_root.join("sources");
    if sources_dir.exists() && sources_dir.is_dir() {
        validate_sources_dir(&sources_dir, &mut report)?;
    }

    Ok(report)
}

fn validate_collections_dir(collections_dir: &Path, report: &mut ValidationReport) -> Result<()> {
    for entry in fs::read_dir(collections_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let collection_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();

            // Iterate items in collection
            for item_entry in fs::read_dir(&path)? {
                let item_entry = item_entry?;
                let item_dir = item_entry.path();
                if item_dir.is_dir() {
                    let item_file = item_dir.join("item.json");
                    if item_file.exists() {
                        validate_item_file(&item_file, collection_name, report)?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn validate_item_file(
    item_file: &Path,
    expected_collection: &str,
    report: &mut ValidationReport,
) -> Result<()> {
    report.items_checked += 1;
    let content = match fs::read_to_string(item_file) {
        Ok(c) => c,
        Err(e) => {
            report.errors.push(format!(
                "Failed to read item file {}: {}",
                item_file.display(),
                e
            ));
            return Ok(());
        }
    };

    let item: Item<Value> = match serde_json::from_str(&content) {
        Ok(item) => item,
        Err(e) => {
            report.errors.push(format!(
                "Item JSON parse error in {}: {}",
                item_file.display(),
                e
            ));
            return Ok(());
        }
    };

    let mut item_valid = true;

    // Check schema version
    if item.schema != "specimen.item/1" {
        report.errors.push(format!(
            "{}: unsupported schema '{}', expected 'specimen.item/1'",
            item_file.display(),
            item.schema
        ));
        item_valid = false;
    }

    // Check collection folder match
    if item.item_type.collection_dir_name() != expected_collection {
        report.errors.push(format!(
            "{}: item type '{:?}' does not match collection folder '{}'",
            item_file.display(),
            item.item_type,
            expected_collection
        ));
        item_valid = false;
    }

    // Content hash determinism and integrity verification
    match to_canonical_json(&item.data) {
        Ok(canonical_str) => {
            let expected_id = ItemId::generate(item.item_type, item.id.slug(), &canonical_str);
            if expected_id.as_str() != item.id.as_str() {
                report.errors.push(format!(
                    "{}: Item ID tamper detected! ID is '{}' but computed hash from canonical data is '{}'",
                    item_file.display(),
                    item.id,
                    expected_id
                ));
                item_valid = false;
            }
        }
        Err(e) => {
            report.errors.push(format!(
                "{}: Failed to canonicalize item data: {}",
                item_file.display(),
                e
            ));
            item_valid = false;
        }
    }

    // Type-specific schema validation
    match item.item_type {
        ItemType::Palette => {
            if let Err(e) = serde_json::from_value::<PaletteData>(item.data) {
                report.errors.push(format!(
                    "{}: PaletteData schema validation failed: {}",
                    item_file.display(),
                    e
                ));
                item_valid = false;
            }
        }
        ItemType::Typography => {
            if let Err(e) = serde_json::from_value::<TypographyData>(item.data) {
                report.errors.push(format!(
                    "{}: TypographyData schema validation failed: {}",
                    item_file.display(),
                    e
                ));
                item_valid = false;
            }
        }
        ItemType::Theme => {
            if let Err(e) = serde_json::from_value::<ThemeData>(item.data) {
                report.errors.push(format!(
                    "{}: ThemeData schema validation failed: {}",
                    item_file.display(),
                    e
                ));
                item_valid = false;
            }
        }
        ItemType::Icon => {
            if let Err(e) = serde_json::from_value::<IconData>(item.data) {
                report.errors.push(format!(
                    "{}: IconData schema validation failed: {}",
                    item_file.display(),
                    e
                ));
                item_valid = false;
            }
        }
        ItemType::Effect => {
            if let Err(e) = serde_json::from_value::<EffectData>(item.data) {
                report.errors.push(format!(
                    "{}: EffectData schema validation failed: {}",
                    item_file.display(),
                    e
                ));
                item_valid = false;
            }
        }
        ItemType::Layout => {
            if let Err(e) = serde_json::from_value::<LayoutData>(item.data) {
                report.errors.push(format!(
                    "{}: LayoutData schema validation failed: {}",
                    item_file.display(),
                    e
                ));
                item_valid = false;
            }
        }
        _ => {}
    }

    // License validation
    if item.license.basis.trim().is_empty() {
        report.warnings.push(format!(
            "{}: item has empty license basis",
            item_file.display()
        ));
    }

    // Provenance validation
    if item.provenance.is_empty() {
        report.errors.push(format!(
            "{}: item must have at least one provenance record",
            item_file.display()
        ));
        item_valid = false;
    }

    if item_valid {
        report.items_valid += 1;
    }

    Ok(())
}

fn validate_sources_dir(sources_dir: &Path, report: &mut ValidationReport) -> Result<()> {
    for host_entry in fs::read_dir(sources_dir)? {
        let host_entry = host_entry?;
        if host_entry.path().is_dir() {
            for ts_entry in fs::read_dir(host_entry.path())? {
                let ts_entry = ts_entry?;
                let path = ts_entry.path();
                if path.is_dir() {
                    let manifest_file = path.join("manifest.json");
                    if manifest_file.exists() {
                        report.sources_checked += 1;
                        match fs::read_to_string(&manifest_file) {
                            Ok(c) => match serde_json::from_str::<Value>(&c) {
                                Ok(val) => {
                                    if val.get("url").is_some() && val.get("captured_at").is_some()
                                    {
                                        report.sources_valid += 1;
                                    } else {
                                        report.errors.push(format!(
                                            "{}: manifest missing required 'url' or 'captured_at'",
                                            manifest_file.display()
                                        ));
                                    }
                                }
                                Err(e) => {
                                    report.errors.push(format!(
                                        "{}: manifest JSON parse error: {}",
                                        manifest_file.display(),
                                        e
                                    ));
                                }
                            },
                            Err(e) => {
                                report.errors.push(format!(
                                    "{}: failed to read manifest: {}",
                                    manifest_file.display(),
                                    e
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_validate_clean_library() {
        let temp_dir = TempDir::new().unwrap();
        crate::init_library(temp_dir.path(), Some("test-lib")).unwrap();

        let report = validate_library(temp_dir.path()).unwrap();
        assert!(report.is_valid());
        assert_eq!(report.errors.len(), 0);
    }
}
