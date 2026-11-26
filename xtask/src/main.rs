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

            println!("✅ Generated schemas in {}/", out_dir.display());
        }
        Commands::Fixtures => {
            println!("==> Refreshing fixtures (stub for Phase 0)...");
        }
        Commands::Browser => {
            println!("==> Running browser tests (stub for Phase 0)...");
        }
    }
    Ok(())
}
