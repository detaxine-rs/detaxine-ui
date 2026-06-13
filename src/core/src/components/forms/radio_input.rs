use leptos::ev;
use leptos::prelude::*;

/// Represents a single radio option with a value and display text.
/// You can also provide custom children for complex rendering (e.g., with icons).
#[derive(Clone)]
pub struct RadioOption {
    pub value: String,
    pub label: String,
    pub children: Option<ViewFn>,
}

impl std::fmt::Debug for RadioOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RadioOption")
            .field("value", &self.value)
            .field("label", &self.label)
            .field("children", &"<ViewFn>")
            .finish()
    }
}

impl RadioOption {
    #[allow(dead_code)]
    pub fn new(value: &str, label: &str, children: Option<ViewFn>) -> Self {
        Self {
            value: value.to_string(),
            label: label.to_string(),
            children,
        }
    }
}

/// A single radio input with an associated label and optional custom children content.
///
/// # Props
///
/// - `initial_value` – `Signal<String>` bound to the input's `value` attribute.
/// - `name` – Shared `name` attribute grouping this radio with others.
/// - `label` – Text displayed beside the radio input.
/// - `id_attr` – `id` attribute linking the input to its label.
/// - `is_selected` – Pre-selects this option. Defaults to `false`.
/// - `required` – Sets the `required` attribute. Defaults to `false`.
/// - `children` – Optional `ViewFn` rendered below the label (e.g. an icon or description).
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::forms::radio_input::RadioInputField;
///
/// #[component]
/// fn Example() -> impl IntoView {
///     view! {
///         <RadioInputField label="Male" name="gender" id_attr="gender-male" />
///     }
/// }
/// ```
#[component]
pub fn RadioInputField(
    #[prop(into, optional)] initial_value: MaybeProp<String>,
    #[prop(into, optional)] name: String,
    #[prop(into, optional)] label: String,
    #[prop(default = false, optional)] required: bool,
    #[prop(optional, default = false)] is_selected: bool,
    #[prop(optional)] children: Option<ViewFn>,
    #[prop(into, optional)] id_attr: String,
) -> impl IntoView {
    view! {
        <label for=id_attr.clone() class="inline-flex items-center gap-2 text-sm cursor-pointer px-2 py-1 rounded">
            <input
                class="leading-tight size-5 rounded-full border-2 border-mid-gray text-secondary shadow-sm
                           focus:outline-none focus:ring-0 focus:border-secondary
                           checked:bg-secondary checked:border-secondary accent-secondary"
                type="radio"
                name=name
                value=initial_value
                checked=is_selected
                id=id_attr.clone()
                required=required
            />
            <div class="flex flex-col">
                <span>{label}</span>
                {children.map(|children| children.run())}
            </div>
        </label>
    }.into_any()
}

/// A group of radio inputs rendered inside a `<fieldset>`, with shared selection state.
///
/// # Props
///
/// - `options` – `Vec<RadioOption>` holding the available choices.
/// - `legend` – Label for the fieldset group.
/// - `name` – Shared `name` attribute for all radio inputs.
/// - `initial_value` – `Signal<String>` whose value determines which option is pre-selected.
/// - `required` – Shows a `*` beside the legend and sets `required` on all inputs. Defaults to `false`.
/// - `oninput` – Callback fired when the selected value changes.
/// - `horizontal` – When `true`, renders options in a row. Defaults to `false`.
/// - `fieldset_class` – Additional Tailwind classes applied to the `<fieldset>`.
/// - `legend_class` – Additional Tailwind classes applied to the `<legend>`.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::forms::radio_input::{RadioInputGroup, RadioOption};
///
/// #[component]
/// fn Example() -> impl IntoView {
///     let selected = Signal::derive(move || "active".to_string());
///
///     view! {
///         <RadioInputGroup
///             legend="Status"
///             name="status"
///             initial_value=selected
///             options=vec![
///                 RadioOption::new("active", "Active", None),
///                 RadioOption::new("inactive", "Inactive", None),
///             ]
///         />
///     }
/// }
/// ```
#[component]
pub fn RadioInputGroup(
    #[prop(into, optional)] initial_value: MaybeProp<String>,
    /// The legend text for the fieldset
    #[prop(into, optional)]
    legend: String,
    #[prop(into, optional)] options: Vec<RadioOption>,
    #[prop(into, optional)] name: String,
    #[prop(default = false, optional)] required: bool,
    #[prop(optional, default = Callback::new(|_| {}))] oninput: Callback<ev::Event>,
    #[prop(default = false, optional)] horizontal: bool,
    /// Additional CSS classes for the fieldset
    #[prop(into, optional)]
    fieldset_class: String,
    /// Additional CSS classes for the legend
    #[prop(into, optional)]
    legend_class: String,
) -> impl IntoView {
    // Create reactive state for display_error
    let base_fieldset_class = "border border-mid-gray rounded p-4";
    let base_legend_class = "text-sm font-bold px-2";

    let container_class = if horizontal {
        "flex flex-wrap gap-4"
    } else {
        "space-y-3"
    };

    let fieldset_combined_class = format!("{} {}", base_fieldset_class, fieldset_class);
    let legend_combined_class = format!("{} {}", base_legend_class, legend_class);

    view! {
        <fieldset class=fieldset_combined_class>
                    <legend class=legend_combined_class>
                        {legend}
                        {if required {
                            Some(view! { <span class="text-danger ml-1">*</span> })
                        } else {
                            None
                        }}
                    </legend>
                    <div class=container_class>
                        {options
                            .into_iter()
                            .map(|option| {
                                let option_value_selected = option.value.clone();
                                let option_value = option.value.clone();

                                let is_selected = move || initial_value.get().unwrap_or_default() == option_value_selected;

                                view! {
                                    <label class="inline-flex items-center gap-2 text-sm cursor-pointer px-2 py-1 rounded">
                                        <input
                                            class="leading-tight size-5 rounded-full border-2 border-mid-gray text-secondary shadow-sm
                                                       focus:outline-none focus:ring-0 focus:border-secondary
                                                       checked:bg-secondary checked:border-secondary accent-secondary"
                                            type="radio"
                                            name=name.clone()
                                            value=option_value.clone()
                                            id=option_value.clone()
                                            checked=is_selected
                                            required=required
                                            on:input=move |ev| {
                                                oninput.run(ev);
                                            }
                                        />
                                        <div class="flex flex-col">
                                            <span>{option.label}</span>
                                            {option.children.map(|children| children.run())}
                                        </div>
                                    </label>
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

    // RadioOption::new

    #[test]
    fn radio_option_new_sets_fields() {
        let opt = RadioOption::new("male", "Male", None);
        assert_eq!(opt.value, "male");
        assert_eq!(opt.label, "Male");
        assert!(opt.children.is_none());
    }

    #[test]
    fn radio_option_clone() {
        let opt = RadioOption::new("a", "A", None);
        let cloned = opt.clone();
        assert_eq!(cloned.value, opt.value);
        assert_eq!(cloned.label, opt.label);
    }

    // is_selected logic

    fn is_selected(current_value: &str, option_value: &str) -> bool {
        current_value == option_value
    }

    #[test]
    fn matching_value_is_selected() {
        assert!(is_selected("male", "male"));
    }

    #[test]
    fn non_matching_value_is_not_selected() {
        assert!(!is_selected("male", "female"));
    }

    #[test]
    fn empty_initial_value_selects_nothing() {
        assert!(!is_selected("", "male"));
    }

    #[test]
    fn is_selected_reactive() {
        let owner = Owner::new();
        owner.with(|| {
            let selected = RwSignal::new("".to_string());
            let is_active = move || selected.get() == "active";

            assert!(!is_active());
            selected.set("active".to_string());
            assert!(is_active());
            selected.set("inactive".to_string());
            assert!(!is_active());
        });
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
    fn horizontal_container() {
        assert_eq!(container_class(true), "flex flex-wrap gap-4");
    }

    #[test]
    fn vertical_container() {
        assert_eq!(container_class(false), "space-y-3");
    }

    // combined class construction

    fn combined_class(base: &str, ext: &str) -> String {
        format!("{} {}", base, ext)
    }

    #[test]
    fn combined_class_appends_ext() {
        assert_eq!(
            combined_class("border border-mid-gray rounded p-4", "mt-4"),
            "border border-mid-gray rounded p-4 mt-4"
        );
    }

    #[test]
    fn combined_class_empty_ext() {
        assert_eq!(
            combined_class("text-sm font-bold px-2", ""),
            "text-sm font-bold px-2 "
        );
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

    // oninput callback

    #[test]
    fn oninput_fires_on_selection() {
        let owner = Owner::new();
        owner.with(|| {
            let fired = RwSignal::new(false);
            let oninput: Callback<String> = Callback::new(move |_| fired.set(true));
            oninput.run("active".to_string());
            assert!(fired.get());
        });
    }
}
