#![forbid(unsafe_code)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Agent execution timed out: {0}")]
    Timeout(String),

    #[error("Failed to spawn agent process: {0}")]
    SpawnFailed(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Agent returned non-zero exit status: {0}")]
    ProcessFailed(String),

    #[error("Unsupported agent task: {0}")]
    UnsupportedTask(String),

    #[error("Core error: {0}")]
    Core(#[from] specimen_core::CoreError),
}

pub type Result<T> = std::result::Result<T, AgentError>;
