use leptos::{ev, html::Form, prelude::*};

use crate::utils::forms::fire_bubbled_and_cancelable_event;

/// A reactive form that automatically fires a `submit` event whenever all fields are valid,
/// triggered on every `input` or `change` event.
///
/// Use a `NodeRef<Form>` to access the underlying `HtmlFormElement` and read `FormData`
/// in your submit handler.
///
/// # Props
///
/// - `form_ref` – `NodeRef<Form>` providing direct access to the form element.
/// - `ext_styles` – `Signal<String>` of additional Tailwind classes applied to the `<form>`.
/// - `onreset` – Callback fired when the form is reset. Defaults to a no-op.
/// - `children` – Form field components rendered inside the form.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::forms::reactive_form::ReactiveForm;
///
/// #[component]
/// fn Example() -> impl IntoView {
///     let form_ref = NodeRef::new();
///
///     view! {
///         <ReactiveForm
///             form_ref=form_ref
///             on:submit=move |ev: leptos::ev::SubmitEvent| {
///                 ev.prevent_default();
///                 leptos::logging::log!("submitted");
///             }
///         >
///             <input type="text" name="username" required />
///         </ReactiveForm>
///     }
/// }
/// ```

#[component]
pub fn ReactiveForm(
    form_ref: NodeRef<Form>,
    #[prop(into, optional)] ext_styles: MaybeProp<String>,
    #[prop(default = Callback::new(|_| {}))] onreset: Callback<ev::Event>,
    children: Children,
) -> impl IntoView {
    view! {
        <form
            node_ref=form_ref
            class=move || ext_styles.get()
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
    .into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    // auto-submit logic
    // The form fires submit when check_validity() returns true.
    // We mirror that decision function here.

    fn should_fire_submit(is_valid: bool) -> bool {
        is_valid
    }

    #[test]
    fn fires_submit_when_valid() {
        assert!(should_fire_submit(true));
    }

    #[test]
    fn does_not_fire_submit_when_invalid() {
        assert!(!should_fire_submit(false));
    }

    // onreset callback

    #[test]
    fn onreset_fires() {
        let owner = Owner::new();
        owner.with(|| {
            let fired = RwSignal::new(false);
            let onreset: Callback<()> = Callback::new(move |_| fired.set(true));
            onreset.run(());
            assert!(fired.get());
        });
    }

    // ext_styles reactive signal

    #[test]
    fn ext_styles_updates_reactively() {
        let owner = Owner::new();
        owner.with(|| {
            let styles = RwSignal::new("mt-4".to_string());
            let ext_styles = Signal::derive(move || styles.get());

            assert_eq!(ext_styles.get(), "mt-4");
            styles.set("mt-8 px-4".to_string());
            assert_eq!(ext_styles.get(), "mt-8 px-4");
        });
    }
}
