#![forbid(unsafe_code)]

//! Specimen Extract: pure extractors transforming evidence into design items.

pub mod color;
pub mod palette;
pub mod theme;
pub mod typography;

pub use color::{parse_hex, relative_luminance, rgb_to_hex, srgb_to_oklch, wcag_contrast_ratio};
pub use palette::{ObservedColor, PaletteExtractor};
pub use theme::ThemeExtractor;
pub use typography::TypographyExtractor;

pub use specimen_core as core;
