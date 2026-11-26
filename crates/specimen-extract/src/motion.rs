#![forbid(unsafe_code)]

use std::collections::BTreeMap;

use specimen_core::{
    AnimationData, Keyframe, License, LicenseStatus, PreviewClip, Target, Timing, Trigger,
};

/// Observed animation data before normalization into an Item.
#[derive(Debug, Clone)]
pub struct ObservedAnimation {
    pub name: String,
    pub kind: String,
    pub driver: String,
    pub keyframes: Vec<Keyframe>,
    pub timing: Timing,
    pub trigger: Option<Trigger>,
    pub targets: Vec<Target>,
    pub stagger_ms: Option<f64>,
    pub preview: Option<PreviewClip>,
}

pub struct MotionExtractor;

impl MotionExtractor {
    /// Extracts animations and applies confidence and license attributes.
    pub fn extract(&self, observed: Vec<ObservedAnimation>) -> Vec<(AnimationData, f64, License)> {
        let mut results = Vec::new();

        for obs in observed {
            let data = AnimationData {
                kind: obs.kind,
                name: obs.name,
                driver: obs.driver,
                keyframes: obs.keyframes,
                timing: obs.timing,
                trigger: obs.trigger,
                targets: obs.targets,
                stagger_ms: obs.stagger_ms,
                preview: obs.preview,
            };

            let license = License {
                status: LicenseStatus::Reusable,
                basis: "mathematical timing function and keyframe transformations".to_string(),
                spdx: None,
            };

            results.push((data, 1.0, license));
        }

        results
    }

    /// Parses CSS styles and `@keyframes` declarations from CSS text.
    pub fn extract_from_css(&self, css: &str) -> Vec<ObservedAnimation> {
        let mut animations = Vec::new();
        let keyframes_map = Self::parse_keyframes_blocks(css);

        // 1. Match animation declarations with keyframes
        for (name, kfs) in &keyframes_map {
            // Find timing details from css rules using this animation name
            let timing = Self::find_animation_timing_for_name(css, name);
            let target_selector = Self::find_selector_using_animation(css, name);

            let mut targets = Vec::new();
            if let Some(sel) = target_selector {
                targets.push(Target {
                    selector: sel,
                    role: None,
                });
            }

            animations.push(ObservedAnimation {
                name: name.clone(),
                kind: "css-keyframes".to_string(),
                driver: "css".to_string(),
                keyframes: kfs.clone(),
                timing,
                trigger: None,
                targets,
                stagger_ms: None,
                preview: None,
            });
        }

        // 2. Parse transition declarations
        let transitions = Self::parse_transition_rules(css);
        animations.extend(transitions);

        animations
    }

    /// Converts runtime animation records (e.g. from WAAPI getAnimations) into ObservedAnimations.
    pub fn extract_from_waapi(
        &self,
        name: &str,
        duration_ms: f64,
        delay_ms: f64,
        easing: &str,
        iterations: f64,
        target_selector: Option<String>,
    ) -> ObservedAnimation {
        let mut targets = Vec::new();
        if let Some(sel) = target_selector {
            targets.push(Target {
                selector: sel,
                role: None,
            });
        }

        ObservedAnimation {
            name: name.to_string(),
            kind: "waapi".to_string(),
            driver: "waapi".to_string(),
            keyframes: Vec::new(),
            timing: Timing {
                duration_ms,
                delay_ms,
                easing: easing.to_string(),
                iterations,
                direction: "normal".to_string(),
                fill: "none".to_string(),
            },
            trigger: None,
            targets,
            stagger_ms: None,
            preview: None,
        }
    }

    fn parse_keyframes_blocks(css: &str) -> BTreeMap<String, Vec<Keyframe>> {
        let mut result = BTreeMap::new();
        let mut remaining = css;

        while let Some(kf_idx) = remaining.find("@keyframes") {
            let slice = &remaining[kf_idx + 10..];
            if let Some(brace_start) = slice.find('{') {
                let name = slice[..brace_start].trim().to_string();
                let inner_slice = &slice[brace_start + 1..];

                // Find matching closing brace
                let mut depth = 1;
                let mut end_idx = None;
                for (idx, ch) in inner_slice.char_indices() {
                    if ch == '{' {
                        depth += 1;
                    } else if ch == '}' {
                        depth -= 1;
                        if depth == 0 {
                            end_idx = Some(idx);
                            break;
                        }
                    }
                }

                if let Some(end) = end_idx {
                    let kf_body = &inner_slice[..end];
                    let keyframes = Self::parse_keyframe_steps(kf_body);
                    if !name.is_empty() {
                        result.insert(name, keyframes);
                    }
                    remaining = &inner_slice[end + 1..];
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        result
    }

    fn parse_keyframe_steps(body: &str) -> Vec<Keyframe> {
        let mut steps = Vec::new();
        let mut remaining = body;

        while let Some(open_brace) = remaining.find('{') {
            let selector_part = remaining[..open_brace].trim();
            let after_open = &remaining[open_brace + 1..];
            if let Some(close_brace) = after_open.find('}') {
                let decl_part = &after_open[..close_brace];
                let props = Self::parse_declarations(decl_part);

                let offsets = Self::parse_offsets(selector_part);
                for offset in offsets {
                    steps.push(Keyframe {
                        offset,
                        props: props.clone(),
                        easing: None,
                    });
                }
                remaining = &after_open[close_brace + 1..];
            } else {
                break;
            }
        }

        steps.sort_by(|a, b| {
            a.offset
                .partial_cmp(&b.offset)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        steps
    }

    fn parse_offsets(sel: &str) -> Vec<f64> {
        let mut offsets = Vec::new();
        for part in sel.split(',') {
            let part = part.trim();
            if part == "from" || part == "0%" {
                offsets.push(0.0);
            } else if part == "to" || part == "100%" {
                offsets.push(1.0);
            } else if let Some(pct) = part.strip_suffix('%') {
                if let Ok(val) = pct.trim().parse::<f64>() {
                    offsets.push(val / 100.0);
                }
            }
        }
        offsets
    }

    fn parse_declarations(decl: &str) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        for item in decl.split(';') {
            let item = item.trim();
            if let Some(colon) = item.find(':') {
                let key = item[..colon].trim().to_string();
                let val = item[colon + 1..].trim().to_string();
                if !key.is_empty() && !val.is_empty() {
                    map.insert(key, val);
                }
            }
        }
        map
    }

    fn find_animation_timing_for_name(css: &str, anim_name: &str) -> Timing {
        for line in css.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("animation:") && trimmed.contains(anim_name) {
                return Self::parse_shorthand_animation(trimmed);
            }
        }
        Timing::default()
    }

    fn find_selector_using_animation(css: &str, anim_name: &str) -> Option<String> {
        let mut current_selector = None;
        for line in css.lines() {
            let trimmed = line.trim();
            if trimmed.ends_with('{') {
                current_selector = Some(trimmed.trim_end_matches('{').trim().to_string());
            } else if trimmed.contains("animation:") && trimmed.contains(anim_name) {
                return current_selector;
            }
        }
        None
    }

    fn parse_shorthand_animation(decl: &str) -> Timing {
        let val = decl
            .strip_prefix("animation:")
            .unwrap_or(decl)
            .trim_end_matches(';')
            .trim();

        let mut duration_ms = 0.0;
        let mut easing = "ease".to_string();
        let mut iterations = 1.0;
        let mut fill = "none".to_string();
        let mut direction = "normal".to_string();

        let tokens: Vec<&str> = val.split_whitespace().collect();
        for token in tokens {
            if token.ends_with("ms") {
                if let Ok(ms) = token.trim_end_matches("ms").parse::<f64>() {
                    duration_ms = ms;
                }
            } else if token.ends_with('s') && !token.contains("ease") {
                if let Ok(s) = token.trim_end_matches('s').parse::<f64>() {
                    duration_ms = s * 1000.0;
                }
            } else if token.starts_with("cubic-bezier")
                || token == "linear"
                || token == "ease-in"
                || token == "ease-out"
                || token == "ease-in-out"
                || token == "ease"
            {
                easing = token.to_string();
            } else if token == "infinite" {
                iterations = -1.0;
            } else if token == "both" || token == "forwards" || token == "backwards" {
                fill = token.to_string();
            } else if token == "alternate" || token == "reverse" {
                direction = token.to_string();
            }
        }

        // Check if there was a comma in cubic-bezier split by whitespace
        if val.contains("cubic-bezier(") {
            if let Some(start) = val.find("cubic-bezier(") {
                if let Some(end) = val[start..].find(')') {
                    easing = val[start..start + end + 1].to_string();
                }
            }
        }

        Timing {
            duration_ms,
            delay_ms: 0.0,
            easing,
            iterations,
            direction,
            fill,
        }
    }

    fn parse_transition_rules(css: &str) -> Vec<ObservedAnimation> {
        let mut results = Vec::new();
        let mut current_selector = String::new();

        for line in css.lines() {
            let trimmed = line.trim();
            if trimmed.ends_with('{') {
                current_selector = trimmed.trim_end_matches('{').trim().to_string();
            } else if trimmed.starts_with("transition:") {
                let val = trimmed
                    .strip_prefix("transition:")
                    .unwrap_or(trimmed)
                    .trim_end_matches(';')
                    .trim();

                for part in val.split(',') {
                    let part = part.trim();
                    let tokens: Vec<&str> = part.split_whitespace().collect();
                    if !tokens.is_empty() {
                        let prop = tokens[0].to_string();
                        let mut duration_ms = 0.0;
                        let mut easing = "ease".to_string();

                        for &token in &tokens[1..] {
                            if token.ends_with("ms") {
                                if let Ok(ms) = token.trim_end_matches("ms").parse::<f64>() {
                                    duration_ms = ms;
                                }
                            } else if token.ends_with('s') {
                                if let Ok(s) = token.trim_end_matches('s').parse::<f64>() {
                                    duration_ms = s * 1000.0;
                                }
                            } else if token == "linear"
                                || token == "ease-in"
                                || token == "ease-out"
                                || token == "ease-in-out"
                                || token == "ease"
                            {
                                easing = token.to_string();
                            }
                        }

                        let mut targets = Vec::new();
                        if !current_selector.is_empty() {
                            targets.push(Target {
                                selector: current_selector.clone(),
                                role: None,
                            });
                        }

                        results.push(ObservedAnimation {
                            name: format!("{}-transition", prop),
                            kind: "css-transition".to_string(),
                            driver: "css".to_string(),
                            keyframes: Vec::new(),
                            timing: Timing {
                                duration_ms,
                                delay_ms: 0.0,
                                easing,
                                iterations: 1.0,
                                direction: "normal".to_string(),
                                fill: "none".to_string(),
                            },
                            trigger: Some(Trigger {
                                r#type: "hover".to_string(),
                                selector: Some(current_selector.clone()),
                                threshold: None,
                            }),
                            targets,
                            stagger_ms: None,
                            preview: None,
                        });
                    }
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_css_keyframes_and_timing() {
        let css = r#"
        @keyframes fade-up {
          0% {
            opacity: 0;
            transform: translateY(16px);
          }
          100% {
            opacity: 1;
            transform: translateY(0);
          }
        }

        .feature-card {
          animation: fade-up 600ms cubic-bezier(0.16, 1, 0.3, 1) both;
        }
        "#;

        let extractor = MotionExtractor;
        let animations = extractor.extract_from_css(css);
        assert_eq!(animations.len(), 1);
        let anim = &animations[0];
        assert_eq!(anim.name, "fade-up");
        assert_eq!(anim.keyframes.len(), 2);
        assert_eq!(anim.timing.duration_ms, 600.0);
        assert_eq!(anim.timing.fill, "both");
        assert_eq!(anim.targets[0].selector, ".feature-card");
    }

    #[test]
    fn test_extract_css_transition() {
        let css = r#"
        .btn-hover {
          transition: background-color 200ms ease-in-out;
        }
        "#;

        let extractor = MotionExtractor;
        let animations = extractor.extract_from_css(css);
        assert_eq!(animations.len(), 1);
        let trans = &animations[0];
        assert_eq!(trans.kind, "css-transition");
        assert_eq!(trans.name, "background-color-transition");
        assert_eq!(trans.timing.duration_ms, 200.0);
        assert_eq!(trans.timing.easing, "ease-in-out");
    }
}
