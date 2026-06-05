use crate::components::forms::checkbox::CheckboxInputField;
use crate::utils::forms::fire_bubbled_and_cancelable_event;
use leptos::ev;
use leptos::prelude::*;

/// ToggleSwitch is a component that renders a toggle switch input field.
/// It can be used in forms to collect user input.
///
/// This is suitable for fields that have two states e.g. `on` or `off`, `true` or false.
///
/// Example usage:
/// ```
/// <ToggleSwitch
///    initial_active_state=true
///    label_active="Enabled"
///    label_inactive="Disabled"
///    name="status"
/// />
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
