#![forbid(unsafe_code)]

//! Specimen Library: filesystem layout, index, normalisation and deduplication.

pub mod error;
pub mod init;

pub use error::{LibraryError, Result};
pub use init::{init_library, InitSummary};
pub use specimen_core as core;
