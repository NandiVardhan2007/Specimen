use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest, Sha256};
use std::fmt;
use std::str::FromStr;

use crate::error::{CoreError, Result};
use crate::schema::ItemType;

/// A canonical Item ID adhering to `<type>-<slug>-<hash8>`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, JsonSchema)]
pub struct ItemId(String);

impl ItemId {
    /// Generates a new ItemId from an item type, slug, and canonical data JSON representation.
    pub fn generate(item_type: ItemType, raw_slug: &str, canonical_data_json: &str) -> Self {
        let sanitized_slug = sanitize_slug(raw_slug);
        let slug = if sanitized_slug.is_empty() {
            "item".to_string()
        } else {
            sanitized_slug
        };

        // Compute SHA-256 over canonical data
        let mut hasher = Sha256::new();
        hasher.update(canonical_data_json.as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        let hash8 = &hash[..8];

        let id = format!("{}-{}-{}", item_type.as_str(), slug, hash8);
        ItemId(id)
    }

    pub fn new(id_str: &str) -> Result<Self> {
        let parts: Vec<&str> = id_str.split('-').collect();
        if parts.len() < 3 {
            return Err(CoreError::InvalidId(format!(
                "Item ID must match `<type>-<slug>-<hash8>`: {id_str}"
            )));
        }

        let type_str = parts[0];
        let hash8 = parts[parts.len() - 1];
        let slug = parts[1..parts.len() - 1].join("-");

        // Validate type
        ItemType::from_str(type_str).map_err(|_| {
            CoreError::InvalidId(format!("Unknown item type in ID '{type_str}': {id_str}"))
        })?;

        // Validate hash8: exactly 8 lowercase hex characters
        if hash8.len() != 8
            || !hash8
                .chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
        {
            return Err(CoreError::InvalidId(format!(
                "hash8 must be exactly 8 lowercase hex characters: {hash8}"
            )));
        }

        // Validate slug: non-empty, lowercase ascii alphanumeric and hyphens
        if slug.is_empty()
            || !slug
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            return Err(CoreError::InvalidId(format!(
                "slug must contain only lowercase alphanumeric characters and hyphens: {slug}"
            )));
        }

        Ok(ItemId(id_str.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn item_type(&self) -> ItemType {
        let type_str = self.0.split('-').next().unwrap_or_default();
        ItemType::from_str(type_str).unwrap_or(ItemType::Component)
    }

    pub fn hash8(&self) -> &str {
        self.0.rsplit('-').next().unwrap_or_default()
    }

    pub fn slug(&self) -> &str {
        let start = self.0.find('-').map(|i| i + 1).unwrap_or(0);
        let end = self.0.rfind('-').unwrap_or(self.0.len());
        if start < end {
            &self.0[start..end]
        } else {
            ""
        }
    }
}

fn sanitize_slug(input: &str) -> String {
    let mut result = String::new();
    let mut last_was_dash = false;

    for c in input.chars() {
        if c.is_ascii_alphanumeric() {
            result.push(c.to_ascii_lowercase());
            last_was_dash = false;
        } else if (c == ' ' || c == '-' || c == '_' || c == '.')
            && !last_was_dash
            && !result.is_empty()
        {
            result.push('-');
            last_was_dash = true;
        }
    }

    if result.ends_with('-') {
        result.pop();
    }

    result
}

impl AsRef<str> for ItemId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ItemId {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self> {
        ItemId::new(s)
    }
}

impl Serialize for ItemId {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ItemId {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ItemId::new(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_generation_and_parsing() {
        let id = ItemId::generate(ItemType::Effect, "Frosted Glass Card", "{\"blur\": 16}");
        assert!(id.as_str().starts_with("effect-frosted-glass-card-"));
        assert_eq!(id.hash8().len(), 8);
        assert_eq!(id.slug(), "frosted-glass-card");
        assert_eq!(id.item_type(), ItemType::Effect);

        let parsed = ItemId::new(id.as_str()).unwrap();
        assert_eq!(parsed, id);
    }

    #[test]
    fn test_id_slug_sanitization() {
        assert_eq!(sanitize_slug("  Hello World! 123  "), "hello-world-123");
        assert_eq!(sanitize_slug("card---glass...blur"), "card-glass-blur");
    }

    #[test]
    fn test_invalid_ids() {
        assert!(ItemId::new("invalid").is_err());
        assert!(ItemId::new("effect-slug").is_err());
        assert!(ItemId::new("unknowntype-slug-12345678").is_err());
        assert!(ItemId::new("effect-SLUG-12345678").is_err());
        assert!(ItemId::new("effect-slug-1234567G").is_err()); // non-hex
        assert!(ItemId::new("effect-slug-12345").is_err()); // too short hash
    }
}
