#![forbid(unsafe_code)]

use specimen_core::{LayoutData, License, LicenseStatus};

#[derive(Debug, Clone)]
pub struct ObservedSection {
    pub tag: String,
    pub class_name: Option<String>,
    pub display: String, // "flex", "grid", "block"
    pub columns: Option<u32>,
    pub gap_px: Option<u32>,
}

pub struct LayoutExtractor;

impl LayoutExtractor {
    /// Extracts content-free layout sections and generates wireframe SVGs.
    pub fn extract(&self, sections: Vec<ObservedSection>) -> Vec<(LayoutData, f64, License)> {
        let mut results = Vec::new();

        for sec in sections {
            let section_type = Self::infer_section_type(&sec.tag, sec.class_name.as_deref());
            let wireframe_svg =
                Self::generate_wireframe_svg(&section_type, &sec.display, sec.columns);

            let data = LayoutData {
                section_type,
                display: sec.display,
                columns: sec.columns,
                gap_px: sec.gap_px,
                wireframe_svg: Some(wireframe_svg),
            };

            let license = License {
                status: LicenseStatus::Reusable,
                basis: "content-free structural layout skeleton".to_string(),
                spdx: None,
            };

            results.push((data, 1.0, license));
        }

        results
    }

    fn infer_section_type(tag: &str, class_name: Option<&str>) -> String {
        let class_lower = class_name.unwrap_or_default().to_lowercase();
        if tag == "nav" || class_lower.contains("nav") || class_lower.contains("header") {
            "nav".to_string()
        } else if class_lower.contains("hero") {
            "hero".to_string()
        } else if class_lower.contains("grid") || class_lower.contains("features") {
            "grid".to_string()
        } else if class_lower.contains("pricing") {
            "pricing".to_string()
        } else if tag == "footer" || class_lower.contains("footer") {
            "footer".to_string()
        } else {
            "section".to_string()
        }
    }

    fn generate_wireframe_svg(section_type: &str, display: &str, columns: Option<u32>) -> String {
        let cols = columns.unwrap_or(if display == "grid" { 3 } else { 1 });
        let mut svg = format!(
            r##"<svg viewBox="0 0 800 200" width="800" height="200" xmlns="http://www.w3.org/2000/svg">
  <rect width="100%" height="100%" fill="#1e293b" rx="8"/>
  <!-- Wireframe: {} ({}) -->
"##,
            section_type, display
        );

        let col_width = (760 - (cols - 1) * 20) / cols;
        for i in 0..cols {
            let x = 20 + i * (col_width + 20);
            svg.push_str(&format!(
                r##"  <rect x="{}" y="20" width="{}" height="160" fill="#334155" rx="4"/>
"##,
                x, col_width
            ));
        }
        svg.push_str("</svg>");
        svg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_layout_sections() {
        let input = vec![
            ObservedSection {
                tag: "nav".to_string(),
                class_name: Some("top-nav".to_string()),
                display: "flex".to_string(),
                columns: None,
                gap_px: Some(16),
            },
            ObservedSection {
                tag: "section".to_string(),
                class_name: Some("features-grid".to_string()),
                display: "grid".to_string(),
                columns: Some(3),
                gap_px: Some(24),
            },
        ];

        let extractor = LayoutExtractor;
        let layouts = extractor.extract(input);
        assert_eq!(layouts.len(), 2);
        assert_eq!(layouts[0].0.section_type, "nav");
        assert_eq!(layouts[1].0.section_type, "grid");
        assert_eq!(layouts[1].0.columns, Some(3));
        assert!(layouts[1].0.wireframe_svg.is_some());
    }
}
