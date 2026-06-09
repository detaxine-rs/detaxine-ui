use anyhow::Result;

pub fn write(project: &str) -> Result<()> {
    let contents = format!(
        r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{project}</title>
    <link data-trunk rel="rust" />
    <link data-trunk rel="css" href="/styles/output.css" />
  </head>
  <body>
    <div id="modal-root"></div>
  </body>
</html>
"#
    );
    std::fs::write(format!("{}/index.html", project), contents)?;
    Ok(())
}
