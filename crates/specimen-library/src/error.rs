use thiserror::Error;

#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("Core error: {0}")]
    Core(#[from] specimen_core::CoreError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Library already initialized at '{0}'")]
    AlreadyInitialized(String),

    #[error("TOML serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),
}

pub type Result<T> = std::result::Result<T, LibraryError>;
