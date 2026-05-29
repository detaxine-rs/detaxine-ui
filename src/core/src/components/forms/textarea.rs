use leptos::html::*;
use leptos::prelude::*;

/// This component represents a textarea input field.
/// Example usage:
/// ```
/// <Textarea
///    initial_value="Initial text"
///    label="Description"
///    name="description"
///    required=true
///    placeholder="Enter your description..."
///    ext_input_styles="bg-gray-100"
/// />
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
                    "form-input ring-0 shadow appearance-none border border-mid-gray rounded w-full py-2 px-3 leading-tight focus:outline-none focus:ring-2 focus:ring-secondary focus:border-transparent flex-grow bg-transparent {}",
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
