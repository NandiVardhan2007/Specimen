#![forbid(unsafe_code)]

//! Specimen Capture: browser lifecycle, CDP session, drivers, and network policy.

pub mod browser;
pub mod discovery;
pub mod driver;
pub mod evidence;
pub mod policy;
pub mod session;

pub use browser::{BrowserError, BrowserGuard, LaunchOptions};
pub use discovery::{find_browser, BrowserInfo};
pub use driver::{
    CapturedEvidence, CrawlDriver, CrawlGroupEvidence, CrawlOptions, DriverOptions, FlowDriver,
    FlowDriverOptions, FlowEvidence, LabeledCapture, SinglePageDriver,
};
pub use evidence::{CaptureManifest, EvidenceWriter};
pub use policy::{
    is_private_or_restricted, resolve_and_validate_host, validate_url_syntax, PolicyError,
    RateLimiter, RobotsTxt,
};
pub use session::{
    AnimationRecord, CaptureError, ColorScheme, DomSnapshot, Image, MockSession, NavOutcome,
    NetworkEntry, NetworkLog, Pseudo, Result, Session, SettlePolicy, ShotOpts, Trace, Viewport,
};
pub use specimen_core as core;
