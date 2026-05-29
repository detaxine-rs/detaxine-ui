use leptos::{ev, html::Form, prelude::*};

use crate::utils::forms::fire_bubbled_and_cancelable_event;

/// This is a basic form component that can be used to create reactive forms.
/// It emits a `submit` event when the form is valid.
/// You can use a form ref to access the form element and its properties, including form data.
/// Example usage:
/// ```
/// let form_ref = NodeRef::new();
///
/// let handle_step_form_submit = move |ev: SubmitEvent| {
///    ev.prevent_default();
///    ev.stop_propagation();
///    // Implement logic to show form validity
///    let target = ev
///        .target()
///        .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());
///
///    if let Some(form) = target {
///        set_form_is_valid.set(form.check_validity());
///        if let Ok(form_data) = FormData::new_with_form(&form) {
///            // Implement logic to handle form data
///            // e.g. you can deserialize the form data into a struct
///            let deserialized_form_data = deserialize_form_data_to_struct::<MyFormStruct>(&form_data);
///            // Do something with the data e.g. serialize to JSON and send to the server
///        };
///    }
/// };
///
/// <ReactiveForm on:submit=handle_step_form_submit form_ref=form_ref>
///    {...form_elememts}
/// </ReactiveForm>
/// ```

#[component]
pub fn ReactiveForm(
    form_ref: NodeRef<Form>,
    #[prop(into, optional)] ext_styles: Signal<String>,
    #[prop(default = Callback::new(|_| {}))] onreset: Callback<ev::Event>,
    children: Children,
) -> impl IntoView {
    view! {
        <form
            node_ref=form_ref
            class=ext_styles
            on:input=move |_| {
                if let Some(form) = form_ref.get() {
                    if form.check_validity() {
                        fire_bubbled_and_cancelable_event("submit", true, true, &form);
                    }
                }
            }
            on:change=move |_| {
                if let Some(form) = form_ref.get() {
                    if form.check_validity() {
                        fire_bubbled_and_cancelable_event("submit", true, true, &form);
                    }
                }
            }
            on:reset=move |ev| onreset.run(ev)
        >
            {children()}
        </form>
    }
}
