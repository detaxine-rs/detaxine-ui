use anyhow::{Context, Result, bail};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::process::Command;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub fn download_binary(project: &str) -> Result<()> {
    let url = platform_url()?;
    let bin_path = tailwind_bin_path(project);

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message("Downloading Tailwind binary...");
    pb.enable_steady_tick(std::time::Duration::from_millis(80));

    let status = Command::new("curl")
        .args(["-L", "-o", &bin_path, url])
        .status()
        .context("Failed to run curl — is it installed?")?;

    if !status.success() {
        bail!("curl failed to download Tailwind binary");
    }

    #[cfg(unix)]
    fs::set_permissions(&bin_path, fs::Permissions::from_mode(0o755))?;

    pb.finish_and_clear();
    Ok(())
}

pub fn tailwind_bin_path(project: &str) -> String {
    #[cfg(windows)]
    return format!("{}/bin/tailwindcss.exe", project);

    #[cfg(not(windows))]
    format!("{}/bin/tailwindcss", project)
}

fn platform_url() -> Result<&'static str> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "aarch64") => Ok(
            "https://github.com/tailwindlabs/tailwindcss/releases/download/v4.1.10/tailwindcss-macos-arm64",
        ),
        ("macos", "x86_64") => Ok(
            "https://github.com/tailwindlabs/tailwindcss/releases/download/v4.1.10/tailwindcss-macos-x64",
        ),
        ("linux", "x86_64") => Ok(
            "https://github.com/tailwindlabs/tailwindcss/releases/download/v4.1.10/tailwindcss-linux-x64",
        ),
        ("linux", "aarch64") => Ok(
            "https://github.com/tailwindlabs/tailwindcss/releases/download/v4.1.10/tailwindcss-linux-arm64",
        ),
        ("windows", "x86_64") => Ok(
            "https://github.com/tailwindlabs/tailwindcss/releases/download/v4.1.10/tailwindcss-windows-x64.exe",
        ),
        (os, arch) => bail!(
            "Unsupported platform: {}/{}. Please download the Tailwind binary manually \
             from https://github.com/tailwindlabs/tailwindcss/releases/download/v4.1.10",
            os,
            arch
        ),
    }
}
