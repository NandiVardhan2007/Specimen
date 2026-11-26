#![forbid(unsafe_code)]

use std::collections::BTreeMap;

use specimen_core::{EffectData, License, LicenseStatus};

#[derive(Debug, Clone)]
pub struct ObservedEffect {
    pub properties: BTreeMap<String, String>,
    pub elements_count: u32,
    pub role_hint: Option<String>,
}

pub struct EffectExtractor;

impl EffectExtractor {
    /// Extracts and categorizes visual effects and applies rule-based recipes.
    pub fn extract(&self, observed: Vec<ObservedEffect>) -> Vec<(EffectData, f64, License)> {
        let mut results = Vec::new();

        for obs in observed {
            let (category, recipe) = Self::classify_recipe(&obs.properties);

            let data = EffectData {
                category,
                recipe,
                css_properties: obs.properties,
                elements_count: obs.elements_count,
            };

            let license = License {
                status: LicenseStatus::Reusable,
                basis: "plain CSS values and recipes".to_string(),
                spdx: None,
            };

            results.push((data, 1.0, license));
        }

        results
    }

    fn classify_recipe(props: &BTreeMap<String, String>) -> (String, Option<String>) {
        // 1. Glassmorphism: backdrop-filter blur + translucent surface
        if let Some(bf) = props.get("backdrop-filter") {
            if bf.contains("blur") {
                return ("glass".to_string(), Some("glassmorphism".to_string()));
            }
        }

        // 2. Gradients
        if let Some(bg) = props
            .get("background")
            .or_else(|| props.get("background-image"))
        {
            if bg.contains("linear-gradient") {
                return ("gradient".to_string(), Some("linear-gradient".to_string()));
            } else if bg.contains("radial-gradient") {
                return ("gradient".to_string(), Some("radial-gradient".to_string()));
            }
        }

        // 3. Shadows (Elevation scale)
        if let Some(shadow) = props.get("box-shadow") {
            let recipe = if shadow.contains("0 1px") || shadow.contains("0 2px") {
                "elevation-sm"
            } else if shadow.contains("0 4px") || shadow.contains("0 6px") {
                "elevation-md"
            } else {
                "elevation-lg"
            };
            return ("shadow".to_string(), Some(recipe.to_string()));
        }

        // 4. Default / Border
        if props.contains_key("border") || props.contains_key("border-radius") {
            ("border".to_string(), Some("border-box".to_string()))
        } else {
            ("effect".to_string(), None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_glassmorphism() {
        let mut props = BTreeMap::new();
        props.insert("backdrop-filter".to_string(), "blur(12px)".to_string());
        props.insert(
            "background".to_string(),
            "rgba(255, 255, 255, 0.05)".to_string(),
        );

        let obs = ObservedEffect {
            properties: props,
            elements_count: 5,
            role_hint: Some("glass-panel".to_string()),
        };

        let extractor = EffectExtractor;
        let effects = extractor.extract(vec![obs]);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0].0.category, "glass");
        assert_eq!(effects[0].0.recipe.as_deref(), Some("glassmorphism"));
    }

    #[test]
    fn test_classify_elevation_shadow() {
        let mut props = BTreeMap::new();
        props.insert(
            "box-shadow".to_string(),
            "0 4px 6px -1px rgba(0, 0, 0, 0.1)".to_string(),
        );

        let obs = ObservedEffect {
            properties: props,
            elements_count: 12,
            role_hint: None,
        };

        let extractor = EffectExtractor;
        let effects = extractor.extract(vec![obs]);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0].0.category, "shadow");
        assert_eq!(effects[0].0.recipe.as_deref(), Some("elevation-md"));
    }
}
