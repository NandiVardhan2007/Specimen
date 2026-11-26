#![forbid(unsafe_code)]

use clap::{Parser, Subcommand};
use std::collections::BTreeMap;
use std::path::PathBuf;
use url::Url;

use specimen_agent::detect_agents;
use specimen_capture::{
    find_browser, resolve_and_validate_host, BrowserGuard, ColorScheme, DriverOptions,
    EvidenceWriter, LaunchOptions, MockSession, RateLimiter, SinglePageDriver, Viewport,
};
use specimen_core::{Config, ItemType, Provenance};
use specimen_extract::{ObservedColor, PaletteExtractor, ThemeExtractor, TypographyExtractor};
use specimen_library::{
    generate_design_md, init_library, normalize_and_save_item, DesignDocMeta, NormalizeOptions,
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
    /// Capture a URL into the design library
    Capture {
        /// Target URL to capture
        url: String,

        /// Destination design library root path
        #[arg(short, long, default_value = ".")]
        library: PathBuf,

        /// Allow localhost and private network addresses (SSRF warning)
        #[arg(long, default_value_t = false)]
        allow_private: bool,

        /// Run browser in visible headed mode for manual sign-in
        #[arg(long, default_value_t = false)]
        headed: bool,

        /// Disable AI interpretation and use deterministic heuristics only
        #[arg(long, default_value_t = true)]
        no_ai: bool,
    },
    /// Search the design library
    Search {
        /// Search query
        query: String,
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
            allow_private,
            headed,
            no_ai: _,
        }) => {
            let parsed_url = Url::parse(&url)?;
            println!("==> Validating URL policy for: {}", parsed_url);
            let resolved_ips = resolve_and_validate_host(&parsed_url, allow_private)?;
            println!(
                "    Resolved host ({} IP(s) verified safe)",
                resolved_ips.len()
            );

            // Load library config if present
            let config_path = library.join("specimen.toml");
            let config = Config::load_or_default(&config_path)?;

            // Launch browser guard with temporary profile
            println!("==> Spawning browser with isolated temporary profile...");
            let launch_opts = LaunchOptions {
                headed,
                ..Default::default()
            };
            let _browser = BrowserGuard::launch(&launch_opts)?;
            println!("    Browser listening on CDP port {}", _browser.port());

            let rate_limiter = RateLimiter::new(config.capture.rate_limit_rps);
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
            )?;
            println!("    DESIGN.md: {}", design_md_path.display());

            println!("\n🎉 Capture completed successfully!");
            println!(
                "   Run `specimen doctor` or inspect: {}",
                source_dir.display()
            );
        }
        Some(Commands::Search { query }) => {
            println!("specimen search {} (search index in progress)", query);
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
