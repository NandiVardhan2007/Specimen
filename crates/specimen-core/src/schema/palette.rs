use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ColorUsage {
    pub area_pct: f64,
    pub elements: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PaletteColor {
    pub role: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hex: String,
    pub oklch: [f64; 3],
    pub alpha: f64,
    pub usage: ColorUsage,
    #[serde(default)]
    pub css_variables: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ContrastPair {
    pub fg: String,
    pub bg: String,
    pub wcag_ratio: f64,
    pub aa: bool,
    pub aaa: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PaletteData {
    pub scheme: String,
    pub colors: Vec<PaletteColor>,
    #[serde(default)]
    pub pairs: Vec<ContrastPair>,
}
