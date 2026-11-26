#![forbid(unsafe_code)]

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// High-level scripted interaction flow (SPEC §4.2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FlowScript {
    pub start: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewport: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    pub steps: Vec<FlowStep>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FlowStep {
    #[serde(flatten)]
    pub action: FlowAction,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub optional: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum FlowAction {
    Wait {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        selector: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timeout_ms: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        ms: Option<u64>,
    },
    Screenshot {
        name: String,
        #[serde(default)]
        full_page: bool,
    },
    Hover {
        selector: String,
    },
    Click {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        selector: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        text: Option<String>,
    },
    Type {
        selector: String,
        text: String,
    },
    Press {
        key: String,
    },
    Scroll {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        step_px: Option<i32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pause_ms: Option<u64>,
    },
    Capture {
        label: String,
    },
    #[serde(rename = "set_scheme")]
    SetScheme {
        scheme: String,
    },
    Resize {
        width: u32,
        height: u32,
    },
    Eval {
        script: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow_script_yaml_and_json() {
        let yaml_text = r#"
start: https://example.com
viewport: mobile
scheme: dark
steps:
  - wait:
      selector: "header"
      timeout_ms: 5000
  - screenshot:
      name: "home-initial"
      full_page: false
  - hover:
      selector: "nav a:first-child"
  - click:
      selector: "button[aria-label='Menu']"
  - scroll:
      to: "bottom"
      step_px: 400
      pause_ms: 150
  - type:
      selector: "input[type=email]"
      text: "name@example.com"
  - press:
      key: "Tab"
  - capture:
      label: "pricing-annual"
"#;
        let script: FlowScript = serde_yaml::from_str(yaml_text).unwrap();
        assert_eq!(script.start, "https://example.com");
        assert_eq!(script.viewport.as_deref(), Some("mobile"));
        assert_eq!(script.scheme.as_deref(), Some("dark"));
        assert_eq!(script.steps.len(), 8);
    }
}
