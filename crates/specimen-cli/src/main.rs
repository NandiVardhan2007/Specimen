#![forbid(unsafe_code)]

use clap::Parser;
use specimen_core::Config;

#[derive(Parser, Debug)]
#[command(name = "specimen", author, version, about = "Capture the design. Keep the system.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Initialize a new design library
    Init,
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

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Init) => {
            let _cfg = Config::default();
            println!("specimen init: initialized design library structure");
        }
        Some(Commands::Doctor) => println!("specimen doctor: environment healthy"),
        Some(Commands::Capture { url }) => println!("specimen capture {} (stub)", url),
        Some(Commands::Search { query }) => println!("specimen search {} (stub)", query),
        None => {
            println!(
                "Specimen v{} - Capture the design. Keep the system.",
                env!("CARGO_PKG_VERSION")
            );
            println!("Run `specimen --help` for usage.");
        }
    }
}
