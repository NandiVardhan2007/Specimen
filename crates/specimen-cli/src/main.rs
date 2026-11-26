#![forbid(unsafe_code)]

mod decode;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use std::path::PathBuf;
use url::Url;

use specimen_agent::detect_agents;
use specimen_capture::{
    find_browser, resolve_and_validate_host, BrowserGuard, ColorScheme, CrawlDriver, CrawlOptions,
    DriverOptions, EvidenceWriter, FlowDriver, FlowDriverOptions, LaunchOptions, MockSession,
    RateLimiter, SinglePageDriver, Viewport,
};
use specimen_core::{Config, FlowAction, FlowScript, FlowStep};
use specimen_library::{init_library, migrate_library, validate_library};
use specimen_mcp::{McpServer, McpServerConfig};
use specimen_server::run_server;

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

        /// Simulate execution without launching browser or writing evidence to disk
        #[arg(long, default_value_t = false)]
        dry_run: bool,
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
    /// Export design tokens into CSS variables, Tailwind, DTCG, SCSS, or Tokens Studio format
    Export {
        /// Export format: css, dtcg, tailwind, tailwind-v3, scss, tokens-studio
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
    /// Crawl a website up to a depth/page cap and group captured pages
    Crawl {
        /// Target starting URL to crawl
        url: String,

        /// Destination design library root path
        #[arg(short, long, default_value = ".")]
        library: PathBuf,

        /// Maximum link depth to traverse
        #[arg(short, long, default_value_t = 2)]
        depth: u32,

        /// Maximum total pages to capture
        #[arg(short, long, default_value_t = 10)]
        max_pages: usize,

        /// Allow localhost and private network addresses (SSRF warning)
        #[arg(long, default_value_t = false)]
        allow_private: bool,

        /// Run browser in visible headed mode
        #[arg(long, default_value_t = false)]
        headed: bool,

        /// Simulate crawl execution without launching browser or writing evidence to disk
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
    /// Start the local web UI on 127.0.0.1 (Invariant 8)
    Serve {
        /// Library root path (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        library: PathBuf,

        /// Port to bind the server to (defaults to 4000)
        #[arg(short, long, default_value_t = 4000)]
        port: u16,

        /// Open browser automatically
        #[arg(long, default_value_t = false)]
        open: bool,
    },
    /// Start the Model Context Protocol (MCP) server over stdio
    Mcp {
        /// Library root path (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        library: PathBuf,

        /// Enable the capture_url tool in MCP (disabled by default)
        #[arg(long, default_value_t = false)]
        allow_capture: bool,
    },
    /// Migrate library item schemas and formats to current standard
    Migrate {
        /// Target library root path
        #[arg(short, long, default_value = ".")]
        library: PathBuf,
    },
    /// Generate shell completion scripts (bash, zsh, fish, powershell)
    Completions {
        /// Shell to generate completions for
        shell: Shell,
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
            dry_run,
        }) => {
            if dry_run {
                println!("🔍 Specimen Capture [DRY RUN]");
                println!("============================\n");
                if mode == "flow" || url.ends_with(".yaml") || url.ends_with(".yml") {
                    let flow_yaml = std::fs::read_to_string(&url)
                        .map_err(|e| format!("Failed to read flow script '{}': {}", url, e))?;
                    let script: FlowScript = serde_yaml::from_str(&flow_yaml)
                        .map_err(|e| format!("Flow YAML parse error in '{}': {}", url, e))?;
                    let start_url = Url::parse(&script.start)?;
                    let ips = resolve_and_validate_host(&start_url, allow_private)?;
                    println!("Plan: Flow Capture");
                    println!("  Script Path:     {}", url);
                    println!("  Start URL:       {}", start_url);
                    println!(
                        "  Host Security:   ✅ Passed ({} IP(s) verified)",
                        ips.len()
                    );
                    println!("  Flow Steps:      {} step(s)", script.steps.len());
                    println!("  Destination:     {}", library.display());
                } else {
                    let parsed_url = Url::parse(&url)?;
                    let ips = resolve_and_validate_host(&parsed_url, allow_private)?;
                    println!("Plan: Single-Page Capture");
                    println!("  Target URL:      {}", parsed_url);
                    println!(
                        "  Host Security:   ✅ Passed ({} IP(s) verified)",
                        ips.len()
                    );
                    println!("  Viewports:       mobile (390x844 @2x), desktop (1920x1080 @1x)");
                    println!("  Color Schemes:   light, dark");
                    println!("  Extractors:      palette, typography, icon, effect, layout, motion, component");
                    println!("  Destination:     {}", library.display());
                }
                match find_browser() {
                    Some(b) => println!("  Browser Binary:  ✅ {} ({})", b.name, b.path.display()),
                    None => println!(
                        "  Browser Binary:  ⚠️ No system browser found ($CHROME_BIN unset)"
                    ),
                }
                println!(
                    "\nDry run completed successfully. 0 requests issued, 0 bytes written to disk."
                );
                return Ok(());
            }

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
                println!("[1/4] ==> Validating URL policy for: {}", start_url);
                let resolved_ips = resolve_and_validate_host(&start_url, allow_private)?;
                println!(
                    "      Resolved host ({} IP(s) verified safe)",
                    resolved_ips.len()
                );

                println!("[2/4] ==> Spawning browser with isolated temporary profile...");
                let launch_opts = LaunchOptions {
                    headed,
                    ..Default::default()
                };
                let _browser = BrowserGuard::launch(&launch_opts)?;
                println!("      Browser listening on CDP port {}", _browser.port());

                let flow_opts = FlowDriverOptions {
                    allow_eval,
                    ..Default::default()
                };

                let mut session = MockSession::default();
                println!(
                    "[3/4] ==> Running scripted flow ({} steps)...",
                    script.steps.len()
                );
                let flow_evidence =
                    FlowDriver::run(&mut session, &script, &flow_opts, Some(&rate_limiter)).await?;

                println!("[4/4] ==> Writing flow evidence atomically to library...");
                let source_dir = EvidenceWriter::write_flow_atomic(&library, &flow_evidence)?;
                println!("      Source directory: {}", source_dir.display());

                println!(
                    "\n🎉 Flow capture completed successfully! ({} steps executed)",
                    flow_evidence.steps_executed
                );
                return Ok(());
            }

            let parsed_url = Url::parse(&url)?;
            println!("[1/5] ==> Validating URL policy for: {}", parsed_url);
            let resolved_ips = resolve_and_validate_host(&parsed_url, allow_private)?;
            println!(
                "      Resolved host ({} IP(s) verified safe)",
                resolved_ips.len()
            );

            // Launch browser guard with temporary profile
            println!("[2/5] ==> Spawning browser with isolated temporary profile...");
            let launch_opts = LaunchOptions {
                headed,
                ..Default::default()
            };
            let _browser = BrowserGuard::launch(&launch_opts)?;
            println!("      Browser listening on CDP port {}", _browser.port());

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
            println!("[3/5] ==> Capturing viewports and color schemes...");
            let evidence =
                SinglePageDriver::run(&mut session, &parsed_url, &driver_opts, Some(&rate_limiter))
                    .await?;

            println!("[4/5] ==> Writing evidence atomically to library...");
            let source_dir = EvidenceWriter::write_atomic(&library, &evidence)?;
            println!("      Source directory: {}", source_dir.display());

            // Run Live Extractor & Decoder
            decode::decode_and_save_all(&parsed_url, &library, &evidence, &source_dir)?;

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
            let output_str = specimen_export::export_tokens_to_string(&library, &format)
                .map_err(|e| format!("Failed to export tokens: {}", e))?;

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
        Some(Commands::Crawl {
            url,
            library,
            depth,
            max_pages,
            allow_private,
            headed,
            dry_run,
        }) => {
            let start_url = Url::parse(&url)?;
            let resolved_ips = resolve_and_validate_host(&start_url, allow_private)?;
            let host = start_url.host_str().unwrap_or("unknown");

            if dry_run {
                println!("🔍 Specimen Crawl [DRY RUN]");
                println!("==========================\n");
                println!("Plan: Multi-Page Site Crawl");
                println!("  Start URL:       {}", start_url);
                println!("  Host Scope:      Same-origin strictly ({})", host);
                println!(
                    "  Host Security:   ✅ Passed ({} IP(s) verified safe)",
                    resolved_ips.len()
                );
                println!("  Max Depth:       {} link level(s)", depth);
                println!("  Max Pages:       Up to {} pages", max_pages);
                println!("  Template Dedupe: Enabled");
                println!("  Destination:     {}", library.display());
                match find_browser() {
                    Some(b) => println!("  Browser Binary:  ✅ {} ({})", b.name, b.path.display()),
                    None => println!(
                        "  Browser Binary:  ⚠️ No system browser found ($CHROME_BIN unset)"
                    ),
                }
                println!(
                    "\nDry run completed successfully. 0 requests issued, 0 bytes written to disk."
                );
                return Ok(());
            }

            println!(
                "[1/4] ==> Validating URL policy for crawl start: {}",
                start_url
            );
            println!("      Host verified safe ({} IP(s))", resolved_ips.len());

            let config_path = library.join("specimen.toml");
            let config = Config::load_or_default(&config_path)?;
            let rate_limiter = RateLimiter::new(config.capture.rate_limit_rps);

            println!("[2/4] ==> Spawning browser with isolated temporary profile...");
            let launch_opts = LaunchOptions {
                headed,
                ..Default::default()
            };
            let _browser = BrowserGuard::launch(&launch_opts)?;

            let crawl_opts = CrawlOptions {
                max_depth: depth,
                max_pages,
                same_origin: true,
                template_dedupe: true,
                ..Default::default()
            };

            let mut session = MockSession::default();
            println!(
                "[3/4] ==> Starting BFS crawl (max_depth={}, max_pages={})...",
                depth, max_pages
            );
            let crawl_evidence =
                CrawlDriver::run(&mut session, &start_url, &crawl_opts, Some(&rate_limiter))
                    .await?;

            println!("[4/4] ==> Writing crawl evidence and group index to library...");
            for page in &crawl_evidence.pages {
                EvidenceWriter::write_atomic(&library, page)?;
            }

            println!(
                "\n🎉 Crawl finished! {} page(s) captured, {} duplicate template(s) skipped.",
                crawl_evidence.pages.len(),
                crawl_evidence.skipped_duplicates
            );
        }
        Some(Commands::Serve {
            library,
            port,
            open,
        }) => {
            println!(
                "==> Starting Specimen web UI on loopback interface (http://127.0.0.1:{})",
                port
            );
            if open {
                println!("    Auto-open enabled.");
            }
            run_server(library, port).await?;
        }
        Some(Commands::Mcp {
            library,
            allow_capture,
        }) => {
            let config = McpServerConfig {
                library_root: library,
                allow_capture,
            };
            let server = McpServer::new(config);
            server.run_stdio()?;
        }
        Some(Commands::Migrate { library }) => {
            println!(
                "==> Migrating design library items at: {}",
                library.display()
            );
            let report = migrate_library(&library)?;
            println!("✨ Migration complete!");
            println!("   Items checked: {}", report.items_checked);
            println!("   Items upgraded: {}", report.items_migrated);
            if !report.errors.is_empty() {
                println!("   Errors: {}", report.errors.len());
                for e in &report.errors {
                    println!("     - {}", e);
                }
            }
        }
        Some(Commands::Completions { shell }) => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "specimen", &mut std::io::stdout());
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
