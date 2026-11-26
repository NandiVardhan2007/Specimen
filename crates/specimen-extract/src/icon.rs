#![forbid(unsafe_code)]

use specimen_core::{IconData, IconSetInfo, License, LicenseStatus};

use crate::svg::SvgSanitizer;

#[derive(Debug, Clone)]
pub struct ObservedIcon {
    pub raw_svg: String,
    pub source_kind: String, // "inline", "sprite", "image"
    pub role_hint: Option<String>,
}

pub struct IconExtractor;

impl IconExtractor {
    /// Extracts, sanitizes, and deduplicates icons from observed SVGs.
    pub fn extract(&self, observed: Vec<ObservedIcon>) -> Vec<(IconData, f64, License)> {
        let mut results = Vec::new();
        let mut seen_hashes = std::collections::HashSet::new();

        for obs in observed {
            if let Some(sanitized) = SvgSanitizer::sanitize(&obs.raw_svg) {
                if !seen_hashes.insert(sanitized.geometry_hash.clone()) {
                    continue; // Skip duplicate geometry
                }

                let icon_data = IconData {
                    svg: sanitized.clean_svg,
                    view_box: sanitized.view_box,
                    width: sanitized.width,
                    height: sanitized.height,
                    is_current_color: sanitized.has_current_color,
                    geometry_hash: sanitized.geometry_hash,
                    set: Some(IconSetInfo {
                        name: "Custom / Unmatched".to_string(),
                        license: "Reference only (extracted)".to_string(),
                        version: None,
                    }),
                };

                let license = License {
                    status: LicenseStatus::Unknown,
                    basis: format!("extracted from {}", obs.source_kind),
                    spdx: None,
                };

                results.push((icon_data, 0.95, license));
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_and_deduplicate_icons() {
        let svg1 = r#"<svg width="24" height="24" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10"></circle></svg>"#.to_string();
        let svg2 = r#"<svg width="24" height="24" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10"></circle></svg>"#.to_string(); // Duplicate
        let svg3 = r#"<svg width="24" height="24" viewBox="0 0 24 24"><line x1="1" y1="1" x2="2" y2="2"></line></svg>"#.to_string();

        let input = vec![
            ObservedIcon {
                raw_svg: svg1,
                source_kind: "inline".to_string(),
                role_hint: Some("icon-circle".to_string()),
            },
            ObservedIcon {
                raw_svg: svg2,
                source_kind: "inline".to_string(),
                role_hint: Some("icon-circle-dup".to_string()),
            },
            ObservedIcon {
                raw_svg: svg3,
                source_kind: "inline".to_string(),
                role_hint: Some("icon-line".to_string()),
            },
        ];

        let extractor = IconExtractor;
        let icons = extractor.extract(input);
        assert_eq!(icons.len(), 2);
    }
}
