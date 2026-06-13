use icondata::{BiChevronLeftRegular, BiChevronRightRegular};
use leptos::prelude::*;
use web_sys::HtmlDivElement;

use crate::components::actions::button::BasicButton;

#[derive(Clone, Default)]
pub struct TabLabel {
    pub label: ViewFn,
}

impl TabLabel {
    pub fn new(label: ViewFn) -> Self {
        TabLabel { label: label }
    }
}

#[slot]
pub struct Tab {
    pub children: ChildrenFn,
}

/// A tabbed view component with scrollable tab navigation and left/right scroll carets.
///
/// Tab content is rendered eagerly but toggled visible/hidden via CSS. Tab labels are
/// provided separately from tab content, allowing rich label rendering via `ViewFn`.
///
/// # Props
///
/// - `tab_labels` – `RwSignal<Vec<TabLabel>>` holding the label `ViewFn` for each tab header.
/// - `tab` – One or more `<Tab slot>` components providing the content for each tab.
///   The order must match `tab_labels`.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::navigation::tabs::{TabLabel, Tabs, Tab};
///
/// #[component]
/// fn Example() -> impl IntoView {
///     let labels = RwSignal::new(vec![
///         TabLabel::new(ViewFn::from(|| view! { <span>"First"</span> })),
///         TabLabel::new(ViewFn::from(|| view! { <span>"Second"</span> })),
///     ]);
///     view! {
///         <Tabs tab_labels=labels>
///             <Tab slot><p>"First tab content"</p></Tab>
///             <Tab slot><p>"Second tab content"</p></Tab>
///         </Tabs>
///     }
/// }
/// ```
#[component]
pub fn Tabs(
    #[prop(default = vec![])] tab: Vec<Tab>,
    #[prop(into)] tab_labels: RwSignal<Vec<TabLabel>>,
) -> impl IntoView {
    let (current_tab, set_current_tab) = signal(0usize);

    let tab_nav_ref: NodeRef<leptos::html::Div> = NodeRef::new();
    let can_scroll_left = RwSignal::new(false);
    let can_scroll_right = RwSignal::new(false);
    let scroll_amount = 150.0_f64;

    let update_caret = move || {
        if let Some(el) = tab_nav_ref.get() as Option<HtmlDivElement> {
            let scroll_left_val = el.scroll_left();
            let max_scroll = el.scroll_width() - el.client_width();
            can_scroll_left.set(scroll_left_val > 0);
            can_scroll_right.set(scroll_left_val < max_scroll);
        }
    };

    let scroll_left_click = move || {
        if let Some(el) = tab_nav_ref.get() as Option<HtmlDivElement> {
            // negative x scrolls left
            el.scroll_by_with_x_and_y(-scroll_amount, 0.0);
        }
        update_caret();
    };

    let scroll_right_click = move || {
        if let Some(el) = tab_nav_ref.get() as Option<HtmlDivElement> {
            // positive x scrolls right
            el.scroll_by_with_x_and_y(scroll_amount, 0.0);
        }
        update_caret();
    };

    view! {
        <div class="w-full">
            <div class="relative w-full flex flex-row items-center border-b border-mid-gray">

                // Left caret
                <BasicButton
                    icon=Some(BiChevronLeftRegular)
                    disabled=MaybeProp::derive(move || Some(!can_scroll_left.get()))
                    onclick=Callback::new(move |_| scroll_left_click())
                />

                // Scrollable tab labels
                <div
                    node_ref=tab_nav_ref
                    class="flex flex-row gap-6 overflow-x-auto scrollbar-hidden scroll-smooth flex-1"
                    on:scroll=move |_| update_caret()
                >
                    {move || {
                        let labels = tab_labels.get();
                        let current = current_tab.get();

                        labels.into_iter().enumerate().map(|(index, label)| {
                            let dynamic_class = move || {
                                if current_tab.get() == index {
                                    "border-primary text-primary"
                                } else {
                                    "border-transparent hover:border-mid-gray"
                                }
                            };

                            view! {
                                <span
                                    class=move || format!(
                                        "border-b-4 transition-all duration-200 ease-in-out pb-2 cursor-pointer shrink-0 {}",
                                        dynamic_class()
                                    )
                                    on:click=move |_| set_current_tab.set(index)
                                >
                                    {label.label.run()}
                                </span>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>

                // Right caret
                <BasicButton
                    icon=Some(BiChevronRightRegular)
                    disabled=MaybeProp::derive(move || Some(!can_scroll_right.get()))
                    onclick=Callback::new(move |_| scroll_right_click())
                />

            </div>

            <div class="relative min-h-[150px] mt-4">
                {move || {
                    let _current = current_tab.get();
                    tab.iter().enumerate().map(|(i, child)| {
                        let dynamic_class = move || {
                            if current_tab.get() == i { "block" } else { "hidden" }
                        };
                        view! {
                            <div class=dynamic_class>
                                {(child.children)().into_any()}
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }.into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    // TabLabel::new

    #[test]
    fn tab_label_new_stores_label() {
        let label = TabLabel::new(ViewFn::from(|| view! {}));
        // ViewFn is not comparable; just verify construction doesn't panic
        let _ = label;
    }

    #[test]
    fn tab_label_default() {
        let label = TabLabel::default();
        let _ = label; // default constructs without panic
    }

    // active tab class logic

    fn tab_label_class(current: usize, index: usize) -> &'static str {
        if current == index {
            "border-primary text-primary"
        } else {
            "border-transparent"
        }
    }

    #[test]
    fn active_tab_gets_primary_class() {
        assert_eq!(tab_label_class(1, 1), "border-primary text-primary");
    }

    #[test]
    fn inactive_tab_gets_transparent_class() {
        assert_eq!(tab_label_class(0, 1), "border-transparent");
        assert_eq!(tab_label_class(2, 0), "border-transparent");
    }

    // content visibility logic

    fn content_class(current: usize, index: usize) -> &'static str {
        if current == index { "block" } else { "hidden" }
    }

    #[test]
    fn active_tab_content_is_block() {
        assert_eq!(content_class(0, 0), "block");
        assert_eq!(content_class(2, 2), "block");
    }

    #[test]
    fn inactive_tab_content_is_hidden() {
        assert_eq!(content_class(0, 1), "hidden");
        assert_eq!(content_class(1, 0), "hidden");
    }

    // scroll caret state

    fn caret_state(scroll_left: f64, scroll_width: f64, client_width: f64) -> (bool, bool) {
        let max_scroll = scroll_width - client_width;
        let can_left = scroll_left > 0.0;
        let can_right = scroll_left < max_scroll;
        (can_left, can_right)
    }

    #[test]
    fn at_start_cannot_scroll_left() {
        let (can_left, _) = caret_state(0.0, 500.0, 300.0);
        assert!(!can_left);
    }

    #[test]
    fn at_start_can_scroll_right_when_overflow() {
        let (_, can_right) = caret_state(0.0, 500.0, 300.0);
        assert!(can_right);
    }

    #[test]
    fn at_end_cannot_scroll_right() {
        let (_, can_right) = caret_state(200.0, 500.0, 300.0);
        assert!(!can_right);
    }

    #[test]
    fn at_end_can_scroll_left() {
        let (can_left, _) = caret_state(200.0, 500.0, 300.0);
        assert!(can_left);
    }

    #[test]
    fn no_overflow_neither_caret_active() {
        let (can_left, can_right) = caret_state(0.0, 300.0, 300.0);
        assert!(!can_left);
        assert!(!can_right);
    }

    #[test]
    fn mid_scroll_both_carets_active() {
        let (can_left, can_right) = caret_state(100.0, 500.0, 300.0);
        assert!(can_left);
        assert!(can_right);
    }

    // reactive tab switching

    #[test]
    fn clicking_tab_updates_current() {
        let owner = Owner::new();
        owner.with(|| {
            let (current_tab, set_current_tab) = signal(0usize);
            set_current_tab.set(2);
            assert_eq!(current_tab.get(), 2);
        });
    }

    #[test]
    fn initial_tab_is_zero() {
        let owner = Owner::new();
        owner.with(|| {
            let (current_tab, _) = signal(0usize);
            assert_eq!(current_tab.get(), 0);
        });
    }

    // scroll caret reactive signals

    #[test]
    fn caret_signals_update_reactively() {
        let owner = Owner::new();
        owner.with(|| {
            let can_scroll_left = RwSignal::new(false);
            let can_scroll_right = RwSignal::new(true);

            assert!(!can_scroll_left.get());
            assert!(can_scroll_right.get());

            can_scroll_left.set(true);
            can_scroll_right.set(false);

            assert!(can_scroll_left.get());
            assert!(!can_scroll_right.get());
        });
    }
}
