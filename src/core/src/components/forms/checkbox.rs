use leptos::html::*;
use leptos::prelude::*;
use std::collections::HashSet;

/// Represents a single checkbox option with a value and display text.
/// You can also provide custom children for complex rendering (e.g., with icons).
#[derive(Clone)]
#[allow(dead_code)]
pub struct CheckboxOption {
    pub value: String,
    pub label: String,
    pub children: Option<ViewFn>,
}

impl std::fmt::Debug for CheckboxOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CheckboxOption")
            .field("value", &self.value)
            .field("label", &self.label)
            .field("children", &"<ViewFn>")
            .finish()
    }
}

impl CheckboxOption {
    #[allow(dead_code)]
    pub fn new(value: &str, label: &str, children: Option<ViewFn>) -> Self {
        Self {
            value: value.to_string(),
            label: label.to_string(),
            children,
        }
    }
}

/// CheckboxInputField is a component that renders a single checkbox input field.
/// It can be used in forms to collect user input.
/// Example usage:
/// ```
/// <CheckboxInputField
///     label="Remember me"
///     name="remember"
/// />
/// ```
#[component]
pub fn CheckboxInputField(
    #[prop(into, optional)] initial_value: Signal<String>,
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] name: String,
    #[prop(optional)] input_node_ref: NodeRef<Input>,
    #[prop(default = false, optional)] readonly: bool,
    #[prop(default = false, optional)] required: bool,
    #[prop(into, default = Signal::derive(move || false), optional)] checked: Signal<bool>,
    #[prop(into, optional)] placeholder: String,
    #[prop(into, optional)] id_attr: String,
    #[prop(into, optional)] ext_input_styles: String,
    #[prop(into, optional)] ext_wrapper_styles: String,
    #[prop(into, optional,default = "off".to_string())] autocomplete: String,
) -> impl IntoView {
    view! {
        <div class=format!("{}", ext_wrapper_styles)>
            <label
                class="inline-flex items-start gap-2"
                for=id_attr.clone()
            >
                <input
                    class=format!("leading-tight size-5 shrink-0 rounded-[5px] border-2 border-mid-gray text-secondary shadow-sm
                               focus:outline-none focus:ring-0 bg-transparent focus:border-secondary
                               checked:bg-secondary checked:border-secondary
                               accent-secondary {}", ext_input_styles)
                    type="checkbox"
                    value=initial_value
                    name=name
                    node_ref=input_node_ref
                    readonly=readonly
                    placeholder=placeholder
                    autocomplete=autocomplete
                    id=id_attr.clone()
                    required=required
                    checked=checked
                />
                <div class="flex flex-col">
                    <span>{label}</span>
                </div>
            </label>
        </div>
    }
}

/// CheckboxGroup is a component that renders multiple checkboxes in a fieldset.
/// It can be used in forms to collect user input.
/// Example usage:
/// ```
/// let selected_options = RwSignal::new(HashSet::new());
/// <CheckboxGroup
///     legend="Choose your interests"
///     name="interests"
///     options=RwSignal::new(vec![
///         CheckboxOption {
///             value: "sports".to_string(),
///             label: "Sports".to_string(),
///             children: None,
///         },
///         CheckboxOption {
///             value: "music".to_string(),
///             label: "Music".to_string(),
///             children: None,
///         },
///     ])
///     selected_values=selected_options
///     required=true
/// />
/// ```
#[component]
pub fn CheckboxGroup(
    /// The legend text for the fieldset
    #[prop(into)]
    legend: String,
    /// Options for multiple checkboxes
    #[prop(into)]
    options: RwSignal<Vec<CheckboxOption>>,
    /// Selected values for multiple checkboxes
    #[prop(into, optional)]
    selected_values: RwSignal<HashSet<String>>,
    #[prop(into)] name: String,
    #[prop(default = false, optional)] readonly: bool,
    #[prop(default = false, optional)] required: bool,
    #[prop(into, optional)] ext_input_styles: String,
    #[prop(into, optional,default = "off".to_string())] autocomplete: String,
    /// Display options horizontally instead of vertically
    #[prop(default = false, optional)]
    horizontal: bool,
) -> impl IntoView {
    let base_fieldset_class = "border border-mid-gray rounded p-4";
    let base_legend_class = "text-sm font-bold px-2";

    let container_class = if horizontal {
        "flex flex-wrap gap-4"
    } else {
        "space-y-3"
    };

    let fieldset_combined_class = base_fieldset_class.to_string();
    let legend_combined_class = base_legend_class.to_string();

    view! {
        <fieldset class=fieldset_combined_class>
            <legend class=legend_combined_class>
                {legend}
                {move || required.then_some(view! {
                    <span class="text-danger ml-1">*</span>
                })}
            </legend>
            <div class=container_class>
                {move || options.get()
                    .into_iter()
                    .map(|option| {
                        let option_value = option.value.clone();
                        let option_value_checked = option.value.clone();

                        let is_checked = move || selected_values.get().contains(&option_value_checked);
                        let option_id = format!("{}-{}", name, option_value);

                        view! {
                            <div>
                                <label
                                    class="flex gap-2 text-sm cursor-pointer"
                                    for=option_id.clone()
                                >
                                    <input
                                        class=format!("leading-tight shrink-0 size-5 rounded-[5px] border-2 border-mid-gray text-secondary shadow-sm
                                                focus:outline-none focus:ring-0 focus:border-secondary bg-transparent
                                                checked:bg-secondary checked:border-secondary
                                                accent-secondary mt-0.5 {}", ext_input_styles)
                                        type="checkbox"
                                        value=option_value.clone()
                                        name=name.clone()
                                        checked=is_checked
                                        readonly=readonly
                                        autocomplete=autocomplete.clone()
                                        id=option_id.clone()
                                        required=required
                                    />
                                    <div class="flex flex-col">
                                        <span>{option.label}</span>
                                        {option.children.map(|child| child.run())}
                                    </div>
                                </label>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </fieldset>
    }
}
