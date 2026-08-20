use icondata::{BsDashLg, BsPlusLg};
use leptos::{ev, html::*, prelude::*};
use tailwind_fuse::tw_merge;

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

fn clamp_value(raw: i64, min: i64, max: i64) -> i64 {
    raw.clamp(min, max)
}

#[component]
pub fn CustomNumberInput(
    #[prop(into)] name: String,
    /// Reactive, externally-owned value. This component never stores its
    /// own copy — it always displays exactly what the parent gives it.
    #[prop(into)]
    value: Signal<i64>,
    /// Called with the proposed new value on every button press, arrow
    /// key, or typed change. The parent decides whether/how to apply it
    /// (e.g. writing to cart) — the input's displayed value only changes
    /// once that write flows back through `value`.
    #[prop(into, optional, default = Callback::new(move |_| {}))]
    on_change: Callback<i64>,
    #[prop(optional, into, default = MaybeProp::derive(move || Some(0)))] min: MaybeProp<i64>,
    #[prop(optional, into, default = MaybeProp::derive(move || Some(i64::MAX)))] max: MaybeProp<
        i64,
    >,
    #[prop(optional, default = 1)] step: i64,
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

    let step_down_disabled = Memo::new(move |_| value.get() <= min.get().unwrap_or_default());
    let step_up_disabled = Memo::new(move |_| value.get() >= max.get().unwrap_or_default());

    let commit = move |raw: i64| {
        let clamped = clamp_value(
            raw,
            min.get().unwrap_or_default(),
            max.get().unwrap_or_default(),
        );
        on_change.run(clamped);
    };

    let decrement = move |_| commit(value.get_untracked().saturating_sub(step));
    let increment = move |_| commit(value.get_untracked().saturating_add(step));

    let handle_keydown = move |ev: ev::KeyboardEvent| match ev.key().as_str() {
        "ArrowUp" => {
            ev.prevent_default();
            commit(value.get_untracked().saturating_add(step));
        }
        "ArrowDown" => {
            ev.prevent_default();
            commit(value.get_untracked().saturating_sub(step));
        }
        "Enter" => {
            if let Some(el) = input_node_ref.get_untracked() {
                let _ = el.blur();
            }
        }
        _ => {}
    };

    let handle_input_change = move |ev: ev::Event| {
        if let Ok(raw) = event_target_value(&ev).parse::<i64>() {
            commit(raw);
        }
    };

    view! {
        <div class=opts.class>
            <BasicButton
                class=tw_merge!("{} rounded-r-none", opts.button_class.clone())
                disabled=step_down_disabled
                on:click=decrement
                icon=Some(BsDashLg)
            />
            <input
                node_ref=input_node_ref
                type="text"
                inputmode="numeric"
                pattern="-?[0-9]*"
                name=name
                class=opts.input_class
                prop:value=move || value.get()
                disabled=move || disabled.get()
                required=required
                on:keydown=handle_keydown
                on:change=handle_input_change
            />
            <BasicButton
                class=tw_merge!("{} rounded-l-none", opts.button_class.clone())
                disabled=step_up_disabled
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
}
