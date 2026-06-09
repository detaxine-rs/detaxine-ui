use crate::steps::tailwind::tailwind_bin_path;
use anyhow::Result;

pub fn write(project: &str) -> Result<()> {
    let bin = tailwind_bin_path(".");

    let contents = format!(
        r#"[build]
target = "index.html"

[[hooks]]
stage = "pre_build"
command = "{bin}"
command_arguments = [
    "-i", "./styles/input.css",
    "-o", "./styles/output.css",
    "--minify",
]
"#
    );
    std::fs::write(format!("{}/Trunk.toml", project), contents)?;
    Ok(())
}
