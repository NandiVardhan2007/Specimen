#![forbid(unsafe_code)]

//! Specimen Export: token exporters (CSS, Tailwind, DTCG, SCSS, Tokens Studio).

pub mod css;
pub mod dtcg;
pub mod error;
pub mod loader;
pub mod tailwind;

pub use css::{export_css, CssExportInput};
pub use dtcg::{export_dtcg, DtcgExportInput};
pub use error::{ExportError, Result};
pub use loader::{load_library_tokens, LoadedTokens};
pub use specimen_core as core;
pub use specimen_library as library;
pub use tailwind::{export_tailwind, TailwindExportInput};

use std::path::Path;

/// Exports loaded design tokens from library into the requested text format.
pub fn export_tokens_to_string(library_root: &Path, format: &str) -> Result<String> {
    let tokens = load_library_tokens(library_root)?;
    match format {
        "dtcg" => {
            let input = DtcgExportInput {
                palette: tokens
                    .light_palette
                    .as_ref()
                    .or(tokens.dark_palette.as_ref()),
                typography: tokens.typography.as_ref(),
                theme: tokens.theme.as_ref(),
            };
            Ok(serde_json::to_string_pretty(&export_dtcg(&input))?)
        }
        "tailwind" => {
            let input = TailwindExportInput {
                palette: tokens
                    .light_palette
                    .as_ref()
                    .or(tokens.dark_palette.as_ref()),
                typography: tokens.typography.as_ref(),
                theme: tokens.theme.as_ref(),
                version: 4,
            };
            Ok(export_tailwind(&input))
        }
        "tailwind-v3" => {
            let input = TailwindExportInput {
                palette: tokens
                    .light_palette
                    .as_ref()
                    .or(tokens.dark_palette.as_ref()),
                typography: tokens.typography.as_ref(),
                theme: tokens.theme.as_ref(),
                version: 3,
            };
            Ok(export_tailwind(&input))
        }
        _ => {
            let input = CssExportInput {
                light_palette: tokens.light_palette.as_ref(),
                dark_palette: tokens.dark_palette.as_ref(),
                typography: tokens.typography.as_ref(),
                theme: tokens.theme.as_ref(),
            };
            Ok(export_css(&input))
        }
    }
}
