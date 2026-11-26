use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::policy::PolicyError;

#[derive(Error, Debug)]
pub enum CaptureError {
    #[error("Policy error: {0}")]
    Policy(#[from] PolicyError),

    #[error("Browser error: {0}")]
    Browser(#[from] crate::browser::BrowserError),

    #[error("Navigation failed: {0}")]
    Navigation(String),

    #[error("CDP command failed: {0}")]
    Cdp(String),

    #[error("Settle timeout after {0}ms")]
    SettleTimeout(u64),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, CaptureError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorScheme {
    Light,
    Dark,
}

impl ColorScheme {
    pub fn as_str(&self) -> &'static str {
        match self {
            ColorScheme::Light => "light",
            ColorScheme::Dark => "dark",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Viewport {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub dpr: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavOutcome {
    pub status: u16,
    pub url: String,
    pub timing_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettlePolicy {
    pub network_quiet_ms: u64,
    pub layout_shift_quiet_ms: u64,
    pub timeout_ms: u64,
}

impl Default for SettlePolicy {
    fn default() -> Self {
        Self {
            network_quiet_ms: 750,
            layout_shift_quiet_ms: 500,
            timeout_ms: 60_000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShotOpts {
    pub full_page: bool,
    pub quality: u8,
}

impl Default for ShotOpts {
    fn default() -> Self {
        Self {
            full_page: false,
            quality: 90,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Image {
    pub bytes: Vec<u8>,
    pub mime: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomSnapshot {
    pub raw_json: String,
    pub node_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkEntry {
    pub url: String,
    pub method: String,
    pub status: u16,
    pub mime: String,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkLog {
    pub entries: Vec<NetworkEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Pseudo {
    Hover,
    Focus,
    Active,
    Visited,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationRecord {
    pub name: String,
    pub duration_ms: f64,
    pub delay_ms: f64,
    pub easing: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Trace {
    pub keyframes: Vec<serde_json::Value>,
}
