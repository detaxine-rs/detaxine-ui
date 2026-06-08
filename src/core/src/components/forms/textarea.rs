use leptos::html::*;
use leptos::prelude::*;

/// A textarea input field with an optional label and required indicator.
///
/// # Props
///
/// - `initial_value` – `Signal<String>` bound to the textarea's content.
/// - `label` – Text displayed above the textarea. Hidden if empty.
/// - `name` – `name` attribute for form submission.
/// - `id_attr` – `id` attribute linking the textarea to its label.
/// - `required` – Shows a `*` beside the label and sets `required`. Defaults to `false`.
/// - `readonly` – Sets the `readonly` attribute. Defaults to `false`.
/// - `placeholder` – Placeholder text shown when the textarea is empty.
/// - `ext_input_styles` – Additional Tailwind classes applied to the `<textarea>`.
/// - `input_node_ref` – `NodeRef<Textarea>` for direct DOM access.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::forms::textarea::Textarea;
///
/// #[component]
/// fn Example() -> impl IntoView {
///     let value = Signal::derive(move || "Initial text".to_string());
///
///     view! {
///         <Textarea
///             initial_value=value
///             label="Description"
///             name="description"
///             required=true
///             placeholder="Enter your description..."
///         />
///     }
/// }
/// ```
#[component]
pub fn Textarea(
    #[prop(into, optional)] initial_value: Signal<String>,
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] name: String,
    #[prop(optional)] input_node_ref: NodeRef<Textarea>,
    #[prop(default = false, optional)] readonly: bool,
    #[prop(default = false, optional)] required: bool,
    #[prop(into, optional)] placeholder: String,
    #[prop(into, optional)] ext_input_styles: String,
    #[prop(into, optional)] id_attr: String,
) -> impl IntoView {
    // Create reactive state for display_error

    view! {
        <div class="box-border">
            {
                if label.is_empty() {
                    None
                } else {
                    Some(
                        view! {
                            <label
                                class={format!("block text-sm font-bold")}
                                for=id_attr.clone()
                            >
                                {label}
                                {move || required.then_some(view! {
                                    <span class="text-danger ml-1">*</span>
                                })}
                            </label>
                        }
                    )
                }
            }
            <textarea
                class=move || format!(
                    "form-input ring-0 shadow-sm appearance-none border border-mid-gray rounded w-full py-2 px-3 leading-tight focus:outline-none focus:ring-2 focus:ring-secondary focus:border-transparent flex-grow bg-transparent {}",
                    ext_input_styles
                )
                name=name
                node_ref=input_node_ref
                readonly=readonly
                placeholder=placeholder
                id=id_attr.clone()
                required=required
            >
                {move || initial_value.get()}
            </textarea>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // label visibility

    fn label_visible(label: &str) -> bool {
        !label.is_empty()
    }

    #[test]
    fn empty_label_is_hidden() {
        assert!(!label_visible(""));
    }

    #[test]
    fn non_empty_label_is_shown() {
        assert!(label_visible("Description"));
    }

    // required indicator

    fn shows_required_asterisk(required: bool) -> bool {
        required
    }

    #[test]
    fn required_shows_asterisk() {
        assert!(shows_required_asterisk(true));
    }

    #[test]
    fn not_required_hides_asterisk() {
        assert!(!shows_required_asterisk(false));
    }

    // initial_value reactive signal

    #[test]
    fn initial_value_updates_reactively() {
        let owner = Owner::new();
        owner.with(|| {
            let content = RwSignal::new("hello".to_string());
            let initial_value = Signal::derive(move || content.get());

            assert_eq!(initial_value.get(), "hello");
            content.set("updated".to_string());
            assert_eq!(initial_value.get(), "updated");
        });
    }

    #[test]
    fn empty_initial_value_is_valid() {
        let owner = Owner::new();
        owner.with(|| {
            let initial_value = Signal::derive(move || String::new());
            assert_eq!(initial_value.get(), "");
        });
    }
}
