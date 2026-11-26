use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Invalid item ID: {0}")]
    InvalidId(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, CoreError>;
