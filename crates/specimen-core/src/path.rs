use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::path::{Component, Path, PathBuf};
use std::str::FromStr;

use crate::error::{CoreError, Result};

/// A normalized, safe relative path within the library.
///
/// Disallows absolute paths, drive letters, UNC paths, and traversal (`..`).
/// Normalizes path separators to `/`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, JsonSchema)]
pub struct RelPath(String);

impl RelPath {
    pub fn new(path_str: &str) -> Result<Self> {
        let trimmed = path_str.trim();
        if trimmed.is_empty() {
            return Err(CoreError::InvalidPath("Path cannot be empty".to_string()));
        }

        // Check for Windows drive letter (e.g. C:) or UNC path (starts with \\ or //)
        if trimmed.starts_with('/') || trimmed.starts_with('\\') {
            return Err(CoreError::InvalidPath(format!(
                "Path cannot be absolute: {path_str}"
            )));
        }
        if trimmed.len() >= 2 && trimmed.chars().nth(1) == Some(':') {
            return Err(CoreError::InvalidPath(format!(
                "Drive letters not allowed: {path_str}"
            )));
        }

        let p = Path::new(trimmed);
        let mut segments = Vec::new();

        for component in p.components() {
            match component {
                Component::Normal(os_str) => {
                    let s = os_str.to_str().ok_or_else(|| {
                        CoreError::InvalidPath("Path contains non-UTF-8 characters".to_string())
                    })?;
                    segments.push(s);
                }
                Component::CurDir => {
                    // Ignore '.'
                }
                Component::ParentDir => {
                    return Err(CoreError::InvalidPath(format!(
                        "Path traversal ('..') is strictly forbidden: {path_str}"
                    )));
                }
                Component::RootDir | Component::Prefix(_) => {
                    return Err(CoreError::InvalidPath(format!(
                        "Absolute paths and prefixes are not allowed: {path_str}"
                    )));
                }
            }
        }

        if segments.is_empty() {
            return Err(CoreError::InvalidPath(
                "Path resolves to empty relative path".to_string(),
            ));
        }

        Ok(RelPath(segments.join("/")))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_path_buf(&self) -> PathBuf {
        PathBuf::from(&self.0)
    }

    /// Resolves this relative path under a given root directory, ensuring
    /// the target stays within the root boundary (canonical prefix check).
    pub fn resolve_under(&self, root: &Path) -> Result<PathBuf> {
        let candidate = root.join(&self.0);

        // Ensure that normalizing components remains within root
        let mut resolved = PathBuf::new();
        for comp in candidate.components() {
            match comp {
                Component::ParentDir => {
                    return Err(CoreError::InvalidPath(format!(
                        "Resolved path escapes root directory: {}",
                        candidate.display()
                    )));
                }
                c => resolved.push(c.as_os_str()),
            }
        }

        Ok(resolved)
    }
}

impl AsRef<str> for RelPath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<Path> for RelPath {
    fn as_ref(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl fmt::Display for RelPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for RelPath {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self> {
        RelPath::new(s)
    }
}

impl Serialize for RelPath {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for RelPath {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        RelPath::new(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_paths() {
        let p = RelPath::new("assets/fonts/inter.woff2").unwrap();
        assert_eq!(p.as_str(), "assets/fonts/inter.woff2");

        let p_backslashes = RelPath::new("assets\\fonts\\inter.woff2").unwrap();
        assert_eq!(p_backslashes.as_str(), "assets/fonts/inter.woff2");

        let p_dots = RelPath::new("./assets/./icons/icon.svg").unwrap();
        assert_eq!(p_dots.as_str(), "assets/icons/icon.svg");
    }

    #[test]
    fn test_invalid_paths() {
        assert!(RelPath::new("../secret").is_err());
        assert!(RelPath::new("assets/../../secret").is_err());
        assert!(RelPath::new("/root/path").is_err());
        assert!(RelPath::new("\\windows\\path").is_err());
        assert!(RelPath::new("C:\\Windows").is_err());
        assert!(RelPath::new("D:/foo").is_err());
        assert!(RelPath::new("").is_err());
        assert!(RelPath::new("   ").is_err());
        assert!(RelPath::new(".").is_err());
    }

    #[test]
    fn test_serialization() {
        let p = RelPath::new("preview.png").unwrap();
        let json = serde_json::to_string(&p).unwrap();
        assert_eq!(json, "\"preview.png\"");

        let de: RelPath = serde_json::from_str(&json).unwrap();
        assert_eq!(de, p);
    }
}
