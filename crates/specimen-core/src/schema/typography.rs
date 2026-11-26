use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FontLicenseHints {
    pub name_table_license: Option<String>,
    pub license_url: Option<String>,
    pub fs_type: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FontFileMeta {
    pub file: String,
    pub format: String,
    pub sha256: String,
    pub weight: String,
    pub style: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FontFamily {
    pub name: String,
    pub stack: Vec<String>,
    pub delivery: String,
    #[serde(default)]
    pub files: Vec<FontFileMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_hints: Option<FontLicenseHints>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TypeScaleEntry {
    pub role: String,
    pub family: String,
    pub size_px: f64,
    pub weight: u32,
    pub line_height: f64,
    pub letter_spacing_em: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TypographyData {
    pub families: Vec<FontFamily>,
    pub scale: Vec<TypeScaleEntry>,
}
