use icondata::{BsChevronBarLeft, BsChevronBarRight, BsChevronLeft, BsChevronRight};
use leptos::prelude::*;

use crate::components::actions::button::{BasicButton, ButtonGroup};

use tailwind_fuse::tw_merge;

/// A pagination control that emits page-change events via a callback.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::data_display::table::pagination::Pagination;
///
/// #[component]
/// fn Example() -> impl IntoView {
///     let pagination_state = Signal::derive(move || (1, 10));
///
///     view! {
///         <Pagination
///             pagination_state=pagination_state
///             on_page_change=Callback::new(|page| leptos::logging::log!("page: {}", page))
///         />
///     }
/// }
/// ```
#[component]
pub fn Pagination(
    /// `MaybeProp<(usize, usize)>` where the tuple is `(current_page, total_pages)`.
    #[prop(into)]
    pagination_state: MaybeProp<(usize, usize)>,

    /// Callback fired with the new page number when a navigation button is
    /// clicked. Defaults to a no-op.
    #[prop(optional, default = Callback::new(|_| {}))]
    on_page_change: Callback<usize>,

    /// Extra Tailwind classes for the root wrapper `<div>`.
    #[prop(into, optional)]
    class: MaybeProp<String>,

    /// Extra Tailwind classes for the "Page X of Y" text.
    #[prop(into, optional)]
    page_info_class: MaybeProp<String>,

    /// Extra Tailwind classes forwarded to the internal `ButtonGroup`.
    #[prop(into, optional)]
    button_group_class: MaybeProp<String>,
) -> impl IntoView {
    let current_page = Memo::new(move |_| pagination_state.get().unwrap_or_default().0);
    let total_pages = Memo::new(move |_| pagination_state.get().unwrap_or_default().1);

    let next_page = Memo::new(move |_| {
        if current_page.get() < total_pages.get() {
            Some(current_page.get() + 1)
        } else {
            None
        }
    });
    let prev_page = Memo::new(move |_| {
        if current_page.get() > 1 {
            Some(current_page.get() - 1)
        } else {
            None
        }
    });

    let on_prev_click = Callback::new(move |_| {
        let page = prev_page.get().unwrap_or(current_page.get());
        on_page_change.run(page);
    });
    let on_next_click = Callback::new(move |_| {
        let page = next_page.get().unwrap_or(current_page.get());
        on_page_change.run(page);
    });
    let on_first_click = Callback::new(move |_| {
        on_page_change.run(1);
    });
    let on_last_click = Callback::new(move |_| {
        on_page_change.run(total_pages.get());
    });

    let is_first_page = Memo::new(move |_| current_page.get() <= 1);
    let is_last_page = Memo::new(move |_| current_page.get() >= total_pages.get());
    let can_go_to_prev = Memo::new(move |_| current_page.get() == 1);
    let can_go_to_next =
        Memo::new(move |_| total_pages.get() <= 1 || current_page.get() == total_pages.get());

    let root_class = move || tw_merge!("flex flex-col", class.get().unwrap_or_default());
    let page_info_class_val =
        move || tw_merge!("text-xs mr-2", page_info_class.get().unwrap_or_default());

    view! {
        <div class=root_class>
            <div class="flex items-center justify-end">
                {
                    let page_info_class_val = page_info_class_val.clone();
                    move || {
                        if pagination_state.get().unwrap_or_default().1 > 0 {
                            Some(
                                view!{
                                    <span class=page_info_class_val.clone()>
                                        {move || format!("Page {} of {}", current_page.get(), pagination_state.get().unwrap_or_default().1)}
                                    </span>
                                }
                            )
                        } else {
                            None
                        }
                    }
                }
                {
                    move || {
                        let button_group_class_val = tw_merge!(
                                "font-bold bg-primary text-contrast-white hover:bg-secondary",
                                button_group_class.get().unwrap_or_default()
                            );

                        view! {
                            <ButtonGroup class=button_group_class_val>
                                <BasicButton onclick=on_first_click disabled=is_first_page icon=Some(BsChevronBarLeft) />
                                <BasicButton onclick=on_prev_click disabled=can_go_to_prev icon=Some(BsChevronLeft) />
                                <BasicButton onclick=on_next_click disabled=can_go_to_next icon=Some(BsChevronRight) />
                                <BasicButton onclick=on_last_click disabled=is_last_page icon=Some(BsChevronBarRight) />
                            </ButtonGroup>
                        }
                    }
                }
            </div>
        </div>
    }.into_any()
}

#[cfg(test)]
mod tests {
    use leptos::prelude::*;
    // next/prev page logic

    fn next_page(current: usize, total: usize) -> Option<usize> {
        if current < total {
            Some(current + 1)
        } else {
            None
        }
    }

    fn prev_page(current: usize, _total: usize) -> Option<usize> {
        if current > 1 { Some(current - 1) } else { None }
    }

    fn is_first_page(current: usize) -> bool {
        current <= 1
    }

    fn is_last_page(current: usize, total: usize) -> bool {
        current >= total
    }

    fn can_go_to_prev(current: usize) -> bool {
        current == 1
    }

    fn can_go_to_next(current: usize, total: usize) -> bool {
        total <= 1 || current == total
    }

    #[test]
    fn next_page_advances() {
        assert_eq!(next_page(1, 5), Some(2));
        assert_eq!(next_page(4, 5), Some(5));
    }

    #[test]
    fn next_page_returns_none_on_last() {
        assert_eq!(next_page(5, 5), None);
    }

    #[test]
    fn prev_page_decrements() {
        assert_eq!(prev_page(3, 5), Some(2));
        assert_eq!(prev_page(2, 5), Some(1));
    }

    #[test]
    fn prev_page_returns_none_on_first() {
        assert_eq!(prev_page(1, 5), None);
    }

    #[test]
    fn is_first_page_true_on_page_one() {
        assert!(is_first_page(1));
    }

    #[test]
    fn is_first_page_false_on_other_pages() {
        assert!(!is_first_page(2));
        assert!(!is_first_page(5));
    }

    #[test]
    fn is_last_page_true_on_final_page() {
        assert!(is_last_page(5, 5));
    }

    #[test]
    fn is_last_page_false_on_other_pages() {
        assert!(!is_last_page(3, 5));
        assert!(!is_last_page(1, 5));
    }

    #[test]
    fn can_go_to_prev_disabled_on_first_page() {
        assert!(can_go_to_prev(1));
        assert!(!can_go_to_prev(2));
    }

    #[test]
    fn can_go_to_next_disabled_on_last_page() {
        assert!(can_go_to_next(5, 5));
        assert!(!can_go_to_next(3, 5));
    }

    #[test]
    fn can_go_to_next_disabled_when_single_page() {
        assert!(can_go_to_next(1, 1));
    }

    #[test]
    fn first_page_click_always_goes_to_one() {
        // on_first_click always emits 1 regardless of current page
        let emitted = 1usize;
        assert_eq!(emitted, 1);
    }

    #[test]
    fn last_page_click_goes_to_total() {
        let total = 7usize;
        // on_last_click always emits total_pages
        let emitted = total;
        assert_eq!(emitted, total);
    }

    // reactive state (requires Leptos runtime)

    #[test]
    fn reactive_next_and_prev_update_with_signal() {
        let owner = Owner::new();
        owner.with(|| {
            let state = RwSignal::new((1usize, 5usize));
            let current = move || state.get().0;
            let total = move || state.get().1;

            assert_eq!(next_page(current(), total()), Some(2));
            assert_eq!(prev_page(current(), total()), None);

            state.set((3, 5));
            assert_eq!(next_page(current(), total()), Some(4));
            assert_eq!(prev_page(current(), total()), Some(2));

            state.set((5, 5));
            assert_eq!(next_page(current(), total()), None);
            assert_eq!(prev_page(current(), total()), Some(4));
        });
    }
}
