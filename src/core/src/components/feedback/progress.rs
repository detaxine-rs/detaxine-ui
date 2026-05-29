use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum ProgressComponentSize {
    Sm,
    Md,
    Lg,
}

/// A reusable progress bar component styled with Tailwind CSS.
/// The progress prop should be a RwSignal<f64> where 0.0 is 0% and 1.0 is 100%.
/// For indeterminate progress, set indeterminate=true to show an animated sliding bar.
/// Requires the 'animate-progress-indeterminate' class defined in your CSS.
///
/// Example usage:
/// ```
/// <ProgressBar
///     indeterminate=true
///     color="bg-primary"
///     size="md"
/// />
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
            format!("w-full bg-light-gray rounded-full {} relative", height)
        } else {
            format!("w-full bg-light-gray rounded-full {}", height)
        }
    };

    let fill_class = move || {
        if indeterminate {
            format!(
                "{} rounded-full animate-progress-indeterminate absolute",
                color
            )
        } else {
            format!("{} rounded-full transition-all duration-300", color)
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

/// A circular progress indicator component.
/// Displays progress as a circular ring with optional center text.
/// The progress_percentage prop should be a RwSignal<f64> representing 0.0 to 100.0%.
///
/// Example usage:
/// ```
/// <CircularProgress
///     progress_percentage=progress_signal
///     size="lg"
///     show_percentage=true
/// />
/// ```
#[component]
pub fn CircularProgress(
    #[prop(into, default = RwSignal::new(0.0))] progress_percentage: RwSignal<f64>,
    #[prop(into, optional, default = ProgressComponentSize::Md)] size: ProgressComponentSize,
    #[prop(default = true)] show_percentage: bool,
) -> impl IntoView {
    let progress_val = move || progress_percentage.get().min(100.0).max(0.0);
    let svg_size = match size {
        ProgressComponentSize::Sm => 60,
        ProgressComponentSize::Md => 80, // md
        ProgressComponentSize::Lg => 120,
    };
    let stroke_width = match size {
        ProgressComponentSize::Sm => 6,
        ProgressComponentSize::Md => 8,
        ProgressComponentSize::Lg => 12,
    };
    let r = 40.0 - (stroke_width as f64) / 2.0;
    let circ = 2.0 * std::f64::consts::PI * r;
    let font_size = match size {
        ProgressComponentSize::Sm => 12,
        ProgressComponentSize::Md => 16,
        ProgressComponentSize::Lg => 24,
    };

    view! {
        <div class="flex justify-center items-center">
            <svg width={svg_size} height={svg_size} viewBox="0 0 80 80" class="transform -rotate-90">
                <circle
                    cx="40"
                    cy="40"
                    r={r}
                    stroke="#e0e0e0"
                    stroke-width={stroke_width}
                    fill="none"
                />
                <circle
                    cx="40"
                    cy="40"
                    r={r}
                    stroke="#3b82f6"
                    stroke-width={stroke_width}
                    fill="none"
                    stroke-dasharray={circ}
                    stroke-dashoffset=move || circ - (progress_val() / 100.0 * circ)
                    stroke-linecap="round"
                    class="transition-all duration-300"
                />
                {if show_percentage {
                    Some(view! {
                        <text
                            x="40"
                            y="45"
                            text-anchor="middle"
                            font-size={font_size}
                            fill="#333"
                            class="font-bold"
                        >
                            {move || format!("{:.0}%", progress_val())}
                        </text>
                    })
                } else { None }}
            </svg>
        </div>
    }
}
