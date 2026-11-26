use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::error::{CoreError, Result};

/// Full `specimen.toml` configuration matching SPEC §11.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub library: LibraryConfig,
    #[serde(default)]
    pub capture: CaptureConfig,
    #[serde(default)]
    pub ai: AiConfig,
    #[serde(default)]
    pub assets: AssetsConfig,
    #[serde(default)]
    pub privacy: PrivacyConfig,
    #[serde(default)]
    pub web: WebConfig,
    #[serde(default)]
    pub mcp: McpConfig,
    #[serde(default)]
    pub export: ExportConfig,
}

impl Config {
    /// Loads a configuration from a file path.
    pub fn load_from_path(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| CoreError::Config(format!("Failed to parse {}: {}", path.display(), e)))?;
        Ok(config)
    }

    /// Loads config from path if it exists, otherwise returns default.
    pub fn load_or_default(path: &Path) -> Result<Self> {
        if path.exists() {
            Self::load_from_path(path)
        } else {
            Ok(Self::default())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LibraryConfig {
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
    #[serde(default = "default_library_name")]
    pub name: String,
    #[serde(default)]
    pub notice_acknowledged: bool,
}

fn default_schema_version() -> u32 {
    1
}

fn default_library_name() -> String {
    "My design library".to_string()
}

impl Default for LibraryConfig {
    fn default() -> Self {
        Self {
            schema_version: default_schema_version(),
            name: default_library_name(),
            notice_acknowledged: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ViewportConfig {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub dpr: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureConfig {
    #[serde(default = "default_viewports")]
    pub viewports: Vec<ViewportConfig>,
    #[serde(default = "default_schemes")]
    pub schemes: Vec<String>,
    #[serde(default = "default_robots")]
    pub robots: String,
    #[serde(default = "default_rate_limit")]
    pub rate_limit_rps: f64,
    #[serde(default = "default_timeout_s")]
    pub timeout_s: u64,
    #[serde(default = "default_user_agent")]
    pub user_agent: String,
}

fn default_viewports() -> Vec<ViewportConfig> {
    vec![
        ViewportConfig {
            name: "mobile".to_string(),
            width: 390,
            height: 844,
            dpr: 2.0,
        },
        ViewportConfig {
            name: "tablet".to_string(),
            width: 768,
            height: 1024,
            dpr: 2.0,
        },
        ViewportConfig {
            name: "laptop".to_string(),
            width: 1280,
            height: 800,
            dpr: 1.0,
        },
        ViewportConfig {
            name: "desktop".to_string(),
            width: 1920,
            height: 1080,
            dpr: 1.0,
        },
    ]
}

fn default_schemes() -> Vec<String> {
    vec!["light".to_string(), "dark".to_string()]
}

fn default_robots() -> String {
    "respect".to_string()
}

fn default_rate_limit() -> f64 {
    1.0
}

fn default_timeout_s() -> u64 {
    60
}

fn default_user_agent() -> String {
    "default".to_string()
}

impl Default for CaptureConfig {
    fn default() -> Self {
        Self {
            viewports: default_viewports(),
            schemes: default_schemes(),
            robots: default_robots(),
            rate_limit_rps: default_rate_limit(),
            timeout_s: default_timeout_s(),
            user_agent: default_user_agent(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AiConfig {
    #[serde(default = "default_agent")]
    pub agent: String,
    #[serde(default)]
    pub require: bool,
    #[serde(default = "default_ai_timeout_s")]
    pub timeout_s: u64,
    #[serde(default = "default_max_items_per_call")]
    pub max_items_per_call: u32,
}

fn default_agent() -> String {
    "auto".to_string()
}

fn default_ai_timeout_s() -> u64 {
    300
}

fn default_max_items_per_call() -> u32 {
    40
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            agent: default_agent(),
            require: false,
            timeout_s: default_ai_timeout_s(),
            max_items_per_call: default_max_items_per_call(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AssetsConfig {
    #[serde(default = "default_store")]
    pub store: String,
    #[serde(default = "default_restricted_dir")]
    pub restricted_dir: String,
}

fn default_store() -> String {
    "all".to_string()
}

fn default_restricted_dir() -> String {
    "assets-restricted".to_string()
}

impl Default for AssetsConfig {
    fn default() -> Self {
        Self {
            store: default_store(),
            restricted_dir: default_restricted_dir(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PrivacyConfig {
    #[serde(default)]
    pub redact_text: bool,
    #[serde(default = "default_redact_headers")]
    pub redact_headers: Vec<String>,
}

fn default_redact_headers() -> Vec<String> {
    vec![
        "authorization".to_string(),
        "cookie".to_string(),
        "set-cookie".to_string(),
        "x-api-key".to_string(),
    ]
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            redact_text: false,
            redact_headers: default_redact_headers(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WebConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    4747
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct McpConfig {
    #[serde(default)]
    pub allow_capture: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExportConfig {
    #[serde(default)]
    pub names: BTreeMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_parses_and_serializes() {
        let cfg = Config::default();
        let toml_str = toml::to_string(&cfg).unwrap();
        let reloaded: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(cfg, reloaded);
    }
}
