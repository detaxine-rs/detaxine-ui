use leptos::{prelude::*, wasm_bindgen::JsCast};
use tailwind_fuse::tw_merge;
use web_sys::HtmlInputElement;

/// Extract only ASCII digits from a string.
fn extract_digits(raw: &str) -> String {
    raw.chars().filter(|c| c.is_ascii_digit()).collect()
}

/// Check if a PIN character slice represents a completed code.
/// A PIN is complete when every slot contains an ASCII digit.
fn is_pin_complete(pin: &[char], length: usize) -> bool {
    pin.len() == length && pin.iter().all(|&c| c.is_ascii_digit())
}

/// Build the CSS class string for a single digit input based on its state.
fn build_digit_class(
    base: &str,
    focused_class: &str,
    filled_class: &str,
    is_focused: bool,
    is_filled: bool,
) -> String {
    tw_merge!(
        base.to_string(),
        if is_focused {
            focused_class.to_string()
        } else {
            String::new()
        },
        if is_filled {
            filled_class.to_string()
        } else {
            String::new()
        }
    )
}

#[derive(Clone, Debug)]
pub struct PinInputOptions {
    pub length: usize,
    pub placeholder: String,
    pub class: String,
    pub digit_class: String,
    pub focused_class: String,
    pub filled_class: String,
}

impl Default for PinInputOptions {
    fn default() -> Self {
        Self {
            length: 6,
            placeholder: "*".to_string(),
            class: "flex gap-2 justify-center".to_string(),
            digit_class: "w-12 h-14 text-center text-xl font-semibold border-2 border-light-gray rounded-[10px] bg-white text-gray focus:outline-none focus:border-secondary focus:ring-2 focus:ring-secondary/30 transition-all duration-200".to_string(),
            focused_class: "border-secondary ring-2 ring-secondary caret-transparent".to_string(),
            filled_class: "border-mid-gray bg-white".to_string(),
        }
    }
}

#[component]
pub fn PinInput(
    #[prop(into, optional)] name: String,
    #[prop(optional)] length: usize,
    #[prop(into, optional, default = "*".to_string())] placeholder: String,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] digit_class: String,
    #[prop(into, optional)] focused_class: String,
    #[prop(into, optional)] filled_class: String,
    #[prop(optional, default = Callback::new(move |_| {}))] on_complete: Callback<String>,
    #[prop(optional, default = Callback::new(move |_| {}))] on_change: Callback<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional, default = false)] required: bool,
) -> impl IntoView {
    let length = length.max(1).min(12);
    let opts = PinInputOptions {
        length,
        placeholder,
        class: tw_merge!(PinInputOptions::default().class, class),
        digit_class: tw_merge!(PinInputOptions::default().digit_class, digit_class),
        focused_class: tw_merge!(PinInputOptions::default().focused_class, focused_class),
        filled_class: tw_merge!(PinInputOptions::default().filled_class, filled_class),
    };

    let (pin_digits, set_pin_digits) = signal(vec![' '; length]);
    let (focused_index, set_focused_index) = signal(0usize);

    // Store refs in StoredValue so closures can access them without move issues
    let input_refs: StoredValue<Vec<NodeRef<leptos::html::Input>>> =
        StoredValue::new((0..length).map(|_| NodeRef::new()).collect());

    let pin_value = Memo::new(move |_| {
        pin_digits
            .get()
            .into_iter()
            .filter(|&c| c != ' ')
            .collect::<String>()
    });

    // Notify parent on change
    Effect::new(move |_| {
        let val = pin_value.get();
        on_change.run(val.clone());
        if is_pin_complete(&val.chars().collect::<Vec<_>>(), length) {
            on_complete.run(val);
        }
    });

    let hidden_input_ref = NodeRef::<leptos::html::Input>::new();

    let handle_input = StoredValue::new(move |index: usize, ev: web_sys::Event| {
        let input = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(input) = input {
            let val = input.value();

            if let Some(digit) = val.chars().last().filter(|c| c.is_ascii_digit()) {
                set_pin_digits.update(|digits| {
                    if index < digits.len() {
                        digits[index] = digit;
                    }
                });

                if index + 1 < length {
                    set_focused_index.set(index + 1);
                    let refs = input_refs.get_value();
                    if let Some(next) = refs.get(index + 1) {
                        if let Some(el) = next.get() {
                            let _ = el.focus();
                        }
                    }
                }
            } else {
                input.set_value("");
            }

            if let Some(hidden) = hidden_input_ref.get() {
                hidden.set_value(&pin_value.get());
            }
        }
    });

    let handle_keydown = StoredValue::new(move |index: usize, ev: web_sys::KeyboardEvent| {
        match ev.key().as_str() {
            "Backspace" => {
                ev.prevent_default();
                let refs = input_refs.get_value();
                let current_filled = pin_digits
                    .get_untracked()
                    .get(index)
                    .map_or(false, |&c| c != ' ');

                if current_filled {
                    // Clear current field, stay put
                    set_pin_digits.update(|digits| {
                        if index < digits.len() {
                            digits[index] = ' ';
                        }
                    });
                    if let Some(el) = refs.get(index).and_then(|r| r.get()) {
                        el.set_value("");
                    }
                } else if index > 0 {
                    // Current field already empty -> clear previous, move back
                    set_pin_digits.update(|digits| {
                        digits[index - 1] = ' ';
                    });
                    set_focused_index.set(index - 1);
                    if let Some(el) = refs.get(index - 1).and_then(|r| r.get()) {
                        el.set_value("");
                        let _ = el.focus();
                    }
                }

                if let Some(hidden) = hidden_input_ref.get() {
                    hidden.set_value(&pin_value.get());
                }
            }
            "ArrowLeft" => {
                if index > 0 {
                    set_focused_index.set(index - 1);
                    let refs = input_refs.get_value();
                    if let Some(el) = refs.get(index - 1).and_then(|r| r.get()) {
                        let _ = el.focus();
                    }
                }
                ev.prevent_default();
            }
            "ArrowRight" => {
                if index + 1 < length {
                    set_focused_index.set(index + 1);
                    let refs = input_refs.get_value();
                    if let Some(el) = refs.get(index + 1).and_then(|r| r.get()) {
                        let _ = el.focus();
                    }
                }
                ev.prevent_default();
            }
            _ => {}
        }
    });

    let handle_paste = StoredValue::new(move |ev: web_sys::ClipboardEvent| {
        ev.prevent_default();
        if let Some(data) = ev.clipboard_data() {
            if let Ok(text) = data.get_data("text") {
                let digits: Vec<char> = extract_digits(&text).chars().take(length).collect();

                set_pin_digits.update(|current| {
                    for (i, &digit) in digits.iter().enumerate() {
                        if i < current.len() {
                            current[i] = digit;
                        }
                    }
                });

                let filled = digits.len().min(length - 1);
                set_focused_index.set(filled);
                let refs = input_refs.get_value();
                if let Some(el_ref) = refs.get(filled) {
                    if let Some(el) = el_ref.get() {
                        let _ = el.focus();
                    }
                }

                if let Some(hidden) = hidden_input_ref.get() {
                    hidden.set_value(&pin_value.get());
                }
            }
        }
    });

    let handle_focus = StoredValue::new(move |index: usize| {
        set_focused_index.set(index);
        let refs = input_refs.get_value();
        if let Some(input_ref) = refs.get(index) {
            if let Some(input) = input_ref.get() {
                let _ = input.select();
            }
        }
    });

    view! {
        <div class=opts.class>
            <input
                node_ref=hidden_input_ref
                type="text"
                class="sr-only"
                inputmode="numeric"
                name=name
                prop:value=move || pin_value.get()
                pattern=r"\d{6}"
                minlength=opts.length
                maxlength=opts.length
                required=required
            />

            {move || {
                let digits = pin_digits.get();
                let refs = input_refs.get_value();
                (0..opts.length).map(|i| {
                    let is_focused = focused_index.get() == i;
                    let is_filled = digits.get(i).map_or(false, |&c| c != ' ');

                    let classes = build_digit_class(
                        &opts.digit_class,
                        &opts.focused_class,
                        &opts.filled_class,
                        is_focused,
                        is_filled,
                    );
                    let placeholder_for_value = opts.placeholder.clone();

                    view! {
                        <input
                            node_ref=refs[i]
                            type="text"
                            inputmode="numeric"
                            maxlength="1"
                            autocomplete="one-time-code"
                            class=classes
                            placeholder=opts.placeholder.clone()
                            prop:value=move || {
                                let d = pin_digits.get();
                                if let Some(&c) = d.get(i) {
                                    if c != ' ' { placeholder_for_value.clone() } else { "".to_string() }
                                } else {
                                    "".to_string()
                                }
                            }
                            disabled=move || disabled.get()
                            on:input=move |ev| {
                                handle_input.with_value(|f| f(i, ev));
                            }
                            on:keydown=move |ev| {
                                handle_keydown.with_value(|f| f(i, ev));
                            }
                            on:paste=move |ev| {
                                handle_paste.with_value(|f| f(ev));
                            }
                            on:focus=move |_| {
                                handle_focus.with_value(|f| f(i));
                            }
                        />
                    }
                }).collect::<Vec<_>>()
            }}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- extract_digits ---

    #[test]
    fn extract_digits_keeps_only_ascii_digits() {
        assert_eq!(extract_digits("a1b2c3"), "123");
    }

    #[test]
    fn extract_digits_strips_unicode_digits() {
        // Unicode digits like '٣' (Arabic-Indic) are not ASCII — should be stripped.
        assert_eq!(extract_digits("1٣2"), "12");
    }

    #[test]
    fn extract_digits_empty_input() {
        assert_eq!(extract_digits(""), "");
    }

    #[test]
    fn extract_digits_all_garbage_becomes_empty() {
        assert_eq!(extract_digits("abc!@#"), "");
    }

    #[test]
    fn extract_digits_already_clean() {
        assert_eq!(extract_digits("123456"), "123456");
    }

    // --- is_pin_complete ---

    #[test]
    fn is_pin_complete_all_digits() {
        assert!(is_pin_complete(&['1', '2', '3', '4', '5', '6'], 6));
    }

    #[test]
    fn is_pin_complete_with_placeholder() {
        assert!(!is_pin_complete(&['1', '2', ' ', '4', '5', '6'], 6));
    }

    #[test]
    fn is_pin_complete_wrong_length() {
        assert!(!is_pin_complete(&['1', '2', '3'], 6));
    }

    #[test]
    fn is_pin_complete_empty() {
        assert!(!is_pin_complete(&[' '; 6], 6));
    }

    // --- build_digit_class ---

    #[test]
    fn build_digit_class_base_only() {
        let result = build_digit_class("base", "focused", "filled", false, false);
        assert_eq!(result, "base");
    }

    #[test]
    fn build_digit_class_focused_only() {
        let result = build_digit_class("base", "focused", "filled", true, false);
        assert!(result.contains("base"));
        assert!(result.contains("focused"));
        assert!(!result.contains("filled"));
    }

    #[test]
    fn build_digit_class_filled_only() {
        let result = build_digit_class("base", "focused", "filled", false, true);
        assert!(result.contains("base"));
        assert!(!result.contains("focused"));
        assert!(result.contains("filled"));
    }

    #[test]
    fn build_digit_class_both_states() {
        let result = build_digit_class("base", "focused", "filled", true, true);
        assert!(result.contains("base"));
        assert!(result.contains("focused"));
        assert!(result.contains("filled"));
    }
}
