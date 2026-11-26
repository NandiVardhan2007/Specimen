use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use crate::error::{CoreError, Result};
use crate::id::ItemId;
use crate::path::RelPath;

pub mod effect;
pub mod icon;
pub mod layout;
pub mod palette;
pub mod theme;
pub mod typography;

pub use effect::EffectData;
pub use icon::{IconData, IconSetInfo};
pub use layout::LayoutData;
pub use palette::{ColorUsage, ContrastPair, PaletteColor, PaletteData};
pub use theme::ThemeData;
pub use typography::{FontFamily, FontFileMeta, FontLicenseHints, TypeScaleEntry, TypographyData};

/// High-level item types supported by Specimen.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Icon,
    Palette,
    Typography,
    Theme,
    Layout,
    Effect,
    Animation,
    Component,
}

impl ItemType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ItemType::Icon => "icon",
            ItemType::Palette => "palette",
            ItemType::Typography => "typography",
            ItemType::Theme => "theme",
            ItemType::Layout => "layout",
            ItemType::Effect => "effect",
            ItemType::Animation => "animation",
            ItemType::Component => "component",
        }
    }

    pub fn collection_dir_name(&self) -> &'static str {
        match self {
            ItemType::Icon => "icons",
            ItemType::Palette => "palettes",
            ItemType::Typography => "typography",
            ItemType::Theme => "themes",
            ItemType::Layout => "layouts",
            ItemType::Effect => "effects",
            ItemType::Animation => "animations",
            ItemType::Component => "components",
        }
    }
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ItemType {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "icon" | "icons" => Ok(ItemType::Icon),
            "palette" | "palettes" => Ok(ItemType::Palette),
            "typography" => Ok(ItemType::Typography),
            "theme" | "themes" => Ok(ItemType::Theme),
            "layout" | "layouts" => Ok(ItemType::Layout),
            "effect" | "effects" => Ok(ItemType::Effect),
            "animation" | "animations" => Ok(ItemType::Animation),
            "component" | "components" => Ok(ItemType::Component),
            _ => Err(CoreError::InvalidId(format!("Unknown item type: {s}"))),
        }
    }
}

/// License categorization status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum LicenseStatus {
    Reusable,
    ReferenceOnly,
    Unknown,
}

/// License declaration with legal basis and optional SPDX identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct License {
    pub status: LicenseStatus,
    pub basis: String,
    pub spdx: Option<String>,
}

impl Default for License {
    fn default() -> Self {
        Self {
            status: LicenseStatus::Unknown,
            basis: "unverified".to_string(),
            spdx: None,
        }
    }
}

/// Provenance metadata tracking the origin of captured elements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Provenance {
    pub source: String,
    pub url: String,
    pub selector: Option<String>,
    pub viewport: String,
    pub scheme: String,
    pub captured_at: String,
}

/// Annotation metadata attached by AI agents or manual review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Annotations {
    pub by: String,
    pub at: String,
    pub prompt_hash: String,
}

/// User interaction metadata (favorites, custom notes).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct UserMeta {
    #[serde(default)]
    pub favorite: bool,
    #[serde(default)]
    pub notes: String,
}

/// Primary generic Item representation matching SPEC §7.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Item<D> {
    pub schema: String,
    pub id: ItemId,
    #[serde(rename = "type")]
    pub item_type: ItemType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub slug: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub tags: BTreeSet<String>,
    pub data: D,
    #[serde(default)]
    pub provenance: Vec<Provenance>,
    pub license: License,
    pub confidence: f64,
    #[serde(default)]
    pub similar_to: Vec<ItemId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
    #[serde(default)]
    pub user: UserMeta,
    #[serde(default)]
    pub previews: Vec<RelPath>,
    pub created_at: String,
    pub updated_at: String,
}
