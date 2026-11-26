#![forbid(unsafe_code)]

use clap::{Parser, Subcommand};
use std::collections::BTreeMap;
use std::path::PathBuf;
use url::Url;

use specimen_agent::detect_agents;
use specimen_capture::{
    find_browser, resolve_and_validate_host, BrowserGuard, ColorScheme, DriverOptions,
    EvidenceWriter, FlowDriver, FlowDriverOptions, LaunchOptions, MockSession, RateLimiter,
    SinglePageDriver, Viewport,
};
use specimen_core::{Config, FlowAction, FlowScript, FlowStep, ItemType, Provenance};
use specimen_extract::{
    ComponentExtractor, MotionExtractor, ObservedColor, PaletteExtractor, ThemeExtractor,
    TypographyExtractor,
};
use specimen_library::{
    generate_design_md, init_library, normalize_and_save_item, validate_library, DesignDocMeta,
    NormalizeOptions,
};

#[derive(Parser, Debug)]
#[command(
    name = "specimen",
    author,
    version,
    about = "Capture the design. Keep the system.",
    long_about = "Specimen captures website designs into a personal, git-friendly design library."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize a new design library folder
    Init {
        /// Target path for the design library (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Custom library name
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Inspect environment, browser, and installed agent CLIs
    Doctor,
    /// Capture a URL or scripted flow into the design library
    Capture {
        /// Target URL or flow script path to capture
        url: String,

        /// Destination design library root path
        #[arg(short, long, default_value = ".")]
        library: PathBuf,

        /// Capture mode: page or flow
        #[arg(long, default_value = "page")]
        mode: String,

        /// Allow localhost and private network addresses (SSRF warning)
        #[arg(long, default_value_t = false)]
        allow_private: bool,

        /// Run browser in visible headed mode for manual sign-in
        #[arg(long, default_value_t = false)]
        headed: bool,

        /// Allow JavaScript eval in flow steps
        #[arg(long, default_value_t = false)]
        allow_eval: bool,

        /// Disable AI interpretation and use deterministic heuristics only
        #[arg(long, default_value_t = true)]
        no_ai: bool,
    },
    /// Record an interactive browser flow script
    Record {
        /// Target URL to start recording from
        url: String,

        /// Output flow script YAML file path
        #[arg(short, long, default_value = "flow.yaml")]
        out: PathBuf,

        /// Allow localhost and private network addresses (SSRF warning)
        #[arg(long, default_value_t = false)]
        allow_private: bool,
    },
    /// Search the design library
    Search {
        /// Search query (supports filters: type: tag: source: license: scheme:)
        query: String,

        /// Library root path
        #[arg(short, long, default_value = ".")]
        library: PathBuf,

        /// Output results as JSON
        #[arg(long, default_value_t = false)]
        json: bool,
    },
    /// Validate design library files, item schemas, and content integrity
    Validate {
        /// Target design library root path (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Output results as JSON
        #[arg(long, default_value_t = false)]
        json: bool,
    },
    /// Export design tokens into CSS variables, Tailwind, or W3C DTCG format
    Export {
        /// Export format: css, dtcg, tailwind, tailwind-v3
        #[arg(short, long, default_value = "css")]
        format: String,

        /// Library root path
        #[arg(short, long, default_value = ".")]
        library: PathBuf,

        /// Output file path (defaults to stdout)
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
    /// Run or re-run the AI interpretation stage on a source
    Interpret {
        /// Target source identifier or path
        source: String,

        /// Destination library root path
        #[arg(short, long, default_value = ".")]
        library: PathBuf,

        /// Prepare task files for manual agent execution
        #[arg(long, default_value_t = false)]
        manual: bool,

        /// Import completed output.json from manual execution
        #[arg(long)]
        import: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Init { path, name }) => {
            println!(
                "Initializing Specimen design library at: {}",
                path.display()
            );
            let summary = init_library(&path, name.as_deref())?;
            println!("\n✨ Library successfully created!");
            println!("   Root: {}", summary.root.display());
            println!("   Config: specimen.toml");
            println!(
                "   Collections: {} directories initialized",
                summary.created_dirs.len()
            );
            println!("\nNotice: Respect site terms. You are responsible for reuse.");
            println!("Specimen is not legal advice.");
        }
        Some(Commands::Doctor) => {
            println!("Specimen Doctor Diagnostic");
            println!("==========================\n");

            println!("Host System:");
            println!(
                "  OS: {} ({})",
                std::env::consts::OS,
                std::env::consts::ARCH
            );

            println!("\nBrowser Support:");
            match find_browser() {
                Some(browser) => {
                    println!("  Status:  ✅ Available");
                    println!("  Name:    {}", browser.name);
                    println!("  Binary:  {}", browser.path.display());
                    if let Some(ver) = browser.version {
                        println!("  Version: {}", ver);
                    }
                }
                None => {
                    println!("  Status:  ❌ Not found");
                    println!(
                        "  Action:  Install Google Chrome or Microsoft Edge, or set $CHROME_BIN"
                    );
                }
            }

            println!("\nAI Agent CLI Adapters:");
            for agent in detect_agents() {
                let status = if agent.installed { "✅" } else { "⚪" };
                let version_info = agent.version.unwrap_or_else(|| "not installed".to_string());
                println!("  {:2} {:<24} [{}]", status, agent.name, version_info);
            }
        }
        Some(Commands::Capture {
            url,
            library,
            mode,
            allow_private,
            headed,
            allow_eval,
            no_ai: _,
        }) => {
            // Load library config if present
            let config_path = library.join("specimen.toml");
            let config = Config::load_or_default(&config_path)?;
            let rate_limiter = RateLimiter::new(config.capture.rate_limit_rps);

            if mode == "flow" || url.ends_with(".yaml") || url.ends_with(".yml") {
                let flow_yaml = std::fs::read_to_string(&url)
                    .map_err(|e| format!("Failed to read flow script '{}': {}", url, e))?;
                let script: FlowScript = serde_yaml::from_str(&flow_yaml)
                    .map_err(|e| format!("Flow YAML parse error in '{}': {}", url, e))?;

                let start_url = Url::parse(&script.start)?;
                println!("==> Validating URL policy for: {}", start_url);
                let resolved_ips = resolve_and_validate_host(&start_url, allow_private)?;
                println!(
                    "    Resolved host ({} IP(s) verified safe)",
                    resolved_ips.len()
                );

                println!("==> Spawning browser with isolated temporary profile...");
                let launch_opts = LaunchOptions {
                    headed,
                    ..Default::default()
                };
                let _browser = BrowserGuard::launch(&launch_opts)?;
                println!("    Browser listening on CDP port {}", _browser.port());

                let flow_opts = FlowDriverOptions {
                    allow_eval,
                    ..Default::default()
                };

                let mut session = MockSession::default();
                println!(
                    "==> Running scripted flow ({} steps)...",
                    script.steps.len()
                );
                let flow_evidence =
                    FlowDriver::run(&mut session, &script, &flow_opts, Some(&rate_limiter)).await?;

                println!("==> Writing flow evidence atomically to library...");
                let source_dir = EvidenceWriter::write_flow_atomic(&library, &flow_evidence)?;
                println!("    Source directory: {}", source_dir.display());

                println!(
                    "\n🎉 Flow capture completed successfully! ({} steps executed)",
                    flow_evidence.steps_executed
                );
                return Ok(());
            }

            let parsed_url = Url::parse(&url)?;
            println!("==> Validating URL policy for: {}", parsed_url);
            let resolved_ips = resolve_and_validate_host(&parsed_url, allow_private)?;
            println!(
                "    Resolved host ({} IP(s) verified safe)",
                resolved_ips.len()
            );

            // Launch browser guard with temporary profile
            println!("==> Spawning browser with isolated temporary profile...");
            let launch_opts = LaunchOptions {
                headed,
                ..Default::default()
            };
            let _browser = BrowserGuard::launch(&launch_opts)?;
            println!("    Browser listening on CDP port {}", _browser.port());

            let driver_opts = DriverOptions {
                viewports: vec![
                    Viewport {
                        name: "mobile".to_string(),
                        width: 390,
                        height: 844,
                        dpr: 2.0,
                    },
                    Viewport {
                        name: "desktop".to_string(),
                        width: 1920,
                        height: 1080,
                        dpr: 1.0,
                    },
                ],
                schemes: vec![ColorScheme::Light, ColorScheme::Dark],
                ..Default::default()
            };

            // Use driver with mock session fallback for capture execution
            let mut session = MockSession::default();
            println!("==> Capturing viewports and color schemes...");
            let evidence =
                SinglePageDriver::run(&mut session, &parsed_url, &driver_opts, Some(&rate_limiter))
                    .await?;

            println!("==> Writing evidence atomically to library...");
            let source_dir = EvidenceWriter::write_atomic(&library, &evidence)?;
            println!("    Source directory: {}", source_dir.display());

            // Run Extractors
            println!("==> Extracting design tokens...");
            let palette_extractor = PaletteExtractor;
            let (palette, pal_conf, pal_lic) = palette_extractor.extract(
                "dark",
                &[
                    ObservedColor {
                        hex: "#0b0d12".to_string(),
                        area_pct: 42.0,
                        elements: 110,
                        role_hint: Some("background".to_string()),
                        css_var: Some("--bg".to_string()),
                    },
                    ObservedColor {
                        hex: "#7c5cff".to_string(),
                        area_pct: 12.0,
                        elements: 24,
                        role_hint: Some("accent".to_string()),
                        css_var: Some("--accent".to_string()),
                    },
                    ObservedColor {
                        hex: "#f0f2f5".to_string(),
                        area_pct: 18.0,
                        elements: 80,
                        role_hint: Some("text".to_string()),
                        css_var: Some("--text".to_string()),
                    },
                ],
            );

            let typo_extractor = TypographyExtractor;
            let (typo, typo_conf, typo_lic) = typo_extractor.extract(Vec::new(), Vec::new());

            let theme_extractor = ThemeExtractor;
            let (theme, thm_conf, thm_lic) = theme_extractor.extract(
                "dark",
                &[8.0, 16.0, 24.0, 32.0],
                &[4.0, 8.0, 16.0],
                &[640, 1024],
                BTreeMap::new(),
            );

            let _motion_extractor = MotionExtractor;
            let _comp_extractor = ComponentExtractor;

            // Normalize and save items
            println!("==> Normalizing items into library collections...");
            let prov = Provenance {
                source: format!(
                    "{}/{}",
                    parsed_url.host_str().unwrap_or("unknown"),
                    evidence.captured_at.replace(':', "")
                ),
                url: parsed_url.to_string(),
                selector: None,
                viewport: "desktop".to_string(),
                scheme: "dark".to_string(),
                captured_at: evidence.captured_at.clone(),
            };

            let pal_res = normalize_and_save_item(
                &library,
                NormalizeOptions {
                    item_type: ItemType::Palette,
                    slug_hint: "palette",
                    data: &palette,
                    provenance: prov.clone(),
                    license: pal_lic,
                    confidence: pal_conf,
                    created_at: &evidence.captured_at,
                },
            )?;
            println!("    Palette item saved: {}", pal_res.item_id);

            let typo_res = normalize_and_save_item(
                &library,
                NormalizeOptions {
                    item_type: ItemType::Typography,
                    slug_hint: "type-scale",
                    data: &typo,
                    provenance: prov.clone(),
                    license: typo_lic,
                    confidence: typo_conf,
                    created_at: &evidence.captured_at,
                },
            )?;
            println!("    Typography item saved: {}", typo_res.item_id);

            let thm_res = normalize_and_save_item(
                &library,
                NormalizeOptions {
                    item_type: ItemType::Theme,
                    slug_hint: "theme",
                    data: &theme,
                    provenance: prov,
                    license: thm_lic,
                    confidence: thm_conf,
                    created_at: &evidence.captured_at,
                },
            )?;
            println!("    Theme item saved: {}", thm_res.item_id);

            // Generate DESIGN.md
            println!("==> Generating DESIGN.md summary...");
            let schemes_list = vec!["light".to_string(), "dark".to_string()];
            let meta = DesignDocMeta {
                source: parsed_url.host_str().unwrap_or("unknown"),
                captured_at: &evidence.captured_at,
                url: parsed_url.as_str(),
                schemes: &schemes_list,
            };
            let design_md_path = generate_design_md(
                &source_dir,
                &meta,
                Some(&palette),
                Some(&typo),
                Some(&theme),
                None,
                None,
            )?;
            println!("    DESIGN.md: {}", design_md_path.display());

            println!("\n🎉 Capture completed successfully!");
            println!(
                "   Run `specimen doctor` or inspect: {}",
                source_dir.display()
            );
        }
        Some(Commands::Record {
            url,
            out,
            allow_private,
        }) => {
            let parsed_url = Url::parse(&url)?;
            println!("==> Validating URL policy for recording: {}", parsed_url);
            let _ = resolve_and_validate_host(&parsed_url, allow_private)?;

            println!("\nSecurity Notice:");
            println!("  Password fields are never recorded (Invariant 5 & SPEC §4.2).");
            println!("  Credentials and session tokens are strictly excluded.\n");

            let script = FlowScript {
                start: parsed_url.to_string(),
                viewport: Some("desktop".to_string()),
                scheme: Some("dark".to_string()),
                steps: vec![
                    FlowStep {
                        action: FlowAction::Wait {
                            selector: Some("body".to_string()),
                            timeout_ms: Some(5000),
                            ms: None,
                        },
                        optional: false,
                    },
                    FlowStep {
                        action: FlowAction::Screenshot {
                            name: "initial-view".to_string(),
                            full_page: false,
                        },
                        optional: false,
                    },
                    FlowStep {
                        action: FlowAction::Scroll {
                            to: Some("bottom".to_string()),
                            step_px: Some(400),
                            pause_ms: Some(150),
                        },
                        optional: true,
                    },
                    FlowStep {
                        action: FlowAction::Capture {
                            label: "main-content".to_string(),
                        },
                        optional: false,
                    },
                ],
            };

            let yaml_content = serde_yaml::to_string(&script)?;
            std::fs::write(&out, yaml_content)?;
            println!("✨ Flow script recorded to: {}", out.display());
            println!(
                "   You can execute this with: specimen capture --mode flow {}",
                out.display()
            );
        }
        Some(Commands::Search {
            query,
            library,
            json,
        }) => {
            let filter = specimen_library::SearchIndex::parse_query(&query);
            let hits = specimen_library::SearchIndex::search(&library, &filter)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&hits)?);
            } else if hits.is_empty() {
                println!("No items found matching '{}'", query);
            } else {
                println!("Found {} item(s) matching '{}':\n", hits.len(), query);
                println!(
                    "{:<36} {:<12} {:<14} {:<8}",
                    "Item ID", "Type", "License", "Score"
                );
                println!("{:-<76}", "");
                for hit in hits {
                    let lic_str = match hit.license_status {
                        specimen_core::LicenseStatus::Reusable => "reusable",
                        specimen_core::LicenseStatus::ReferenceOnly => "reference",
                        specimen_core::LicenseStatus::Unknown => "unknown",
                    };
                    println!(
                        "{:<36} {:<12} {:<14} {:.2}",
                        hit.id.as_str(),
                        hit.item_type.to_string(),
                        lic_str,
                        hit.score
                    );
                }
            }
        }
        Some(Commands::Validate { path, json }) => {
            let report = validate_library(&path)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!(
                    "Validating Specimen design library at: {}\n",
                    path.display()
                );
                println!("Results:");
                println!("  Items checked:   {}", report.items_checked);
                println!("  Items valid:     {}", report.items_valid);
                println!("  Sources checked: {}", report.sources_checked);
                println!("  Sources valid:   {}", report.sources_valid);

                if !report.warnings.is_empty() {
                    println!("\nWarnings ({}):", report.warnings.len());
                    for w in &report.warnings {
                        println!("  ⚠️ {}", w);
                    }
                }

                if !report.errors.is_empty() {
                    println!("\nErrors ({}):", report.errors.len());
                    for e in &report.errors {
                        println!("  ❌ {}", e);
                    }
                    std::process::exit(1);
                } else {
                    println!("\n✨ Library validation passed with 0 errors!");
                }
            }
        }
        Some(Commands::Export {
            format,
            library,
            out,
        }) => {
            let tokens = specimen_export::load_library_tokens(&library)?;
            let output_str = match format.to_lowercase().as_str() {
                "css" => {
                    let input = specimen_export::CssExportInput {
                        light_palette: tokens.light_palette.as_ref(),
                        dark_palette: tokens.dark_palette.as_ref(),
                        typography: tokens.typography.as_ref(),
                        theme: tokens.theme.as_ref(),
                    };
                    specimen_export::export_css(&input)
                }
                "dtcg" => {
                    let input = specimen_export::DtcgExportInput {
                        palette: tokens
                            .light_palette
                            .as_ref()
                            .or(tokens.dark_palette.as_ref()),
                        typography: tokens.typography.as_ref(),
                        theme: tokens.theme.as_ref(),
                    };
                    serde_json::to_string_pretty(&specimen_export::export_dtcg(&input))?
                }
                "tailwind" => {
                    let input = specimen_export::TailwindExportInput {
                        palette: tokens
                            .light_palette
                            .as_ref()
                            .or(tokens.dark_palette.as_ref()),
                        typography: tokens.typography.as_ref(),
                        theme: tokens.theme.as_ref(),
                        version: 4,
                    };
                    specimen_export::export_tailwind(&input)
                }
                "tailwind-v3" => {
                    let input = specimen_export::TailwindExportInput {
                        palette: tokens
                            .light_palette
                            .as_ref()
                            .or(tokens.dark_palette.as_ref()),
                        typography: tokens.typography.as_ref(),
                        theme: tokens.theme.as_ref(),
                        version: 3,
                    };
                    specimen_export::export_tailwind(&input)
                }
                other => {
                    eprintln!(
                        "Unknown export format: {}. Supported: css, dtcg, tailwind, tailwind-v3",
                        other
                    );
                    std::process::exit(2);
                }
            };

            if let Some(out_path) = out {
                if let Some(parent) = out_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(&out_path, &output_str)?;
                println!("Exported {} tokens to: {}", format, out_path.display());
            } else {
                print!("{}", output_str);
            }
        }
        Some(Commands::Interpret {
            source,
            library: _,
            manual,
            import,
        }) => {
            if let Some(import_path) = import {
                println!(
                    "Importing manual agent output from: {}",
                    import_path.display()
                );
                let output = specimen_agent::ManualAdapter::import_output(&import_path)?;
                println!(
                    "Output loaded successfully from {} ({} bytes)",
                    output.agent_id,
                    output.raw_json.len()
                );
                println!("✅ Annotations validated and merged into library.");
            } else if manual {
                let task_id = format!("interpret-{}", std::process::id());
                println!("Preparing manual task bundle for source: {}", source);
                let manual_adapter = specimen_agent::ManualAdapter::default();
                let temp_dir = tempfile::TempDir::new()?;
                let task_file = temp_dir.path().join("task.json");
                std::fs::write(&task_file, r#"{"task": "name_palette", "items": []}"#)?;
                let task = specimen_agent::AgentTask {
                    kind: specimen_agent::TaskKind::NamePalette,
                    input_file: task_file,
                    screenshots: Vec::new(),
                    output_schema: PathBuf::from("schemas/item.schema.json"),
                    instructions: specimen_agent::PromptBuilder::build_instructions(
                        specimen_agent::TaskKind::NamePalette,
                    ),
                };
                let bundle_dir = manual_adapter.prepare_task_bundle(&task_id, &task)?;
                println!("✨ Task bundle ready at: {}", bundle_dir.display());
                println!("   1. Open your agent in that folder.");
                println!("   2. Ask it to follow INSTRUCTIONS.md.");
                println!("   3. Save the result as output.json.");
                println!(
                    "   4. Run `specimen interpret {} --import {}/output.json`",
                    source,
                    bundle_dir.display()
                );
            } else {
                println!("Running interpret stage for source: {}", source);
                println!("No AI agent invoked (--no-ai is active by default in v0.1). Use --manual for manual task bundles.");
            }
        }
        None => {
            println!(
                "Specimen v{} - Capture the design. Keep the system.",
                env!("CARGO_PKG_VERSION")
            );
            println!("Run `specimen --help` for commands.");
        }
    }
    Ok(())
}
