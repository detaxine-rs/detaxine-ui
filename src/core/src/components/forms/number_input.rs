use icondata::{BsDashLg, BsPlusLg};
use leptos::{prelude::*, wasm_bindgen::JsCast};
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

/// Clamp a committed value to the allowed range.
fn clamp_value(raw: i64, min: i64, max: i64) -> i64 {
    raw.clamp(min, max)
}

/// Sanitize a raw `<input>` value down to digits, plus a leading `-` when
/// negative values are allowed (`min < 0`). Only a *leading* minus survives;
/// any `-` elsewhere in the string is dropped along with other non-digits.
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

/// Parse the text box's contents on commit (blur / Enter), falling back to
/// the last known-good value if the field is empty or unparseable (e.g. a
/// lone "-").
fn parse_or_fallback(text: &str, fallback: i64) -> i64 {
    text.parse::<i64>().unwrap_or(fallback)
}

/// Whether the decrement button should be enabled.
fn can_step_down(current: i64, min: i64, disabled: bool) -> bool {
    current > min && !disabled
}

/// Whether the increment button should be enabled.
fn can_step_up(current: i64, max: i64, disabled: bool) -> bool {
    current < max && !disabled
}

/// A quantity stepper: minus button, editable numeric field, plus button.
/// Uncontrolled — keeps its own signal internally and reports changes via `on_change`.
/// Typing is unrestricted while focused (sanitized to digits/optional leading `-`),
/// clamping to `min`/`max` happens on blur, Enter, or via the +/- buttons.
///
/// Responsive: the buttons are `shrink-0` (fixed size), the input is `flex-1 min-w-0`
/// so it absorbs whatever width the parent gives it instead of forcing a fixed track
/// width. The outer container is `w-fit max-w-full` by default — sized to its content
/// but never wider than its parent. Pass `class="w-full"` to stretch it to fill.
///
/// `class`/`button_class`/`input_class` are merged on top of the defaults with
/// `tw_merge!`, not swapped wholesale — passing `class="w-full"` keeps the base
/// `flex`/`border`/`rounded`/`bg-white` and only overrides the conflicting `w-fit`.
#[component]
pub fn CustomNumberInput(
    #[prop(into)] name: String,
    #[prop(optional, default = 0)] initial_value: i64,
    #[prop(optional, default = 0)] min: i64,
    #[prop(optional, default = i64::MAX)] max: i64,
    #[prop(optional, default = 1)] step: i64,
    #[prop(optional)] on_change: Option<Callback<i64>>,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] button_class: String,
    #[prop(into, optional)] input_class: String,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional, default = false)] required: bool,
) -> impl IntoView {
    let clamped_initial = clamp_value(initial_value, min, max);
    let opts = NumberInputOptions {
        class: tw_merge!(NumberInputOptions::default().class, class),
        button_class: tw_merge!(NumberInputOptions::default().button_class, button_class),
        input_class: tw_merge!(NumberInputOptions::default().input_class, input_class),
    };

    let (count, set_count) = signal(clamped_initial);
    // What's actually shown in the box. Kept separate from `count` so the user can
    // freely type/clear without being clamped on every keystroke.
    let (text, set_text) = signal(clamped_initial.to_string());

    let input_ref = NodeRef::<leptos::html::Input>::new();

    // Notify the parent whenever the committed value changes.
    Effect::new(move |_| {
        let val = count.get();
        if let Some(cb) = on_change {
            cb.run(val);
        }
    });

    let commit = move |raw: i64| {
        let clamped = clamp_value(raw, min, max);
        set_count.set(clamped);
        set_text.set(clamped.to_string());
    };

    let can_decrement = Memo::new(move |_| can_step_down(count.get(), min, disabled.get()));
    let can_increment = Memo::new(move |_| can_step_up(count.get(), max, disabled.get()));

    let decrement = move |_| {
        if disabled.get_untracked() {
            return;
        }
        commit(count.get_untracked().saturating_sub(step));
    };

    let increment = move |_| {
        if disabled.get_untracked() {
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
        if disabled.get_untracked() {
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

    // --- clamp_value ---

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
    fn clamp_value_min_equals_max_forces_exact_value() {
        assert_eq!(clamp_value(50, 7, 7), 7);
    }

    // --- sanitize_numeric_input ---

    #[test]
    fn sanitize_keeps_only_digits_by_default() {
        assert_eq!(sanitize_numeric_input("12a3b", false), "123");
    }

    #[test]
    fn sanitize_strips_all_non_digits() {
        assert_eq!(sanitize_numeric_input("$12.34", false), "1234");
    }

    #[test]
    fn sanitize_keeps_leading_minus_when_negatives_allowed() {
        assert_eq!(sanitize_numeric_input("-42", true), "-42");
    }

    #[test]
    fn sanitize_drops_leading_minus_when_negatives_disallowed() {
        assert_eq!(sanitize_numeric_input("-42", false), "42");
    }

    #[test]
    fn sanitize_drops_non_leading_minus_even_when_allowed() {
        // Only a leading "-" is treated as a sign; a stray "-" mid-string is junk.
        assert_eq!(sanitize_numeric_input("4-2", true), "42");
    }

    #[test]
    fn sanitize_empty_input_stays_empty() {
        assert_eq!(sanitize_numeric_input("", true), "");
    }

    #[test]
    fn sanitize_all_garbage_becomes_empty() {
        assert_eq!(sanitize_numeric_input("abc", false), "");
    }

    #[test]
    fn sanitize_lone_minus_with_negatives_allowed() {
        assert_eq!(sanitize_numeric_input("-", true), "-");
    }

    // --- parse_or_fallback ---

    #[test]
    fn parse_valid_number() {
        assert_eq!(parse_or_fallback("42", 0), 42);
    }

    #[test]
    fn parse_valid_negative_number() {
        assert_eq!(parse_or_fallback("-7", 0), -7);
    }

    #[test]
    fn parse_empty_string_falls_back() {
        assert_eq!(parse_or_fallback("", 3), 3);
    }

    #[test]
    fn parse_lone_minus_falls_back() {
        assert_eq!(parse_or_fallback("-", 3), 3);
    }

    // --- can_step_down / can_step_up ---

    #[test]
    fn can_step_down_true_above_min() {
        assert!(can_step_down(5, 0, false));
    }

    #[test]
    fn can_step_down_false_at_min() {
        assert!(!can_step_down(0, 0, false));
    }

    #[test]
    fn can_step_down_false_when_disabled_even_above_min() {
        assert!(!can_step_down(5, 0, true));
    }

    #[test]
    fn can_step_up_true_below_max() {
        assert!(can_step_up(5, 99, false));
    }

    #[test]
    fn can_step_up_false_at_max() {
        assert!(!can_step_up(99, 99, false));
    }

    #[test]
    fn can_step_up_false_when_disabled_even_below_max() {
        assert!(!can_step_up(5, 99, true));
    }
}
