#![forbid(unsafe_code)]

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Computed CSS styles for a specific interaction state (e.g., hover, focus).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StateStyles {
    pub styles: BTreeMap<String, String>,
}

/// A component variant declaration (e.g. secondary button).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ComponentVariant {
    pub name: String,
    pub selector: String,
}

/// Schema for captured component items (`specimen.item/1` with type `component`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ComponentData {
    /// "button", "link", "input", "select", "checkbox", "card", "badge", "tabs", "modal", "nav", "table", "tooltip", "toast", "unknown"
    pub role: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    pub selector: String,
    pub html_skeleton: String,
    /// Interactivity states: "default", "hover", "focus", "active", "disabled"
    pub states: BTreeMap<String, StateStyles>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<ComponentVariant>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub state_screenshots: BTreeMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_data_serialization() {
        let mut default_styles = BTreeMap::new();
        default_styles.insert("background-color".to_string(), "#2563eb".to_string());
        default_styles.insert("color".to_string(), "#ffffff".to_string());
        default_styles.insert("border-radius".to_string(), "6px".to_string());

        let mut hover_styles = BTreeMap::new();
        hover_styles.insert("background-color".to_string(), "#1d4ed8".to_string());

        let mut states = BTreeMap::new();
        states.insert(
            "default".to_string(),
            StateStyles {
                styles: default_styles,
            },
        );
        states.insert(
            "hover".to_string(),
            StateStyles {
                styles: hover_styles,
            },
        );

        let comp = ComponentData {
            role: "button".to_string(),
            name: "Primary Button".to_string(),
            variant: Some("primary".to_string()),
            selector: ".btn-primary".to_string(),
            html_skeleton: "<button class=\"btn-primary\">{{text}}</button>".to_string(),
            states,
            variants: vec![ComponentVariant {
                name: "secondary".to_string(),
                selector: ".btn-secondary".to_string(),
            }],
            state_screenshots: BTreeMap::new(),
        };

        let json = serde_json::to_string(&comp).unwrap();
        let parsed: ComponentData = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.role, "button");
        assert_eq!(parsed.name, "Primary Button");
        assert_eq!(parsed.states.len(), 2);
    }
}
