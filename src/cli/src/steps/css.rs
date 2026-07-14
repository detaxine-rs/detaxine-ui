use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::process::Command;

const LIB_REPO: &str = "https://github.com/elonaire/detaxine-ui";

pub fn download_input_css(project: &str) -> Result<()> {
    let pb = spinner("Cloning detaxine-ui styles...");

    let tmp = std::env::temp_dir().join("detaxine-ui-src");
    if tmp.exists() {
        fs::remove_dir_all(&tmp)?;
    }
    let status = Command::new("git")
        .args(["clone", "--depth=1", LIB_REPO, tmp.to_str().unwrap()])
        .status()
        .context("git clone failed — is git installed?")?;
    if !status.success() {
        anyhow::bail!("Failed to clone detaxine-ui repository");
    }

    let lib_input_css = tmp.join("src/core/styles/input.css");
    fs::create_dir_all(format!("{}/styles", project))?;
    fs::copy(&lib_input_css, format!("{}/styles/input.css", project))
        .context("Could not find src/core/styles/input.css in detaxine-ui repo")?;

    fs::remove_dir_all(&tmp)?;
    pb.finish_and_clear();
    Ok(())
}

fn spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(std::time::Duration::from_millis(80));
    pb
}
