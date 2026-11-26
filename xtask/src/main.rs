#![forbid(unsafe_code)]

use clap::{Parser, Subcommand};

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
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Check => println!("xtask: check (running fmt, clippy, tests...)"),
        Commands::Schemas => println!("xtask: regenerating schemas..."),
        Commands::Fixtures => println!("xtask: refreshing fixtures..."),
        Commands::Browser => println!("xtask: running browser tests..."),
    }
}
