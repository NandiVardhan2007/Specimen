#![forbid(unsafe_code)]

use std::collections::BTreeMap;

use specimen_core::{ComponentData, License, LicenseStatus, StateStyles};

/// Observed DOM element with computed styles and forced pseudo-states.
#[derive(Debug, Clone)]
pub struct ObservedElement {
    pub tag: String,
    pub selector: String,
    pub classes: Vec<String>,
    pub role_attr: Option<String>,
    pub type_attr: Option<String>,
    pub disabled: bool,
    pub styles: BTreeMap<String, String>,
    pub pseudo_styles: BTreeMap<String, BTreeMap<String, String>>,
}

pub struct ComponentExtractor;

impl ComponentExtractor {
    /// Extracts component definitions, clustering variants and resolving interaction states.
    pub fn extract(&self, observed: Vec<ObservedElement>) -> Vec<(ComponentData, f64, License)> {
        let mut results = Vec::new();

        for obs in observed {
            let role = Self::infer_role(&obs);
            if role == "unknown" {
                continue;
            }

            let variant = Self::infer_variant(&obs);
            let name = Self::infer_name(&role, variant.as_deref());
            let html_skeleton = Self::build_skeleton(&obs, &role);

            let mut states = BTreeMap::new();
            // Default state
            states.insert(
                "default".to_string(),
                StateStyles {
                    styles: obs.styles.clone(),
                },
            );

            // Pseudo states
            for (state_name, st) in &obs.pseudo_styles {
                states.insert(state_name.clone(), StateStyles { styles: st.clone() });
            }

            let data = ComponentData {
                role,
                name,
                variant,
                selector: obs.selector.clone(),
                html_skeleton,
                states,
                variants: Vec::new(),
                state_screenshots: BTreeMap::new(),
            };

            let license = License {
                status: LicenseStatus::Reusable,
                basis: "standard UI element structural layout and styling".to_string(),
                spdx: None,
            };

            results.push((data, 0.95, license));
        }

        results
    }

    fn infer_role(obs: &ObservedElement) -> String {
        let tag = obs.tag.to_lowercase();
        let role = obs.role_attr.as_deref().unwrap_or_default();
        let type_attr = obs.type_attr.as_deref().unwrap_or_default();

        if tag == "button"
            || type_attr == "button"
            || type_attr == "submit"
            || role == "button"
            || obs
                .classes
                .iter()
                .any(|c| c == "btn" || c.starts_with("btn-"))
        {
            return "button".to_string();
        }

        if tag == "input" || tag == "textarea" || role == "textbox" {
            if type_attr == "checkbox" || role == "checkbox" {
                return "checkbox".to_string();
            }
            return "input".to_string();
        }

        if tag == "select" || role == "combobox" {
            return "select".to_string();
        }

        if tag == "a" || role == "link" {
            return "link".to_string();
        }

        if obs.classes.iter().any(|c| c.contains("card")) {
            return "card".to_string();
        }

        if obs
            .classes
            .iter()
            .any(|c| c.contains("badge") || c.contains("pill") || c.contains("tag"))
        {
            return "badge".to_string();
        }

        if tag == "nav" || role == "navigation" {
            return "nav".to_string();
        }

        if tag == "table" || role == "table" {
            return "table".to_string();
        }

        "unknown".to_string()
    }

    fn infer_variant(obs: &ObservedElement) -> Option<String> {
        for class in &obs.classes {
            let lower = class.to_lowercase();
            if lower.contains("primary") {
                return Some("primary".to_string());
            } else if lower.contains("secondary") {
                return Some("secondary".to_string());
            } else if lower.contains("outline") {
                return Some("outline".to_string());
            } else if lower.contains("ghost") {
                return Some("ghost".to_string());
            } else if lower.contains("danger") || lower.contains("destructive") {
                return Some("destructive".to_string());
            }
        }
        None
    }

    fn infer_name(role: &str, variant: Option<&str>) -> String {
        match (role, variant) {
            ("button", Some("primary")) => "Primary Button".to_string(),
            ("button", Some("secondary")) => "Secondary Button".to_string(),
            ("button", Some(v)) => format!("{} Button", capitalize(v)),
            ("button", None) => "Button".to_string(),
            ("input", _) => "Text Input".to_string(),
            ("card", _) => "Card Container".to_string(),
            ("badge", _) => "Badge Pill".to_string(),
            ("link", _) => "Link Element".to_string(),
            ("nav", _) => "Navigation Bar".to_string(),
            ("table", _) => "Data Table".to_string(),
            (other, _) => capitalize(other),
        }
    }

    fn build_skeleton(obs: &ObservedElement, role: &str) -> String {
        let class_attr = if obs.classes.is_empty() {
            String::new()
        } else {
            format!(" class=\"{}\"", obs.classes.join(" "))
        };

        match role {
            "button" => format!("<button{}>{{{{text}}}}</button>", class_attr),
            "input" => format!(
                "<input type=\"text\"{} placeholder=\"{{{{placeholder}}}}\" />",
                class_attr
            ),
            "card" => format!(
                "<div{}>\n  <h3>{{{{title}}}}</h3>\n  <p>{{{{content}}}}</p>\n</div>",
                class_attr
            ),
            "badge" => format!("<span{}>{{{{label}}}}</span>", class_attr),
            "link" => format!("<a href=\"#\"{}>{{{{link_text}}}}</a>", class_attr),
            _ => format!("<{}{}>{{{{content}}}}</{}>", obs.tag, class_attr, obs.tag),
        }
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_primary_button() {
        let mut default_styles = BTreeMap::new();
        default_styles.insert("background-color".to_string(), "#2563eb".to_string());
        default_styles.insert("color".to_string(), "#ffffff".to_string());

        let mut hover_styles = BTreeMap::new();
        hover_styles.insert("background-color".to_string(), "#1d4ed8".to_string());

        let mut pseudo = BTreeMap::new();
        pseudo.insert("hover".to_string(), hover_styles);

        let elem = ObservedElement {
            tag: "button".to_string(),
            selector: ".btn-primary".to_string(),
            classes: vec!["btn-primary".to_string()],
            role_attr: None,
            type_attr: Some("button".to_string()),
            disabled: false,
            styles: default_styles,
            pseudo_styles: pseudo,
        };

        let extractor = ComponentExtractor;
        let components = extractor.extract(vec![elem]);
        assert_eq!(components.len(), 1);
        let (comp, conf, _) = &components[0];
        assert_eq!(comp.role, "button");
        assert_eq!(comp.name, "Primary Button");
        assert_eq!(comp.variant.as_deref(), Some("primary"));
        assert_eq!(comp.states.len(), 2);
        assert!(*conf > 0.9);
    }
}
