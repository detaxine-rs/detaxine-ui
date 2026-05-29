use icondata::{BsChevronBarLeft, BsChevronBarRight, BsChevronLeft, BsChevronRight};
use leptos::prelude::*;

use crate::components::actions::button::{BasicButton, ButtonGroup};

/// This is a pagination component which emits an `on_page_change` event using a callback
#[component]
pub fn Pagination(
    /// pagination_state: (current_page, total_pages)
    #[prop(into)]
    pagination_state: Signal<(usize, usize)>,
    #[prop(optional, default = Callback::new(|_| {}))] on_page_change: Callback<usize>,
) -> impl IntoView {
    let current_page = Memo::new(move |_| pagination_state.get().0);
    let total_pages = Memo::new(move |_| pagination_state.get().1);

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

    view! {
        <div class="flex flex-col">
            <div class="flex items-center justify-end">
                {
                    move || {
                        if pagination_state.get().1 > 0 {
                            Some(
                                view!{
                                    <span class="text-xs mr-2">
                                        {move || format!("Page {} of {}", current_page.get(), pagination_state.get().1)}
                                    </span>
                                }
                            )
                        } else {
                            None
                        }
                    }
                }
                <ButtonGroup style_ext="font-bold bg-primary text-contrast-white hover:bg-secondary".to_string()>
                    <BasicButton onclick=on_first_click disabled=is_first_page icon=Some(BsChevronBarLeft) />
                    <BasicButton onclick=on_prev_click disabled=can_go_to_prev icon=Some(BsChevronLeft) />
                    <BasicButton onclick=on_next_click disabled=can_go_to_next icon=Some(BsChevronRight) />
                    <BasicButton onclick=on_last_click disabled=is_last_page icon=Some(BsChevronBarRight) />
                </ButtonGroup>
            </div>
        </div>
    }
}
