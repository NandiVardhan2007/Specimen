#![forbid(unsafe_code)]

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

fn default_easing() -> String {
    "ease".to_string()
}

fn default_iterations() -> f64 {
    1.0
}

fn default_direction() -> String {
    "normal".to_string()
}

fn default_fill() -> String {
    "none".to_string()
}

/// Keyframe in an animation timeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Keyframe {
    pub offset: f64,
    pub props: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub easing: Option<String>,
}

/// Timing parameters for an animation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Timing {
    pub duration_ms: f64,
    #[serde(default)]
    pub delay_ms: f64,
    #[serde(default = "default_easing")]
    pub easing: String,
    #[serde(default = "default_iterations")]
    pub iterations: f64,
    #[serde(default = "default_direction")]
    pub direction: String,
    #[serde(default = "default_fill")]
    pub fill: String,
}

impl Default for Timing {
    fn default() -> Self {
        Self {
            duration_ms: 0.0,
            delay_ms: 0.0,
            easing: default_easing(),
            iterations: default_iterations(),
            direction: default_direction(),
            fill: default_fill(),
        }
    }
}

/// Trigger condition for an animation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Trigger {
    /// "load", "hover", "focus", "click", "scroll-into-view", "scroll-linked", "timer", "media", "unknown"
    pub r#type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
}

/// Target element for an animation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Target {
    pub selector: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// Recorded preview clip metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PreviewClip {
    pub clip: String,
    pub fps: u32,
    pub duration_ms: f64,
}

/// Schema for captured motion items (`specimen.item/1` with type `animation`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AnimationData {
    /// "css-keyframes", "css-transition", "waapi", "js-sampled", "lottie", "canvas-recording"
    pub kind: String,
    pub name: String,
    /// "css", "waapi", "gsap", "framer-motion", "lottie", "anime", "unknown"
    pub driver: String,
    pub keyframes: Vec<Keyframe>,
    pub timing: Timing,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Trigger>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<Target>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stagger_ms: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<PreviewClip>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_data_serialization() {
        let mut props_0 = BTreeMap::new();
        props_0.insert("opacity".to_string(), "0".to_string());
        props_0.insert("transform".to_string(), "translateY(16px)".to_string());

        let mut props_1 = BTreeMap::new();
        props_1.insert("opacity".to_string(), "1".to_string());
        props_1.insert("transform".to_string(), "none".to_string());

        let anim = AnimationData {
            kind: "css-keyframes".to_string(),
            name: "fade-up".to_string(),
            driver: "css".to_string(),
            keyframes: vec![
                Keyframe {
                    offset: 0.0,
                    props: props_0,
                    easing: Some("ease-out".to_string()),
                },
                Keyframe {
                    offset: 1.0,
                    props: props_1,
                    easing: None,
                },
            ],
            timing: Timing {
                duration_ms: 600.0,
                delay_ms: 0.0,
                easing: "cubic-bezier(0.16,1,0.3,1)".to_string(),
                iterations: 1.0,
                direction: "normal".to_string(),
                fill: "both".to_string(),
            },
            trigger: Some(Trigger {
                r#type: "scroll-into-view".to_string(),
                selector: Some(".feature-card".to_string()),
                threshold: Some(0.2),
            }),
            targets: vec![Target {
                selector: ".feature-card".to_string(),
                role: Some("card".to_string()),
            }],
            stagger_ms: Some(80.0),
            preview: Some(PreviewClip {
                clip: "preview.webp".to_string(),
                fps: 30,
                duration_ms: 1200.0,
            }),
        };

        let json = serde_json::to_string(&anim).unwrap();
        let parsed: AnimationData = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.name, "fade-up");
        assert_eq!(parsed.keyframes.len(), 2);
        assert_eq!(parsed.timing.duration_ms, 600.0);
    }
}
