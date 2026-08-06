use icondata::{BsDashLg, BsPlusLg};
use leptos::{html::*, prelude::*, wasm_bindgen::JsCast};
use tailwind_fuse::tw_merge;
use web_sys::HtmlInputElement;

use crate::components::actions::button::BasicButton;

#[derive(Clone, Debug)]
struct NumberInputOptions {
    pub class: String,
    pub button_class: String,
    pub input_class: String,
}

impl Default for NumberInputOptions {
    fn default() -> Self {
        Self {
            class: "flex h-7 items-center border border-light-gray rounded-[10px] overflow-hidden bg-white"
                .to_string(),
            button_class: "w-7 h-full shrink-0 flex items-center justify-center text-mid-gray hover:bg-light-gray \
                active:bg-bg-main disabled:opacity-40 disabled:cursor-not-allowed \
                disabled:hover:bg-transparent transition-colors duration-150 focus:outline-none \
                focus:ring-2 focus:ring-secondary/30 focus:z-10"
                .to_string(),
            input_class: "w-full h-full text-center text-sm font-medium text-gray bg-transparent \
                border-x border-light-gray focus:outline-none focus:ring-2 focus:ring-secondary/30 \
                focus:z-10 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none \
                [&::-webkit-inner-spin-button]:appearance-none"
                .to_string(),
        }
    }
}

// --- Pure logic, kept free of signals/DOM so it's directly unit-testable. ---

fn clamp_value(raw: i64, min: i64, max: i64) -> i64 {
    raw.clamp(min, max)
}

fn sanitize_numeric_input(raw: &str, allow_negative: bool) -> String {
    let mut sanitized = String::new();
    let mut chars = raw.chars().peekable();
    if allow_negative && chars.peek() == Some(&'-') {
        sanitized.push('-');
        chars.next();
    }
    sanitized.extend(chars.filter(|c| c.is_ascii_digit()));
    sanitized
}

fn parse_or_fallback(text: &str, fallback: i64) -> i64 {
    text.parse::<i64>().unwrap_or(fallback)
}

fn can_step_down(current: i64, min: i64, disabled: bool) -> bool {
    current > min && !disabled
}

fn can_step_up(current: i64, max: i64, disabled: bool) -> bool {
    current < max && !disabled
}

/// Quantity stepper: minus button, editable numeric field, plus button.
///
/// *Uncontrolled* internally, but will **sync** when the `initial_value` prop
/// changes from the parent (e.g. cart quantity updated elsewhere).
/// `on_change` is only fired from explicit user actions, never from the sync
/// effect, so there is no feedback loop.
#[component]
pub fn CustomNumberInput(
    #[prop(into)] name: String,
    #[prop(into, optional)] initial_value: MaybeProp<i64>,
    #[prop(optional, default = 0)] min: i64,
    #[prop(optional, default = i64::MAX)] max: i64,
    #[prop(optional, default = 1)] step: i64,
    #[prop(optional, default = Callback::new(move |_| {}))] on_change: Callback<i64>,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] button_class: String,
    #[prop(into, optional)] input_class: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional, default = false)] required: bool,
    #[prop(optional)] input_node_ref: NodeRef<Input>,
) -> impl IntoView {
    let opts = NumberInputOptions {
        class: tw_merge!(NumberInputOptions::default().class, class),
        button_class: tw_merge!(NumberInputOptions::default().button_class, button_class),
        input_class: tw_merge!(NumberInputOptions::default().input_class, input_class),
    };

    let (count, set_count) = signal(0i64);
    let (text, set_text) = signal(String::new());

    // ── One-way sync: prop -> internal state (does NOT call on_change) ──
    Effect::new(move |_| {
        let val = clamp_value(initial_value.get().unwrap_or_default(), min, max);
        set_count.set(val);
        set_text.set(val.to_string());
    });

    let input_ref = NodeRef::<leptos::html::Input>::new();

    let commit = move |raw: i64| {
        let clamped = clamp_value(raw, min, max);
        set_count.set(clamped);
        set_text.set(clamped.to_string());
        on_change.run(clamped);
    };

    let can_decrement =
        Memo::new(move |_| can_step_down(count.get(), min, disabled.get().unwrap_or_default()));
    let can_increment =
        Memo::new(move |_| can_step_up(count.get(), max, disabled.get().unwrap_or_default()));

    let decrement = move |_| {
        if disabled.get_untracked().unwrap_or_default() {
            return;
        }
        commit(count.get_untracked().saturating_sub(step));
    };

    let increment = move |_| {
        if disabled.get_untracked().unwrap_or_default() {
            return;
        }
        commit(count.get_untracked().saturating_add(step));
    };

    let handle_input = move |ev: web_sys::Event| {
        let Some(input) = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
        else {
            return;
        };
        set_text.set(sanitize_numeric_input(&input.value(), min < 0));
    };

    let handle_blur = move |_| {
        let raw = text.get_untracked();
        let parsed = parse_or_fallback(&raw, count.get_untracked());
        commit(parsed);
    };

    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        if disabled.get_untracked().unwrap_or_default() {
            return;
        }
        match ev.key().as_str() {
            "ArrowUp" => {
                ev.prevent_default();
                commit(count.get_untracked().saturating_add(step));
            }
            "ArrowDown" => {
                ev.prevent_default();
                commit(count.get_untracked().saturating_sub(step));
            }
            "Enter" => {
                if let Some(el) = input_ref.get_untracked() {
                    let _ = el.blur();
                }
            }
            _ => {}
        }
    };

    view! {
        <div class=opts.class>
            <BasicButton
                class=tw_merge!("{} rounded-r-none", opts.button_class.clone())
                disabled=Signal::derive(move || !can_decrement.get())
                on:click=decrement
                icon=Some(BsDashLg)
            />
            <input
                node_ref=input_ref
                type="text"
                inputmode="numeric"
                pattern="-?[0-9]*"
                name=name
                class=opts.input_class
                prop:value=move || text.get()
                disabled=move || disabled.get()
                required=required
                on:input=handle_input
                on:blur=handle_blur
                on:keydown=handle_keydown
            />
            <BasicButton
                class=tw_merge!("{} rounded-l-none", opts.button_class.clone())
                disabled=Signal::derive(move || !can_increment.get())
                on:click=increment
                icon=Some(BsPlusLg)
            />
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_value_within_range_is_unchanged() {
        assert_eq!(clamp_value(5, 0, 99), 5);
    }

    #[test]
    fn clamp_value_below_min_clamps_up() {
        assert_eq!(clamp_value(-10, 1, 99), 1);
    }

    #[test]
    fn clamp_value_above_max_clamps_down() {
        assert_eq!(clamp_value(1000, 1, 99), 99);
    }

    #[test]
    fn sanitize_keeps_only_digits_by_default() {
        assert_eq!(sanitize_numeric_input("12a3b", false), "123");
    }

    #[test]
    fn sanitize_keeps_leading_minus_when_negatives_allowed() {
        assert_eq!(sanitize_numeric_input("-42", true), "-42");
    }

    #[test]
    fn parse_valid_number() {
        assert_eq!(parse_or_fallback("42", 0), 42);
    }

    #[test]
    fn parse_empty_string_falls_back() {
        assert_eq!(parse_or_fallback("", 3), 3);
    }
}
