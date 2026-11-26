use specimen_core::{
    ColorUsage, ContrastPair, ItemType, License, LicenseStatus, PaletteColor, PaletteData,
};

use crate::color::{parse_hex, srgb_to_oklch, wcag_contrast_ratio};

#[derive(Debug, Clone)]
pub struct ObservedColor {
    pub hex: String,
    pub area_pct: f64,
    pub elements: u32,
    pub role_hint: Option<String>,
    pub css_var: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct PaletteExtractor;

impl PaletteExtractor {
    pub fn kind(&self) -> ItemType {
        ItemType::Palette
    }

    pub fn version(&self) -> u32 {
        1
    }

    /// Extracts a normalized PaletteData from observed color usage records.
    pub fn extract(&self, scheme: &str, observed: &[ObservedColor]) -> (PaletteData, f64, License) {
        let mut colors = Vec::new();

        for obs in observed {
            if let Some((r, g, b)) = parse_hex(&obs.hex) {
                let oklch = srgb_to_oklch(r, g, b);
                let role = obs.role_hint.clone().unwrap_or_else(|| {
                    if obs.area_pct > 30.0 {
                        "background".to_string()
                    } else if obs.area_pct > 15.0 {
                        "surface".to_string()
                    } else {
                        "accent".to_string()
                    }
                });

                let mut css_variables = Vec::new();
                if let Some(ref v) = obs.css_var {
                    css_variables.push(v.clone());
                }

                colors.push(PaletteColor {
                    role,
                    name: None,
                    hex: obs.hex.clone(),
                    oklch,
                    alpha: 1.0,
                    usage: ColorUsage {
                        area_pct: obs.area_pct,
                        elements: obs.elements,
                    },
                    css_variables,
                });
            }
        }

        // Compute contrast pairs between text-like roles and background-like roles
        let mut pairs = Vec::new();
        let bg_colors: Vec<&PaletteColor> = colors
            .iter()
            .filter(|c| c.role == "background" || c.role == "surface")
            .collect();
        let text_colors: Vec<&PaletteColor> = colors
            .iter()
            .filter(|c| c.role == "text" || c.role == "text-muted" || c.role == "accent")
            .collect();

        for bg in &bg_colors {
            for fg in &text_colors {
                if let (Some(c_bg), Some(c_fg)) = (parse_hex(&bg.hex), parse_hex(&fg.hex)) {
                    let ratio = wcag_contrast_ratio(c_fg, c_bg);
                    pairs.push(ContrastPair {
                        fg: fg.hex.clone(),
                        bg: bg.hex.clone(),
                        wcag_ratio: ratio,
                        aa: ratio >= 4.5,
                        aaa: ratio >= 7.0,
                    });
                }
            }
        }

        let data = PaletteData {
            scheme: scheme.to_string(),
            colors,
            pairs,
        };

        // Measured plain values are reusable per SPEC §7.3
        let license = License {
            status: LicenseStatus::Reusable,
            basis: "plain value, not a distributed asset (not legal advice)".to_string(),
            spdx: None,
        };

        (data, 1.0, license)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_extractor() {
        let extractor = PaletteExtractor;
        let observed = vec![
            ObservedColor {
                hex: "#0b0d12".to_string(),
                area_pct: 45.0,
                elements: 120,
                role_hint: Some("background".to_string()),
                css_var: Some("--bg".to_string()),
            },
            ObservedColor {
                hex: "#f0f2f5".to_string(),
                area_pct: 12.0,
                elements: 85,
                role_hint: Some("text".to_string()),
                css_var: Some("--text".to_string()),
            },
        ];

        let (palette, confidence, license) = extractor.extract("dark", &observed);
        assert_eq!(palette.scheme, "dark");
        assert_eq!(palette.colors.len(), 2);
        assert_eq!(palette.pairs.len(), 1);
        assert!(palette.pairs[0].aa);
        assert!(palette.pairs[0].aaa);
        assert_eq!(confidence, 1.0);
        assert_eq!(license.status, LicenseStatus::Reusable);
    }
}
