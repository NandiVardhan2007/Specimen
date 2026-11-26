#![forbid(unsafe_code)]

//! Specimen Library: filesystem layout, index, normalisation, deduplication, and DESIGN.md generation.

pub mod design_md;
pub mod error;
pub mod init;
pub mod normalize;

pub use design_md::{generate_design_md, DesignDocMeta};
pub use error::{LibraryError, Result};
pub use init::{init_library, InitSummary};
pub use normalize::{normalize_and_save_item, NormalizeOptions, NormalizeOutcome};
pub use specimen_core as core;
