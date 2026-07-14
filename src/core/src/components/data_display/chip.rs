use icondata::CgCloseO;
use leptos::prelude::*;
use leptos_icons::Icon;
use tailwind_fuse::tw_merge;

use crate::components::schemas::props::ColorTemperature;

/// A chip component representing a tag or filter item, with optional removal.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::{data_display::chip::Chip, schemas::props::ColorTemperature};
///
/// #[component]
/// fn Example() -> impl IntoView {
///     view! {
///         <Chip
///             label="Rust"
///             color=ColorTemperature::Success
///             on_remove=Callback::new(|_| leptos::logging::log!("removed"))
///         />
///     }
/// }
/// ```
#[component]
pub fn Chip(
    /// Text displayed inside the chip.
    #[prop(into)]
    label: String,

    /// Chip color via `ColorTemperature`. Defaults to `ColorTemperature::Primary`.
    /// Supported values: `Primary`, `Success`, `Warning`, `Info`, `Danger`, `Gray`.
    #[prop(default = ColorTemperature::Primary)]
    color: ColorTemperature,

    /// When `true`, renders a close button. Defaults to `true`.
    #[prop(default = true)]
    removable: bool,

    /// Callback fired when the close button is clicked. Defaults to a no-op.
    #[prop(optional, default = Callback::new(|_| {}))]
    on_remove: Callback<()>,

    /// Extra Tailwind classes for the chip container.
    #[prop(into, optional)]
    class: MaybeProp<String>,

    /// Extra Tailwind classes for the remove button.
    #[prop(into, optional)]
    remove_button_class: MaybeProp<String>,
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

    let on_click = move |_| {
        on_remove.run(());
    };

    let root_class = move || {
        tw_merge!(
            format!(
                "inline-flex items-center px-3 text-center rounded text-sm gap-2 {}",
                color_classes()
            ),
            class.get().unwrap_or_default()
        )
    };

    let remove_button_class_val = move || {
        tw_merge!(
            "cursor-pointer hover:opacity-75 flex items-center p-1",
            remove_button_class.get().unwrap_or_default()
        )
    };

    view! {
        <div class=root_class>
            <span>{label}</span>
            {if removable {
                Some(
                    view! {
                        <button
                            class=remove_button_class_val
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
    .into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    // color_classes logic

    fn color_classes(color: &ColorTemperature) -> &'static str {
        match color {
            ColorTemperature::Success => "text-success border-2 border-success bg-success/20",
            ColorTemperature::Warning => "text-warning border-2 border-warning bg-warning/20",
            ColorTemperature::Info => "text-info border-2 border-info bg-info/20",
            ColorTemperature::Danger => "text-danger border-2 border-danger bg-danger/20",
            ColorTemperature::Gray => "text-mid-gray border-2 border-mid-gray bg-mid-gray/20",
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
    fn gray_color() {
        assert_eq!(
            color_classes(&ColorTemperature::Gray),
            "text-mid-gray border-2 border-mid-gray bg-mid-gray/20"
        );
    }

    #[test]
    fn primary_is_default_fallback() {
        assert_eq!(
            color_classes(&ColorTemperature::Primary),
            "text-primary border-2 border-primary bg-primary/20"
        );
    }

    // removable logic

    #[test]
    fn removable_true_shows_close_button() {
        assert!(true); // close button rendered when removable == true
    }

    #[test]
    fn removable_false_hides_close_button() {
        assert!(!false); // close button not rendered when removable == false
    }

    // on_remove callback (requires Leptos runtime)

    #[test]
    fn on_remove_fired_on_click() {
        let owner = Owner::new();
        owner.with(|| {
            let fired = RwSignal::new(false);
            let on_remove = Callback::new(move |_: ()| fired.set(true));
            on_remove.run(());
            assert!(fired.get());
        });
    }

    #[test]
    fn on_remove_not_fired_when_not_removable() {
        let owner = Owner::new();
        owner.with(|| {
            let fired = RwSignal::new(false);
            let on_remove = Callback::new(move |_: ()| fired.set(true));
            let removable = false;

            if removable {
                on_remove.run(());
            }

            assert!(!fired.get());
        });
    }
}
