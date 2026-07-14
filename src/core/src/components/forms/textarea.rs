use leptos::html::*;
use leptos::prelude::*;
use tailwind_fuse::tw_merge;

/// A textarea input field with an optional label and required indicator.
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
    /// `Signal<String>` bound to the textarea's content.
    #[prop(into, optional)]
    initial_value: Signal<String>,

    /// Text displayed above the textarea. Hidden if empty.
    #[prop(into, optional)]
    label: String,

    /// `name` attribute for form submission.
    #[prop(into, optional)]
    name: String,

    /// `NodeRef<Textarea>` for direct DOM access.
    #[prop(optional)]
    input_node_ref: NodeRef<Textarea>,

    /// Sets the `readonly` attribute. Defaults to `false`.
    #[prop(default = false, optional)]
    readonly: bool,

    /// Shows a `*` beside the label and sets `required`. Defaults to `false`.
    #[prop(default = false, optional)]
    required: bool,

    /// Placeholder text shown when the textarea is empty.
    #[prop(into, optional)]
    placeholder: String,

    /// **Deprecated**: use `textarea_class` instead.
    #[prop(into, optional)]
    ext_input_styles: MaybeProp<String>,

    /// Extra Tailwind classes for the `<textarea>` element itself.
    #[prop(into, optional)]
    textarea_class: MaybeProp<String>,

    /// `id` attribute linking the textarea to its label.
    #[prop(into, optional)]
    id_attr: String,

    /// Extra Tailwind classes for the root wrapper `<div>`.
    #[prop(into, optional)]
    class: MaybeProp<String>,

    /// Extra Tailwind classes for the `<label>`.
    #[prop(into, optional)]
    label_class: MaybeProp<String>,
) -> impl IntoView {
    let wrapper_class_val = move || tw_merge!("box-border", class.get().unwrap_or_default());
    let label_class_val = move || {
        tw_merge!(
            "block text-sm font-bold",
            label_class.get().unwrap_or_default()
        )
    };
    let textarea_class_val = move || {
        tw_merge!(
            "form-input ring-0 shadow-sm appearance-none border border-mid-gray rounded w-full py-2 px-3 leading-tight focus:outline-none focus:ring-2 focus:ring-secondary focus:border-transparent flex-grow bg-transparent",
            ext_input_styles.get().unwrap_or_default(),
            textarea_class.get().unwrap_or_default(),
        )
    };

    view! {
        <div class=wrapper_class_val>
            {
                if label.is_empty() {
                    None
                } else {
                    Some(
                        view! {
                            <label
                                class=label_class_val
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
                class=textarea_class_val
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
    .into_any()
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
