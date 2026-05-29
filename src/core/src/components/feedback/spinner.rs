use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum SpinnerSize {
    Sm,
    Md,
    Lg,
}

/// A loading spinner component styled with Tailwind CSS.
/// Uses an SVG circle with animated stroke-dashoffset for a smooth rotating effect.
/// Colors can be customized via Tailwind classes.
///
/// Example usage:
/// ```
/// <Spinner size=SpinnerSize::Md color="text-primary" />
/// ```
#[component]
pub fn Spinner(
    #[prop(into, optional, default = SpinnerSize::Md)] size: SpinnerSize,
    #[prop(into, optional, default = "text-primary".to_string())] color: String,
    #[prop(default = true)] with_backdrop: bool,
) -> impl IntoView {
    let (svg_size, stroke_width) = match size {
        SpinnerSize::Sm => (24, 4),
        SpinnerSize::Md => (48, 6),
        SpinnerSize::Lg => (72, 8),
    };

    let center = svg_size / 2;
    let radius = (svg_size / 2 - stroke_width / 2) as f64;
    let circumference = 2.0 * std::f64::consts::PI * radius;

    let spinner = view! {
        <svg
            class=format!("{} animate-spin", color)
            width=svg_size
            height=svg_size
            viewBox=format!("0 0 {} {}", svg_size, svg_size)
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <circle
                cx=center
                cy=center
                r=radius
                stroke="currentColor"
                stroke-width=stroke_width
                stroke-linecap="round"
                stroke-dasharray=circumference
                stroke-dashoffset={circumference * 0.75}
                class="opacity-25"
            />
            <circle
                cx=center
                cy=center
                r=radius
                stroke="currentColor"
                stroke-width=stroke_width
                stroke-linecap="round"
                stroke-dasharray=circumference
                stroke-dashoffset={circumference * 0.25}
                class="opacity-75"
            />
        </svg>
    };

    if with_backdrop {
        view! {
            <div class="fixed inset-0 bg-light-gray opacity-50 flex items-center justify-center z-50">
                {spinner}
            </div>
        }
    } else {
        view! {
            <div class="flex items-center justify-center">
                {spinner}
            </div>
        }
    }
}
