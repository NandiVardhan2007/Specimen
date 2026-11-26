#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;

use specimen_core::{Item, PaletteData, ThemeData, TypographyData};

use crate::error::Result;

#[derive(Default)]
pub struct LoadedTokens {
    pub light_palette: Option<PaletteData>,
    pub dark_palette: Option<PaletteData>,
    pub typography: Option<TypographyData>,
    pub theme: Option<ThemeData>,
}

pub fn load_library_tokens(library_root: &Path) -> Result<LoadedTokens> {
    let mut tokens = LoadedTokens::default();
    let collections_dir = library_root.join("collections");
    if !collections_dir.exists() {
        return Ok(tokens);
    }

    // Load palettes
    let pal_dir = collections_dir.join("palettes");
    if pal_dir.exists() && pal_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(pal_dir) {
            for entry in entries.flatten() {
                let item_file = entry.path().join("item.json");
                if item_file.exists() {
                    if let Ok(content) = fs::read_to_string(&item_file) {
                        if let Ok(item) = serde_json::from_str::<Item<PaletteData>>(&content) {
                            if item.data.scheme == "dark" && tokens.dark_palette.is_none() {
                                tokens.dark_palette = Some(item.data);
                            } else if tokens.light_palette.is_none() {
                                tokens.light_palette = Some(item.data);
                            }
                        }
                    }
                }
            }
        }
    }

    // Load typography
    let typo_dir = collections_dir.join("typography");
    if typo_dir.exists() && typo_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(typo_dir) {
            for entry in entries.flatten() {
                let item_file = entry.path().join("item.json");
                if item_file.exists() {
                    if let Ok(content) = fs::read_to_string(&item_file) {
                        if let Ok(item) = serde_json::from_str::<Item<TypographyData>>(&content) {
                            tokens.typography = Some(item.data);
                            break;
                        }
                    }
                }
            }
        }
    }

    // Load theme
    let thm_dir = collections_dir.join("themes");
    if thm_dir.exists() && thm_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(thm_dir) {
            for entry in entries.flatten() {
                let item_file = entry.path().join("item.json");
                if item_file.exists() {
                    if let Ok(content) = fs::read_to_string(&item_file) {
                        if let Ok(item) = serde_json::from_str::<Item<ThemeData>>(&content) {
                            tokens.theme = Some(item.data);
                            break;
                        }
                    }
                }
            }
        }
    }

    Ok(tokens)
}
