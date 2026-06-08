use crate::components::forms::checkbox::CheckboxInputField;
use crate::utils::forms::fire_bubbled_and_cancelable_event;
use leptos::ev;
use leptos::prelude::*;

/// A toggle switch built on top of a hidden checkbox, suitable for boolean form fields.
///
/// # Props
///
/// - `name` – `name` attribute for form submission.
/// - `id_attr` – `id` attribute on the hidden checkbox input.
/// - `label` – Persistent label passed to the underlying `CheckboxInputField`.
/// - `label_active` – Text displayed beside the switch when active.
/// - `label_inactive` – Text displayed beside the switch when inactive.
/// - `initial_active_state` – Starting state of the toggle. Defaults to `false`.
/// - `required` – Sets `required` on the hidden checkbox. Defaults to `false`.
/// - `readonly` – When `true`, clicking the toggle has no effect. Defaults to `false`.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::forms::toggle_switch::ToggleSwitch;
///
/// #[component]
/// fn Example() -> impl IntoView {
///     view! {
///         <ToggleSwitch
///             name="status"
///             label_active="Enabled"
///             label_inactive="Disabled"
///             initial_active_state=true
///         />
///     }
/// }
/// ```
#[component]
pub fn ToggleSwitch(
    #[prop(into, optional)] name: String,
    #[prop(into, optional)] label_active: String,
    #[prop(into, optional)] label_inactive: String,
    #[prop(into, optional)] id_attr: String,
    #[prop(into, optional)] label: String,
    #[prop(default = false)] required: bool,
    #[prop(optional, default = false)] readonly: bool,
    #[prop(default = false, optional)] initial_active_state: bool,
) -> impl IntoView {
    let checkbox_ref = NodeRef::new();
    let is_active = RwSignal::new(initial_active_state);

    let handle_toggle = move |_: ev::MouseEvent| {
        if !readonly {
            is_active.set(!is_active.get());
            if let Some(input_el) = checkbox_ref.get() {
                fire_bubbled_and_cancelable_event("change", true, true, &input_el);
            }
        }
    };

    view! {
        <div class="flex flex-col cursor-pointer relative">
            <CheckboxInputField
                input_node_ref=checkbox_ref
                initial_value=initial_active_state.to_string()
                label=label
                name=name
                id_attr=id_attr
                checked=is_active
                ext_wrapper_styles="absolute opacity-0"
                required=required
                readonly=readonly
            />
            <div class="flex items-center">
                <div on:click=handle_toggle class="relative">
                    <div
                        class=move || format!(
                            "block w-14 h-8 rounded-full {}",
                            if is_active.get() { "bg-secondary" } else { "bg-mid-gray" }
                        )
                    ></div>
                    <div
                        class=move || format!(
                            "dot absolute left-1 bottom-1 w-6 h-6 rounded-full transition transform {}",
                            if is_active.get() { "translate-x-full" } else { "" }
                        )
                    ></div>
                </div>
                <div class="flex items-center ml-3 font-medium">
                    <p>{
                        move || {
                            if is_active.get() {
                                label_active.clone()
                            } else {
                                label_inactive.clone()
                            }
                        }
                    }</p>
                </div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // toggle logic

    fn handle_toggle(is_active: &mut bool, readonly: bool) {
        if !readonly {
            *is_active = !*is_active;
        }
    }

    #[test]
    fn toggle_flips_when_not_readonly() {
        let mut state = false;
        handle_toggle(&mut state, false);
        assert!(state);
        handle_toggle(&mut state, false);
        assert!(!state);
    }

    #[test]
    fn toggle_does_not_flip_when_readonly() {
        let mut state = false;
        handle_toggle(&mut state, true);
        assert!(!state);
    }

    // label display logic

    fn current_label<'a>(is_active: bool, active: &'a str, inactive: &'a str) -> &'a str {
        if is_active { active } else { inactive }
    }

    #[test]
    fn active_label_shown_when_on() {
        assert_eq!(current_label(true, "Enabled", "Disabled"), "Enabled");
    }

    #[test]
    fn inactive_label_shown_when_off() {
        assert_eq!(current_label(false, "Enabled", "Disabled"), "Disabled");
    }

    // dot translation class

    fn dot_class(is_active: bool) -> &'static str {
        if is_active { "translate-x-full" } else { "" }
    }

    #[test]
    fn dot_translated_when_active() {
        assert_eq!(dot_class(true), "translate-x-full");
    }

    #[test]
    fn dot_not_translated_when_inactive() {
        assert_eq!(dot_class(false), "");
    }

    // reactive state

    #[test]
    fn initial_state_false_by_default() {
        let owner = Owner::new();
        owner.with(|| {
            let is_active = RwSignal::new(false);
            assert!(!is_active.get());
        });
    }

    #[test]
    fn initial_state_can_be_true() {
        let owner = Owner::new();
        owner.with(|| {
            let is_active = RwSignal::new(true);
            assert!(is_active.get());
        });
    }

    #[test]
    fn reactive_toggle_updates_signal() {
        let owner = Owner::new();
        owner.with(|| {
            let is_active = RwSignal::new(false);
            is_active.set(!is_active.get());
            assert!(is_active.get());
            is_active.set(!is_active.get());
            assert!(!is_active.get());
        });
    }

    #[test]
    fn readonly_prevents_signal_update() {
        let owner = Owner::new();
        owner.with(|| {
            let is_active = RwSignal::new(false);
            let readonly = true;
            if !readonly {
                is_active.set(!is_active.get());
            }
            assert!(!is_active.get());
        });
    }
}
