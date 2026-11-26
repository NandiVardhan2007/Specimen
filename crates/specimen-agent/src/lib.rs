#![forbid(unsafe_code)]

//! Specimen Agent: AI agent adapter framework and output validation.

pub mod detection;

pub use detection::{detect_agents, AgentDetection};
pub use specimen_core as core;
