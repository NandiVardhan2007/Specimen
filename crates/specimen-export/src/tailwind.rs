#![forbid(unsafe_code)]

use specimen_core::{PaletteData, ThemeData, TypographyData};

pub struct TailwindExportInput<'a> {
    pub palette: Option<&'a PaletteData>,
    pub typography: Option<&'a TypographyData>,
    pub theme: Option<&'a ThemeData>,
    pub version: u32, // 3 or 4
}

/// Exports design tokens into Tailwind CSS (v4 @theme block or v3 preset module).
pub fn export_tailwind(input: &TailwindExportInput<'_>) -> String {
    if input.version == 3 {
        export_tailwind_v3(input)
    } else {
        export_tailwind_v4(input)
    }
}

fn export_tailwind_v4(input: &TailwindExportInput<'_>) -> String {
    let mut out = String::new();
    out.push_str("/* Specimen Design Tokens Export - Tailwind CSS v4 */\n\n");
    out.push_str("@theme {\n");

    // Colors
    if let Some(pal) = input.palette {
        out.push_str("  /* Colors */\n");
        for c in &pal.colors {
            out.push_str(&format!("  --color-{}: {};\n", c.role, c.hex));
        }
    }

    // Typography
    if let Some(typo) = input.typography {
        out.push_str("\n  /* Fonts */\n");
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
            out.push_str(&format!("  --text-{}: {}px;\n", entry.role, entry.size_px));
        }
    }

    // Spacing & Radii
    if let Some(thm) = input.theme {
        out.push_str("\n  /* Spacing */\n");
        for (i, &val) in thm.spacing_scale.iter().enumerate() {
            out.push_str(&format!("  --spacing-{}: {}px;\n", i + 1, val));
        }

        if !thm.radii_px.is_empty() {
            out.push_str("\n  /* Radii */\n");
            let labels = ["sm", "md", "lg", "xl", "2xl"];
            for (i, &val) in thm.radii_px.iter().enumerate() {
                let lbl = labels.get(i).copied().unwrap_or("var");
                out.push_str(&format!("  --radius-{}: {}px;\n", lbl, val));
            }
        }
    }

    out.push_str("}\n");
    out
}

fn export_tailwind_v3(input: &TailwindExportInput<'_>) -> String {
    let mut out = String::new();
    out.push_str("/** @type {import('tailwindcss').Config} */\n");
    out.push_str("module.exports = {\n  theme: {\n    extend: {\n");

    // Colors
    if let Some(pal) = input.palette {
        out.push_str("      colors: {\n");
        for c in &pal.colors {
            out.push_str(&format!("        '{}': '{}',\n", c.role, c.hex));
        }
        out.push_str("      },\n");
    }

    // Fonts
    if let Some(typo) = input.typography {
        out.push_str("      fontFamily: {\n");
        for fam in &typo.families {
            let slug = fam.name.to_lowercase().replace(' ', "-");
            out.push_str(&format!(
                "        '{}': {:?},\n",
                slug,
                if fam.stack.is_empty() {
                    vec![fam.name.clone()]
                } else {
                    fam.stack.clone()
                }
            ));
        }
        out.push_str("      },\n");
    }

    out.push_str("    },\n  },\n};\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use specimen_core::{ColorUsage, PaletteColor, PaletteData};

    #[test]
    fn test_export_tailwind_v4() {
        let palette = PaletteData {
            scheme: "dark".to_string(),
            colors: vec![PaletteColor {
                role: "accent".to_string(),
                name: None,
                hex: "#7c5cff".to_string(),
                oklch: [0.5, 0.2, 280.0],
                alpha: 1.0,
                usage: ColorUsage {
                    area_pct: 10.0,
                    elements: 5,
                },
                css_variables: Vec::new(),
            }],
            pairs: Vec::new(),
        };

        let input = TailwindExportInput {
            palette: Some(&palette),
            typography: None,
            theme: None,
            version: 4,
        };

        let res = export_tailwind(&input);
        assert!(res.contains("@theme {"));
        assert!(res.contains("--color-accent: #7c5cff;"));
    }
}
