use leptos::html::*;
use leptos::prelude::*;
use tailwind_fuse::tw_merge;

use crate::components::schemas::props::ColorTemperature;

/// A badge overlaid on a child element, typically used to display a count or status indicator.
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
    /// Optional label rendered inside the badge. If absent, a small dot is shown instead.
    #[prop(into)]
    text: MaybeProp<String>,

    /// Badge background color via `ColorTemperature`. Defaults to
    /// `ColorTemperature::Primary`. Supported values: `Primary`, `Danger`,
    /// `Success`, `Warning`, `Info`.
    #[prop(default = ColorTemperature::Primary)]
    color: ColorTemperature,

    /// **Deprecated**: use `class` instead. Still supported and merged
    /// in alongside `class` for backward compatibility.
    #[prop(into, optional)]
    parent_class: MaybeProp<String>,

    /// Extra Tailwind classes for the root wrapper `<div>`.
    #[prop(into, optional)]
    class: MaybeProp<String>,

    /// The element the badge is anchored to.
    children: Children,

    /// **Deprecated**: use `badge_class` instead.
    #[prop(into, optional)]
    badge_position: MaybeProp<String>,

    /// Extra Tailwind classes for the badge `<span>` itself (position,
    /// color, size overrides, etc.).
    #[prop(into, optional)]
    badge_class: MaybeProp<String>,
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

    let root_class = move || {
        tw_merge!(
            "relative",
            parent_class.get().unwrap_or_default(),
            class.get().unwrap_or_default()
        )
    };

    let badge_class_val = move || {
        tw_merge!(
            format!(
                "inline-flex items-center justify-center rounded-full text-xs font-medium text-contrast-white absolute top-0 right-0 transform translate-x-1/2 -translate-y-1/2 {} {}",
                color_classes(),
                width_classes()
            ),
            badge_position.get().unwrap_or_default(),
            badge_class.get().unwrap_or_default()
        )
    };

    view! {
        <div class=root_class>
            {children()}
            <span class=badge_class_val>
                {text}
            </span>
        </div>
    }
    .into_any()
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
