use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::process::Command;

const LIB_REPO: &str = "https://github.com/detaxine-rs/detaxine-ui";

#[derive(Deserialize)]
struct CrateResponse {
    #[serde(rename = "crate")]
    krate: CrateInfo,
}

#[derive(Deserialize)]
struct CrateInfo {
    max_stable_version: String,
}

pub fn latest_published_version() -> Result<String> {
    let resp = reqwest::blocking::Client::new()
        .get("https://crates.io/api/v1/crates/detaxine-ui")
        .header(
            "User-Agent",
            "dtx-cli (https://github.com/detaxine-rs/detaxine-ui)",
        )
        .send()
        .context("failed to reach crates.io")?;

    if !resp.status().is_success() {
        anyhow::bail!("crates.io returned {} for detaxine-ui", resp.status());
    }

    let parsed: CrateResponse = resp.json().context("bad response from crates.io")?;
    Ok(parsed.krate.max_stable_version)
}

/// Strips a legacy inline `@source inline("...");` block from input.css, if present.
/// Returns true if something was removed.
fn strip_inline_safelist(contents: &str) -> (String, bool) {
    let re = Regex::new(r#"@source\s+inline\(\s*"(?:[^"\\]|\\.)*"\s*\)\s*;\s*\n?"#)
        .expect("invalid regex");

    if !re.is_match(contents) {
        return (contents.to_string(), false);
    }
    (re.replace_all(contents, "").to_string(), true)
}

/// Migrates input.css for `dtx update`: removes any legacy inline safelist
/// and ensures `@import "source.css";` is present. Returns (removed_old, added_import).
pub fn migrate_input_css(project: &str) -> Result<(bool, bool)> {
    let path = format!("{}/styles/input.css", project);
    let contents = fs::read_to_string(&path).context("could not read styles/input.css")?;

    let (contents, removed_old) = strip_inline_safelist(&contents);

    let already_imports =
        contents.contains("@import \"source.css\"") || contents.contains("@import 'source.css'");

    let (final_contents, added_import) = if already_imports {
        (contents, false)
    } else {
        (
            format!("{}\n@import \"source.css\";\n", contents.trim_end()),
            true,
        )
    };

    if removed_old || added_import {
        fs::write(&path, final_contents).context("could not write styles/input.css")?;
    }

    Ok((removed_old, added_import))
}

// steps/css.rs
pub fn sync_source_css(project: &str) -> Result<()> {
    let pb = spinner("Fetching latest detaxine-ui safelist...");
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
    let lib_source_css = tmp.join("src/core/styles/source.css");
    let dest = format!("{}/styles/source.css", project);
    if !std::path::Path::new(&format!("{}/styles", project)).exists() {
        anyhow::bail!(
            "'{}' doesn't look like a dtx project (no styles/ directory found)",
            project
        );
    }
    fs::copy(&lib_source_css, &dest)
        .context("Could not find src/core/styles/source.css in detaxine-ui repo")?;
    fs::remove_dir_all(&tmp)?;
    pb.finish_and_clear();
    Ok(())
}

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
