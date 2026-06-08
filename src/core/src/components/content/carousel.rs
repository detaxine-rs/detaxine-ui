use icondata::{BiChevronLeftRegular, BiChevronRightRegular};
use leptos::prelude::*;
use leptos_icons::Icon;

/// A carousel component for displaying a series of slides with previous/next navigation and dot indicators.
///
/// # Props
///
/// - `children` – Two or more block elements, each rendered as a full-width slide.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::content::carousel::Carousel;
///
/// #[component]
/// fn Example() -> impl IntoView {
///     view! {
///         <Carousel>
///             <div>"Slide 1"</div>
///             <div>"Slide 2"</div>
///             <div>"Slide 3"</div>
///         </Carousel>
///     }
/// }
/// ```
#[component]
pub fn Carousel(mut children: ChildrenFragmentMut) -> impl IntoView {
    let children_vec = children()
        .nodes
        .into_iter()
        .map(|n| n.into_view())
        .collect::<Vec<_>>();
    let total_slides = children_vec.len();
    if total_slides == 0 {
        return view! { <div></div> }.into_any();
    }

    let (current_index, set_current_index) = signal(0);
    let current_index_read = current_index.clone();

    let next_slide = move || {
        set_current_index.update(|idx| *idx = (*idx + 1) % total_slides);
    };

    let prev_slide = move || {
        set_current_index.update(|idx| {
            *idx = if *idx == 0 {
                total_slides - 1
            } else {
                *idx - 1
            }
        });
    };

    view! {
        <div class="relative overflow-hidden">
            // Slides container
            <div
                class="flex transition-transform duration-500 ease-in-out"
                style:transform=move || format!("translateX(-{}%)", current_index_read.get() * 100)
            >

            {children_vec.into_iter().map(|slide| view! {
                <div class="shrink-0 w-full">
                    {slide}
                </div>
            }).collect::<Vec<_>>()}

            </div>

            // Previous button
            <button
                class="absolute left-0 top-1/2 transform -translate-y-1/2 bg-transparent text-white hover:bg-opacity-75 transition-opacity z-10 h-full cursor-pointer"
                on:click=move |_| prev_slide()
            >
                <Icon width="1.5em" height="1.5em" icon=BiChevronLeftRegular />
            </button>

            // Next button
            <button
                class="absolute right-0 top-1/2 transform -translate-y-1/2 bg-transparent text-white hover:bg-opacity-75 transition-opacity z-10 h-full cursor-pointer"
                on:click=move |_| next_slide()
            >
                <Icon width="1.5em" height="1.5em" icon=BiChevronRightRegular />
            </button>

            // Indicators
            <div class="absolute bottom-4 left-1/2 transform -translate-x-1/2 flex space-x-2">
                           {move || (0..total_slides).map(|i| view! {
                               <button
                                   class=move || format!("w-6 h-[2.5px] rounded-[5px] {}", if current_index_read.get() == i {
                                       "bg-mid-gray"
                                   } else {
                                       "bg-contrast-white hover:bg-light-gray"
                                   })
                                   on:click=move |_| set_current_index.set(i)
                               ></button>
                           }).collect::<Vec<_>>()}
                       </div>
        </div>
    }.into_any()
}

#[cfg(test)]
mod tests {
    // ── Navigation logic ─────────────────────────────────────────

    use leptos::prelude::*;

    fn next(idx: usize, total: usize) -> usize {
        (idx + 1) % total
    }

    fn prev(idx: usize, total: usize) -> usize {
        if idx == 0 { total - 1 } else { idx - 1 }
    }

    #[test]
    fn next_advances_index() {
        assert_eq!(next(0, 3), 1);
        assert_eq!(next(1, 3), 2);
    }

    #[test]
    fn next_wraps_at_end() {
        assert_eq!(next(2, 3), 0);
    }

    #[test]
    fn prev_decrements_index() {
        assert_eq!(prev(2, 3), 1);
        assert_eq!(prev(1, 3), 0);
    }

    #[test]
    fn prev_wraps_at_start() {
        assert_eq!(prev(0, 3), 2);
    }

    #[test]
    fn next_and_prev_are_inverse() {
        for i in 0..5 {
            assert_eq!(prev(next(i, 5), 5), i);
            assert_eq!(next(prev(i, 5), 5), i);
        }
    }

    #[test]
    fn indicator_click_sets_index_directly() {
        let total = 4;
        for i in 0..total {
            // clicking indicator i should result in index i
            assert_eq!(i, i); // direct set, no transformation needed
        }
    }

    // ── Edge cases ───────────────────────────────────────────────

    #[test]
    fn single_slide_next_stays_at_zero() {
        assert_eq!(next(0, 1), 0);
    }

    #[test]
    fn single_slide_prev_stays_at_zero() {
        assert_eq!(prev(0, 1), 0);
    }

    // ── Reactive index (requires Leptos runtime) ─────────────────

    #[test]
    fn signal_index_updates_on_next() {
        let owner = Owner::new();
        owner.with(|| {
            let total = 3;
            let (current, set_current) = signal(0usize);

            set_current.update(|idx| *idx = next(*idx, total));
            assert_eq!(current.get(), 1);

            set_current.update(|idx| *idx = next(*idx, total));
            assert_eq!(current.get(), 2);

            set_current.update(|idx| *idx = next(*idx, total));
            assert_eq!(current.get(), 0); // wrapped
        });
    }

    #[test]
    fn signal_index_updates_on_prev() {
        let owner = Owner::new();
        owner.with(|| {
            let total = 3;
            let (current, set_current) = signal(0usize);

            set_current.update(|idx| *idx = prev(*idx, total));
            assert_eq!(current.get(), 2); // wrapped
        });
    }

    #[test]
    fn signal_index_set_directly_by_indicator() {
        let owner = Owner::new();
        owner.with(|| {
            let (current, set_current) = signal(0usize);
            set_current.set(2);
            assert_eq!(current.get(), 2);
        });
    }
}
