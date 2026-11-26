#![forbid(unsafe_code)]

use serde_json::Value;
use std::fs;
use std::path::Path;

use specimen_core::{ItemId, ItemType};

/// Evaluates similarity between items of the same type across sources (SPEC §7 & TRD §7).
pub fn find_similar_items(
    library_root: &Path,
    item_type: ItemType,
    current_id: &ItemId,
    current_data: &Value,
) -> Vec<ItemId> {
    let mut similar = Vec::new();
    let coll_dir = library_root
        .join("collections")
        .join(item_type.collection_dir_name());

    if !coll_dir.exists() {
        return similar;
    }

    let entries = match fs::read_dir(coll_dir) {
        Ok(e) => e,
        Err(_) => return similar,
    };

    for entry in entries.flatten() {
        if !entry.path().is_dir() {
            continue;
        }

        let dir_name = entry.file_name().to_string_lossy().to_string();
        if dir_name == current_id.as_str() {
            continue; // Skip self
        }

        let other_file = entry.path().join("item.json");
        if !other_file.exists() {
            continue;
        }

        if let Ok(content) = fs::read_to_string(other_file) {
            if let Ok(other_item) = serde_json::from_str::<Value>(&content) {
                if let Some(other_data) = other_item.get("data") {
                    if are_items_similar(item_type, current_data, other_data) {
                        if let Some(other_id_str) = other_item.get("id").and_then(|v| v.as_str()) {
                            if let Ok(other_id) = other_id_str.parse::<ItemId>() {
                                similar.push(other_id);
                            }
                        }
                    }
                }
            }
        }
    }

    similar
}

fn are_items_similar(item_type: ItemType, a: &Value, b: &Value) -> bool {
    match item_type {
        ItemType::Palette => {
            // Check if primary background or text colors are closely related
            let a_colors = a.get("colors").and_then(|v| v.as_array());
            let b_colors = b.get("colors").and_then(|v| v.as_array());
            if let (Some(ac), Some(bc)) = (a_colors, b_colors) {
                let mut matches = 0;
                for c1 in ac {
                    if let Some(hex1) = c1.get("hex").and_then(|v| v.as_str()) {
                        if bc
                            .iter()
                            .any(|c2| c2.get("hex").and_then(|v| v.as_str()) == Some(hex1))
                        {
                            matches += 1;
                        }
                    }
                }
                matches >= 2
            } else {
                false
            }
        }
        ItemType::Typography => {
            let a_fams = a.get("families").and_then(|v| v.as_array());
            let b_fams = b.get("families").and_then(|v| v.as_array());
            if let (Some(af), Some(bf)) = (a_fams, b_fams) {
                af.iter().any(|f1| {
                    let n1 = f1.get("name").and_then(|v| v.as_str());
                    bf.iter()
                        .any(|f2| f2.get("name").and_then(|v| v.as_str()) == n1)
                })
            } else {
                false
            }
        }
        ItemType::Effect => {
            let a_cat = a.get("category").and_then(|v| v.as_str());
            let b_cat = b.get("category").and_then(|v| v.as_str());
            let a_rec = a.get("recipe").and_then(|v| v.as_str());
            let b_rec = b.get("recipe").and_then(|v| v.as_str());
            a_cat.is_some() && a_cat == b_cat && a_rec == b_rec
        }
        ItemType::Component => {
            let a_role = a.get("role").and_then(|v| v.as_str());
            let b_role = b.get("role").and_then(|v| v.as_str());
            let a_var = a.get("variant").and_then(|v| v.as_str());
            let b_var = b.get("variant").and_then(|v| v.as_str());
            a_role.is_some() && a_role == b_role && a_var == b_var
        }
        ItemType::Animation => {
            let a_driver = a.get("driver").and_then(|v| v.as_str());
            let b_driver = b.get("driver").and_then(|v| v.as_str());
            let a_kind = a.get("kind").and_then(|v| v.as_str());
            let b_kind = b.get("kind").and_then(|v| v.as_str());
            a_driver.is_some() && a_driver == b_driver && a_kind == b_kind
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_similarity_palette_overlap() {
        let a = json!({
            "colors": [
                { "hex": "#0b0d12", "role": "background" },
                { "hex": "#7c5cff", "role": "accent" }
            ]
        });
        let b = json!({
            "colors": [
                { "hex": "#0b0d12", "role": "surface" },
                { "hex": "#7c5cff", "role": "primary" }
            ]
        });

        assert!(are_items_similar(ItemType::Palette, &a, &b));
    }
}
