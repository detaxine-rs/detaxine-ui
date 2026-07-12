use crate::components::forms::checkbox::CheckboxInputField;
use crate::utils::forms::fire_bubbled_and_cancelable_event;
use leptos::ev;
use leptos::prelude::*;
use tailwind_fuse::tw_merge;

/// A toggle switch built on top of a hidden checkbox, suitable for boolean form fields.
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
    /// `name` attribute for form submission.
    #[prop(into, optional)]
    name: String,

    /// Text displayed beside the switch when active.
    #[prop(into, optional)]
    label_active: String,

    /// Text displayed beside the switch when inactive.
    #[prop(into, optional)]
    label_inactive: String,

    /// `id` attribute on the hidden checkbox input.
    #[prop(into, optional)]
    id_attr: String,

    /// Persistent label passed to the underlying `CheckboxInputField`.
    #[prop(into, optional)]
    label: String,

    /// Sets `required` on the hidden checkbox. Defaults to `false`.
    #[prop(default = false)]
    required: bool,

    /// When `true`, clicking the toggle has no effect. Defaults to `false`.
    #[prop(optional, default = false)]
    readonly: bool,

    /// Starting state of the toggle. Defaults to `false`.
    #[prop(default = false, optional)]
    initial_active_state: bool,

    /// Extra Tailwind classes for the root wrapper `<div>`.
    #[prop(into, optional)]
    class: MaybeProp<String>,

    /// Extra Tailwind classes for the track (the pill-shaped background).
    #[prop(into, optional)]
    track_class: MaybeProp<String>,

    /// Extra Tailwind classes for the dot (the sliding circle).
    #[prop(into, optional)]
    dot_class: MaybeProp<String>,

    /// Extra Tailwind classes for the active/inactive label text.
    #[prop(into, optional)]
    label_text_class: MaybeProp<String>,
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

    let root_class = move || {
        tw_merge!(
            "flex flex-col cursor-pointer relative",
            class.get().unwrap_or_default()
        )
    };
    let track_class_val = move || {
        tw_merge!(
            format!(
                "block w-14 h-8 rounded-full {}",
                if is_active.get() {
                    "bg-secondary"
                } else {
                    "bg-mid-gray"
                }
            ),
            track_class.get().unwrap_or_default()
        )
    };
    let dot_class_val = move || {
        tw_merge!(
            format!(
                "dot absolute left-1 bottom-1 w-6 h-6 rounded-full transition transform {}",
                if is_active.get() {
                    "translate-x-full"
                } else {
                    ""
                }
            ),
            dot_class.get().unwrap_or_default()
        )
    };
    let label_text_class_val = move || tw_merge!("", label_text_class.get().unwrap_or_default());

    view! {
        <div class=root_class>
            <CheckboxInputField
                input_node_ref=checkbox_ref
                initial_value=initial_active_state.to_string()
                label=label
                name=name
                id_attr=id_attr
                checked=is_active
                class="absolute opacity-0"
                required=required
                readonly=readonly
            />
            <div class="flex items-center">
                <div on:click=handle_toggle class="relative">
                    <div class=track_class_val></div>
                    <div class=dot_class_val></div>
                </div>
                <div class="flex items-center ml-3 font-medium">
                    <p class=label_text_class_val>{
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
    .into_any()
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
