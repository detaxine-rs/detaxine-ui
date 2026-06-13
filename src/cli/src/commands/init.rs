use crate::steps::{cargo, css, html, trunk};
use anyhow::{Result, bail};
use colored::Colorize;
use std::fs;

pub fn run_init(name: &str) -> Result<()> {
    if std::path::Path::new(name).exists() {
        bail!("directory '{}' already exists", name);
    }

    println!("\n{} {}\n", "Scaffolding".green().bold(), name.bold());

    // 1. Create directory structure
    fs::create_dir_all(format!("{}/src", name))?;
    fs::create_dir_all(format!("{}/styles", name))?;
    println!("{} {}/", "✔".green(), name);

    // 2. Cargo.toml + src/main.rs
    cargo::write_manifest(name)?;
    println!("{} Cargo.toml", "✔".green());

    cargo::write_main(name)?;
    println!("{} src/main.rs", "✔".green());

    // 3. Clone lib and copy its input.css as the project's style base.
    //    Consumer can extend/override from here.
    css::download_input_css(name)?;
    println!("{} styles/input.css", "✔".green());

    // 4. index.html
    html::write(name)?;
    println!("{} index.html", "✔".green());

    // 5. Trunk.toml with pre-build hook
    trunk::write(name)?;
    println!("{} Trunk.toml", "✔".green());

    // 6. .gitignore
    write_gitignore(name)?;
    println!("{} .gitignore", "✔".green());

    println!("\n{}", "Done! Next steps:".bold());
    println!("  cd {}", name);
    println!("  trunk serve\n");

    Ok(())
}

fn write_gitignore(project: &str) -> Result<()> {
    let contents = "/target\n/dist\nstyles/output.css\nbin/\n";
    std::fs::write(format!("{}/.gitignore", project), contents)?;
    Ok(())
}
