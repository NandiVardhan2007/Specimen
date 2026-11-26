#![forbid(unsafe_code)]

use serde_json::{json, Value};
use specimen_core::{PaletteData, ThemeData, TypographyData};

#[derive(Debug, Clone)]
pub struct TokensStudioExportInput<'a> {
    pub palette: Option<&'a PaletteData>,
    pub typography: Option<&'a TypographyData>,
    pub theme: Option<&'a ThemeData>,
}

/// Generates Tokens Studio for Figma compliant JSON.
pub fn export_tokens_studio(input: &TokensStudioExportInput<'_>) -> Value {
    let mut colors = serde_json::Map::new();
    if let Some(pal) = input.palette {
        for c in &pal.colors {
            let name = c.role.replace(' ', "-");
            colors.insert(
                name,
                json!({
                    "value": c.hex,
                    "type": "color",
                    "description": format!("Area usage: {:.1}%", c.usage.area_pct)
                }),
            );
        }
    }

    let mut font_sizes = serde_json::Map::new();
    let mut line_heights = serde_json::Map::new();
    if let Some(typo) = input.typography {
        for s in &typo.scale {
            let name = s.role.replace(' ', "-");
            font_sizes.insert(
                name.clone(),
                json!({
                    "value": format!("{}px", s.size_px),
                    "type": "fontSizes"
                }),
            );
            line_heights.insert(
                name,
                json!({
                    "value": s.line_height.to_string(),
                    "type": "lineHeights"
                }),
            );
        }
    }

    let mut spacing = serde_json::Map::new();
    if let Some(thm) = input.theme {
        spacing.insert(
            "base".to_string(),
            json!({
                "value": format!("{}px", thm.spacing_base_unit),
                "type": "spacing"
            }),
        );
        for (idx, &s) in thm.spacing_scale.iter().enumerate() {
            spacing.insert(
                (idx + 1).to_string(),
                json!({
                    "value": format!("{}px", s),
                    "type": "spacing"
                }),
            );
        }
    }

    json!({
        "global": {
            "colors": colors,
            "fontSizes": font_sizes,
            "lineHeights": line_heights,
            "spacing": spacing
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use specimen_core::ColorUsage;

    #[test]
    fn test_export_tokens_studio_structure() {
        let pal = PaletteData {
            scheme: "dark".to_string(),
            colors: vec![specimen_core::PaletteColor {
                role: "accent".to_string(),
                name: None,
                hex: "#7c5cff".to_string(),
                oklch: [0.55, 0.22, 280.0],
                alpha: 1.0,
                usage: ColorUsage {
                    area_pct: 12.0,
                    elements: 8,
                },
                css_variables: vec![],
            }],
            pairs: vec![],
        };

        let input = TokensStudioExportInput {
            palette: Some(&pal),
            typography: None,
            theme: None,
        };

        let val = export_tokens_studio(&input);
        assert_eq!(val["global"]["colors"]["accent"]["value"], "#7c5cff");
        assert_eq!(val["global"]["colors"]["accent"]["type"], "color");
    }
}
