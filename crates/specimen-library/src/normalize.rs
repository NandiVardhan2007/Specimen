use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{LibraryError, Result};
use specimen_core::{to_canonical_json, Item, ItemId, ItemType, License, Provenance, UserMeta};

pub struct NormalizeOutcome {
    pub item_id: ItemId,
    pub path: PathBuf,
    pub is_new: bool,
}

pub struct NormalizeOptions<'a, D> {
    pub item_type: ItemType,
    pub slug_hint: &'a str,
    pub data: &'a D,
    pub provenance: Provenance,
    pub license: License,
    pub confidence: f64,
    pub created_at: &'a str,
}

/// Normalizes and persists a design item into `collections/<type>/<id>/item.json`.
///
/// Ensures:
/// 1. Data is formatted as canonical JSON.
/// 2. ID is generated deterministically from `<type>-<slug>-<hash8>`.
/// 3. If item already exists, merges provenance without duplicating, and preserves user notes/annotations.
pub fn normalize_and_save_item<D: Serialize>(
    library_root: &Path,
    opts: NormalizeOptions<'_, D>,
) -> Result<NormalizeOutcome> {
    let canonical_data_str = to_canonical_json(opts.data)?;
    let item_id = ItemId::generate(opts.item_type, opts.slug_hint, &canonical_data_str);

    let item_dir = library_root
        .join("collections")
        .join(opts.item_type.collection_dir_name())
        .join(item_id.as_str());
    fs::create_dir_all(&item_dir)?;

    let item_file = item_dir.join("item.json");
    let (item, is_new) = if item_file.exists() {
        // Read existing item to merge provenance and preserve user annotations
        let content = fs::read_to_string(&item_file)?;
        let mut existing: Item<Value> = serde_json::from_str(&content).map_err(|e| {
            LibraryError::Core(specimen_core::CoreError::Serialization(e.to_string()))
        })?;

        // Merge provenance if not already present
        let already_present = existing.provenance.iter().any(|p| {
            p.source == opts.provenance.source
                && p.viewport == opts.provenance.viewport
                && p.scheme == opts.provenance.scheme
        });
        if !already_present {
            existing.provenance.push(opts.provenance);
        }
        existing.updated_at = opts.created_at.to_string();

        (existing, false)
    } else {
        let raw_data_val: Value = serde_json::from_str(&canonical_data_str).map_err(|e| {
            LibraryError::Core(specimen_core::CoreError::Serialization(e.to_string()))
        })?;

        let similar_to = crate::similarity::find_similar_items(
            library_root,
            opts.item_type,
            &item_id,
            &raw_data_val,
        );

        let new_item = Item {
            schema: "specimen.item/1".to_string(),
            id: item_id.clone(),
            item_type: opts.item_type,
            name: None,
            slug: item_id.slug().to_string(),
            description: None,
            tags: BTreeSet::new(),
            data: raw_data_val,
            provenance: vec![opts.provenance],
            license: opts.license,
            confidence: opts.confidence,
            similar_to,
            annotations: None,
            user: UserMeta::default(),
            previews: Vec::new(),
            created_at: opts.created_at.to_string(),
            updated_at: opts.created_at.to_string(),
        };

        (new_item, true)
    };

    let canonical_item_json = to_canonical_json(&item)?;
    fs::write(&item_file, canonical_item_json)?;

    Ok(NormalizeOutcome {
        item_id,
        path: item_file,
        is_new,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use specimen_core::{LicenseStatus, Provenance};

    #[test]
    fn test_normalize_and_deduplicate_provenance() {
        let temp = tempfile::tempdir().unwrap();
        let lib_root = temp.path().join("lib");
        crate::init_library(&lib_root, None).unwrap();

        let data = serde_json::json!({
            "hex": "#7c5cff",
            "role": "accent"
        });

        let prov1 = Provenance {
            source: "example.com/2026-11-26T000000Z".to_string(),
            url: "https://example.com/".to_string(),
            selector: Some(".btn".to_string()),
            viewport: "desktop".to_string(),
            scheme: "dark".to_string(),
            captured_at: "2026-11-26T00:00:00Z".to_string(),
        };

        let license = License {
            status: LicenseStatus::Reusable,
            basis: "plain value".to_string(),
            spdx: None,
        };

        // First save
        let out1 = normalize_and_save_item(
            &lib_root,
            NormalizeOptions {
                item_type: ItemType::Palette,
                slug_hint: "accent",
                data: &data,
                provenance: prov1.clone(),
                license: license.clone(),
                confidence: 1.0,
                created_at: "2026-11-26T00:00:00Z",
            },
        )
        .unwrap();
        assert!(out1.is_new);
        assert!(out1.path.exists());

        // Second save with different viewport -> merges provenance
        let mut prov2 = prov1.clone();
        prov2.viewport = "mobile".to_string();

        let out2 = normalize_and_save_item(
            &lib_root,
            NormalizeOptions {
                item_type: ItemType::Palette,
                slug_hint: "accent",
                data: &data,
                provenance: prov2,
                license,
                confidence: 1.0,
                created_at: "2026-11-26T00:01:00Z",
            },
        )
        .unwrap();

        assert!(!out2.is_new);
        assert_eq!(out1.item_id, out2.item_id);

        let content = fs::read_to_string(&out2.path).unwrap();
        let loaded: Item<Value> = serde_json::from_str(&content).unwrap();
        assert_eq!(loaded.provenance.len(), 2);
    }
}
