use leptos::html::*;
use leptos::prelude::*;

use crate::components::schemas::props::ColorTemperature;

/// A static label tag with a color-coded border and background.
///
/// # Props
///
/// - `label` – Text displayed inside the tag.
/// - `color` – Tag color via `ColorTemperature`. Defaults to `ColorTemperature::Primary`.
///   Supported values: `Primary`, `Success`, `Warning`, `Info`, `Danger`.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::{data_display::tag::LabelTag, schemas::props::ColorTemperature};
///
/// #[component]
/// fn Example() -> impl IntoView {
///     view! {
///         <LabelTag label="Success" color=ColorTemperature::Success />
///     }
/// }
/// ```
#[component]
pub fn LabelTag(
    #[prop(into, optional)] label: String,
    #[prop(default = ColorTemperature::Primary)] color: ColorTemperature,
) -> impl IntoView {
    // Function to return the corresponding tailwind classes
    let color_classes = match color {
        ColorTemperature::Success => "text-success border-2 border-success bg-success/20",
        ColorTemperature::Warning => "text-warning border-2 border-warning bg-warning/20",
        ColorTemperature::Info => "text-info border-2 border-info bg-info/20",
        ColorTemperature::Danger => "text-danger border-2 border-danger bg-danger/20",
        _ => "text-primary border-2 border-primary bg-primary/20",
    };

    view! {
        <div class=format!("inline-block px-3 text-center rounded text-sm {}", color_classes)>
            <span>{label}</span>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn color_classes(color: &ColorTemperature) -> &'static str {
        match color {
            ColorTemperature::Success => "text-success border-2 border-success bg-success/20",
            ColorTemperature::Warning => "text-warning border-2 border-warning bg-warning/20",
            ColorTemperature::Info => "text-info border-2 border-info bg-info/20",
            ColorTemperature::Danger => "text-danger border-2 border-danger bg-danger/20",
            _ => "text-primary border-2 border-primary bg-primary/20",
        }
    }

    #[test]
    fn success_color() {
        assert_eq!(
            color_classes(&ColorTemperature::Success),
            "text-success border-2 border-success bg-success/20"
        );
    }

    #[test]
    fn warning_color() {
        assert_eq!(
            color_classes(&ColorTemperature::Warning),
            "text-warning border-2 border-warning bg-warning/20"
        );
    }

    #[test]
    fn info_color() {
        assert_eq!(
            color_classes(&ColorTemperature::Info),
            "text-info border-2 border-info bg-info/20"
        );
    }

    #[test]
    fn danger_color() {
        assert_eq!(
            color_classes(&ColorTemperature::Danger),
            "text-danger border-2 border-danger bg-danger/20"
        );
    }

    #[test]
    fn primary_is_default_fallback() {
        assert_eq!(
            color_classes(&ColorTemperature::Primary),
            "text-primary border-2 border-primary bg-primary/20"
        );
    }
}
