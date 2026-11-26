#![forbid(unsafe_code)]

use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub library_root: Arc<PathBuf>,
}

impl AppState {
    pub fn new(library_root: PathBuf) -> Self {
        Self {
            library_root: Arc::new(library_root),
        }
    }
}
