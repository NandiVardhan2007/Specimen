#![forbid(unsafe_code)]

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct IconSetInfo {
    pub name: String,
    pub license: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct IconData {
    pub svg: String,
    pub view_box: String,
    pub width: u32,
    pub height: u32,
    pub is_current_color: bool,
    pub geometry_hash: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<IconSetInfo>,
}
