use icondata::{BsDashLg, BsPlusLg};
use leptos::html::*;
use leptos::prelude::*;
use leptos_icons::Icon;
use web_sys::CustomEvent;

use crate::utils::forms::fire_custom_bubbled_and_cancelable_event;

#[derive(Clone)]
pub struct PanelInfo {
    pub title: ViewFn,
    pub is_open: RwSignal<bool>,
    pub children: ViewFn,
}

impl Default for PanelInfo {
    fn default() -> Self {
        Self {
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

/// This is a component for displaying a panel with a title and content.
/// Example usage:
/// ```
/// // This example uses a single panel
/// <Panel is_open=panel_is_open title="Elonaire" icon=IconId::BsNodePlusFill >
///     <p>"Hey there, I am Mr Elonaire!"</p>
/// </Panel>
///```
/// ```
/// // You can also group multiple panels by using the Collapse component
/// // is_accordion prop enables only one panel to be open at a time. It's optional and the default is false.
/// <Collapse is_accordion=true panel_items=RwSignal::new(vec![
/// PanelInfo {
///     title: ViewFn::from(move || view!{ <p>"title 1"</p> }),
///     children: ViewFn::from(move || view!{ <p>"Panel content"</p> }),
///     ..Default::default()
/// },
/// PanelInfo {
///     title: ViewFn::from(move || view!{ <p>"title 2"</p> }),
///     children: ViewFn::from(move || view!{ <p>"Panel content"</p> }),
///     ..Default::default()
/// },
/// PanelInfo {
///     title: ViewFn::from(move || view!{ <p>"title 3"</p> }),
///     children: ViewFn::from(move || view!{ <p>"Panel content"</p> }),
///     ..Default::default()
/// },
/// PanelInfo {
///     title: ViewFn::from(move || view!{ <p>"title 4"</p> }),
///     children: ViewFn::from(move || view!{ <p>"Panel content"</p> }),
///     ..Default::default()
/// },
/// ]) />
/// ```
#[component]
pub fn Panel(
    title: ViewFn,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(into)] is_open: RwSignal<bool>,
    #[prop(optional)] is_accordion: bool,
    #[prop(into, optional)] ext_panel_title_styles: String,
) -> impl IntoView {
    let panel_ref = NodeRef::new();
    let (children, _set_children) = signal(children);
    let toggle_content = move |_| {
        if let Some(panel_element) = panel_ref.get() {
            fire_custom_bubbled_and_cancelable_event("togglepanel", true, true, &panel_element);
        }

        if !is_accordion {
            is_open.update(|value| *value = !*value);
        }
    };

    Effect::new(move || {
        leptos::logging::log!("Effect -> is_open: {}", is_open.get());
    });

    view! {
        <div node_ref=panel_ref>
            <span
                on:click=toggle_content
                class=move || format!("flex flex-row items-center justify-between gap-4 mb-2 p-2 rounded cursor-pointer ring ring-primary hover:bg-primary hover:text-light-gray {} {}", ext_panel_title_styles, if is_open.get() { "bg-primary text-light-gray" } else { "" })
            >
                {title.run()}
                {
                    move || {
                        if children.get().is_some() {
                            let icon_id = if is_open.get() {
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
                    if is_open.get() {
                        "transition-max-height duration-700 ease-in-out overflow-hidden max-h-svh p-2 ml-2"
                    } else {
                        "overflow-hidden h-0 transition-max-height duration-700 ease-in-out"
                    }
                }
            >
                {move || children.get().map(|c| c())}
            </div>
        </div>
    }
}

/// ```
/// // You can also group multiple panels by using the Collapse component
/// // is_accordion prop enables only one panel to be open at a time. It's optional and the default is false.
/// <Collapse is_accordion=true panel_items=RwSignal::new(vec![
/// PanelInfo {
///     title: "title 1",
///     content: ViewFn::from(move || view!{ <p>"Panel content"</p> }),
///     ..Default::default()
/// },
/// PanelInfo {
///     title: "title 2",
///     content: ViewFn::from(move || view!{ <p>"Panel content"</p> }),
///     ..Default::default()
/// },
/// PanelInfo {
///     title: "title 3",
///     content: ViewFn::from(move || view!{ <p>"Panel content"</p> }),
///     ..Default::default()
/// },
/// PanelInfo {
///     title: "title 4",
///     content: ViewFn::from(move || view!{ <p>"Panel content"</p> }),
///     ..Default::default()
/// },
/// ]) />
/// ```
#[component]
pub fn Collapse(
    #[prop(into)] panel_items: RwSignal<Vec<PanelInfo>>,
    #[prop(default = false)] is_accordion: bool,
) -> impl IntoView {
    let handle_panel_toggle = move |index| {
        if is_accordion {
            panel_items.update(|panels| {
                let mut updated_panels = Vec::new();
                for (i, panel) in panels.iter().enumerate() {
                    if i == index {
                        panel.is_open.update(|val| *val = !*val);
                    } else {
                        panel.is_open.set(false);
                    }

                    updated_panels.push(panel.clone());
                }

                *panels = updated_panels;
            });
        }
    };

    view! {
        <div class="flex flex-col">
            <For
                each=move || panel_items.get().into_iter().enumerate()
                key=|(index, _)| *index
                let:((index, panel_item))
            >
                {
                    leptos::logging::log!("panel_item.is_open: {}", panel_item.is_open.get());
                    view! {
                        <Panel on:togglepanel=move |ev: CustomEvent| {
                            leptos::logging::log!("togglepanel event fired");
                            ev.stop_propagation();
                            handle_panel_toggle(index)
                        } title=panel_item.title.clone() is_open=panel_item.is_open is_accordion=is_accordion>
                            {panel_item.children.run()}
                        </Panel>
                    }
                }
            </For>
        </div>
    }
}
