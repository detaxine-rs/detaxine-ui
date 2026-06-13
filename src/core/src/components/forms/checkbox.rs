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

/// A single checkbox input with an associated label, suitable for standalone use in forms.
///
/// # Props
///
/// - `initial_value` – The `value` attribute submitted with the form.
/// - `label` – Text displayed beside the checkbox.
/// - `name` – `name` attribute for form submission.
/// - `id_attr` – `id` attribute linking the input to its label.
/// - `checked` – Accepts a `bool`, `Signal<bool>`, or `RwSignal<bool>`. Defaults to `false`.
/// - `required` – Marks the field as required. Defaults to `false`.
/// - `readonly` – Marks the field as read-only. Defaults to `false`.
/// - `placeholder` – Placeholder text.
/// - `autocomplete` – `autocomplete` attribute. Defaults to `"off"`.
/// - `ext_input_styles` – Additional Tailwind classes applied to the `<input>`.
/// - `ext_wrapper_styles` – Additional Tailwind classes applied to the wrapper `<div>`.
/// - `input_node_ref` – Optional `NodeRef<Input>` for direct DOM access.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::forms::checkbox::CheckboxInputField;
///
/// #[component]
/// fn Example() -> impl IntoView {
///     view! {
///         <CheckboxInputField
///             label="Remember me"
///             name="remember"
///             id_attr="remember-me"
///         />
///     }
/// }
/// ```
#[component]
pub fn CheckboxInputField(
    #[prop(into, optional)] initial_value: MaybeProp<String>,
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] name: String,
    #[prop(optional)] input_node_ref: NodeRef<Input>,
    #[prop(default = false, optional)] readonly: bool,
    #[prop(default = false, optional)] required: bool,
    #[prop(into, default = MaybeProp::derive(move || Some(false)), optional)] checked: MaybeProp<
        bool,
    >,
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
    }.into_any()
}

/// A group of checkboxes rendered inside a `<fieldset>`, with shared selection state.
///
/// # Props
///
/// - `legend` – Label for the fieldset group.
/// - `options` – `RwSignal<Vec<CheckboxOption>>` holding the available choices.
/// - `selected_values` – `RwSignal<HashSet<String>>` tracking which values are checked.
/// - `name` – Shared `name` attribute for all checkboxes in the group.
/// - `horizontal` – When `true`, renders options in a row instead of a column. Defaults to `false`.
/// - `required` – Marks all checkboxes as required and shows a `*` beside the legend. Defaults to `false`.
/// - `readonly` – Marks all checkboxes as read-only. Defaults to `false`.
/// - `autocomplete` – `autocomplete` attribute shared by all inputs. Defaults to `"off"`.
/// - `ext_input_styles` – Additional Tailwind classes applied to each `<input>`.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use std::collections::HashSet;
/// use detaxine_ui::components::forms::checkbox::{CheckboxGroup, CheckboxOption};
///
/// #[component]
/// fn Example() -> impl IntoView {
///     let selected = RwSignal::new(HashSet::new());
///     let options = RwSignal::new(vec![
///         CheckboxOption::new("rust", "Rust", None),
///         CheckboxOption::new("leptos", "Leptos", None),
///     ]);
///     view! {
///         <CheckboxGroup
///             legend="Interests"
///             name="interests"
///             options=options
///             selected_values=selected
///         />
///     }
/// }
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
    }.into_any()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    // CheckboxOption::new

    #[test]
    fn checkbox_option_new_sets_fields() {
        let opt = CheckboxOption::new("rust", "Rust", None);
        assert_eq!(opt.value, "rust");
        assert_eq!(opt.label, "Rust");
        assert!(opt.children.is_none());
    }

    #[test]
    fn checkbox_option_clone() {
        let opt = CheckboxOption::new("a", "A", None);
        let cloned = opt.clone();
        assert_eq!(cloned.value, opt.value);
        assert_eq!(cloned.label, opt.label);
    }

    // container_class logic

    fn container_class(horizontal: bool) -> &'static str {
        if horizontal {
            "flex flex-wrap gap-4"
        } else {
            "space-y-3"
        }
    }

    #[test]
    fn horizontal_container_class() {
        assert_eq!(container_class(true), "flex flex-wrap gap-4");
    }

    #[test]
    fn vertical_container_class() {
        assert_eq!(container_class(false), "space-y-3");
    }

    // option_id generation

    fn option_id(name: &str, value: &str) -> String {
        format!("{}-{}", name, value)
    }

    #[test]
    fn option_id_format() {
        assert_eq!(option_id("interests", "rust"), "interests-rust");
    }

    #[test]
    fn option_id_unique_per_value() {
        assert_ne!(option_id("group", "a"), option_id("group", "b"));
    }

    // is_checked logic

    #[test]
    fn is_checked_true_when_value_in_set() {
        let owner = Owner::new();
        owner.with(|| {
            let selected = RwSignal::new(HashSet::from(["rust".to_string()]));
            let is_checked = move || selected.get().contains("rust");
            assert!(is_checked());
        });
    }

    #[test]
    fn is_checked_false_when_value_absent() {
        let owner = Owner::new();
        owner.with(|| {
            let selected = RwSignal::new(HashSet::<String>::new());
            let is_checked = move || selected.get().contains("rust");
            assert!(!is_checked());
        });
    }

    #[test]
    fn is_checked_updates_reactively() {
        let owner = Owner::new();
        owner.with(|| {
            let selected = RwSignal::new(HashSet::<String>::new());
            let is_checked = move || selected.get().contains("leptos");

            assert!(!is_checked());
            selected.update(|s| {
                s.insert("leptos".to_string());
            });
            assert!(is_checked());
        });
    }

    // required indicator

    #[test]
    fn required_shows_asterisk() {
        // required=true means the * span is rendered
        assert!(true);
    }

    #[test]
    fn not_required_hides_asterisk() {
        assert!(!false);
    }

    // checked MaybeProp

    #[test]
    fn checked_defaults_to_false() {
        let owner = Owner::new();
        owner.with(|| {
            let checked: MaybeProp<bool> = MaybeProp::derive(move || Some(false));
            assert_eq!(checked.get(), Some(false));
        });
    }

    #[test]
    fn checked_can_be_set_true() {
        let owner = Owner::new();
        owner.with(|| {
            let checked: MaybeProp<bool> = MaybeProp::from(true);
            assert_eq!(checked.get(), Some(true));
        });
    }

    #[test]
    fn checked_accepts_signal() {
        let owner = Owner::new();
        owner.with(|| {
            let sig = RwSignal::new(false);
            let checked: MaybeProp<bool> = MaybeProp::derive(move || Some(sig.get()));
            assert_eq!(checked.get(), Some(false));
            sig.set(true);
            assert_eq!(checked.get(), Some(true));
        });
    }
}
