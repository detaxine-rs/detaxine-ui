use crate::steps::{cargo, css, html, preflight, trunk};
use anyhow::{Result, bail};
use colored::Colorize;
use inquire::Select;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectMode {
    Csr,
    Ssr,
}

impl std::fmt::Display for ProjectMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectMode::Csr => write!(f, "CSR (client-side rendered, Trunk + WASM)"),
            ProjectMode::Ssr => write!(f, "SSR (server-rendered, Axum + cargo-leptos)"),
        }
    }
}

pub fn run_init(name: &str) -> Result<()> {
    if std::path::Path::new(name).exists() {
        bail!("directory '{}' already exists", name);
    }

    let mode = Select::new(
        "Which rendering mode should this project use?",
        vec![ProjectMode::Csr, ProjectMode::Ssr],
    )
    .prompt()?;

    if mode == ProjectMode::Ssr {
        preflight::check_cargo_leptos()?;
    }

    println!("\n{} {}\n", "Scaffolding".green().bold(), name.bold());

    // 1. Create directory structure
    fs::create_dir_all(format!("{}/src", name))?;
    fs::create_dir_all(format!("{}/styles", name))?;
    println!("{} {}/", "✔".green(), name);

    // 2. Cargo.toml + src/main.rs (+ src/lib.rs, src/app.rs for SSR)
    match mode {
        ProjectMode::Csr => {
            cargo::write_manifest(name)?;
            println!("{} Cargo.toml", "✔".green());
            cargo::write_main(name)?;
            println!("{} src/main.rs", "✔".green());
        }
        ProjectMode::Ssr => {
            cargo::write_manifest_ssr(name)?;
            println!("{} Cargo.toml", "✔".green());
            cargo::write_main_ssr(name)?;
            println!("{} src/main.rs", "✔".green());
            cargo::write_lib_ssr(name)?;
            println!("{} src/lib.rs", "✔".green());
            cargo::write_app_ssr(name)?;
            println!("{} src/app.rs", "✔".green());
            cargo::write_cargo_config_ssr(name)?;
            println!("{} .cargo/config.toml", "✔".green());
        }
    }

    // 3. Clone lib and copy its input.css as the project's style base.
    //    Same for both modes.
    css::download_input_css(name)?;
    println!("{} styles/input.css", "✔".green());

    // 4 & 5. Mode-specific scaffolding
    match mode {
        ProjectMode::Csr => {
            html::write(name)?;
            println!("{} index.html", "✔".green());

            trunk::write(name)?;
            println!("{} Trunk.toml", "✔".green());
        }
        ProjectMode::Ssr => {
            // No index.html / Trunk.toml — cargo-leptos owns the HTML shell
            // (via shell() in app.rs) and the build pipeline instead.
        }
    }

    // 6. .gitignore
    write_gitignore(name, mode)?;
    println!("{} .gitignore", "✔".green());

    println!("\n{}", "Done! Next steps:".bold());
    println!("  cd {}", name);
    match mode {
        ProjectMode::Csr => println!("  trunk serve\n"),
        ProjectMode::Ssr => println!("  cargo leptos watch\n"),
    }

    Ok(())
}

fn write_gitignore(project: &str, mode: ProjectMode) -> Result<()> {
    let contents = match mode {
        ProjectMode::Csr => "/target\n/dist\nstyles/output.css\nbin/\n",
        ProjectMode::Ssr => "/target\nstyles/output.css\n",
    };
    std::fs::write(format!("{}/.gitignore", project), contents)?;
    Ok(())
}
