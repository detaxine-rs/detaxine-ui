use crate::stacks::{
    helper::overlay_root,
    z_stack::{ZONE_CRITICAL, expect_z_stack},
};
use leptos::{portal::Portal, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] // added Copy — same reasoning as Position
#[allow(dead_code)]
pub enum SpinnerSize {
    Sm,
    Md,
    Lg,
}

/// A loading spinner using an animated SVG circle, with an optional full-screen backdrop.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::feedback::spinner::{Spinner, SpinnerSize};
///
/// #[component]
/// fn Example() -> impl IntoView {
///     view! {
///         <Spinner size=SpinnerSize::Md color="text-primary" with_backdrop=false />
///     }
/// }
/// ```
#[component]
pub fn Spinner(
    /// `SpinnerSize::Sm`, `Md`, or `Lg` controlling the SVG dimensions and stroke width. Defaults to `Md`.
    #[prop(into, optional, default = SpinnerSize::Md)]
    size: SpinnerSize,

    /// Tailwind text color class applied to the SVG (uses `currentColor`). Defaults to `"text-primary"`.
    #[prop(into, optional, default = "text-primary".to_string())]
    color: String,

    /// When `true`, renders the spinner centered over a fixed full-screen overlay. Defaults to `true`.
    #[prop(default = true)]
    with_backdrop: bool,
) -> impl IntoView {
    let (svg_size, stroke_width) = match size {
        SpinnerSize::Sm => (24, 4),
        SpinnerSize::Md => (48, 6),
        SpinnerSize::Lg => (72, 8),
    };
    let center = svg_size / 2;
    let radius = (svg_size / 2 - stroke_width / 2) as f64;
    let circumference = 2.0 * std::f64::consts::PI * radius;

    // build_spinner is Fn (Copy, even) — everything it captures is a
    // plain stack value (i32/f64/String-by-ref-clone), so it can be
    // called repeatedly instead of constructing the AnyView once
    let build_spinner = StoredValue::new(move || {
        view! {
            <svg
                class=format!("{} animate-spin", color)
                width=svg_size
                height=svg_size
                viewBox=format!("0 0 {} {}", svg_size, svg_size)
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
            >
                <circle cx=center cy=center r=radius stroke="currentColor" stroke-width=stroke_width
                    stroke-linecap="round" stroke-dasharray=circumference
                    stroke-dashoffset={circumference * 0.75} class="opacity-25" />
                <circle cx=center cy=center r=radius stroke="currentColor" stroke-width=stroke_width
                    stroke-linecap="round" stroke-dasharray=circumference
                    stroke-dashoffset={circumference * 0.25} class="opacity-75" />
            </svg>
        }
    });

    if with_backdrop {
        let z_stack = expect_z_stack();
        let (_, z) = z_stack.acquire_pair(ZONE_CRITICAL);

        view! {
                {move || overlay_root().map(|root| view! {
                    <Portal mount=root>
                        <div
                            class="fixed inset-0 bg-light-gray opacity-50 flex items-center justify-center"
                            style=format!("z-index: {}", z)
                        >
                            {build_spinner.get_value()()}
                        </div>
                    </Portal>
                })}
            }.into_any()
    } else {
        view! {
            <div class="flex items-center justify-center">
                {build_spinner.get_value()()}
            </div>
        }
        .into_any()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    // SpinnerSize

    #[test]
    fn size_eq() {
        assert_eq!(SpinnerSize::Md, SpinnerSize::Md);
        assert_ne!(SpinnerSize::Sm, SpinnerSize::Lg);
    }

    #[test]
    fn size_clone() {
        assert_eq!(SpinnerSize::Lg.clone(), SpinnerSize::Lg);
    }

    // svg_size / stroke_width mapping

    fn dimensions(size: &SpinnerSize) -> (i32, i32) {
        match size {
            SpinnerSize::Sm => (24, 4),
            SpinnerSize::Md => (48, 6),
            SpinnerSize::Lg => (72, 8),
        }
    }

    #[test]
    fn sm_dimensions() {
        assert_eq!(dimensions(&SpinnerSize::Sm), (24, 4));
    }

    #[test]
    fn md_dimensions() {
        assert_eq!(dimensions(&SpinnerSize::Md), (48, 6));
    }

    #[test]
    fn lg_dimensions() {
        assert_eq!(dimensions(&SpinnerSize::Lg), (72, 8));
    }

    // derived geometry

    fn geometry(size: &SpinnerSize) -> (i32, f64, f64) {
        let (svg_size, stroke_width) = dimensions(size);
        let radius = (svg_size / 2 - stroke_width / 2) as f64;
        let circumference = 2.0 * PI * radius;
        (svg_size / 2, radius, circumference)
    }

    #[test]
    fn center_is_half_svg_size() {
        for size in [SpinnerSize::Sm, SpinnerSize::Md, SpinnerSize::Lg] {
            let (svg_size, _) = dimensions(&size);
            let (center, _, _) = geometry(&size);
            assert_eq!(center, svg_size / 2);
        }
    }

    #[test]
    fn radius_accounts_for_stroke() {
        let (svg_size, stroke_width) = dimensions(&SpinnerSize::Md);
        let (_, radius, _) = geometry(&SpinnerSize::Md);
        assert_eq!(radius, (svg_size / 2 - stroke_width / 2) as f64);
    }

    #[test]
    fn circumference_derived_from_radius() {
        let (_, radius, circumference) = geometry(&SpinnerSize::Md);
        assert!((circumference - 2.0 * PI * radius).abs() < 1e-9);
    }

    // stroke-dashoffset fractions

    #[test]
    fn background_arc_dashoffset_is_75_percent() {
        let (_, _, circ) = geometry(&SpinnerSize::Md);
        let offset = circ * 0.75;
        assert!((offset - circ * 0.75).abs() < 1e-9);
    }

    #[test]
    fn foreground_arc_dashoffset_is_25_percent() {
        let (_, _, circ) = geometry(&SpinnerSize::Md);
        let offset = circ * 0.25;
        assert!((offset - circ * 0.25).abs() < 1e-9);
    }
}
