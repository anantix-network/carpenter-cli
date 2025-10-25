mod commands;
mod config;
mod env;
mod script;
mod utils;

use anyhow::Result;
use clap::Parser;
use config::ConfigLoader;
use log::info;
use std::path::PathBuf;

// ? Carpet CLI - A script runner for your projects
#[derive(Parser, Debug)]
#[command(
    name = "carpet",
    author = "EvarinthoSec <evarin@anantix.network>",
    version,
    about = "Carpet CLI - Your Project's Script Runner",
    long_about = "A powerful and interactive CLI tool for running project scripts"
)]
#[command(subcommand_required = true)]
struct Args {
    /// ! Path to the config file (carpet.toml, carpet.yaml, or carpet.yml)
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Initialize a new carpet config file
    Init {
        /// Force creation even if file exists
        #[arg(short, long)]
        force: bool,
        #[arg(short, long)]
        yaml: bool,
    },
    /// Run a script from config
    Run {
        /// Name of the script to run
        #[arg(required = false)]
        script: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let config_loader = ConfigLoader::new()?;
    utils::setup_logging(config_loader.get_log_path())?;
    info!("Carpet CLI");
    let args = Args::parse();
    match args.command {
        Commands::Init { force, yaml } => { commands::init(force, yaml)?; }
        Commands::Run { script } => { commands::run(script, &config_loader).await?; }
    }
    Ok(())
}
