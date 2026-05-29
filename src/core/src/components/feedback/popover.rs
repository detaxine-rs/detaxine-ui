use leptos::{ev, html::*, prelude::*};
use leptos_router::hooks::use_location;

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum Position {
    Top,
    Bottom,
}

/// The Popover component is a reusable UI element that displays a popover with customizable content and positioning.
/// ```
/// <Popover display_item=|| view!{ <p>"Elonaire here"</p> } showing=popover_open>
///    <div class="flex flex-row">
///        <span class="text-gray">"Tenka"</span>
///        <img src="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRij6dtiHizH96qpCOe8WeXXP3yLyQJkPdGVg&s" />
///    </div>
/// </Popover>
/// ```
#[component]
pub fn Popover(
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(into)] display_item: ViewFn,
    #[prop(default = Position::Bottom, optional)] position: Position,
    #[prop(into, optional)] style_ext: String,
    #[prop(into)] showing: RwSignal<bool>,
) -> impl IntoView {
    let (children, _set_children) = signal(children);
    let trigger_ref = NodeRef::<Div>::new();
    let align = RwSignal::new((
        "left-1/2 -translate-x-1/2".to_string(),
        "left-1/2 -translate-x-1/2".to_string(),
    ));
    let location = use_location();

    let onclick_toggle_handler = move |_| {
        showing.update(|val| *val = !*val);
    };

    let position_class = StoredValue::new(match position {
        Position::Top => "bottom-full mb-2",
        Position::Bottom => "top-full mt-2",
    });

    let arrow_class = StoredValue::new(match position {
        Position::Top => "-bottom-[10px] rotate-180",
        Position::Bottom => "-top-[10px]",
    });

    let style_ext = StoredValue::new(style_ext);

    Effect::new(move |_| {
        // any time the pathname changes, close the popover
        let _ = location.pathname.get();
        showing.set(false);
    });

    let recalculate = StoredValue::new(move || {
        if let Some(trigger) = trigger_ref.get_untracked() {
            let rect = trigger.get_bounding_client_rect();
            if let Some(window) = web_sys::window() {
                let vw = window
                    .inner_width()
                    .unwrap_or_default()
                    .as_f64()
                    .unwrap_or(375.0);

                let (popover_align, arrow_align) = if rect.left() < vw / 3.0 {
                    // Near left edge — align popover left, arrow near left
                    ("left-0".to_string(), "left-4 translate-x-0".to_string())
                } else if rect.right() > (vw * 2.0 / 3.0) {
                    // Near right edge — align popover right, arrow near right
                    ("right-0".to_string(), "right-4 translate-x-0".to_string())
                } else {
                    // Center
                    (
                        "left-1/2 -translate-x-1/2".to_string(),
                        "left-1/2 -translate-x-1/2".to_string(),
                    )
                };

                align.set((popover_align, arrow_align));
            };
        }
    });

    Effect::new(move |_| {
        if showing.get() {
            request_animation_frame(move || recalculate.get_value()());
        } else {
            align.set((
                "left-1/2 -translate-x-1/2".to_string(),
                "left-1/2 -translate-x-1/2".to_string(),
            ));
        }
    });

    let window_resize_listener = window_event_listener(ev::resize, move |_| {
        recalculate.get_value()();
    });

    // Ensure removal when component goes out of scope
    on_cleanup(move || {
        window_resize_listener.remove(); // Explicitly detach
    });

    view! {
        <div class="relative">
            <div node_ref=trigger_ref on:click=onclick_toggle_handler class="cursor-pointer">
                {display_item.run()}
            </div>
            <Show when=move || showing.get() fallback=|| ()>
                <div
                    on:click=onclick_toggle_handler
                    class="fixed inset-0 z-20 bg-transparent"
                ></div>
                <div
                    class=move || format!(
                        "absolute {} {} z-30
                         w-max min-w-32 max-w-[calc(100vw-1rem)]
                         bg-contrast-white border border-light-gray
                         shadow-lg text-sm rounded-[5px] {}",
                        align.get().0,
                        position_class.get_value(),
                        style_ext.get_value()
                    )
                >
                    <div
                        class=move || format!(
                            "absolute {} {}",
                            align.get().1,
                            arrow_class.get_value()
                        )
                    >
                        <div class="w-[20px] h-[20px] bg-contrast-white border-l border-t border-light-gray rotate-45"></div>
                    </div>
                    <div class="relative z-10 bg-contrast-white rounded-[5px]">
                        {move || children.get().map(|child| child())}
                    </div>
                </div>
            </Show>
        </div>
    }
}
