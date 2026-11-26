#![forbid(unsafe_code)]

use specimen_core::{PaletteData, ThemeData, TypographyData};

pub struct CssExportInput<'a> {
    pub light_palette: Option<&'a PaletteData>,
    pub dark_palette: Option<&'a PaletteData>,
    pub typography: Option<&'a TypographyData>,
    pub theme: Option<&'a ThemeData>,
}

pub fn export_css(input: &CssExportInput<'_>) -> String {
    let mut out = String::new();
    out.push_str("/* Specimen Design Tokens Export - CSS Variables */\n\n");

    // Root block
    out.push_str(":root {\n");

    // Colors (light or default palette)
    if let Some(pal) = input.light_palette.or(input.dark_palette) {
        out.push_str("  /* Colors */\n");
        for color in &pal.colors {
            let var_name = format!("--color-{}", color.role);
            out.push_str(&format!("  {}: {};\n", var_name, color.hex));
        }
    }

    // Typography
    if let Some(typo) = input.typography {
        out.push_str("\n  /* Typography Families */\n");
        for fam in &typo.families {
            let slug = fam.name.to_lowercase().replace(' ', "-");
            let stack_str = if fam.stack.is_empty() {
                format!("'{}', sans-serif", fam.name)
            } else {
                fam.stack
                    .iter()
                    .map(|s| {
                        if s.contains(' ') {
                            format!("'{}'", s)
                        } else {
                            s.clone()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            out.push_str(&format!("  --font-{}: {};\n", slug, stack_str));
        }

        out.push_str("\n  /* Type Scale */\n");
        for entry in &typo.scale {
            out.push_str(&format!(
                "  --text-{}-size: {}px;\n",
                entry.role, entry.size_px
            ));
            out.push_str(&format!(
                "  --text-{}-weight: {};\n",
                entry.role, entry.weight
            ));
            out.push_str(&format!(
                "  --text-{}-line-height: {};\n",
                entry.role, entry.line_height
            ));
        }
    }

    // Theme (Spacing, Radii, Breakpoints)
    if let Some(thm) = input.theme {
        out.push_str("\n  /* Spacing */\n");
        out.push_str(&format!("  --spacing-base: {}px;\n", thm.spacing_base_unit));
        for (i, &val) in thm.spacing_scale.iter().enumerate() {
            out.push_str(&format!("  --spacing-{}: {}px;\n", i + 1, val));
        }

        if !thm.radii_px.is_empty() {
            out.push_str("\n  /* Border Radii */\n");
            let labels = ["sm", "md", "lg", "xl", "2xl"];
            for (i, &val) in thm.radii_px.iter().enumerate() {
                let lbl = labels.get(i).copied().unwrap_or("var");
                out.push_str(&format!("  --radius-{}: {}px;\n", lbl, val));
            }
        }
    }

    out.push_str("}\n\n");

    // Dark Mode scheme block
    if let (Some(_), Some(dark_pal)) = (input.light_palette, input.dark_palette) {
        out.push_str("@media (prefers-color-scheme: dark) {\n  :root {\n");
        for color in &dark_pal.colors {
            let var_name = format!("--color-{}", color.role);
            out.push_str(&format!("    {}: {};\n", var_name, color.hex));
        }
        out.push_str("  }\n}\n\n");

        out.push_str("[data-theme=\"dark\"] {\n");
        for color in &dark_pal.colors {
            let var_name = format!("--color-{}", color.role);
            out.push_str(&format!("  {}: {};\n", var_name, color.hex));
        }
        out.push_str("}\n");
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use specimen_core::{ColorUsage, PaletteColor, PaletteData};

    #[test]
    fn test_export_css_basic() {
        let palette = PaletteData {
            scheme: "light".to_string(),
            colors: vec![
                PaletteColor {
                    role: "background".to_string(),
                    name: None,
                    hex: "#ffffff".to_string(),
                    oklch: [1.0, 0.0, 0.0],
                    alpha: 1.0,
                    usage: ColorUsage {
                        area_pct: 50.0,
                        elements: 10,
                    },
                    css_variables: Vec::new(),
                },
                PaletteColor {
                    role: "text".to_string(),
                    name: None,
                    hex: "#000000".to_string(),
                    oklch: [0.0, 0.0, 0.0],
                    alpha: 1.0,
                    usage: ColorUsage {
                        area_pct: 20.0,
                        elements: 30,
                    },
                    css_variables: Vec::new(),
                },
            ],
            pairs: Vec::new(),
        };

        let input = CssExportInput {
            light_palette: Some(&palette),
            dark_palette: None,
            typography: None,
            theme: None,
        };

        let css = export_css(&input);
        assert!(css.contains("--color-background: #ffffff;"));
        assert!(css.contains("--color-text: #000000;"));
    }
}
