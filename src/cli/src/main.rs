mod commands;
mod steps;

use clap::{Parser, Subcommand};
use commands::init::run_init;

#[derive(Parser)]
#[command(name = "dtx")]
#[command(about = "CLI for scaffolding detaxine-ui projects")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scaffold a new detaxine-ui + Leptos project
    Init {
        /// Project name / directory
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { name } => {
            if let Err(e) = run_init(&name) {
                eprintln!("\n{} {}", colored::Colorize::red("error:"), e);
                std::process::exit(1);
            }
        }
    }
}
