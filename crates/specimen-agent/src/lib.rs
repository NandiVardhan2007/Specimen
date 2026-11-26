#![forbid(unsafe_code)]

//! Specimen Agent: AI agent adapter framework and output validation.

pub mod adapters;
pub mod detection;
pub mod error;
pub mod prompt;
pub mod task;
pub mod validator;

pub use adapters::{cli::CliAdapter, manual::ManualAdapter, mock::MockAdapter, AgentAdapter};
pub use detection::{detect_agents, AgentDetection};
pub use error::{AgentError, Result};
pub use prompt::{PromptBuilder, INSTRUCTIONS_VERSION};
pub use specimen_core as core;
pub use task::{AgentOutput, AgentTask, RunContext, TaskKind};
pub use validator::{
    AgentWarning, OutputValidator, ValidatedPaletteColor, ValidatedPaletteItem, ValidationOutcome,
};
