mod commands {
    pub mod init;
    pub mod update;
}
mod steps {
    pub mod cargo;
    pub mod css;
    pub mod html;
    pub mod preflight;
    pub mod trunk;
}

use clap::{Parser, Subcommand};
use commands::{init::run_init, update::run_update};

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
    /// Pull the latest Tailwind safelist from detaxine-ui into this project
    Update {
        /// Project directory (defaults to current directory)
        #[arg(default_value = ".")]
        path: String,
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
        Commands::Update { path } => {
            if let Err(e) = run_update(&path) {
                eprintln!("\n{} {}", colored::Colorize::red("error:"), e);
                std::process::exit(1);
            }
        }
    }
}
