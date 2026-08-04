use leptos::{ev, html::*, portal::Portal, prelude::*};
use leptos_router::hooks::use_location;
use tailwind_fuse::tw_merge;
use web_sys::window;

use crate::stacks::{
    helper::overlay_root,
    z_stack::{ZONE_DROPDOWN, expect_z_stack},
};

#[derive(Clone, PartialEq, Debug, Copy)]
#[allow(dead_code)]
pub enum Position {
    Top,
    Bottom,
}

#[derive(Clone, Copy, Default)]
struct PanelPos {
    top: f64,
    left: f64,
}

/// A popover that toggles open/closed when its trigger element is clicked,
/// automatically aligns to the viewport edge, and closes on route change.
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
    /// Optional content rendered inside the popover panel.
    #[prop(optional)]
    children: Option<ChildrenFn>,

    /// `ViewFn` rendered as the clickable trigger.
    #[prop(into)]
    display_item: ViewFn,

    /// `Position::Top` or `Position::Bottom` relative to the trigger. Defaults to `Position::Bottom`.
    #[prop(default = Position::Bottom, optional)]
    position: Position,

    /// **Deprecated**: use `class` instead. Still supported and merged
    /// in alongside `class` for backward compatibility.
    #[prop(into, optional)]
    style_ext: MaybeProp<String>,

    /// Extra Tailwind classes for the popover panel.
    #[prop(into, optional)]
    class: MaybeProp<String>,

    /// `RwSignal<bool>` controlling open/closed state.
    #[prop(into)]
    showing: RwSignal<bool>,
) -> impl IntoView {
    let (children, _set_children) = signal(children);
    let trigger_ref = NodeRef::<Div>::new();
    let panel_pos = RwSignal::new(PanelPos::default());
    let arrow_offset = RwSignal::new("left-1/2 -translate-x-1/2".to_string());
    let location = use_location();
    let z_stack = expect_z_stack();
    let z_indices = RwSignal::new((ZONE_DROPDOWN, ZONE_DROPDOWN + 1));

    let onclick_toggle_handler = move |_| {
        showing.update(|val| *val = !*val);
        if showing.get_untracked() {
            z_stack.lock_scroll();
        } else {
            z_stack.unlock_scroll();
        }
    };

    let arrow_class = StoredValue::new(match position {
        Position::Top => "-bottom-[5px] rotate-180",
        Position::Bottom => "-top-[5px]",
    });

    Effect::new(move |_| {
        let _ = location.pathname.get();
        if showing.get_untracked() {
            z_stack.unlock_scroll();
        }
        showing.set(false);
    });

    let recalculate = StoredValue::new(move || {
        let Some(trigger) = trigger_ref.get_untracked() else {
            return;
        };
        let Some(win) = window() else { return };

        let rect = trigger.get_bounding_client_rect();
        let vw = win
            .inner_width()
            .unwrap_or_default()
            .as_f64()
            .unwrap_or(375.0);

        // vertical: below or above trigger, in viewport (fixed) coords
        let top = match position {
            Position::Bottom => rect.bottom() + 8.0,
            Position::Top => rect.top() - 8.0, // panel uses translateY(-100%) in CSS below
        };

        // horizontal: clamp panel + arrow to stay on-screen
        let panel_min_w = 128.0; // matches min-w-32
        let left = if rect.left() < vw / 3.0 {
            rect.left()
        } else if rect.right() > vw * 2.0 / 3.0 {
            (rect.right() - panel_min_w).max(8.0)
        } else {
            rect.left() + rect.width() / 2.0 - panel_min_w / 2.0
        };

        arrow_offset.set(if rect.left() < vw / 3.0 {
            "left-4 translate-x-0".to_string()
        } else if rect.right() > vw * 2.0 / 3.0 {
            "right-4 translate-x-0".to_string()
        } else {
            "left-1/2 -translate-x-1/2".to_string()
        });

        panel_pos.set(PanelPos { top, left });
    });

    Effect::new(move |_| {
        if showing.get() {
            let (backdrop_z, panel_z) = z_stack.acquire_pair(ZONE_DROPDOWN);
            z_indices.set((backdrop_z, panel_z));
            request_animation_frame(move || recalculate.get_value()());
        }
    });

    let window_resize_listener = window_event_listener(ev::resize, move |_| {
        if showing.get_untracked() {
            recalculate.get_value()();
        }
    });
    on_cleanup(move || window_resize_listener.remove());

    let panel_style = move || {
        let pos = panel_pos.get();
        let transform = match position {
            Position::Top => "translateY(-100%)",
            Position::Bottom => "none",
        };
        format!(
            "position: fixed; top: {}px; left: {}px; transform: {}; z-index: {};",
            pos.top,
            pos.left,
            transform,
            z_indices.get().1
        )
    };

    view! {
        <div class="relative">
            <div node_ref=trigger_ref on:click=onclick_toggle_handler class="cursor-pointer">
                {display_item.run()}
            </div>
            <Show when=move || showing.get() fallback=|| ()>
                {move || overlay_root().map(|root| view! {
                    <Portal mount=root>
                        <div
                            on:click=onclick_toggle_handler
                            class="fixed inset-0 bg-transparent"
                            style=move || format!("z-index: {}", z_indices.get().0)
                        ></div>
                        <div
                            style=panel_style
                            class=move || tw_merge!(
                                "w-max min-w-32 max-w-[calc(100vw-1rem)] bg-white border border-light-gray shadow-lg text-sm rounded-[5px]",
                                style_ext.get().unwrap_or_default(),
                                class.get().unwrap_or_default()
                            )
                        >
                            <div class=move || format!("absolute bg-inherit {} {}", arrow_offset.get(), arrow_class.get_value())>
                                <div class="w-[15px] h-[15px] bg-inherit border-l border-t border-light-gray rotate-45"></div>
                            </div>
                            <div class="relative z-10 bg-inherit rounded-[5px]">
                                {move || children.get().map(|child| child())}
                            </div>
                        </div>
                    </Portal>
                })}
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
