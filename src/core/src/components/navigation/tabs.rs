use icondata::{BiChevronLeftRegular, BiChevronRightRegular};
use leptos::prelude::*;
use leptos_icons::Icon;
use web_sys::HtmlDivElement;

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

/// This component renders a set of tabs that can be used to switch between different views.
///
/// Tab is a Typed component that represents a single tab within the Tabs component. Do not forget to add the `slot` attribute to the Tab component.
///
/// Example usage:
/// ```
/// <Tabs tab_labels=RwSignal::new(vec![TabLabel::new(ViewFn::from(|| view!{ <p>"First"</p> })), TabLabel::new(ViewFn::from(|| view!{ <p>"Second"</p> }))])>
///     <Tab slot>
///         <p>"First tab"</p>
///     </Tab>
///     <Tab slot>
///         <p>"Second tab"</p>
///     </Tab>
/// </Tabs>
/// ```
#[component]
pub fn Tabs(
    #[prop(default=vec![])] tab: Vec<Tab>,
    #[prop(into)] tab_labels: RwSignal<Vec<TabLabel>>,
) -> impl IntoView {
    let (current_tab, set_current_tab) = signal(0);

    // Refs and scroll signals
    let tab_nav_ref = NodeRef::new();
    let can_scroll_left = RwSignal::new(false);
    let can_scroll_right = RwSignal::new(false);
    let scroll_amount = 150.0; // px per caret click

    // Update caret state
    let update_caret = {
        let tab_nav_ref = tab_nav_ref.clone();
        let can_scroll_left = can_scroll_left.clone();
        let can_scroll_right = can_scroll_right.clone();
        move || {
            if let Some(el) = tab_nav_ref.get() as Option<HtmlDivElement> {
                let scroll_left_val = el.scroll_left();
                let max_scroll = el.scroll_width() - el.client_width();
                can_scroll_left.set(scroll_left_val as f64 > 0.0);
                can_scroll_right.set(scroll_left_val < max_scroll);
            }
        }
    };

    // Scroll helpers
    let scroll_left_click = {
        let tab_nav_ref = tab_nav_ref.clone();
        let update_caret = update_caret.clone();
        move || {
            if let Some(el) = tab_nav_ref.get() as Option<HtmlDivElement> {
                el.scroll_by_with_x_and_y(0.0, scroll_amount);
                // el.scroll_by_with_scroll_to_options(
                //     ScrollToOptions::new().left(el.scroll_left() - scroll_amount),
                // );
            }
            update_caret();
        }
    };

    let scroll_right_click = {
        let tab_nav_ref = tab_nav_ref.clone();
        let update_caret = update_caret.clone();
        move || {
            if let Some(el) = tab_nav_ref.get() as Option<HtmlDivElement> {
                el.scroll_by_with_x_and_y(0.0, scroll_amount);
                // el.scroll_by_with_scroll_to_options(
                //     ScrollToOptions::new().left(el.scroll_left() + scroll_amount),
                // );
            }
            update_caret();
        }
    };

    view! {
        <div class="w-full">
            <div class="relative w-full flex flex-row justify-between items-center border-b border-mid-gray">
                // Left caret
                <button
                    class="dark:bg-gradient-to-r dark:from-navy dark:to-transparent z-10 cursor-pointer"
                    on:click=move |_| scroll_left_click()
                    disabled=move || !can_scroll_left.get()
                >
                    <Icon width="2rem" height="2rem" icon=BiChevronLeftRegular />
                </button>

                // Scrollable container
                <div
                    class="flex flex-row gap-6 overflow-y-auto scrollbar-hidden scroll-smooth"
                    node_ref=tab_nav_ref
                >
                    { /* your tab labels here */ }
                    {move || {
                        let labels = tab_labels.get();
                        let current = current_tab.get();

                        labels.iter().enumerate().map(|(index, label)| {
                            let dynamic_class = move || {
                                if current == index {
                                    "border-primary text-primary"
                                } else {
                                    "border-transparent"
                                }
                            };

                            view! {
                                <div class="flex items-center gap-2 cursor-pointer transition-all duration-300 ease-in-out">
                                    <span
                                        class=format!("border-b-4 transition-all duration-200 ease-in-out pb-1 {}", dynamic_class())
                                        on:click=move |_| set_current_tab.set(index)
                                    >
                                        {label.label.run()}
                                    </span>
                                </div>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>

                // Right caret
                <button
                    class="dark:bg-gradient-to-l dark:from-navy dark:to-transparent z-10 cursor-pointer"
                    on:click=move |_| scroll_right_click()
                    disabled=move || !can_scroll_right.get()
                >
                    <Icon width="2rem" height="2rem" icon=BiChevronRightRegular />
                </button>
            </div>


            <div class="relative min-h-[150px] mt-4"> // Ensure container has min-height
                {
                    move || {
                        let current = current_tab.get();

                        tab
                            .iter()
                            .enumerate()
                            .map(|(i, child)| {
                                let dynamic_class = move || if current == i { "block" } else { "hidden" };

                                view! {
                                    <div class=dynamic_class()>
                                        { (child.children)().into_any() }
                                    </div>
                                }
                            }).collect_view()
                    }
                }
            </div>
        </div>
    }
}
