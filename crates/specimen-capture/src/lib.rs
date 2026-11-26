#![forbid(unsafe_code)]

//! Specimen Capture: browser lifecycle and CDP drivers.

pub mod discovery;

pub use discovery::{find_browser, BrowserInfo};
pub use specimen_core as core;
