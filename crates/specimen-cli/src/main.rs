#![forbid(unsafe_code)]

use clap::{Parser, Subcommand};
use specimen_agent::detect_agents;
use specimen_capture::find_browser;
use specimen_library::init_library;
use std::path::PathBuf;

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
    },
    /// Search the design library
    Search {
        /// Search query
        query: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

            // 1. Host Info
            println!("Host System:");
            println!(
                "  OS: {} ({})",
                std::env::consts::OS,
                std::env::consts::ARCH
            );

            // 2. Browser Discovery
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

            // 3. Agent CLIs
            println!("\nAI Agent CLI Adapters:");
            for agent in detect_agents() {
                let status = if agent.installed { "✅" } else { "⚪" };
                let version_info = agent.version.unwrap_or_else(|| "not installed".to_string());
                println!("  {:2} {:<24} [{}]", status, agent.name, version_info);
            }
        }
        Some(Commands::Capture { url }) => {
            println!("specimen capture {} (capture driver in progress)", url);
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
