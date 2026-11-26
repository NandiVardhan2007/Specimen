#![forbid(unsafe_code)]

//! Specimen Capture: browser lifecycle, CDP session, and network policy.

pub mod browser;
pub mod discovery;
pub mod policy;

pub use browser::{BrowserError, BrowserGuard, LaunchOptions};
pub use discovery::{find_browser, BrowserInfo};
pub use policy::{
    is_private_or_restricted, resolve_and_validate_host, validate_url_syntax, PolicyError,
    RateLimiter, RobotsTxt,
};
pub use specimen_core as core;
