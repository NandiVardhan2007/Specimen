#![forbid(unsafe_code)]

//! Specimen Library: filesystem layout, index, normalisation, deduplication, and DESIGN.md generation.

pub mod design_md;
pub mod error;
pub mod init;
pub mod migrate;
pub mod normalize;
pub mod search;
pub mod similarity;
pub mod validate;

pub use design_md::{generate_design_md, DesignDocMeta};
pub use error::{LibraryError, Result};
pub use init::{init_library, InitSummary};
pub use migrate::{migrate_library, MigrationReport};
pub use normalize::{normalize_and_save_item, NormalizeOptions, NormalizeOutcome};
pub use search::{SearchFilter, SearchHit, SearchIndex};
pub use similarity::find_similar_items;
pub use specimen_core as core;
pub use validate::{validate_library, ValidationReport};
