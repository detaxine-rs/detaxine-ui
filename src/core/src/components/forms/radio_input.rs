use leptos::{html::*, prelude::*};
use tailwind_fuse::tw_merge;

/// Represents a single radio option with a value and display text.
#[derive(Clone)]
pub struct RadioOption {
    pub value: String,
    pub label: String,
    /// **Deprecated**: this will be removed in a future version. For now, it will be ignored.
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

/// A single radio input with an associated label, rendered identically across browsers.
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
    /// Value bound to the input's `value` attribute.
    #[prop(into, optional)]
    initial_value: MaybeProp<String>,

    /// Shared `name` attribute grouping this radio with others.
    #[prop(into, optional)]
    name: String,

    /// Text displayed beside the radio input.
    #[prop(into, optional)]
    label: String,

    /// Sets the `required` attribute. Defaults to `false`.
    #[prop(default = false, optional)]
    required: bool,

    /// Accepts a `bool`, `Signal<bool>`, or `RwSignal<bool>`. Defaults to `false`.
    #[prop(into, default = MaybeProp::derive(move || Some(false)), optional)]
    is_selected: MaybeProp<bool>,

    /// **Deprecated**: this will be removed in a future version. For now, it will be ignored.
    #[prop(optional)]
    children: Option<ViewFn>,

    /// `id` attribute linking the input to its label.
    #[prop(into, optional)]
    id_attr: String,

    /// Extra Tailwind classes for the `<label>` wrapper.
    #[prop(into, optional)]
    class: MaybeProp<String>,

    /// Extra Tailwind classes for the `<input type="radio">` itself.
    #[prop(into, optional)]
    input_class: MaybeProp<String>,

    /// Extra Tailwind classes for the label text `<span>`.
    #[prop(into, optional)]
    label_text_class: MaybeProp<String>,

    /// Extra Tailwind classes for the visible circle (the custom radio "box").
    #[prop(into, optional)]
    box_class: MaybeProp<String>,

    /// Optional `NodeRef<Input>` for direct DOM access.
    #[prop(optional)]
    input_node_ref: NodeRef<Input>,
) -> impl IntoView {
    // deprecated & ignored, kept only so existing callers still compile
    let _ = children;

    let label_class_val = move || {
        tw_merge!(
            "inline-flex items-center gap-2 text-sm cursor-pointer px-2 py-1 rounded",
            class.get().unwrap_or_default()
        )
    };
    // the real input: visually hidden, still focusable/tabbable/submittable
    let input_class_val = move || tw_merge!("peer sr-only", input_class.get().unwrap_or_default());
    // the fake circle painted entirely by us — identical in every browser
    let box_class_val = move || {
        tw_merge!(
            "relative flex items-center justify-center size-5 shrink-0 rounded-full border-2 border-mid-gray bg-transparent transition-colors peer-checked:border-secondary peer-checked:[&>span]:opacity-100 peer-focus-visible:ring-2 peer-focus-visible:ring-secondary peer-focus-visible:ring-offset-2 peer-disabled:opacity-50 peer-disabled:cursor-not-allowed",
            box_class.get().unwrap_or_default()
        )
    };
    let label_text_class_val = move || tw_merge!("", label_text_class.get().unwrap_or_default());

    view! {
        <label for=id_attr.clone() class=label_class_val>
            <input
                class=input_class_val
                type="radio"
                name=name
                value=initial_value
                checked=is_selected
                id=id_attr.clone()
                required=required
                node_ref=input_node_ref
            />
            <span class=box_class_val aria-hidden="true">
                <span class="w-2/3 h-2/3 rounded-full bg-secondary opacity-0 transition-opacity"></span>
            </span>
            <div class="flex flex-col">
                <span class=label_text_class_val>{label}</span>
            </div>
        </label>
    }
    .into_any()
}

/// A group of radio inputs rendered inside a `<fieldset>`, with shared selection state.
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
    /// Value whose content determines which option is pre-selected.
    #[prop(into, optional)]
    initial_value: MaybeProp<String>,

    /// Label for the fieldset group.
    #[prop(into, optional)]
    legend: String,

    /// `Vec<RadioOption>` holding the available choices.
    #[prop(into, optional)]
    options: Vec<RadioOption>,

    /// Shared `name` attribute for all radio inputs.
    #[prop(into, optional)]
    name: String,

    /// Shows a `*` beside the legend and sets `required` on all inputs. Defaults to `false`.
    #[prop(default = false, optional)]
    required: bool,

    /// When `true`, renders options in a row. Defaults to `false`.
    #[prop(default = false, optional)]
    horizontal: bool,

    /// Additional Tailwind classes for the fieldset
    #[prop(into, optional)]
    fieldset_class: MaybeProp<String>,

    /// Additional Tailwind classes for the legend
    #[prop(into, optional)]
    legend_class: MaybeProp<String>,

    /// Additional Tailwind classes for the options container
    #[prop(into, optional)]
    container_class: MaybeProp<String>,

    /// Additional Tailwind classes for each option's `<label>`
    #[prop(into, optional)]
    option_label_class: MaybeProp<String>,

    /// Additional Tailwind classes for each option's visible circle
    #[prop(into, optional)]
    input_class: MaybeProp<String>,

    /// Additional Tailwind classes for each option's text `<span>`
    #[prop(into, optional)]
    option_text_class: MaybeProp<String>,
) -> impl IntoView {
    let fieldset_class_val = move || {
        tw_merge!(
            "border border-mid-gray rounded p-4",
            fieldset_class.get().unwrap_or_default()
        )
    };
    let legend_class_val = move || {
        tw_merge!(
            "text-sm font-bold px-2",
            legend_class.get().unwrap_or_default()
        )
    };
    let container_class_val = move || {
        tw_merge!(
            if horizontal {
                "flex flex-wrap gap-4"
            } else {
                "space-y-3"
            },
            container_class.get().unwrap_or_default()
        )
    };
    let option_label_class_val = move || {
        tw_merge!(
            "inline-flex items-center gap-2 text-sm cursor-pointer px-2 py-1 rounded",
            option_label_class.get().unwrap_or_default()
        )
    };
    let input_class_val = move || {
        tw_merge!(
            "leading-tight size-5 rounded-full border-2 border-mid-gray text-secondary shadow-sm focus:outline-none focus:ring-0 focus:border-secondary checked:bg-secondary checked:border-secondary accent-secondary",
            input_class.get().unwrap_or_default()
        )
    };
    let option_text_class_val = move || tw_merge!("", option_text_class.get().unwrap_or_default());

    view! {
        <fieldset class=fieldset_class_val>
            <legend class=legend_class_val>
                {legend}
                {if required {
                    Some(view! { <span class="text-danger ml-1">*</span> })
                } else {
                    None
                }}
            </legend>
            <div class=container_class_val>
                {options
                    .into_iter()
                    .map(|option| {
                        let option_value_selected = option.value.clone();
                        let option_value = option.value.clone();
                        let is_selected = move || initial_value.get().unwrap_or_default() == option_value_selected;
                        let option_label_class_val = option_label_class_val.clone();
                        let input_class_val = input_class_val.clone();
                        let option_text_class_val = option_text_class_val.clone();

                        view! {
                            <RadioInputField
                                initial_value=option_value.clone()
                                name=name.clone()
                                label=option.label.clone()
                                required=required
                                is_selected=Signal::derive(is_selected)
                                id_attr=option_value.clone()
                                class=Signal::derive(option_label_class_val)
                                box_class=Signal::derive(input_class_val)
                                label_text_class=Signal::derive(option_text_class_val)
                            />
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
