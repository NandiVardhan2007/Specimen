#![forbid(unsafe_code)]

//! Specimen Extract: pure extractors transforming evidence into design items.

pub mod color;
pub mod component;
pub mod effect;
pub mod icon;
pub mod layout;
pub mod motion;
pub mod palette;
pub mod svg;
pub mod theme;
pub mod typography;

pub use color::{parse_hex, relative_luminance, rgb_to_hex, srgb_to_oklch, wcag_contrast_ratio};
pub use component::{ComponentExtractor, ObservedElement};
pub use effect::{EffectExtractor, ObservedEffect};
pub use icon::{IconExtractor, ObservedIcon};
pub use layout::{LayoutExtractor, ObservedSection};
pub use motion::{MotionExtractor, ObservedAnimation};
pub use palette::{ObservedColor, PaletteExtractor};
pub use svg::{SanitizedSvg, SvgSanitizer};
pub use theme::ThemeExtractor;
pub use typography::TypographyExtractor;

pub use specimen_core as core;
