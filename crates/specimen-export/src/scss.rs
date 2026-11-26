#![forbid(unsafe_code)]

use specimen_core::{PaletteData, ThemeData, TypographyData};

#[derive(Debug, Clone)]
pub struct ScssExportInput<'a> {
    pub palette: Option<&'a PaletteData>,
    pub typography: Option<&'a TypographyData>,
    pub theme: Option<&'a ThemeData>,
}

/// Generates SCSS variables from design tokens.
pub fn export_scss(input: &ScssExportInput<'_>) -> String {
    let mut out = String::new();
    out.push_str("// Specimen Design Tokens - SCSS Variables\n");
    out.push_str("// Generated deterministically by Specimen\n\n");

    if let Some(pal) = input.palette {
        out.push_str("// --- Colors ---\n");
        for c in &pal.colors {
            let role_slug = c.role.replace(' ', "-");
            out.push_str(&format!("$color-{}: {};\n", role_slug, c.hex));
        }
        out.push('\n');
    }

    if let Some(typo) = input.typography {
        out.push_str("// --- Typography ---\n");
        for s in &typo.scale {
            let role_slug = s.role.replace(' ', "-");
            out.push_str(&format!("$font-size-{}: {}px;\n", role_slug, s.size_px));
            out.push_str(&format!("$line-height-{}: {};\n", role_slug, s.line_height));
        }
        out.push('\n');
    }

    if let Some(thm) = input.theme {
        out.push_str("// --- Spacing & Theme ---\n");
        out.push_str(&format!("$spacing-base: {}px;\n", thm.spacing_base_unit));
        for (idx, &s) in thm.spacing_scale.iter().enumerate() {
            out.push_str(&format!("$spacing-{}: {}px;\n", idx + 1, s));
        }
        for (idx, &r) in thm.radii_px.iter().enumerate() {
            out.push_str(&format!("$radius-{}: {}px;\n", idx + 1, r));
        }
        for (idx, &bp) in thm.breakpoints_px.iter().enumerate() {
            out.push_str(&format!("$breakpoint-{}: {}px;\n", idx + 1, bp));
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use specimen_core::ColorUsage;

    #[test]
    fn test_export_scss_basic() {
        let pal = PaletteData {
            scheme: "dark".to_string(),
            colors: vec![specimen_core::PaletteColor {
                role: "background".to_string(),
                name: None,
                hex: "#0b0d12".to_string(),
                oklch: [0.17, 0.012, 265.0],
                alpha: 1.0,
                usage: ColorUsage {
                    area_pct: 42.0,
                    elements: 10,
                },
                css_variables: vec![],
            }],
            pairs: vec![],
        };

        let input = ScssExportInput {
            palette: Some(&pal),
            typography: None,
            theme: None,
        };

        let scss = export_scss(&input);
        assert!(scss.contains("$color-background: #0b0d12;"));
    }
}
