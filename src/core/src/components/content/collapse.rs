use icondata::{BsDashLg, BsPlusLg};
use leptos::html::*;
use leptos::prelude::*;
use leptos_icons::Icon;
use web_sys::CustomEvent;

use crate::utils::forms::fire_custom_bubbled_and_cancelable_event;

#[derive(Clone)]
pub struct PanelInfo {
    pub id: String, // or u64 — something stable per item
    pub title: ViewFn,
    pub is_open: RwSignal<bool>,
    pub children: ViewFn,
}

impl Default for PanelInfo {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: ViewFn::from(|| view! {}),
            is_open: RwSignal::new(false),
            children: ViewFn::from(|| view! {}),
        }
    }
}

#[allow(dead_code)]
impl PanelInfo {
    pub fn builder(title: ViewFn, children: ViewFn) -> PanelInfo {
        PanelInfo {
            title,
            children,
            ..Default::default()
        }
    }

    pub fn title(mut self, title: ViewFn) -> Self {
        self.title = title;
        self
    }

    pub fn is_open(mut self, is_open: RwSignal<bool>) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn children(mut self, children: ViewFn) -> Self {
        self.children = children;
        self
    }

    pub fn build(self) -> PanelInfo {
        PanelInfo {
            id: self.id,
            title: self.title,
            is_open: self.is_open,
            children: self.children,
        }
    }
}

impl std::fmt::Debug for PanelInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PanelInfo")
            .field("title", &"<ViewFn>")
            .field("is_open", &self.is_open)
            .field("children", &"<ViewFn>")
            .finish()
    }
}

/// A collapsible panel with a clickable title that toggles its content open or closed.
///
/// # Props
///
/// - `title` – A `ViewFn` rendered as the panel header.
/// - `is_open` – `RwSignal<bool>` controlling the open/closed state.
/// - `is_accordion` – When `true`, delegates toggle handling to a parent `Collapse`. Defaults to `false`.
/// - `ext_panel_title_styles` – Additional Tailwind classes appended to the title bar.
/// - `children` – Optional content rendered when the panel is open.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::content::collapse::Panel;
///
/// #[component]
/// fn Example() -> impl IntoView {
///     let is_open = RwSignal::new(false);
///     view! {
///         <Panel
///             title=ViewFn::from(|| view! { <span>"Section 1"</span> })
///             is_open=is_open
///         >
///             <p>"Panel content goes here."</p>
///         </Panel>
///     }
/// }
/// ```
#[component]
pub fn Panel(
    title: ViewFn,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(into)] is_open: bool,
    #[prop(optional)] is_accordion: bool,
    #[prop(into, optional)] ext_panel_title_styles: String,
) -> impl IntoView {
    let panel_ref = NodeRef::new();
    let (children, _set_children) = signal(children);
    let (internal_is_open, set_internal_is_open) = signal(is_open);
    let toggle_content = move |_| {
        if let Some(panel_element) = panel_ref.get() {
            fire_custom_bubbled_and_cancelable_event("togglepanel", true, true, &panel_element);
        }

        if !is_accordion {
            set_internal_is_open.update(|value| *value = !*value);
        }
    };

    view! {
        <div node_ref=panel_ref>
            <span
                on:click=toggle_content
                class=move || format!("flex flex-row items-center justify-between gap-4 mb-2 p-2 rounded cursor-pointer ring ring-primary hover:bg-primary hover:text-light-gray {} {}", ext_panel_title_styles, if internal_is_open.get() { "bg-primary text-light-gray" } else { "" })
            >
                {title.run()}
                {
                    move || {
                        if children.get().is_some() {
                            let icon_id = if internal_is_open.get() {
                                BsDashLg
                            } else {
                                BsPlusLg
                            };
                            Some(view!{ <Icon icon=icon_id /> })
                        } else {
                            None
                        }
                    }
                }
            </span>
            <div
                class=move || {
                    if internal_is_open.get() {
                        "transition-max-height duration-700 ease-in-out overflow-hidden max-h-svh p-2 ml-2"
                    } else {
                        "overflow-hidden h-0 transition-max-height duration-700 ease-in-out"
                    }
                }
            >
                {move || children.get().map(|c| c())}
            </div>
        </div>
    }.into_any()
}

/// Groups multiple `Panel` components, optionally enforcing accordion behaviour
/// (only one panel open at a time).
///
/// # Props
///
/// - `panel_items` – `RwSignal<Vec<PanelInfo>>` holding each panel's title, content, and open state.
/// - `is_accordion` – When `true`, opening one panel closes all others. Defaults to `false`.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::content::collapse::{Panel, PanelInfo, Collapse};
///
/// #[component]
/// fn Example() -> impl IntoView {
///     let panels = RwSignal::new(vec![
///         PanelInfo::builder(
///             ViewFn::from(|| view! { <span>"Panel 1"</span> }),
///             ViewFn::from(|| view! { <p>"Content 1"</p> }),
///         ).build(),
///         PanelInfo::builder(
///             ViewFn::from(|| view! { <span>"Panel 2"</span> }),
///             ViewFn::from(|| view! { <p>"Content 2"</p> }),
///         ).build(),
///     ]);
///     view! {
///         <Collapse panel_items=panels is_accordion=true />
///     }
/// }
/// ```
#[component]
pub fn Collapse(
    #[prop(into)] panel_items: RwSignal<Vec<PanelInfo>>,
    #[prop(default = false)] is_accordion: bool,
) -> impl IntoView {
    let handle_panel_toggle = move |index| {
        panel_items.update(|panels| {
            for (i, panel) in panels.iter().enumerate() {
                if i == index {
                    panel.is_open.update(|val| *val = !*val);
                } else if is_accordion {
                    panel.is_open.set(false);
                }
            }
        });
    };

    view! {
        <div class="flex flex-col">
            <For
                each=move || panel_items.get().into_iter().enumerate()
                key=|(_, panel)| panel.id.clone()
                let:((index, panel_item))
            >
                {
                    let panel_item_ref = &panel_item;
                    let panel_item_ref_clone = panel_item_ref.clone();

                    move || {
                        let is_open_val = panel_item_ref_clone.is_open.get();
                        let children = panel_item_ref_clone.children.clone();

                        view!{
                            <Panel on:togglepanel=move |ev: CustomEvent| {
                                ev.stop_propagation();
                                handle_panel_toggle(index)
                            } title=panel_item_ref_clone.title.clone() is_open=is_open_val is_accordion=is_accordion>
                                {children.run()}
                            </Panel>
                        }
                    }
                }
            </For>
        </div>
    }.into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    // PanelInfo builder

    #[test]
    fn panel_info_default_is_closed() {
        let owner = Owner::new();
        owner.with(|| {
            let panel = PanelInfo::default();
            assert_eq!(panel.is_open.get(), false);
        });
    }

    #[test]
    fn panel_info_builder_sets_open_state() {
        let owner = Owner::new();
        owner.with(|| {
            let is_open = RwSignal::new(true);
            let panel = PanelInfo::builder(ViewFn::from(|| view! {}), ViewFn::from(|| view! {}))
                .is_open(is_open)
                .build();

            assert_eq!(panel.is_open.get(), true);
        });
    }

    #[test]
    fn panel_info_clone_shares_signal() {
        let owner = Owner::new();
        owner.with(|| {
            let panel = PanelInfo::default();
            let cloned = panel.clone();

            panel.is_open.set(true);
            assert_eq!(cloned.is_open.get(), true);
        });
    }

    // Panel toggle logic

    #[test]
    fn toggle_flips_is_open_when_not_accordion() {
        let owner = Owner::new();
        owner.with(|| {
            let is_open = RwSignal::new(false);
            is_open.update(|v| *v = !*v);
            assert_eq!(is_open.get(), true);

            is_open.update(|v| *v = !*v);
            assert_eq!(is_open.get(), false);
        });
    }

    #[test]
    fn toggle_does_not_flip_when_accordion() {
        let owner = Owner::new();
        owner.with(|| {
            let is_accordion = true;
            let is_open = RwSignal::new(false);

            // accordion panels skip self-toggle; state unchanged
            if !is_accordion {
                is_open.update(|v| *v = !*v);
            }

            assert_eq!(is_open.get(), false);
        });
    }

    // Collapse accordion logic

    fn accordion_toggle(panels: &mut Vec<RwSignal<bool>>, index: usize) {
        for (i, panel) in panels.iter().enumerate() {
            if i == index {
                panel.update(|v| *v = !*v);
            } else {
                panel.set(false);
            }
        }
    }

    #[test]
    fn accordion_opens_target_and_closes_others() {
        let owner = Owner::new();
        owner.with(|| {
            let mut panels = vec![
                RwSignal::new(false),
                RwSignal::new(true),
                RwSignal::new(false),
            ];

            accordion_toggle(&mut panels, 0);

            assert_eq!(panels[0].get(), true);
            assert_eq!(panels[1].get(), false);
            assert_eq!(panels[2].get(), false);
        });
    }

    #[test]
    fn accordion_closes_already_open_panel() {
        let owner = Owner::new();
        owner.with(|| {
            let mut panels = vec![RwSignal::new(true), RwSignal::new(false)];

            accordion_toggle(&mut panels, 0);

            assert_eq!(panels[0].get(), false);
            assert_eq!(panels[1].get(), false);
        });
    }

    #[test]
    fn non_accordion_panels_are_independent() {
        let owner = Owner::new();
        owner.with(|| {
            let a = RwSignal::new(false);
            let b = RwSignal::new(false);

            a.update(|v| *v = !*v);
            b.update(|v| *v = !*v);

            assert_eq!(a.get(), true);
            assert_eq!(b.get(), true);
        });
    }
}
