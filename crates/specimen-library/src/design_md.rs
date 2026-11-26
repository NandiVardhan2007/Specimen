use crate::error::Result;
use specimen_core::{AnimationData, ComponentData, PaletteData, ThemeData, TypographyData};
use std::fs;
use std::path::{Path, PathBuf};

pub struct DesignDocMeta<'a> {
    pub source: &'a str,
    pub captured_at: &'a str,
    pub url: &'a str,
    pub schemes: &'a [String],
}

/// Generates a deterministic DESIGN.md without requiring an AI agent (--no-ai baseline per SPEC §8).
pub fn generate_design_md(
    dest_dir: &Path,
    meta: &DesignDocMeta,
    palette: Option<&PaletteData>,
    typography: Option<&TypographyData>,
    theme: Option<&ThemeData>,
    animations: Option<&[AnimationData]>,
    components: Option<&[ComponentData]>,
) -> Result<PathBuf> {
    let mut doc = String::new();

    // 1. YAML front matter
    doc.push_str("---\n");
    doc.push_str("specimen: 1\n");
    doc.push_str(&format!("source: {}\n", meta.source));
    doc.push_str(&format!("captured_at: {}\n", meta.captured_at));
    doc.push_str(&format!("url: {}\n", meta.url));
    let schemes_str = meta
        .schemes
        .iter()
        .map(|s| format!("\"{}\"", s))
        .collect::<Vec<_>>()
        .join(", ");
    doc.push_str(&format!("schemes: [{}]\n", schemes_str));
    doc.push_str("---\n\n");

    // 2. Title & Overview
    doc.push_str(&format!("# Design System: {}\n\n", meta.source));
    doc.push_str("Measured design tokens extracted by Specimen.\n\n");

    // 3. Color Palette Section
    if let Some(pal) = palette {
        doc.push_str("## Colors\n\n");
        doc.push_str("| Role | Hex | OKLCH | Usage Area | Contrast Pairs |\n");
        doc.push_str("|---|---|---|---|---|\n");
        for c in &pal.colors {
            let oklch_str = format!("{:.2}, {:.3}, {:.0}", c.oklch[0], c.oklch[1], c.oklch[2]);
            let pairs_str: Vec<String> = pal
                .pairs
                .iter()
                .filter(|p| p.fg == c.hex || p.bg == c.hex)
                .map(|p| {
                    format!(
                        "{} ({}:1)",
                        if p.fg == c.hex { &p.bg } else { &p.fg },
                        p.wcag_ratio
                    )
                })
                .take(2)
                .collect();
            let pairs_col = if pairs_str.is_empty() {
                "-".to_string()
            } else {
                pairs_str.join(", ")
            };
            doc.push_str(&format!(
                "| `{}` | `{}` | `{}` | {:.1}% | {} |\n",
                c.role, c.hex, oklch_str, c.usage.area_pct, pairs_col
            ));
        }
        doc.push('\n');
    }

    // 4. Typography Scale Section
    if let Some(typo) = typography {
        doc.push_str("## Typography\n\n");
        if !typo.families.is_empty() {
            doc.push_str("**Primary Families:** ");
            let fam_names: Vec<&str> = typo.families.iter().map(|f| f.name.as_str()).collect();
            doc.push_str(&fam_names.join(", "));
            doc.push_str("\n\n");
        }

        doc.push_str("| Role | Family | Size | Weight | Line Height | Letter Spacing |\n");
        doc.push_str("|---|---|---|---|---|---|\n");
        for s in &typo.scale {
            doc.push_str(&format!(
                "| `{}` | {} | {}px | {} | {} | {}em |\n",
                s.role, s.family, s.size_px, s.weight, s.line_height, s.letter_spacing_em
            ));
        }
        doc.push('\n');
    }

    // 5. Spacing, Radii, & Themes
    if let Some(thm) = theme {
        doc.push_str("## Spacing & Measurements\n\n");
        doc.push_str(&format!("- **Base Unit:** {}px\n", thm.spacing_base_unit));
        if !thm.spacing_scale.is_empty() {
            let scale_str: Vec<String> = thm
                .spacing_scale
                .iter()
                .map(|s| format!("{}px", s))
                .collect();
            doc.push_str(&format!("- **Scale:** {}\n", scale_str.join(", ")));
        }
        if !thm.radii_px.iter().all(|&r| r == 0.0) {
            let radii_str: Vec<String> = thm.radii_px.iter().map(|r| format!("{}px", r)).collect();
            doc.push_str(&format!("- **Border Radii:** {}\n", radii_str.join(", ")));
        }
        if !thm.breakpoints_px.is_empty() {
            let bp_str: Vec<String> = thm
                .breakpoints_px
                .iter()
                .map(|b| format!("{}px", b))
                .collect();
            doc.push_str(&format!("- **Breakpoints:** {}\n", bp_str.join(", ")));
        }
        doc.push('\n');
    }

    // 6. Motion & Animations
    if let Some(anims) = animations {
        if !anims.is_empty() {
            doc.push_str("## Motion & Animations\n\n");
            doc.push_str("| Name | Kind | Driver | Duration | Easing | Trigger |\n");
            doc.push_str("|---|---|---|---|---|---|\n");
            for a in anims {
                let trigger_str = match &a.trigger {
                    Some(t) => format!("`{}`", t.r#type),
                    None => "-".to_string(),
                };
                doc.push_str(&format!(
                    "| `{}` | `{}` | `{}` | {}ms | `{}` | {} |\n",
                    a.name, a.kind, a.driver, a.timing.duration_ms, a.timing.easing, trigger_str
                ));
            }
            doc.push('\n');
        }
    }

    // 7. Components
    if let Some(comps) = components {
        if !comps.is_empty() {
            doc.push_str("## Components\n\n");
            doc.push_str("| Name | Role | Variant | Selector | States |\n");
            doc.push_str("|---|---|---|---|---|\n");
            for c in comps {
                let states_str = c.states.keys().cloned().collect::<Vec<_>>().join(", ");
                let variant_str = c.variant.as_deref().unwrap_or("-");
                doc.push_str(&format!(
                    "| **{}** | `{}` | `{}` | `{}` | {} |\n",
                    c.name, c.role, variant_str, c.selector, states_str
                ));
            }
            doc.push('\n');
        }
    }

    let file_path = dest_dir.join("DESIGN.md");
    fs::write(&file_path, doc)?;
    Ok(file_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use specimen_core::ColorUsage;

    #[test]
    fn test_generate_design_md() {
        let temp = tempfile::tempdir().unwrap();
        let meta = DesignDocMeta {
            source: "example.com",
            captured_at: "2026-11-26T07:45:00Z",
            url: "https://example.com/",
            schemes: &["dark".to_string()],
        };

        let pal = PaletteData {
            scheme: "dark".to_string(),
            colors: vec![specimen_core::PaletteColor {
                role: "background".to_string(),
                name: None,
                hex: "#0b0d12".to_string(),
                oklch: [0.17, 0.012, 265.0],
                alpha: 1.0,
                usage: ColorUsage {
                    area_pct: 45.0,
                    elements: 100,
                },
                css_variables: vec!["--bg".to_string()],
            }],
            pairs: Vec::new(),
        };

        let path =
            generate_design_md(temp.path(), &meta, Some(&pal), None, None, None, None).unwrap();
        assert!(path.exists());
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("specimen: 1"));
        assert!(content.contains("#0b0d12"));
        assert!(content.contains("`background`"));
    }
}
