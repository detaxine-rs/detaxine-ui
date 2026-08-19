use icondata::{BsDashLg, BsPlusLg};
use leptos::{html::*, prelude::*};
use tailwind_fuse::tw_merge;
use web_sys::HtmlInputElement;

use crate::{
    components::actions::button::BasicButton, utils::forms::fire_bubbled_and_cancelable_event,
};

// ── Options struct (unchanged) ──────────────────────────────────────
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

// ── Component ──────────────────────────────────────────────────────
#[component]
pub fn CustomNumberInput(
    #[prop(into)] name: String,
    #[prop(optional, default = 0)] initial_value: i64, // plain, not MaybeProp
    #[prop(optional, default = 0)] min: i64,
    #[prop(optional, default = i64::MAX)] max: i64,
    #[prop(optional, default = 1)] step: i64,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] button_class: String,
    #[prop(into, optional)] input_class: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>, // stays reactive — genuinely meant to live-update
    #[prop(into, optional, default = false)] required: bool,
    #[prop(optional)] input_node_ref: NodeRef<Input>,
) -> impl IntoView {
    let opts = NumberInputOptions {
        class: tw_merge!(NumberInputOptions::default().class, class),
        button_class: tw_merge!(NumberInputOptions::default().button_class, button_class),
        input_class: tw_merge!(NumberInputOptions::default().input_class, input_class),
    };

    // ── Pure logic (unchanged) ──────────────────────────────────────────
    // let clamp_value = move |raw: i64, min: i64, max: i64| -> i64 { raw.clamp(min, max) };

    // Seeded once, exactly like Calendar's start_month/start_year. No Effect,
    // no re-sync, no race with user interaction — there's nothing left to
    // react to.
    let count = RwSignal::new(clamp_value(initial_value, min, max));

    let step_down_disabled = Memo::new(move |_| count.get() == min);

    let step_up_disabled = Memo::new(move |_| count.get() == max);

    let commit = move |raw: i64| {
        let clamped = clamp_value(raw, min, max);
        count.set(clamped);
        if let Some(el) = input_node_ref.get_untracked() as Option<HtmlInputElement> {
            el.set_value(&clamped.to_string());
            fire_bubbled_and_cancelable_event("input", true, true, &el);
            fire_bubbled_and_cancelable_event("change", true, true, &el);
        }
    };

    let decrement = move |_| {
        let current = count.get();
        commit(current.saturating_sub(step));
    };

    let increment = move |_| {
        let current = count.get();
        commit(current.saturating_add(step));
    };

    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        let Some(current) = count.try_get_untracked() else {
            return;
        };
        match ev.key().as_str() {
            "ArrowUp" => {
                ev.prevent_default();
                commit(current.saturating_add(step));
            }
            "ArrowDown" => {
                ev.prevent_default();
                commit(current.saturating_sub(step));
            }
            "Enter" => {
                if let Some(el) = input_node_ref.get_untracked() {
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
                prop:value=move || count.get()
                disabled=move || disabled.get()
                required=required
                on:keydown=handle_keydown
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

// ── Tests (unchanged) ──────────────────────────────────────────────
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
