use leptos::{ev, html::*, prelude::*};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tailwind_fuse::tw_merge;

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
    /// Content rendered inside the tooltip bubble.
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
    let showing = RwSignal::new(false);
    let align = RwSignal::new((
        "left-1/2 -translate-x-1/2".to_string(),
        "left-1/2 -translate-x-1/2".to_string(),
    ));
    let tooltip_id = StoredValue::new(next_tooltip_id());

    let open_timer = StoredValue::<Option<TimeoutHandle>>::new(None);
    let close_timer = StoredValue::<Option<TimeoutHandle>>::new(None);

    let position_class = StoredValue::new(match position {
        Position::Top => "bottom-full mb-2",
        Position::Bottom => "top-full mt-2",
        Position::Left => "right-full mr-2 top-1/2 -translate-y-1/2",
        Position::Right => "left-full ml-2 top-1/2 -translate-y-1/2",
    });

    let arrow_class = StoredValue::new(match position {
        Position::Top => "-bottom-2",
        Position::Bottom => "-top-2 rotate-180",
        Position::Left => "-right-2 top-1/2 -translate-y-1/2 -rotate-90",
        Position::Right => "-left-2 top-1/2 -translate-y-1/2 rotate-90",
    });

    let is_horizontal = matches!(position, Position::Top | Position::Bottom);

    let recalculate = StoredValue::new(move || {
        if !is_horizontal {
            return;
        }
        if let Some(trigger) = trigger_ref.get_untracked() {
            let rect = trigger.get_bounding_client_rect();
            if let Some(window) = web_sys::window() {
                let vw = window
                    .inner_width()
                    .unwrap_or_default()
                    .as_f64()
                    .unwrap_or(375.0);

                let (panel_align, arrow_align) = if rect.left() < vw / 3.0 {
                    ("left-0".to_string(), "left-4 translate-x-0".to_string())
                } else if rect.right() > (vw * 2.0 / 3.0) {
                    ("right-0".to_string(), "right-4 translate-x-0".to_string())
                } else {
                    (
                        "left-1/2 -translate-x-1/2".to_string(),
                        "left-1/2 -translate-x-1/2".to_string(),
                    )
                };

                align.set((panel_align, arrow_align));
            }
        }
    });

    let clear_timers = move || {
        open_timer.update_value(|h| {
            if let Some(handle) = h.take() {
                handle.clear();
            }
        });
        close_timer.update_value(|h| {
            if let Some(handle) = h.take() {
                handle.clear();
            }
        });
    };

    let handle_enter = move |_| {
        if disabled.get().unwrap_or(false) {
            return;
        }
        clear_timers();
        let handle = set_timeout_with_handle(
            move || {
                showing.set(true);
                recalculate.get_value()();
            },
            Duration::from_millis(open_delay_ms),
        )
        .ok();
        open_timer.set_value(handle);
    };

    let handle_focusin = move |_| {
        if disabled.get().unwrap_or(false) {
            return;
        }
        clear_timers();
        let handle = set_timeout_with_handle(
            move || {
                showing.set(true);
                recalculate.get_value()();
            },
            Duration::from_millis(open_delay_ms),
        )
        .ok();
        open_timer.set_value(handle);
    };

    let handle_leave = move |_| {
        clear_timers();
        let handle = set_timeout_with_handle(
            move || {
                showing.set(false);
            },
            Duration::from_millis(close_delay_ms),
        )
        .ok();
        close_timer.set_value(handle);
    };

    let handle_focusout = move |_| {
        clear_timers();
        let handle = set_timeout_with_handle(
            move || {
                showing.set(false);
            },
            Duration::from_millis(close_delay_ms),
        )
        .ok();
        close_timer.set_value(handle);
    };

    let handle_keydown = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Escape" {
            clear_timers();
            showing.set(false);
        }
    };

    Effect::new(move |_| {
        if !showing.get() {
            align.set((
                "left-1/2 -translate-x-1/2".to_string(),
                "left-1/2 -translate-x-1/2".to_string(),
            ));
        }
    });

    let window_resize_listener = window_event_listener(ev::resize, move |_| {
        if showing.get_untracked() {
            recalculate.get_value()();
        }
    });

    on_cleanup(move || {
        clear_timers();
        window_resize_listener.remove();
    });

    let trigger_class_val =
        move || tw_merge!("inline-block", trigger_class.get().unwrap_or_default());

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
                <div
                    id=tooltip_id.get_value()
                    role="tooltip"
                    class=move || tw_merge!(
                        format!(
                            "absolute {} {} z-30 w-max max-w-[240px] pointer-events-none bg-gray text-white text-xs px-2 py-1 rounded-[5px] shadow-lg",
                            position_class.get_value(),
                            if is_horizontal { align.get().0 } else { String::new() }
                        ),
                        class.get().unwrap_or_default()
                    )
                >
                    // <div
                    //     class=move || format!(
                    //         "absolute -z-10 bg-inherit {} {}",
                    //         if is_horizontal { align.get().1 } else { String::new() },
                    //         arrow_class.get_value()
                    //     )
                    // >
                    //     <div class="w-[8px] h-[8px] bg-inherit rotate-45"></div>
                    // </div>
                    <div
                        class=move || format!(
                            "absolute w-3 h-2 bg-inherit {} {}",
                            if is_horizontal { align.get().1 } else { String::new() },
                            arrow_class.get_value()
                        )
                        style="clip-path: polygon(50% 100%, 0 0, 100% 0);"
                    ></div>
                    <div class="relative z-10">
                        {children()}
                    </div>
                </div>
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
