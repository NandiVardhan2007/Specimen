use specimen_core::{FontFamily, ItemType, License, LicenseStatus, TypeScaleEntry, TypographyData};

#[derive(Debug, Clone, Default)]
pub struct TypographyExtractor;

impl TypographyExtractor {
    pub fn kind(&self) -> ItemType {
        ItemType::Typography
    }

    pub fn version(&self) -> u32 {
        1
    }

    /// Extracts a normalized TypographyData from parsed font families and type scale observations.
    pub fn extract(
        &self,
        families: Vec<FontFamily>,
        mut scale: Vec<TypeScaleEntry>,
    ) -> (TypographyData, f64, License) {
        // Sort scale descending by size_px for clean readability
        scale.sort_by(|a, b| {
            b.size_px
                .partial_cmp(&a.size_px)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let data = TypographyData { families, scale };

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
    fn test_typography_extractor() {
        let extractor = TypographyExtractor;
        let families = vec![FontFamily {
            name: "Inter".to_string(),
            stack: vec!["Inter".to_string(), "sans-serif".to_string()],
            delivery: "self-hosted".to_string(),
            files: Vec::new(),
            license_hints: None,
        }];

        let scale = vec![
            TypeScaleEntry {
                role: "body".to_string(),
                family: "Inter".to_string(),
                size_px: 16.0,
                weight: 400,
                line_height: 1.5,
                letter_spacing_em: 0.0,
            },
            TypeScaleEntry {
                role: "h1".to_string(),
                family: "Inter".to_string(),
                size_px: 48.0,
                weight: 700,
                line_height: 1.1,
                letter_spacing_em: -0.02,
            },
        ];

        let (typo, confidence, license) = extractor.extract(families, scale);
        assert_eq!(typo.families.len(), 1);
        assert_eq!(typo.scale[0].role, "h1"); // Verified descending sort
        assert_eq!(confidence, 1.0);
        assert_eq!(license.status, LicenseStatus::Reusable);
    }
}
