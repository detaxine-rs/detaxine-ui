use anyhow::Result;
use std::fs;

pub fn write_manifest(name: &str) -> Result<()> {
    let contents = format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2024"

[dependencies]
detaxine-ui = "0.8.34"
leptos = {{ version = "0.8.19", features = ["csr"] }}
leptos_meta = "0.8.6"
"#
    );
    fs::write(format!("{}/Cargo.toml", name), contents)?;
    Ok(())
}

pub fn write_main(name: &str) -> Result<()> {
    let contents = r#"use leptos::prelude::*;
use leptos_meta::Stylesheet;

use detaxine_ui::{
    components::{
        actions::button::{BasicButton, ButtonGroup},
        forms::toggle_switch::ToggleSwitch,
    },
    icondata::{AiCheckCircleOutlined, BsXCircle},
};

#[component]
fn App() -> impl IntoView {
    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <h1>"Hello from detaxine-ui!"</h1>
        <ButtonGroup style_ext="font-bold bg-primary text-white hover:bg-secondary">
            <BasicButton
                button_text="First"
                icon=Some(AiCheckCircleOutlined)
                icon_before=true
            />
            <BasicButton
                button_text="Second"
                icon=Some(BsXCircle)
                icon_before=false
            />
            <BasicButton
                button_text="Third"
                disabled=true
            />
        </ButtonGroup>
        <ToggleSwitch
           initial_active_state=true
           label_active="Enabled"
           label_inactive="Disabled"
           name="status"
        />
    }
}

fn main() {
    mount_to_body(App)
}
"#;
    fs::write(format!("{}/src/main.rs", name), contents)?;
    Ok(())
}
