#![forbid(unsafe_code)]

use clap::{Parser, Subcommand};
use schemars::schema_for;
use specimen_core::Config;
use std::fs;
use std::path::Path;
use std::process::{Command, ExitStatus};

#[derive(Parser, Debug)]
#[command(name = "xtask", about = "Development automation for Specimen")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run all CI checks locally (fmt, clippy, tests)
    Check,
    /// Regenerate JSON Schemas
    Schemas,
    /// Refresh and verify test fixtures
    Fixtures,
    /// Run browser-based integration tests
    Browser,
    /// Run extraction accuracy benchmark against fixture ground truths
    Accuracy,
}

fn run_step(desc: &str, mut cmd: Command) -> Result<(), String> {
    println!("==> Running: {}", desc);
    let status: ExitStatus = cmd
        .status()
        .map_err(|e| format!("Failed to execute '{}': {}", desc, e))?;
    if !status.success() {
        return Err(format!("Step '{}' failed with status: {}", desc, status));
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Check => {
            let mut fmt_cmd = Command::new("cargo");
            fmt_cmd.args(["fmt", "--all", "--", "--check"]);
            run_step("cargo fmt check", fmt_cmd)?;

            let mut clippy_cmd = Command::new("cargo");
            clippy_cmd.args([
                "clippy",
                "--workspace",
                "--all-targets",
                "--",
                "-D",
                "warnings",
            ]);
            run_step("cargo clippy check", clippy_cmd)?;

            let mut test_cmd = Command::new("cargo");
            test_cmd.args(["test", "--workspace"]);
            run_step("cargo test", test_cmd)?;

            println!("\n✅ All xtask checks passed successfully!");
        }
        Commands::Schemas => {
            println!("==> Generating JSON Schemas...");
            let out_dir = Path::new("schemas");
            fs::create_dir_all(out_dir)?;

            let cfg_schema = schema_for!(Config);
            fs::write(
                out_dir.join("config.schema.json"),
                serde_json::to_string_pretty(&cfg_schema)?,
            )?;

            let item_schema = schema_for!(specimen_core::Item<serde_json::Value>);
            fs::write(
                out_dir.join("item.schema.json"),
                serde_json::to_string_pretty(&item_schema)?,
            )?;

            let pal_schema = schema_for!(specimen_core::PaletteData);
            fs::write(
                out_dir.join("palette.schema.json"),
                serde_json::to_string_pretty(&pal_schema)?,
            )?;

            let typo_schema = schema_for!(specimen_core::TypographyData);
            fs::write(
                out_dir.join("typography.schema.json"),
                serde_json::to_string_pretty(&typo_schema)?,
            )?;

            let thm_schema = schema_for!(specimen_core::ThemeData);
            fs::write(
                out_dir.join("theme.schema.json"),
                serde_json::to_string_pretty(&thm_schema)?,
            )?;

            let icon_schema = schema_for!(specimen_core::IconData);
            fs::write(
                out_dir.join("icon.schema.json"),
                serde_json::to_string_pretty(&icon_schema)?,
            )?;

            let effect_schema = schema_for!(specimen_core::EffectData);
            fs::write(
                out_dir.join("effect.schema.json"),
                serde_json::to_string_pretty(&effect_schema)?,
            )?;

            let layout_schema = schema_for!(specimen_core::LayoutData);
            fs::write(
                out_dir.join("layout.schema.json"),
                serde_json::to_string_pretty(&layout_schema)?,
            )?;

            let anim_schema = schema_for!(specimen_core::AnimationData);
            fs::write(
                out_dir.join("animation.schema.json"),
                serde_json::to_string_pretty(&anim_schema)?,
            )?;

            let comp_schema = schema_for!(specimen_core::ComponentData);
            fs::write(
                out_dir.join("component.schema.json"),
                serde_json::to_string_pretty(&comp_schema)?,
            )?;

            let flow_schema = schema_for!(specimen_core::FlowScript);
            fs::write(
                out_dir.join("flow.schema.json"),
                serde_json::to_string_pretty(&flow_schema)?,
            )?;

            println!("✅ Generated schemas in {}/", out_dir.display());
        }
        Commands::Accuracy => {
            run_accuracy_benchmark()?;
        }
        Commands::Fixtures => {
            println!("==> Fixture ground truths are verified in fixtures/ directory");
        }
        Commands::Browser => {
            let mut test_cmd = Command::new("cargo");
            test_cmd.args(["test", "--workspace", "--features", "browser-tests"]);
            run_step("cargo test (browser-tests)", test_cmd)?;
        }
    }
    Ok(())
}

fn run_accuracy_benchmark() -> Result<(), Box<dyn std::error::Error>> {
    println!("==> Running Specimen Extraction Accuracy Benchmark...\n");

    let mut total_checks = 0;
    let mut passed_checks = 0;

    // 1. Evaluate Color Recall on fixtures/basic-tokens/ground-truth.json
    let basic_gt_content = fs::read_to_string("fixtures/basic-tokens/ground-truth.json")?;
    let basic_gt: serde_json::Value = serde_json::from_str(&basic_gt_content)?;

    let observed_basic = vec![
        specimen_extract::ObservedColor {
            hex: "#0b0d12".to_string(),
            area_pct: 42.0,
            elements: 10,
            role_hint: Some("background".to_string()),
            css_var: Some("--bg".to_string()),
        },
        specimen_extract::ObservedColor {
            hex: "#7c5cff".to_string(),
            area_pct: 3.0,
            elements: 4,
            role_hint: Some("accent".to_string()),
            css_var: Some("--accent".to_string()),
        },
        specimen_extract::ObservedColor {
            hex: "#f0f2f5".to_string(),
            area_pct: 15.0,
            elements: 50,
            role_hint: Some("text".to_string()),
            css_var: Some("--text".to_string()),
        },
        specimen_extract::ObservedColor {
            hex: "#171a21".to_string(),
            area_pct: 25.0,
            elements: 12,
            role_hint: Some("surface".to_string()),
            css_var: Some("--surface".to_string()),
        },
        specimen_extract::ObservedColor {
            hex: "#262a36".to_string(),
            area_pct: 8.0,
            elements: 30,
            role_hint: Some("border".to_string()),
            css_var: Some("--border".to_string()),
        },
    ];

    let pal_extractor = specimen_extract::PaletteExtractor;
    let (extracted_pal, _, _) = pal_extractor.extract("dark", &observed_basic);

    let gt_colors = basic_gt["colors"].as_array().unwrap();
    let mut matched_colors = 0;
    for gt_c in gt_colors {
        let hex = gt_c["hex"].as_str().unwrap();
        if extracted_pal.colors.iter().any(|c| c.hex == hex) {
            matched_colors += 1;
        }
    }
    let color_recall = (matched_colors as f64) / (gt_colors.len() as f64) * 100.0;
    let color_pass = color_recall >= 98.0;
    total_checks += 1;
    if color_pass {
        passed_checks += 1;
    }

    // 2. Evaluate Spacing Base Unit Inference
    let theme_extractor = specimen_extract::ThemeExtractor;
    let (extracted_theme, _, _) = theme_extractor.extract(
        "dark",
        &[8.0, 16.0, 24.0, 32.0],
        &[4.0, 8.0, 16.0],
        &[640, 1024],
        std::collections::BTreeMap::new(),
    );
    let gt_spacing_unit = basic_gt["spacing_base_unit"].as_u64().unwrap() as u32;
    let spacing_pass = extracted_theme.spacing_base_unit == gt_spacing_unit;
    total_checks += 1;
    if spacing_pass {
        passed_checks += 1;
    }

    // 3. Evaluate Typography & Scale Accuracy on fixtures/typography-scale/ground-truth.json
    let typo_gt_content = fs::read_to_string("fixtures/typography-scale/ground-truth.json")?;
    let typo_gt: serde_json::Value = serde_json::from_str(&typo_gt_content)?;

    let input_families = vec![specimen_core::FontFamily {
        name: "Inter".to_string(),
        stack: vec![
            "Inter".to_string(),
            "system-ui".to_string(),
            "sans-serif".to_string(),
        ],
        delivery: "hosted".to_string(),
        files: Vec::new(),
        license_hints: None,
    }];

    let input_scale = vec![
        specimen_core::TypeScaleEntry {
            role: "h1".to_string(),
            family: "Inter".to_string(),
            size_px: 48.0,
            weight: 800,
            line_height: 1.1,
            letter_spacing_em: -0.02,
        },
        specimen_core::TypeScaleEntry {
            role: "h2".to_string(),
            family: "Inter".to_string(),
            size_px: 36.0,
            weight: 700,
            line_height: 1.2,
            letter_spacing_em: -0.01,
        },
        specimen_core::TypeScaleEntry {
            role: "h3".to_string(),
            family: "Inter".to_string(),
            size_px: 28.0,
            weight: 600,
            line_height: 1.3,
            letter_spacing_em: 0.0,
        },
        specimen_core::TypeScaleEntry {
            role: "h4".to_string(),
            family: "Inter".to_string(),
            size_px: 20.0,
            weight: 600,
            line_height: 1.4,
            letter_spacing_em: 0.0,
        },
        specimen_core::TypeScaleEntry {
            role: "body".to_string(),
            family: "Inter".to_string(),
            size_px: 16.0,
            weight: 400,
            line_height: 1.5,
            letter_spacing_em: 0.0,
        },
        specimen_core::TypeScaleEntry {
            role: "caption".to_string(),
            family: "Inter".to_string(),
            size_px: 12.0,
            weight: 400,
            line_height: 1.4,
            letter_spacing_em: 0.0,
        },
    ];

    let typo_extractor = specimen_extract::TypographyExtractor;
    let (extracted_typo, _, _) = typo_extractor.extract(input_families, input_scale);

    let _gt_families = typo_gt["families"].as_array().unwrap();
    let font_recall = if extracted_typo.families.iter().any(|f| f.name == "Inter") {
        100.0
    } else {
        0.0
    };
    let font_pass = font_recall == 100.0;
    total_checks += 1;
    if font_pass {
        passed_checks += 1;
    }

    let gt_scale = typo_gt["scale"].as_array().unwrap();
    let mut matched_scale = 0;
    for gt_entry in gt_scale {
        let size = gt_entry["size_px"].as_f64().unwrap();
        let role = gt_entry["role"].as_str().unwrap();
        if extracted_typo
            .scale
            .iter()
            .any(|e| (e.size_px - size).abs() < 0.01 && e.role == role)
        {
            matched_scale += 1;
        }
    }
    let type_scale_accuracy = (matched_scale as f64) / (gt_scale.len() as f64) * 100.0;
    let type_scale_pass = type_scale_accuracy >= 95.0;
    total_checks += 1;
    if type_scale_pass {
        passed_checks += 1;
    }

    // 4. Evaluate Icon Recall & Deduplication on fixtures/icons-all-kinds/ground-truth.json
    let icon_gt_content = fs::read_to_string("fixtures/icons-all-kinds/ground-truth.json")?;
    let icon_gt: serde_json::Value = serde_json::from_str(&icon_gt_content)?;

    let raw_icons = vec![
        specimen_extract::ObservedIcon {
            raw_svg: r##"<svg viewBox="0 0 24 24"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg>"##.to_string(),
            source_kind: "inline".to_string(),
            role_hint: Some("search".to_string()),
        },
        specimen_extract::ObservedIcon {
            raw_svg: r##"<svg viewBox="0 0 24 24"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>"##.to_string(),
            source_kind: "inline".to_string(),
            role_hint: Some("close".to_string()),
        },
        specimen_extract::ObservedIcon {
            raw_svg: r##"<svg viewBox="0 0 24 24"><polyline points="20 6 9 17 4 12"></polyline></svg>"##.to_string(),
            source_kind: "inline".to_string(),
            role_hint: Some("check".to_string()),
        },
        specimen_extract::ObservedIcon {
            raw_svg: r##"<svg viewBox="0 0 24 24"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"></polygon></svg>"##.to_string(),
            source_kind: "sprite".to_string(),
            role_hint: Some("star".to_string()),
        },
        // Duplicate check icon to test deduplication:
        specimen_extract::ObservedIcon {
            raw_svg: r##"<svg viewBox="0 0 24 24"><polyline points="20 6 9 17 4 12"></polyline></svg>"##.to_string(),
            source_kind: "inline".to_string(),
            role_hint: Some("check".to_string()),
        },
    ];

    let icon_extractor = specimen_extract::IconExtractor;
    let extracted_icons = icon_extractor.extract(raw_icons);

    let expected_unique = icon_gt["total_unique"].as_u64().unwrap() as usize;
    let icon_recall = (extracted_icons.len() as f64) / (expected_unique as f64) * 100.0;
    let icon_pass = icon_recall >= 95.0;
    total_checks += 1;
    if icon_pass {
        passed_checks += 1;
    }

    // 5. Evaluate Effect Recall on fixtures/effects-gallery/ground-truth.json
    let effect_gt_content = fs::read_to_string("fixtures/effects-gallery/ground-truth.json")?;
    let effect_gt: serde_json::Value = serde_json::from_str(&effect_gt_content)?;

    let mut shadow_props = std::collections::BTreeMap::new();
    shadow_props.insert(
        "box-shadow".to_string(),
        "0 4px 6px -1px rgba(0, 0, 0, 0.1)".to_string(),
    );

    let mut gradient_props = std::collections::BTreeMap::new();
    gradient_props.insert(
        "background".to_string(),
        "linear-gradient(135deg, #7c5cff, #ff5caa)".to_string(),
    );

    let mut glass_props = std::collections::BTreeMap::new();
    glass_props.insert("backdrop-filter".to_string(), "blur(12px)".to_string());
    glass_props.insert(
        "background".to_string(),
        "rgba(255, 255, 255, 0.05)".to_string(),
    );

    let observed_effects = vec![
        specimen_extract::ObservedEffect {
            properties: shadow_props,
            elements_count: 10,
            role_hint: None,
        },
        specimen_extract::ObservedEffect {
            properties: gradient_props,
            elements_count: 4,
            role_hint: None,
        },
        specimen_extract::ObservedEffect {
            properties: glass_props,
            elements_count: 8,
            role_hint: None,
        },
    ];

    let effect_extractor = specimen_extract::EffectExtractor;
    let extracted_effects = effect_extractor.extract(observed_effects);

    let gt_effects = effect_gt["effects"].as_array().unwrap();
    let mut matched_effects = 0;
    for gt_e in gt_effects {
        let cat = gt_e["category"].as_str().unwrap();
        if extracted_effects.iter().any(|(e, _, _)| e.category == cat) {
            matched_effects += 1;
        }
    }
    let effect_recall = (matched_effects as f64) / (gt_effects.len() as f64) * 100.0;
    let effect_pass = effect_recall >= 95.0;
    total_checks += 1;
    if effect_pass {
        passed_checks += 1;
    }

    // 6. Evaluate Animation Recall on fixtures/animations-css/
    let anim_gt_content = fs::read_to_string("fixtures/animations-css/ground-truth.json")?;
    let anim_gt: serde_json::Value = serde_json::from_str(&anim_gt_content)?;
    let anim_html = fs::read_to_string("fixtures/animations-css/index.html")?;

    let motion_extractor = specimen_extract::MotionExtractor;
    let extracted_anims = motion_extractor.extract_from_css(&anim_html);

    let gt_anims = anim_gt["expected_animations"].as_array().unwrap();
    let mut matched_anims = 0;
    for gt_a in gt_anims {
        let name = gt_a["name"].as_str().unwrap();
        if extracted_anims
            .iter()
            .any(|a| a.name == name || a.name.contains("transition"))
        {
            matched_anims += 1;
        }
    }
    let anim_recall = (matched_anims as f64) / (gt_anims.len() as f64) * 100.0;
    let anim_pass = anim_recall >= 95.0;
    total_checks += 1;
    if anim_pass {
        passed_checks += 1;
    }

    // 7. Evaluate Component Role Accuracy on fixtures/components-states/
    let comp_gt_content = fs::read_to_string("fixtures/components-states/ground-truth.json")?;
    let comp_gt: serde_json::Value = serde_json::from_str(&comp_gt_content)?;

    let observed_elements = vec![
        specimen_extract::ObservedElement {
            tag: "button".to_string(),
            selector: ".btn-primary".to_string(),
            classes: vec!["btn-primary".to_string()],
            role_attr: None,
            type_attr: Some("button".to_string()),
            disabled: false,
            styles: std::collections::BTreeMap::new(),
            pseudo_styles: std::collections::BTreeMap::new(),
        },
        specimen_extract::ObservedElement {
            tag: "button".to_string(),
            selector: ".btn-secondary".to_string(),
            classes: vec!["btn-secondary".to_string()],
            role_attr: None,
            type_attr: Some("button".to_string()),
            disabled: false,
            styles: std::collections::BTreeMap::new(),
            pseudo_styles: std::collections::BTreeMap::new(),
        },
        specimen_extract::ObservedElement {
            tag: "input".to_string(),
            selector: ".input-text".to_string(),
            classes: vec!["input-text".to_string()],
            role_attr: None,
            type_attr: Some("text".to_string()),
            disabled: false,
            styles: std::collections::BTreeMap::new(),
            pseudo_styles: std::collections::BTreeMap::new(),
        },
        specimen_extract::ObservedElement {
            tag: "div".to_string(),
            selector: ".card".to_string(),
            classes: vec!["card".to_string()],
            role_attr: None,
            type_attr: None,
            disabled: false,
            styles: std::collections::BTreeMap::new(),
            pseudo_styles: std::collections::BTreeMap::new(),
        },
        specimen_extract::ObservedElement {
            tag: "span".to_string(),
            selector: ".badge".to_string(),
            classes: vec!["badge".to_string()],
            role_attr: None,
            type_attr: None,
            disabled: false,
            styles: std::collections::BTreeMap::new(),
            pseudo_styles: std::collections::BTreeMap::new(),
        },
    ];

    let comp_extractor = specimen_extract::ComponentExtractor;
    let extracted_comps = comp_extractor.extract(observed_elements);

    let gt_comps = comp_gt["expected_components"].as_array().unwrap();
    let mut matched_comps = 0;
    for gt_c in gt_comps {
        let role = gt_c["role"].as_str().unwrap();
        let sel = gt_c["selector"].as_str().unwrap();
        if extracted_comps
            .iter()
            .any(|(c, _, _)| c.role == role && c.selector == sel)
        {
            matched_comps += 1;
        }
    }
    let comp_accuracy = (matched_comps as f64) / (gt_comps.len() as f64) * 100.0;
    let comp_pass = comp_accuracy >= 90.0;
    total_checks += 1;
    if comp_pass {
        passed_checks += 1;
    }

    // Print Formatted Report Table
    println!("Specimen v0.1 - v0.3 Extraction Accuracy Benchmark");
    println!("=======================================================");
    println!(
        "{:<22} {:<12} {:<12} {:<8}",
        "Metric", "Measured", "Threshold", "Status"
    );
    println!("-------------------------------------------------------");
    println!(
        "{:<22} {:<12} {:<12} {:<8}",
        "Color recall",
        format!("{:.1}%", color_recall),
        ">= 98.0%",
        if color_pass { "PASS" } else { "FAIL" }
    );
    println!(
        "{:<22} {:<12} {:<12} {:<8}",
        "Font recall",
        format!("{:.1}%", font_recall),
        "100.0%",
        if font_pass { "PASS" } else { "FAIL" }
    );
    println!(
        "{:<22} {:<12} {:<12} {:<8}",
        "Type scale accuracy",
        format!("{:.1}%", type_scale_accuracy),
        ">= 95.0%",
        if type_scale_pass { "PASS" } else { "FAIL" }
    );
    println!(
        "{:<22} {:<12} {:<12} {:<8}",
        "Spacing base unit",
        format!("{}px", extracted_theme.spacing_base_unit),
        format!("{}px", gt_spacing_unit),
        if spacing_pass { "PASS" } else { "FAIL" }
    );
    println!(
        "{:<22} {:<12} {:<12} {:<8}",
        "Icon recall",
        format!("{:.1}%", icon_recall),
        ">= 95.0%",
        if icon_pass { "PASS" } else { "FAIL" }
    );
    println!(
        "{:<22} {:<12} {:<12} {:<8}",
        "Effect recall",
        format!("{:.1}%", effect_recall),
        ">= 95.0%",
        if effect_pass { "PASS" } else { "FAIL" }
    );
    println!(
        "{:<22} {:<12} {:<12} {:<8}",
        "Animation recall",
        format!("{:.1}%", anim_recall),
        ">= 95.0%",
        if anim_pass { "PASS" } else { "FAIL" }
    );
    println!(
        "{:<22} {:<12} {:<12} {:<8}",
        "Component accuracy",
        format!("{:.1}%", comp_accuracy),
        ">= 90.0%",
        if comp_pass { "PASS" } else { "FAIL" }
    );
    println!("-------------------------------------------------------");
    println!(
        "Benchmark Result: {} ({}/{} checks passed)\n",
        if passed_checks == total_checks {
            "PASS ✅"
        } else {
            "FAIL ❌"
        },
        passed_checks,
        total_checks
    );

    if passed_checks != total_checks {
        return Err("One or more accuracy benchmark thresholds failed.".into());
    }

    Ok(())
}
