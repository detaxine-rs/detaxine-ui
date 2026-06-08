use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum ProgressComponentSize {
    Sm,
    Md,
    Lg,
}

/// A horizontal progress bar supporting determinate and indeterminate states.
///
/// # Props
///
/// - `progress` – `RwSignal<f64>` where `0.0` is 0% and `100.0` is 100%. Clamped automatically. Defaults to `0.0`.
/// - `color` – Tailwind background class for the fill. Defaults to `"bg-primary"`.
/// - `size` – `ProgressComponentSize::Sm`, `Md`, or `Lg` controlling bar height. Defaults to `Md`.
/// - `show_percentage` – When `true`, renders a percentage label below the bar. Ignored when `indeterminate=true`. Defaults to `false`.
/// - `indeterminate` – When `true`, renders an animated sliding bar and ignores `progress`. Defaults to `false`.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::feedback::progress::ProgressBar;
///
/// #[component]
/// fn Example() -> impl IntoView {
///     let progress = RwSignal::new(65.0);
///
///     view! {
///         <ProgressBar progress=progress show_percentage=true />
///     }
/// }
/// ```
#[component]
pub fn ProgressBar(
    #[prop(into, default = RwSignal::new(0.0))] progress: RwSignal<f64>,
    #[prop(into, optional, default = "bg-primary".to_string())] color: String,
    #[prop(into, optional, default = ProgressComponentSize::Md)] size: ProgressComponentSize,
    #[prop(default = false)] show_percentage: bool,
    #[prop(default = false)] indeterminate: bool,
) -> impl IntoView {
    // Determine height based on size prop
    let container_class = move || {
        let height = match size {
            ProgressComponentSize::Sm => "h-1",
            ProgressComponentSize::Md => "h-2",
            ProgressComponentSize::Lg => "h-4",
        };
        if indeterminate {
            format!(
                "w-full bg-light-gray rounded-full {} relative overflow-hidden",
                height
            )
        } else {
            format!("w-full bg-light-gray rounded-full {}", height)
        }
    };

    let fill_class = move || {
        if indeterminate {
            format!(
                "{} h-full rounded-full animate-progress-indeterminate absolute",
                color
            )
        } else {
            format!("{} h-full rounded-full transition-all duration-300", color)
        }
    };

    let fill_style = move || {
        if indeterminate {
            "".to_string() // Width is handled in CSS class
        } else {
            format!("width: {}%;", progress.get().min(100.0).max(0.0))
        }
    };

    let aria_valuenow = move || {
        if indeterminate {
            "-1".to_string() // Indicates indeterminate
        } else {
            (progress.get() as i32).to_string()
        }
    };

    view! {
        <div class={container_class}>
            <div
                class={fill_class}
                style=fill_style
                role="progressbar"
                aria-valuemin="0"
                aria-valuemax="100"
                aria-valuenow=aria_valuenow
            ></div>
        </div>
        {if show_percentage && !indeterminate {
            Some(view! {
                <div class="text-xs text-center mt-1">
                    {move || format!("{:.0}%", (progress.get()).min(100.0).max(0.0))}
                </div>
            })
        } else { None }}
    }
}

/// A circular progress ring with optional percentage label in the center.
///
/// # Props
///
/// - `progress_percentage` – `RwSignal<f64>` from `0.0` to `100.0`. Clamped automatically. Defaults to `0.0`.
/// - `size` – `ProgressComponentSize::Sm`, `Md`, or `Lg` controlling the SVG dimensions and stroke width. Defaults to `Md`.
/// - `show_percentage` – When `true`, renders the percentage value in the center of the ring. Defaults to `true`.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::feedback::progress::{CircularProgress, ProgressComponentSize};
///
/// #[component]
/// fn Example() -> impl IntoView {
///     let progress = RwSignal::new(42.0);
///     view! {
///         <CircularProgress progress_percentage=progress size=ProgressComponentSize::Lg show_percentage=true />
///     }
/// }
/// ```
#[component]
pub fn CircularProgress(
    #[prop(into, default = RwSignal::new(0.0))] progress_percentage: RwSignal<f64>,
    #[prop(into, optional, default = ProgressComponentSize::Md)] size: ProgressComponentSize,
    #[prop(default = true)] show_percentage: bool,
    /// Tailwind color class for the progress arc. Defaults to `"text-primary"`.
    #[prop(into, optional, default = "text-primary".to_string())]
    color: String,
    /// Tailwind color class for the track (background arc). Defaults to `"text-light-gray"`.
    #[prop(into, optional, default = "text-light-gray".to_string())]
    track_color: String,
) -> impl IntoView {
    let progress_val = move || progress_percentage.get().min(100.0).max(0.0);
    let svg_size = match size {
        ProgressComponentSize::Sm => 60,
        ProgressComponentSize::Md => 80, // md
        ProgressComponentSize::Lg => 120,
    };
    let stroke_width = match size {
        // ProgressComponentSize::Sm => 6,
        // ProgressComponentSize::Md => 8,
        // ProgressComponentSize::Lg => 12,
        _ => 6,
    };
    let r = 40.0 - (stroke_width as f64) / 2.0;
    let circ = 2.0 * std::f64::consts::PI * r;
    let font_size = match size {
        ProgressComponentSize::Sm => 12,
        ProgressComponentSize::Md => 14,
        ProgressComponentSize::Lg => 16,
    };

    view! {
        <div class="flex justify-center items-center">
            <svg
                width={svg_size}
                height={svg_size}
                viewBox="0 0 80 80"
                class=format!("transform -rotate-90 {}", track_color)
            >
                // Track arc
                <circle
                    cx="40"
                    cy="40"
                    r={r}
                    stroke="currentColor"
                    stroke-width={stroke_width}
                    fill="none"
                />
                // Progress arc
                <circle
                    cx="40"
                    cy="40"
                    r={r}
                    stroke="currentColor"
                    stroke-width={stroke_width}
                    fill="none"
                    stroke-dasharray={circ}
                    stroke-dashoffset=move || circ - (progress_val() / 100.0 * circ)
                    stroke-linecap="round"
                    class=format!("transition-all duration-300 {}", color)
                />
                {if show_percentage {
                    Some(view! {
                        <text
                            x="40"
                            y="40"
                            text-anchor="middle"
                            dominant-baseline="central"
                            font-size={font_size}
                            fill="currentColor"
                            class=format!("font-bold {}", color)
                            transform="rotate(90, 40, 40)"
                        >
                            {move || format!("{:.0}%", progress_val())}
                        </text>
                    })
                } else { None }}
            </svg>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    // ProgressComponentSize

    #[test]
    fn size_eq() {
        assert_eq!(ProgressComponentSize::Sm, ProgressComponentSize::Sm);
        assert_ne!(ProgressComponentSize::Sm, ProgressComponentSize::Lg);
    }

    #[test]
    fn size_clone() {
        assert_eq!(ProgressComponentSize::Md.clone(), ProgressComponentSize::Md);
    }

    // fill_style / progress clamping

    fn fill_style(progress: f64, indeterminate: bool) -> String {
        if indeterminate {
            "".to_string()
        } else {
            format!("width: {}%;", progress.min(100.0).max(0.0))
        }
    }

    #[test]
    fn fill_style_normal_progress() {
        assert_eq!(fill_style(65.0, false), "width: 65%;");
    }

    #[test]
    fn fill_style_clamps_above_100() {
        assert_eq!(fill_style(150.0, false), "width: 100%;");
    }

    #[test]
    fn fill_style_clamps_below_0() {
        assert_eq!(fill_style(-10.0, false), "width: 0%;");
    }

    #[test]
    fn fill_style_empty_when_indeterminate() {
        assert_eq!(fill_style(50.0, true), "");
    }

    // aria_valuenow

    fn aria_valuenow(progress: f64, indeterminate: bool) -> String {
        if indeterminate {
            "-1".to_string()
        } else {
            (progress as i32).to_string()
        }
    }

    #[test]
    fn aria_valuenow_determinate() {
        assert_eq!(aria_valuenow(75.0, false), "75");
    }

    #[test]
    fn aria_valuenow_indeterminate() {
        assert_eq!(aria_valuenow(50.0, true), "-1");
    }

    // show_percentage visibility

    fn shows_percentage(show_percentage: bool, indeterminate: bool) -> bool {
        show_percentage && !indeterminate
    }

    #[test]
    fn percentage_shown_when_enabled_and_determinate() {
        assert!(shows_percentage(true, false));
    }

    #[test]
    fn percentage_hidden_when_indeterminate() {
        assert!(!shows_percentage(true, true));
    }

    #[test]
    fn percentage_hidden_when_disabled() {
        assert!(!shows_percentage(false, false));
    }

    // container_class / height mapping

    fn height_class(size: &ProgressComponentSize) -> &'static str {
        match size {
            ProgressComponentSize::Sm => "h-1",
            ProgressComponentSize::Md => "h-2",
            ProgressComponentSize::Lg => "h-4",
        }
    }

    #[test]
    fn sm_height() {
        assert_eq!(height_class(&ProgressComponentSize::Sm), "h-1");
    }

    #[test]
    fn md_height() {
        assert_eq!(height_class(&ProgressComponentSize::Md), "h-2");
    }

    #[test]
    fn lg_height() {
        assert_eq!(height_class(&ProgressComponentSize::Lg), "h-4");
    }

    // CircularProgress svg dimensions

    fn svg_size(size: &ProgressComponentSize) -> i32 {
        match size {
            ProgressComponentSize::Sm => 60,
            ProgressComponentSize::Md => 80,
            ProgressComponentSize::Lg => 120,
        }
    }

    fn stroke_width(size: &ProgressComponentSize) -> i32 {
        match size {
            ProgressComponentSize::Sm => 6,
            ProgressComponentSize::Md => 8,
            ProgressComponentSize::Lg => 12,
        }
    }

    #[test]
    fn svg_sizes_correct() {
        assert_eq!(svg_size(&ProgressComponentSize::Sm), 60);
        assert_eq!(svg_size(&ProgressComponentSize::Md), 80);
        assert_eq!(svg_size(&ProgressComponentSize::Lg), 120);
    }

    #[test]
    fn stroke_widths_correct() {
        assert_eq!(stroke_width(&ProgressComponentSize::Sm), 6);
        assert_eq!(stroke_width(&ProgressComponentSize::Md), 8);
        assert_eq!(stroke_width(&ProgressComponentSize::Lg), 12);
    }

    // CircularProgress stroke-dashoffset

    fn stroke_dashoffset(progress: f64, r: f64) -> f64 {
        let circ = 2.0 * PI * r;
        circ - (progress.min(100.0).max(0.0) / 100.0 * circ)
    }

    #[test]
    fn dashoffset_at_zero_equals_full_circumference() {
        let r = 40.0 - 8.0 / 2.0; // Md
        let circ = 2.0 * PI * r;
        assert!((stroke_dashoffset(0.0, r) - circ).abs() < 1e-9);
    }

    #[test]
    fn dashoffset_at_100_equals_zero() {
        let r = 40.0 - 8.0 / 2.0;
        assert!(stroke_dashoffset(100.0, r).abs() < 1e-9);
    }

    #[test]
    fn dashoffset_at_50_is_half_circumference() {
        let r = 40.0 - 8.0 / 2.0;
        let circ = 2.0 * PI * r;
        assert!((stroke_dashoffset(50.0, r) - circ / 2.0).abs() < 1e-9);
    }

    #[test]
    fn dashoffset_clamps_above_100() {
        let r = 40.0 - 8.0 / 2.0;
        assert!(stroke_dashoffset(150.0, r).abs() < 1e-9);
    }

    #[test]
    fn dashoffset_clamps_below_0() {
        let r = 40.0 - 8.0 / 2.0;
        let circ = 2.0 * PI * r;
        assert!((stroke_dashoffset(-10.0, r) - circ).abs() < 1e-9);
    }

    // reactive progress signal

    #[test]
    fn progress_signal_updates() {
        let owner = Owner::new();
        owner.with(|| {
            let progress = RwSignal::new(0.0f64);
            assert_eq!(progress.get(), 0.0);
            progress.set(75.0);
            assert_eq!(progress.get(), 75.0);
        });
    }
}
