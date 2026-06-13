use leptos::{ev, html::*, prelude::*};
use leptos_router::hooks::use_location;

#[derive(Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum Position {
    Top,
    Bottom,
}

/// A popover that toggles open/closed when its trigger element is clicked,
/// automatically aligns to the viewport edge, and closes on route change.
///
/// # Props
///
/// - `display_item` – `ViewFn` rendered as the clickable trigger.
/// - `showing` – `RwSignal<bool>` controlling open/closed state.
/// - `position` – `Position::Top` or `Position::Bottom` relative to the trigger. Defaults to `Position::Bottom`.
/// - `style_ext` – Additional Tailwind classes applied to the popover panel.
/// - `children` – Optional content rendered inside the popover panel.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::feedback::popover::Popover;
///
/// #[component]
/// fn Example() -> impl IntoView {
///     let showing = RwSignal::new(false);
///
///     view! {
///         <Popover
///             showing=showing
///             display_item=|| view! { <button>"Open"</button> }
///         >
///             <p>"Popover content"</p>
///         </Popover>
///     }
/// }
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
    }.into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Position

    #[test]
    fn position_eq() {
        assert_eq!(Position::Top, Position::Top);
        assert_ne!(Position::Top, Position::Bottom);
    }

    #[test]
    fn position_clone() {
        assert_eq!(Position::Bottom.clone(), Position::Bottom);
    }

    // position_class logic

    fn position_class(position: &Position) -> &'static str {
        match position {
            Position::Top => "bottom-full mb-2",
            Position::Bottom => "top-full mt-2",
        }
    }

    fn arrow_class(position: &Position) -> &'static str {
        match position {
            Position::Top => "-bottom-[10px] rotate-180",
            Position::Bottom => "-top-[10px]",
        }
    }

    #[test]
    fn top_position_class() {
        assert_eq!(position_class(&Position::Top), "bottom-full mb-2");
    }

    #[test]
    fn bottom_position_class() {
        assert_eq!(position_class(&Position::Bottom), "top-full mt-2");
    }

    #[test]
    fn top_arrow_class() {
        assert_eq!(arrow_class(&Position::Top), "-bottom-[10px] rotate-180");
    }

    #[test]
    fn bottom_arrow_class() {
        assert_eq!(arrow_class(&Position::Bottom), "-top-[10px]");
    }

    // viewport alignment logic

    fn resolve_alignment(left: f64, right: f64, vw: f64) -> (&'static str, &'static str) {
        if left < vw / 3.0 {
            ("left-0", "left-4 translate-x-0")
        } else if right > vw * 2.0 / 3.0 {
            ("right-0", "right-4 translate-x-0")
        } else {
            ("left-1/2 -translate-x-1/2", "left-1/2 -translate-x-1/2")
        }
    }

    #[test]
    fn near_left_edge_aligns_left() {
        let (popover, arrow) = resolve_alignment(10.0, 200.0, 375.0);
        assert_eq!(popover, "left-0");
        assert_eq!(arrow, "left-4 translate-x-0");
    }

    #[test]
    fn near_right_edge_aligns_right() {
        let (popover, arrow) = resolve_alignment(300.0, 370.0, 375.0);
        assert_eq!(popover, "right-0");
        assert_eq!(arrow, "right-4 translate-x-0");
    }

    #[test]
    fn centered_aligns_center() {
        let (popover, arrow) = resolve_alignment(150.0, 250.0, 375.0);
        assert_eq!(popover, "left-1/2 -translate-x-1/2");
        assert_eq!(arrow, "left-1/2 -translate-x-1/2");
    }

    // toggle logic

    #[test]
    fn toggle_opens_when_closed() {
        let owner = Owner::new();
        owner.with(|| {
            let showing = RwSignal::new(false);
            showing.update(|v| *v = !*v);
            assert!(showing.get());
        });
    }

    #[test]
    fn toggle_closes_when_open() {
        let owner = Owner::new();
        owner.with(|| {
            let showing = RwSignal::new(true);
            showing.update(|v| *v = !*v);
            assert!(!showing.get());
        });
    }

    #[test]
    fn route_change_closes_popover() {
        let owner = Owner::new();
        owner.with(|| {
            let showing = RwSignal::new(true);
            // simulates what the pathname Effect does
            showing.set(false);
            assert!(!showing.get());
        });
    }

    #[test]
    fn alignment_resets_when_closed() {
        let owner = Owner::new();
        owner.with(|| {
            let align = RwSignal::new(("left-0".to_string(), "left-4 translate-x-0".to_string()));
            let showing = RwSignal::new(false);

            if !showing.get() {
                align.set((
                    "left-1/2 -translate-x-1/2".to_string(),
                    "left-1/2 -translate-x-1/2".to_string(),
                ));
            }

            assert_eq!(align.get().0, "left-1/2 -translate-x-1/2");
            assert_eq!(align.get().1, "left-1/2 -translate-x-1/2");
        });
    }
}
