#![forbid(unsafe_code)]

use serde_json::{json, Map, Value};
use specimen_core::{PaletteData, ThemeData, TypographyData};

pub struct DtcgExportInput<'a> {
    pub palette: Option<&'a PaletteData>,
    pub typography: Option<&'a TypographyData>,
    pub theme: Option<&'a ThemeData>,
}

pub fn export_dtcg(input: &DtcgExportInput<'_>) -> Value {
    let mut root = Map::new();

    // 1. Color tokens
    if let Some(pal) = input.palette {
        let mut color_group = Map::new();
        for c in &pal.colors {
            let mut token = Map::new();
            token.insert("$type".to_string(), json!("color"));
            token.insert("$value".to_string(), json!(c.hex));
            token.insert(
                "$description".to_string(),
                json!(format!("Role: {} (scheme: {})", c.role, pal.scheme)),
            );
            color_group.insert(c.role.clone(), Value::Object(token));
        }
        root.insert("color".to_string(), Value::Object(color_group));
    }

    // 2. Typography tokens
    if let Some(typo) = input.typography {
        let mut typo_group = Map::new();

        // Font families
        let mut font_fam_group = Map::new();
        for fam in &typo.families {
            let mut token = Map::new();
            token.insert("$type".to_string(), json!("fontFamily"));
            token.insert(
                "$value".to_string(),
                json!(if fam.stack.is_empty() {
                    vec![fam.name.clone()]
                } else {
                    fam.stack.clone()
                }),
            );
            font_fam_group.insert(
                fam.name.to_lowercase().replace(' ', "-"),
                Value::Object(token),
            );
        }
        if !font_fam_group.is_empty() {
            typo_group.insert("family".to_string(), Value::Object(font_fam_group));
        }

        // Font sizes
        let mut size_group = Map::new();
        for entry in &typo.scale {
            let mut token = Map::new();
            token.insert("$type".to_string(), json!("dimension"));
            token.insert("$value".to_string(), json!(format!("{}px", entry.size_px)));
            size_group.insert(entry.role.clone(), Value::Object(token));
        }
        if !size_group.is_empty() {
            typo_group.insert("size".to_string(), Value::Object(size_group));
        }

        root.insert("typography".to_string(), Value::Object(typo_group));
    }

    // 3. Spacing and Dimension tokens
    if let Some(thm) = input.theme {
        let mut spacing_group = Map::new();

        // Base unit
        let mut base_token = Map::new();
        base_token.insert("$type".to_string(), json!("dimension"));
        base_token.insert(
            "$value".to_string(),
            json!(format!("{}px", thm.spacing_base_unit)),
        );
        spacing_group.insert("base".to_string(), Value::Object(base_token));

        // Scale
        for (i, &val) in thm.spacing_scale.iter().enumerate() {
            let mut token = Map::new();
            token.insert("$type".to_string(), json!("dimension"));
            token.insert("$value".to_string(), json!(format!("{}px", val)));
            spacing_group.insert(format!("{}", i + 1), Value::Object(token));
        }
        root.insert("spacing".to_string(), Value::Object(spacing_group));

        // Border radii
        if !thm.radii_px.is_empty() {
            let mut radii_group = Map::new();
            let labels = ["sm", "md", "lg", "xl", "2xl"];
            for (i, &val) in thm.radii_px.iter().enumerate() {
                let lbl = labels.get(i).copied().unwrap_or("var");
                let mut token = Map::new();
                token.insert("$type".to_string(), json!("dimension"));
                token.insert("$value".to_string(), json!(format!("{}px", val)));
                radii_group.insert(lbl.to_string(), Value::Object(token));
            }
            root.insert("radius".to_string(), Value::Object(radii_group));
        }
    }

    Value::Object(root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use specimen_core::{ColorUsage, PaletteColor, PaletteData};

    #[test]
    fn test_export_dtcg_structure() {
        let palette = PaletteData {
            scheme: "dark".to_string(),
            colors: vec![PaletteColor {
                role: "background".to_string(),
                name: None,
                hex: "#0b0d12".to_string(),
                oklch: [0.1, 0.0, 0.0],
                alpha: 1.0,
                usage: ColorUsage {
                    area_pct: 40.0,
                    elements: 12,
                },
                css_variables: Vec::new(),
            }],
            pairs: Vec::new(),
        };

        let input = DtcgExportInput {
            palette: Some(&palette),
            typography: None,
            theme: None,
        };

        let val = export_dtcg(&input);
        assert_eq!(val["color"]["background"]["$type"], "color");
        assert_eq!(val["color"]["background"]["$value"], "#0b0d12");
    }
}
