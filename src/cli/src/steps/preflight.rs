use anyhow::{Result, bail};
use colored::Colorize;
use std::process::Command;

/// Checks that `cargo-leptos` is installed and runnable before we scaffold
/// an SSR project — SSR mode is unusable without it (it drives the build,
/// Tailwind pipeline, and hydration bundle), so failing fast here avoids
/// leaving the user with a generated project that immediately breaks on
/// `cargo leptos watch`.
pub fn check_cargo_leptos() -> Result<()> {
    let found = Command::new("cargo")
        .args(["leptos", "--version"])
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false);

    if !found {
        bail!(
            "{}\n\n  {}\n\n{}",
            "cargo-leptos is required for SSR projects but wasn't found."
                .red()
                .bold(),
            "cargo install cargo-leptos --locked".bold(),
            "Install it, then run this command again."
        );
    }

    Ok(())
}
