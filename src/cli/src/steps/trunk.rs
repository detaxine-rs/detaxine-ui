// use crate::steps::tailwind::tailwind_bin_path;
use anyhow::Result;

pub fn write(project: &str) -> Result<()> {
    // let bin = tailwind_bin_path(".");

    let contents = format!(
        r#"[build]
target = "index.html"

[tools]
tailwindcss = "4.3.2"
"#
    );
    std::fs::write(format!("{}/Trunk.toml", project), contents)?;
    Ok(())
}
