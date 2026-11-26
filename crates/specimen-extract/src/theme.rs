use specimen_core::{ItemType, License, LicenseStatus, ThemeData};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct ThemeExtractor;

impl ThemeExtractor {
    pub fn kind(&self) -> ItemType {
        ItemType::Theme
    }

    pub fn version(&self) -> u32 {
        1
    }

    /// Extracts ThemeData, inferring spacing base unit and compiling scales (SPEC §6.3, §6.4).
    pub fn extract(
        &self,
        scheme: &str,
        raw_spacings: &[f64],
        raw_radii: &[f64],
        raw_breakpoints: &[u32],
        tokens: BTreeMap<String, String>,
    ) -> (ThemeData, f64, License) {
        let mut spacing_scale = Vec::new();
        for &s in raw_spacings {
            if s > 0.0 && !spacing_scale.iter().any(|&x: &f64| (x - s).abs() < 0.01) {
                spacing_scale.push(s);
            }
        }
        spacing_scale.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Infer base unit: find GCD of integer parts or default to 8 / 4
        let base_unit = infer_base_unit(&spacing_scale);

        let mut radii_px = Vec::new();
        for &r in raw_radii {
            if !radii_px.iter().any(|&x: &f64| (x - r).abs() < 0.01) {
                radii_px.push(r);
            }
        }
        radii_px.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let mut breakpoints_px = raw_breakpoints.to_vec();
        breakpoints_px.sort_unstable();
        breakpoints_px.dedup();

        let data = ThemeData {
            scheme: scheme.to_string(),
            spacing_base_unit: base_unit,
            spacing_scale,
            radii_px,
            breakpoints_px,
            tokens,
        };

        let license = License {
            status: LicenseStatus::Reusable,
            basis: "plain value, not a distributed asset (not legal advice)".to_string(),
            spdx: None,
        };

        (data, 1.0, license)
    }
}

fn infer_base_unit(spacing_scale: &[f64]) -> u32 {
    let int_values: Vec<u32> = spacing_scale
        .iter()
        .filter_map(|&s| {
            let rounded = s.round();
            if (s - rounded).abs() < 0.01 && rounded > 0.0 {
                Some(rounded as u32)
            } else {
                None
            }
        })
        .collect();

    if int_values.is_empty() {
        return 8;
    }

    let mut current_gcd = int_values[0];
    for &val in &int_values[1..] {
        current_gcd = gcd(current_gcd, val);
    }

    // Keep reasonable design system base units: 4 or 8 if divisible, otherwise gcd
    if current_gcd >= 8 && current_gcd.is_multiple_of(8) {
        8
    } else if current_gcd >= 4 && current_gcd.is_multiple_of(4) {
        4
    } else {
        current_gcd.max(1)
    }
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_extractor_base_unit_inference() {
        let extractor = ThemeExtractor;
        let spacings = vec![8.0, 16.0, 24.0, 32.0];
        let radii = vec![4.0, 8.0, 16.0];
        let breakpoints = vec![640, 1024];

        let (theme, confidence, license) =
            extractor.extract("light", &spacings, &radii, &breakpoints, BTreeMap::new());

        assert_eq!(theme.spacing_base_unit, 8);
        assert_eq!(theme.spacing_scale, vec![8.0, 16.0, 24.0, 32.0]);
        assert_eq!(theme.radii_px, vec![4.0, 8.0, 16.0]);
        assert_eq!(theme.breakpoints_px, vec![640, 1024]);
        assert_eq!(confidence, 1.0);
        assert_eq!(license.status, LicenseStatus::Reusable);
    }
}
