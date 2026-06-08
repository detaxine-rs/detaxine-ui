use leptos::html::*;
use leptos::prelude::*;

use crate::components::schemas::props::ColorTemperature;

/// A badge overlaid on a child element, typically used to display a count or status indicator.
///
/// # Props
///
/// - `text` – Optional label rendered inside the badge. If absent, a small dot is shown instead.
/// - `color` – Badge background color via `ColorTemperature`. Defaults to `ColorTemperature::Primary`.
///   Supported values: `Primary`, `Danger`, `Success`, `Warning`, `Info`.
/// - `parent_class` – Additional Tailwind classes applied to the wrapping `div`.
/// - `badge_position` – Additional Tailwind classes to adjust the badge position.
/// - `children` – The element the badge is anchored to.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::{data_display::badge::Badge, schemas::props::ColorTemperature};
///
/// #[component]
/// fn Example() -> impl IntoView {
///     view! {
///         <Badge text="5" color=ColorTemperature::Danger>
///             <span>"Notifications"</span>
///         </Badge>
///     }
/// }
/// ```
#[component]
pub fn Badge(
    #[prop(into)] text: MaybeProp<String>,
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
        if text_clone.get().is_none() {
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

#[cfg(test)]
mod tests {
    use super::*;

    // color_classes logic

    fn color_classes(color: &ColorTemperature) -> &'static str {
        match color {
            ColorTemperature::Danger => "bg-danger",
            ColorTemperature::Success => "bg-success",
            ColorTemperature::Warning => "bg-warning",
            ColorTemperature::Info => "bg-info",
            _ => "bg-primary",
        }
    }

    #[test]
    fn danger_maps_to_bg_danger() {
        assert_eq!(color_classes(&ColorTemperature::Danger), "bg-danger");
    }

    #[test]
    fn success_maps_to_bg_success() {
        assert_eq!(color_classes(&ColorTemperature::Success), "bg-success");
    }

    #[test]
    fn warning_maps_to_bg_warning() {
        assert_eq!(color_classes(&ColorTemperature::Warning), "bg-warning");
    }

    #[test]
    fn info_maps_to_bg_info() {
        assert_eq!(color_classes(&ColorTemperature::Info), "bg-info");
    }

    #[test]
    fn primary_maps_to_bg_primary() {
        assert_eq!(color_classes(&ColorTemperature::Primary), "bg-primary");
    }

    // width_classes logic

    fn width_classes(has_text: bool) -> &'static str {
        if !has_text {
            "w-2 h-2"
        } else {
            "min-w-4 h-4 p-1"
        }
    }

    #[test]
    fn no_text_renders_dot() {
        assert_eq!(width_classes(false), "w-2 h-2");
    }

    #[test]
    fn with_text_renders_pill() {
        assert_eq!(width_classes(true), "min-w-4 h-4 p-1");
    }

    // reactive text (requires Leptos runtime)

    #[test]
    fn text_signal_determines_width_class() {
        let owner = Owner::new();
        owner.with(|| {
            let text: MaybeProp<String> = MaybeProp::derive(move || None);
            assert_eq!(width_classes(text.get().is_some()), "w-2 h-2");

            let text: MaybeProp<String> = MaybeProp::from("3".to_string());
            assert_eq!(width_classes(text.get().is_some()), "min-w-4 h-4 p-1");
        });
    }
}
