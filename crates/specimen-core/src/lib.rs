#![forbid(unsafe_code)]

//! Specimen Core: types, schemas, errors, config, canonical JSON, IDs, and path validation.

pub mod canonical;
pub mod config;
pub mod error;
pub mod id;
pub mod path;
pub mod rounding;
pub mod schema;

// Re-exports of common types
pub use canonical::{canonicalize_value, to_canonical_json};
pub use config::Config;
pub use error::{CoreError, Result};
pub use id::ItemId;
pub use path::RelPath;
pub use rounding::{round_2, round_4, round_f64};
pub use schema::{
    AnimationData, Annotations, ColorUsage, ComponentData, ComponentVariant, ContrastPair,
    EffectData, FlowAction, FlowScript, FlowStep, FontFamily, FontFileMeta, FontLicenseHints,
    IconData, IconSetInfo, Item, ItemType, Keyframe, LayoutData, License, LicenseStatus,
    PaletteColor, PaletteData, PreviewClip, Provenance, StateStyles, Target, ThemeData, Timing,
    Trigger, TypeScaleEntry, TypographyData, UserMeta,
};
