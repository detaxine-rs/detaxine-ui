use icondata::CgCloseO;
use leptos::prelude::*;
use leptos_icons::Icon;

use crate::components::schemas::props::ColorTemperature;

/// A chip component that represents a tag or filter item.
/// It supports different color temperatures and optional removal via a close button.
/// The chip is removable by default, but this can be disabled with `removable=false`.
///
/// Example usage:
/// ```rust
/// // Removable chip (default behavior)
/// <Chip
///     label="Removable Chip"
///     color=ColorTemperature::Success
///     on_remove=Some(Callback::new(|_| {
///         log::info!("Chip removed");
///     }))
/// />
///
/// // Non-removable chip (static label-like behavior)
/// <Chip
///     label="Static Chip"
///     color=ColorTemperature::Info
///     removable=false
/// />
/// ```
#[component]
pub fn Chip(
    #[prop(into)] label: String,
    #[prop(default = ColorTemperature::Primary)] color: ColorTemperature,
    #[prop(default = true)] removable: bool,
    #[prop(optional, default = Callback::new(|_| {}))] on_remove: Callback<()>,
) -> impl IntoView {
    // Generate color classes based on the selected temperature
    let color_classes = move || match color {
        ColorTemperature::Success => "text-success border-2 border-success bg-success/20",
        ColorTemperature::Warning => "text-warning border-2 border-warning bg-warning/20",
        ColorTemperature::Info => "text-info border-2 border-info bg-info/20",
        ColorTemperature::Danger => "text-danger border-2 border-danger bg-danger/20",
        ColorTemperature::Gray => "text-mid-gray border-2 border-mid-gray bg-mid-gray/20",
        _ => "text-primary border-2 border-primary bg-primary/20",
    };

    // Handle close button click (only if removable and callback is provided)
    let on_click = move |_| {
        on_remove.run(());
    };

    view! {
        <div class=format!("inline-flex items-center px-3 text-center rounded text-sm gap-2 {}", color_classes())>
            <span>{label}</span>
            // Conditionally render the close button only if removable is true and on_remove is provided
            {if removable {
                Some(
                    view! {
                        <button
                            class="cursor-pointer hover:opacity-75 flex items-center p-1"
                            on:click=on_click
                            aria-label="Remove chip"
                        >
                            <Icon width="1em" height="1em" icon=CgCloseO />
                        </button>
                    }
                )
            } else {
                None
            }}
        </div>
    }
}
