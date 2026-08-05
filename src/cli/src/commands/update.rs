use crate::steps::{cargo, css};
use anyhow::{Result, bail};
use colored::Colorize;

pub fn run_update(path: &str) -> Result<()> {
    let project_dir = std::path::Path::new(path);
    if !project_dir.join("styles/input.css").exists() {
        bail!(
            "'{}' doesn't look like a dtx project (no styles/input.css found)",
            path
        );
    }

    println!("\n{} {}\n", "Updating".green().bold(), path.bold());

    css::sync_source_css(path)?;
    println!("{} styles/source.css", "✔".green());

    let (removed_old, added_import) = css::migrate_input_css(path)?;
    if removed_old {
        println!(
            "{} styles/input.css (removed legacy @source inline() block)",
            "✔".green()
        );
    }
    if added_import {
        println!(
            "{} styles/input.css (added @import \"./source.css\";)",
            "✔".green()
        );
    }

    let version = css::latest_published_version()?;
    cargo::bump_detaxine_ui_version(path, &version)?;
    println!("{} Cargo.toml (detaxine-ui = \"{}\")", "✔".green(), version);

    println!("\n{}", "Done.".bold());
    Ok(())
}
