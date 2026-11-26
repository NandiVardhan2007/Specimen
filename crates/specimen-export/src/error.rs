#![forbid(unsafe_code)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Export formatting error: {0}")]
    Format(String),

    #[error("Library error: {0}")]
    Library(#[from] specimen_library::LibraryError),

    #[error("Core error: {0}")]
    Core(#[from] specimen_core::CoreError),
}

pub type Result<T> = std::result::Result<T, ExportError>;
