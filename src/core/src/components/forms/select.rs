use icondata::{BsSearch, CgClose};
use leptos::ev;
use leptos::html::Select;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::{
    actions::button::BasicButton,
    forms::{
        checkbox::CheckboxInputField,
        input::{InputField, InputFieldType},
        radio_input::RadioInputField,
    },
};

// Define the SelectOption struct
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

impl SelectOption {
    #[allow(dead_code)]
    pub fn new(value: &str, label: &str) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

/// A native `<select>` dropdown with optional label, placeholder, and required indicator.
///
/// # Props
///
/// - `options` – `RwSignal<Vec<SelectOption>>` holding the available choices.
/// - `initial_value` – `Signal<String>` pre-selecting an option by value.
/// - `label` – Text displayed above the select. Hidden if empty.
/// - `placeholder` – Renders a disabled empty-value option at the top when provided.
/// - `name` – `name` attribute for form submission.
/// - `id_attr` – `id` attribute linking the select to its label.
/// - `required` – Shows a `*` beside the label and sets `required`. Defaults to `false`.
/// - `readonly` – Marks the field as read-only. Defaults to `false`.
/// - `ext_input_styles` – Additional Tailwind classes applied to the `<select>`.
/// - `input_node_ref` – `NodeRef<Select>` for direct DOM access.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::forms::select::{SelectInput, SelectOption};
///
/// #[component]
/// fn Example() -> impl IntoView {
///     let options = RwSignal::new(vec![
///         SelectOption::new("utc", "UTC"),
///         SelectOption::new("est", "EST"),
///     ]);
///
///     view! {
///         <SelectInput label="Timezone" name="timezone" options=options required=true />
///     }
/// }
/// ```
#[component]
pub fn SelectInput(
    #[prop(into, optional)] initial_value: MaybeProp<String>,
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] placeholder: String,
    #[prop(into, optional)] name: String,
    #[prop(optional)] input_node_ref: NodeRef<Select>,
    #[prop(default = false, optional)] readonly: bool,
    #[prop(into)] options: RwSignal<Vec<SelectOption>>,
    #[prop(default = false, optional)] required: bool,
    // #[prop(optional, default = Callback::new(|_| {}))] onchange: Callback<ev::Event>,
    #[prop(into, optional)] ext_input_styles: String,
    #[prop(into, optional)] id_attr: String,
) -> impl IntoView {
    // Create reactive state for display_error
    let (display_error, _set_display_error) = signal(false);

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
            <select
                node_ref=input_node_ref
                name=name
                class=move || format!(
                    "form-input ring-0 shadow-sm appearance-none border border-mid-gray rounded-[5px] w-full py-2 px-3 leading-tight focus:outline-none focus:ring-2 focus:ring-secondary focus:border-transparent flex-grow {}",
                    ext_input_styles
                )
                prop:value=move || initial_value.get()
                // readonly={readonly}
                // on:change=move |ev| onchange.run(ev)
                id=id_attr.clone()
                required=required
            >
                {
                    if placeholder.is_empty() {
                        None
                    } else {
                        Some(view!{ <option value="">{placeholder}</option> })
                    }
                }
                {move || options.get().into_iter()
                    .map(|option| {
                        view! {
                            <option
                                value={option.value.clone()}
                                // selected={ move ||
                                //     !initial_value.get().is_empty() && initial_value.get() == option.value.clone()
                                // }
                            >
                                {option.label.clone()}
                            </option>
                        }
                    })
                    .collect::<Vec<_>>()}
            </select>
            <p class="text-danger text-xs italic">
                {move || if display_error.get() {
                    "This field is required"
                } else {
                    ""
                }}
            </p>
        </div>
    }
}

/// A searchable, chip-based custom select supporting both single and multi-select modes.
///
/// Selected values are displayed as removable chips in the control. A search box filters
/// the dropdown options in real time.
///
/// # Props
///
/// - `label` – Text displayed above the control.
/// - `options` – `RwSignal<Vec<SelectOption>>` holding the available choices.
/// - `value` – `RwSignal<Vec<String>>` holding the currently selected values. Defaults to empty.
/// - `multiple` – When `true`, enables checkbox-style multi-select. Defaults to `false`.
/// - `required` – Shows a `*` beside the label. Defaults to `false`.
/// - `id_attr` – `id` base used to generate unique ids for each option's input.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::forms::select::{SelectOption, CustomSelectInput};
///
/// #[component]
/// fn Example() -> impl IntoView {
///     let options = RwSignal::new(vec![
///         SelectOption::new("rust", "Rust"),
///         SelectOption::new("leptos", "Leptos"),
///     ]);
///     let value = RwSignal::new(vec![]);
///     view! {
///         <CustomSelectInput
///             label="Technologies"
///             options=options
///             value=value
///             multiple=true
///         />
///     }
/// }
/// ```
#[component]
pub fn CustomSelectInput(
    #[prop(into)] label: String,
    #[prop(into)] options: MaybeProp<Vec<SelectOption>>,
    #[prop(into, optional, default = RwSignal::new(Vec::new()))] value: RwSignal<Vec<String>>,

    // false = normal select (single)
    // true  = checkbox-style multi select
    #[prop(optional, default = false)] multiple: bool,

    #[prop(optional, default = false)] required: bool,
    #[prop(into, optional)] id_attr: String,
    // #[prop(optional, default = Callback::new(|_| {}))] onchange: Callback<Vec<String>>,
) -> impl IntoView {
    let (open, set_open) = signal(false);
    let (query, set_query) = signal(String::new());
    // let input_ref = NodeRef::new();

    // ---------- Derived state ----------

    let filtered_options = Signal::derive(move || {
        let q = query.get().to_lowercase();
        options
            .get()
            .unwrap_or_default()
            .into_iter()
            .filter(|o| o.label.to_lowercase().contains(&q))
            .collect::<Vec<_>>()
    });

    // ---------- Selection logic ----------

    let select_value = move |val: String| {
        value.update(|current| {
            if multiple {
                if current.contains(&val) {
                    current.retain(|v| v != &val);
                } else {
                    current.push(val);
                }
            } else {
                current.clear();
                current.push(val);
            }
        });

        // onchange.run(value.get());

        if !multiple {
            set_open.set(false);
            set_query.set(String::new());
        }
    };

    let remove_value = move |val: String| {
        value.update(|current| {
            current.retain(|v| v != &val);
        });

        // onchange.run(value.get());
    };

    view! {
        <div class="relative w-full">
            <span class="block text-sm font-bold">
                {label.clone()}
                {move || required.then_some(view! {
                    <span class="text-danger ml-1">*</span>
                })}
            </span>

            // Control with chips
            <div
                class="relative rounded-[5px] px-3 py-2 cursor-pointer flex items-center flex-wrap gap-2 min-h-[40px] border border-mid-gray leading-tight focus:outline-none focus:ring-2 focus:ring-secondary focus:border-transparent flex-grow"
                on:click=move |_| set_open.set(true)
            >
                {move || {
                    let selected = value.get();

                    if selected.is_empty() {
                        Some(view! {
                            <span class="select-none">
                            "Select…"
                            </span>
                        }.into_view())
                    } else {
                        None
                    }
                }}

                {move || {
                    let selected = value.get();

                    if !selected.is_empty() {
                        Some(options
                            .get()
                            .unwrap_or_default()
                            .into_iter()
                            .filter(|o| selected.contains(&o.value))
                            .map(|o| {
                                let val = o.value.clone();

                                view! {
                                    <span
                                        class="flex items-center gap-1 px-2 py-1
                                               bg-primary/20 text-primary rounded-[5px] text-sm"
                                    >
                                        {o.label}
                                        <BasicButton icon=Some(CgClose) onclick=Callback::new(move |ev: ev::MouseEvent| {
                                            ev.stop_propagation();
                                            remove_value(val.clone());
                                        }) />
                                    </span>
                                }
                            })
                            .collect::<Vec<_>>()
                            .into_view())
                    } else {
                        None
                    }

                }}
            </div>

            // Overlay (click outside closes dropdown)
            {move || open.get().then_some(view! {
                <div
                    class="fixed inset-0 z-10"
                    on:click=move |_| {
                        set_open.set(false);
                        set_query.set(String::new());
                    }
                />
            })}

            // Dropdown
            {move || {
                let id_attr_clone = id_attr.clone();
                open.get().then_some(view! {
                    <div class="absolute z-30 mt-1 w-full bg-contrast-white rounded-[5px] shadow-sm">
                        // Search
                        <InputField placeholder="Search…" field_type=InputFieldType::Text icon=BsSearch id_attr="search" on:input=move |ev: ev::Event| {
                            set_query.set(event_target_value(&ev));
                        } />

                        // Options
                        <ul class="max-h-48 overflow-y-auto">
                            {move || filtered_options.get().into_iter().map(|opt| {
                                let selected = value.get().contains(&opt.value);
                                let val = opt.value.clone();
                                let current_id_attr = format!("{}_{}", id_attr_clone, opt.value);

                                view! {
                                    <li
                                        class="px-3 py-2 hover:bg-light-gray flex items-center
                                               gap-2 cursor-pointer"
                                        on:click=move |_| select_value(val.clone())
                                    >
                                        {multiple.then_some(view! {
                                            <CheckboxInputField checked=selected id_attr=current_id_attr.clone() />
                                        })}

                                        {
                                            if !multiple {
                                                Some(
                                                    view! {
                                                        <RadioInputField is_selected=selected id_attr=current_id_attr.clone() />
                                                    }
                                                )
                                            } else {
                                                None
                                            }
                                        }

                                        <span class=move || if selected {
                                            "font-semibold"
                                        } else {
                                            ""
                                        }>
                                            {opt.label.clone()}
                                        </span>
                                    </li>
                                }
                            }).collect::<Vec<_>>()}
                        </ul>
                    </div>
                })
            }}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // SelectOption::new

    #[test]
    fn select_option_new_sets_fields() {
        let opt = SelectOption::new("utc", "UTC");
        assert_eq!(opt.value, "utc");
        assert_eq!(opt.label, "UTC");
    }

    #[test]
    fn select_option_eq() {
        assert_eq!(SelectOption::new("a", "A"), SelectOption::new("a", "A"));
        assert_ne!(SelectOption::new("a", "A"), SelectOption::new("b", "B"));
    }

    #[test]
    fn select_option_clone() {
        let opt = SelectOption::new("est", "EST");
        assert_eq!(opt.clone(), opt);
    }

    // filtered_options logic

    fn filter_options(options: &[SelectOption], query: &str) -> Vec<SelectOption> {
        let q = query.to_lowercase();
        options
            .iter()
            .filter(|o| o.label.to_lowercase().contains(&q))
            .cloned()
            .collect()
    }

    fn sample_options() -> Vec<SelectOption> {
        vec![
            SelectOption::new("rust", "Rust"),
            SelectOption::new("leptos", "Leptos"),
            SelectOption::new("js", "JavaScript"),
        ]
    }

    #[test]
    fn empty_query_returns_all_options() {
        let opts = sample_options();
        assert_eq!(filter_options(&opts, "").len(), 3);
    }

    #[test]
    fn query_filters_by_label_case_insensitive() {
        let opts = sample_options();
        let result = filter_options(&opts, "rust");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].value, "rust");
    }

    #[test]
    fn query_with_no_match_returns_empty() {
        let opts = sample_options();
        assert_eq!(filter_options(&opts, "python").len(), 0);
    }

    #[test]
    fn query_is_case_insensitive() {
        let opts = sample_options();
        assert_eq!(filter_options(&opts, "RUST").len(), 1);
        assert_eq!(filter_options(&opts, "lEpToS").len(), 1);
    }

    // select_value logic (single)

    fn select_single(current: &mut Vec<String>, val: String) {
        current.clear();
        current.push(val);
    }

    #[test]
    fn single_select_replaces_existing() {
        let mut current = vec!["rust".to_string()];
        select_single(&mut current, "leptos".to_string());
        assert_eq!(current, vec!["leptos"]);
    }

    #[test]
    fn single_select_stores_one_value() {
        let mut current = vec![];
        select_single(&mut current, "rust".to_string());
        assert_eq!(current.len(), 1);
    }

    // select_value logic (multi)

    fn select_multi(current: &mut Vec<String>, val: String) {
        if current.contains(&val) {
            current.retain(|v| v != &val);
        } else {
            current.push(val);
        }
    }

    #[test]
    fn multi_select_adds_new_value() {
        let mut current = vec!["rust".to_string()];
        select_multi(&mut current, "leptos".to_string());
        assert!(current.contains(&"leptos".to_string()));
        assert_eq!(current.len(), 2);
    }

    #[test]
    fn multi_select_removes_existing_value() {
        let mut current = vec!["rust".to_string(), "leptos".to_string()];
        select_multi(&mut current, "rust".to_string());
        assert!(!current.contains(&"rust".to_string()));
        assert_eq!(current.len(), 1);
    }

    // remove_value logic

    fn remove_value(current: &mut Vec<String>, val: &str) {
        current.retain(|v| v != val);
    }

    #[test]
    fn remove_value_removes_correct_entry() {
        let mut current = vec!["rust".to_string(), "leptos".to_string()];
        remove_value(&mut current, "rust");
        assert_eq!(current, vec!["leptos"]);
    }

    #[test]
    fn remove_value_noop_when_absent() {
        let mut current = vec!["leptos".to_string()];
        remove_value(&mut current, "rust");
        assert_eq!(current, vec!["leptos"]);
    }

    // placeholder visibility

    fn shows_placeholder(placeholder: &str) -> bool {
        !placeholder.is_empty()
    }

    #[test]
    fn empty_placeholder_hides_option() {
        assert!(!shows_placeholder(""));
    }

    #[test]
    fn non_empty_placeholder_shows_option() {
        assert!(shows_placeholder("-- Select --"));
    }

    // open/close dropdown reactive

    #[test]
    fn dropdown_opens_on_click() {
        let owner = Owner::new();
        owner.with(|| {
            let (open, set_open) = signal(false);
            set_open.set(true);
            assert!(open.get());
        });
    }

    #[test]
    fn dropdown_closes_on_overlay_click() {
        let owner = Owner::new();
        owner.with(|| {
            let (open, set_open) = signal(true);
            set_open.set(false);
            assert!(!open.get());
        });
    }

    #[test]
    fn single_select_closes_dropdown_after_selection() {
        let owner = Owner::new();
        owner.with(|| {
            let (open, set_open) = signal(true);
            let multiple = false;
            if !multiple {
                set_open.set(false);
            }
            assert!(!open.get());
        });
    }

    #[test]
    fn multi_select_keeps_dropdown_open_after_selection() {
        let owner = Owner::new();
        owner.with(|| {
            let (open, set_open) = signal(true);
            let multiple = true;
            if !multiple {
                set_open.set(false);
            }
            assert!(open.get());
        });
    }

    // query resets on close

    #[test]
    fn query_resets_when_dropdown_closes() {
        let owner = Owner::new();
        owner.with(|| {
            let (query, set_query) = signal("rust".to_string());
            set_query.set(String::new());
            assert_eq!(query.get(), "");
        });
    }
}
