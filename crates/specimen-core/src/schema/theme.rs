use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ThemeData {
    pub scheme: String,
    pub spacing_base_unit: u32,
    #[serde(default)]
    pub spacing_scale: Vec<f64>,
    #[serde(default)]
    pub radii_px: Vec<f64>,
    #[serde(default)]
    pub breakpoints_px: Vec<u32>,
    #[serde(default)]
    pub tokens: BTreeMap<String, String>,
}
