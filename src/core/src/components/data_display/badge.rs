use leptos::html::*;
use leptos::prelude::*;

use crate::components::schemas::props::ColorTemperature;

/// This is a badge component that can be used to display a small piece of information.
/// It can be customized with different colors and positions.
/// Example usage:
/// ```
/// <Badge text="2" ><span>"Notifications"</span></Badge>
/// ```
#[component]
pub fn Badge(
    #[prop(into)] text: String,
    #[prop(default = ColorTemperature::Primary)] color: ColorTemperature,
    #[prop(into, optional)] parent_class: String,
    children: Children,
    #[prop(into, optional)] badge_position: String,
) -> impl IntoView {
    let color_classes = move || match color {
        ColorTemperature::Danger => "bg-danger",
        ColorTemperature::Success => "bg-success",
        ColorTemperature::Warning => "bg-warning",
        ColorTemperature::Info => "bg-info",
        _ => "bg-primary", // default color
    };

    let text_clone = text.clone();
    let width_classes = move || {
        if text_clone.is_empty() {
            "w-2 h-2"
        } else {
            "min-w-4 h-4 p-1"
        }
    };

    view! {
        <div class=format!("relative {}", parent_class)>
            {children()}
            <span class=format!(
                "inline-flex items-center justify-center rounded-full text-xs font-medium text-contrast-white absolute top-0 right-0 transform translate-x-1/2 -translate-y-1/2 {} {} {}",
                color_classes(),
                width_classes(),
                badge_position
            )>
                {text}
            </span>
        </div>
    }
}
