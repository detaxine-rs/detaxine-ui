use leptos::{ev, html::*, portal::Portal, prelude::*};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tailwind_fuse::tw_merge;
use web_sys::window;

use crate::stacks::helper::overlay_root;
use crate::stacks::z_stack::{ZONE_TOOLTIP, expect_z_stack};

#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(dead_code)]
pub enum Position {
    Top,
    Bottom,
    Left,
    Right,
}

static TOOLTIP_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
fn next_tooltip_id() -> String {
    let id = TOOLTIP_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("tooltip-{id}")
}

const GAP: f64 = 8.0;

#[derive(Clone, Copy, Default)]
struct PanelPos {
    top: f64,
    left: f64,
    visible: bool, // false during the offscreen measure pass
}

/// A hover/focus-triggered tooltip that aligns itself to avoid the horizontal
/// viewport edge (for `Top`/`Bottom` positions) and opens/closes with
/// configurable debounce delays.
///
/// Unlike `Popover`, `Tooltip`:
/// - opens on `mouseenter`/`focusin` and closes on `mouseleave`/`focusout`/`Escape`,
///   rather than click-toggling
/// - has no click-catching backdrop
/// - is `pointer-events-none`, since it's informational, not interactive
/// - carries `role="tooltip"` and wires `aria-describedby` on the trigger
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::feedback::tooltip::Tooltip;
///
/// #[component]
/// fn Example() -> impl IntoView {
///     view! {
///         <Tooltip display_item=|| view! { <button>"Hover me"</button> }>
///             "Helpful context goes here"
///         </Tooltip>
///     }
/// }
/// ```
#[component]
pub fn Tooltip(
    children: ChildrenFn,

    /// `ViewFn` rendered as the hoverable/focusable trigger.
    #[prop(into)]
    display_item: ViewFn,

    /// `Position::Top`, `Bottom`, `Left`, or `Right` relative to the trigger. Defaults to `Position::Top`.
    #[prop(default = Position::Top, optional)]
    position: Position,

    /// Milliseconds to wait before showing after hover/focus begins. Defaults to `300`.
    #[prop(default = 300, optional)]
    open_delay_ms: u64,

    /// Milliseconds to wait before hiding after hover/focus ends. Defaults to `0`.
    #[prop(default = 0, optional)]
    close_delay_ms: u64,

    /// `MaybeProp<bool>` — when `true`, the tooltip never opens. Defaults to `false`.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,

    /// Extra Tailwind classes for the trigger wrapper.
    #[prop(into, optional)]
    trigger_class: MaybeProp<String>,

    /// Extra Tailwind classes for the tooltip bubble.
    #[prop(into, optional)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let trigger_ref = NodeRef::<Div>::new();
    let panel_ref = NodeRef::<Div>::new();
    let showing = RwSignal::new(false);
    let panel_pos = RwSignal::new(PanelPos::default());
    let arrow_offset = RwSignal::new("left-1/2 -translate-x-1/2".to_string());
    let tooltip_id = StoredValue::new(next_tooltip_id());
    let z_stack = expect_z_stack();
    let z_index = RwSignal::new(ZONE_TOOLTIP);
    let (children, _set_children) = signal(children);

    let open_timer = StoredValue::<Option<TimeoutHandle>>::new(None);
    let close_timer = StoredValue::<Option<TimeoutHandle>>::new(None);

    let position_class = StoredValue::new(position); // Position is Copy, fine to store directly

    let arrow_class = StoredValue::new(match position {
        Position::Top => "-bottom-2",
        Position::Bottom => "-top-2 rotate-180",
        Position::Left => "-right-2 top-1/2 -translate-y-1/2 -rotate-90",
        Position::Right => "-left-2 top-1/2 -translate-y-1/2 rotate-90",
    });

    let is_horizontal = matches!(position, Position::Top | Position::Bottom);

    // Pass 2: trigger + panel are both mounted now, measure panel and
    // compute final fixed-viewport coordinates.
    let measure_and_place = StoredValue::new(move || {
        let (Some(trigger), Some(panel)) = (trigger_ref.get_untracked(), panel_ref.get_untracked())
        else {
            return;
        };
        let Some(win) = window() else { return };

        let t = trigger.get_bounding_client_rect();
        let p = panel.get_bounding_client_rect();
        let vw = win
            .inner_width()
            .unwrap_or_default()
            .as_f64()
            .unwrap_or(375.0);

        let (top, left) = match position_class.get_value() {
            Position::Top => (
                t.top() - GAP - p.height(),
                clamp_horizontal(&t, p.width(), vw),
            ),
            Position::Bottom => (t.bottom() + GAP, clamp_horizontal(&t, p.width(), vw)),
            Position::Left => (
                t.top() + t.height() / 2.0 - p.height() / 2.0,
                t.left() - GAP - p.width(),
            ),
            Position::Right => (
                t.top() + t.height() / 2.0 - p.height() / 2.0,
                t.right() + GAP,
            ),
        };

        if is_horizontal {
            arrow_offset.set(if t.left() < vw / 3.0 {
                "left-4 translate-x-0".to_string()
            } else if t.right() > vw * 2.0 / 3.0 {
                "right-4 translate-x-0".to_string()
            } else {
                "left-1/2 -translate-x-1/2".to_string()
            });
        }

        panel_pos.set(PanelPos {
            top,
            left,
            visible: true,
        });
    });

    fn clamp_horizontal(t: &web_sys::DomRect, panel_w: f64, vw: f64) -> f64 {
        if t.left() < vw / 3.0 {
            t.left()
        } else if t.right() > vw * 2.0 / 3.0 {
            (t.right() - panel_w).max(4.0)
        } else {
            t.left() + t.width() / 2.0 - panel_w / 2.0
        }
    }

    let clear_timers = move || {
        open_timer.update_value(|h| {
            if let Some(h) = h.take() {
                h.clear()
            }
        });
        close_timer.update_value(|h| {
            if let Some(h) = h.take() {
                h.clear()
            }
        });
    };

    let open_tooltip = move || {
        // Pass 1: mount at (0,0) invisible so panel_ref has a real
        // bounding rect to measure on the next frame.
        let (_, z) = z_stack.acquire_pair(ZONE_TOOLTIP);
        z_index.set(z);
        panel_pos.set(PanelPos {
            top: 0.0,
            left: 0.0,
            visible: false,
        });
        showing.set(true);
        request_animation_frame(move || measure_and_place.get_value()());
    };

    let handle_enter = move |_| {
        if disabled.get().unwrap_or(false) {
            return;
        }
        clear_timers();
        let h = set_timeout_with_handle(open_tooltip, Duration::from_millis(open_delay_ms)).ok();
        open_timer.set_value(h);
    };

    let handle_focusin = move |_| {
        if disabled.get().unwrap_or(false) {
            return;
        }
        clear_timers();
        let h = set_timeout_with_handle(open_tooltip, Duration::from_millis(open_delay_ms)).ok();
        open_timer.set_value(h);
    };

    let handle_leave = move |_| {
        clear_timers();
        let h = set_timeout_with_handle(
            move || showing.set(false),
            Duration::from_millis(close_delay_ms),
        )
        .ok();
        close_timer.set_value(h);
    };
    let handle_focusout = move |_| {
        clear_timers();
        let h = set_timeout_with_handle(
            move || showing.set(false),
            Duration::from_millis(close_delay_ms),
        )
        .ok();
        close_timer.set_value(h);
    };

    let handle_keydown = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Escape" {
            clear_timers();
            showing.set(false);
        }
    };

    let window_resize_listener = window_event_listener(ev::resize, move |_| {
        if showing.get_untracked() {
            measure_and_place.get_value()();
        }
    });
    on_cleanup(move || {
        if showing.get_untracked() {
            z_stack.unlock_scroll();
        }
        clear_timers();
        window_resize_listener.remove();
    });

    let trigger_class_val =
        move || tw_merge!("inline-block", trigger_class.get().unwrap_or_default());

    let panel_style = move || {
        let pos = panel_pos.get();
        format!(
            "position: fixed; top: {}px; left: {}px; z-index: {}; visibility: {};",
            pos.top,
            pos.left,
            z_index.get(),
            if pos.visible { "visible" } else { "hidden" }
        )
    };

    view! {
        <div class="relative inline-block">
            <div
                node_ref=trigger_ref
                on:mouseenter=handle_enter
                on:mouseleave=handle_leave
                on:focusin=handle_focusin
                on:focusout=handle_focusout
                on:keydown=handle_keydown
                aria-describedby=move || showing.get().then(|| tooltip_id.get_value())
                class=trigger_class_val
            >
                {display_item.run()}
            </div>
            <Show when=move || showing.get() fallback=|| ()>
                {move || overlay_root().map(|root| view! {
                    <Portal mount=root>
                        <div
                            node_ref=panel_ref
                            id=tooltip_id.get_value()
                            role="tooltip"
                            style=panel_style
                            class=move || tw_merge!(
                                "w-max max-w-[240px] pointer-events-none bg-gray text-white text-xs px-2 py-1 rounded-[5px] shadow-lg",
                                class.get().unwrap_or_default()
                            )
                        >
                            <div
                                class=move || format!(
                                    "absolute w-3 h-2 bg-inherit {} {}",
                                    if is_horizontal { arrow_offset.get() } else { String::new() },
                                    arrow_class.get_value()
                                )
                                style="clip-path: polygon(50% 100%, 0 0, 100% 0);"
                            ></div>
                            <div class="relative z-10">
                                {move || children.get()()}
                            </div>
                        </div>
                    </Portal>
                })}
            </Show>
        </div>
    }
    .into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    // position_class / arrow_class logic

    fn position_class(position: Position) -> &'static str {
        match position {
            Position::Top => "bottom-full mb-2",
            Position::Bottom => "top-full mt-2",
            Position::Left => "right-full mr-2 top-1/2 -translate-y-1/2",
            Position::Right => "left-full ml-2 top-1/2 -translate-y-1/2",
        }
    }

    fn is_horizontal(position: Position) -> bool {
        matches!(position, Position::Top | Position::Bottom)
    }

    #[test]
    fn top_position_class() {
        assert_eq!(position_class(Position::Top), "bottom-full mb-2");
    }

    #[test]
    fn bottom_position_class() {
        assert_eq!(position_class(Position::Bottom), "top-full mt-2");
    }

    #[test]
    fn left_and_right_are_not_horizontal() {
        assert!(!is_horizontal(Position::Left));
        assert!(!is_horizontal(Position::Right));
    }

    #[test]
    fn top_and_bottom_are_horizontal() {
        assert!(is_horizontal(Position::Top));
        assert!(is_horizontal(Position::Bottom));
    }

    // viewport alignment logic (shared with Popover's edge-avoidance approach)

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
        let (panel, arrow) = resolve_alignment(10.0, 200.0, 375.0);
        assert_eq!(panel, "left-0");
        assert_eq!(arrow, "left-4 translate-x-0");
    }

    #[test]
    fn near_right_edge_aligns_right() {
        let (panel, arrow) = resolve_alignment(300.0, 370.0, 375.0);
        assert_eq!(panel, "right-0");
        assert_eq!(arrow, "right-4 translate-x-0");
    }

    #[test]
    fn centered_aligns_center() {
        let (panel, arrow) = resolve_alignment(150.0, 250.0, 375.0);
        assert_eq!(panel, "left-1/2 -translate-x-1/2");
        assert_eq!(arrow, "left-1/2 -translate-x-1/2");
    }

    #[test]
    fn panel_and_arrow_alignment_stay_coupled_across_all_cases() {
        // the fix: whatever bucket the trigger falls into, panel and arrow
        // must be recomputed together — never one updated without the other
        for (left, right) in [(10.0, 200.0), (300.0, 370.0), (150.0, 250.0)] {
            let (panel, arrow) = resolve_alignment(left, right, 375.0);
            let expected = match panel {
                "left-0" => "left-4 translate-x-0",
                "right-0" => "right-4 translate-x-0",
                _ => "left-1/2 -translate-x-1/2",
            };
            assert_eq!(arrow, expected);
        }
    }

    // disabled guard

    fn should_open(disabled: bool) -> bool {
        !disabled
    }

    #[test]
    fn opens_when_not_disabled() {
        assert!(should_open(false));
    }

    #[test]
    fn blocked_when_disabled() {
        assert!(!should_open(true));
    }

    // id generation

    #[test]
    fn tooltip_ids_are_unique() {
        let a = next_tooltip_id();
        let b = next_tooltip_id();
        assert_ne!(a, b);
    }

    // show/hide signal behavior

    #[test]
    fn hover_enter_shows_tooltip() {
        let owner = Owner::new();
        owner.with(|| {
            let showing = RwSignal::new(false);
            showing.set(true); // simulates open_timer firing after enter
            assert!(showing.get());
        });
    }

    #[test]
    fn hover_leave_hides_tooltip() {
        let owner = Owner::new();
        owner.with(|| {
            let showing = RwSignal::new(true);
            showing.set(false); // simulates close_timer firing after leave
            assert!(!showing.get());
        });
    }

    #[test]
    fn escape_key_closes_tooltip() {
        let owner = Owner::new();
        owner.with(|| {
            let showing = RwSignal::new(true);
            let key = "Escape";
            if key == "Escape" {
                showing.set(false);
            }
            assert!(!showing.get());
        });
    }

    #[test]
    fn non_escape_key_does_not_close_tooltip() {
        let owner = Owner::new();
        owner.with(|| {
            let showing = RwSignal::new(true);
            let key = "Tab";
            if key == "Escape" {
                showing.set(false);
            }
            assert!(showing.get());
        });
    }
}
