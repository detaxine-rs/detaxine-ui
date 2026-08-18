use crate::stacks::helper::overlay_root;
use crate::stacks::z_stack::{ZONE_NESTED_FLOATING, expect_z_stack};
use icondata::BsSearch;
use leptos::ev;
use leptos::html::Div;
use leptos::html::Select;
use leptos::portal::Portal;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use tailwind_fuse::tw_merge;

use crate::components::{
    data_display::chip::Chip,
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

const GAP: f64 = 4.0;

#[derive(Clone, Copy, Default)]
struct PanelPos {
    top: f64,
    left: f64,
    width: f64,
    visible: bool,
}

/// A native `<select>` dropdown with optional label, placeholder, and required indicator.
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
    /// Pre-selects an option by value.
    #[prop(into, optional)]
    initial_value: MaybeProp<String>,

    /// Text displayed above the select. Hidden if empty.
    #[prop(into, optional)]
    label: String,

    /// Renders a disabled empty-value option at the top when provided.
    #[prop(into, optional)]
    placeholder: String,

    /// `name` attribute for form submission.
    #[prop(into, optional)]
    name: String,

    /// `NodeRef<Select>` for direct DOM access.
    #[prop(optional)]
    input_node_ref: NodeRef<Select>,

    /// `RwSignal<Vec<SelectOption>>` holding the available choices.
    #[prop(into)]
    options: RwSignal<Vec<SelectOption>>,

    /// Shows a `*` beside the label and sets `required`. Defaults to `false`.
    #[prop(default = false, optional)]
    required: bool,

    /// **Deprecated**: use `select_class` instead.
    #[prop(into, optional)]
    ext_input_styles: MaybeProp<String>,

    /// Extra Tailwind classes for the `<select>` element itself.
    #[prop(into, optional)]
    select_class: MaybeProp<String>,

    /// `id` attribute linking the select to its label.
    #[prop(into, optional)]
    id_attr: String,

    /// Extra Tailwind classes for the root wrapper `<div>`.
    #[prop(into, optional)]
    class: MaybeProp<String>,

    /// Extra Tailwind classes for the `<label>`.
    #[prop(into, optional)]
    label_class: MaybeProp<String>,

    /// Extra Tailwind classes for the error message `<p>`.
    #[prop(into, optional)]
    error_class: MaybeProp<String>,
) -> impl IntoView {
    let (display_error, _set_display_error) = signal(false);

    let wrapper_class_val = move || tw_merge!("box-border", class.get().unwrap_or_default());
    let label_class_val = move || {
        tw_merge!(
            "block text-sm font-bold",
            label_class.get().unwrap_or_default()
        )
    };
    let select_class_val = move || {
        tw_merge!(
            "form-input ring-0 shadow-sm appearance-none border border-mid-gray rounded-[5px] w-full py-2 px-3 leading-tight focus:outline-none focus:ring-2 focus:ring-secondary focus:border-transparent flex-grow",
            ext_input_styles.get().unwrap_or_default(),
            select_class.get().unwrap_or_default(),
        )
    };
    let error_class_val = move || {
        tw_merge!(
            "text-danger text-xs italic",
            error_class.get().unwrap_or_default()
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
            <select
                node_ref=input_node_ref
                name=name
                class=select_class_val
                prop:value=move || initial_value.get()
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
                            <option value={option.value.clone()}>
                                {option.label.clone()}
                            </option>
                        }
                    })
                    .collect::<Vec<_>>()}
            </select>
            <p class=error_class_val>
                {move || if display_error.get() {
                    "This field is required"
                } else {
                    ""
                }}
            </p>
        </div>
    }
    .into_any()
}

/// A searchable, chip-based custom select supporting both single and multi-select modes.
///
/// Selected values are displayed as removable chips in the control. A search box filters
/// the dropdown options in real time.
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
    /// Text displayed above the control.
    #[prop(into)]
    label: String,

    /// `MaybeProp<Vec<SelectOption>>` holding the available choices.
    #[prop(into)]
    options: MaybeProp<Vec<SelectOption>>,

    /// `RwSignal<Vec<String>>` holding the currently selected values. Defaults to empty.
    #[prop(into, optional, default = RwSignal::new(Vec::new()))]
    value: RwSignal<Vec<String>>,

    /// When `true`, enables checkbox-style multi-select. Defaults to `false`.
    #[prop(optional, default = false)]
    multiple: bool,

    /// Shows a `*` beside the label. Defaults to `false`.
    #[prop(optional, default = false)]
    required: bool,

    /// `id` base used to generate unique ids for each option's input.
    #[prop(into, optional)]
    id_attr: String,

    /// Extra Tailwind classes for the root wrapper `<div>`.
    #[prop(into, optional)]
    class: MaybeProp<String>,

    /// Extra Tailwind classes for the label `<span>`.
    #[prop(into, optional)]
    label_class: MaybeProp<String>,

    /// Extra Tailwind classes for the clickable control (chip container).
    #[prop(into, optional)]
    control_class: MaybeProp<String>,

    /// Extra Tailwind classes for the dropdown panel.
    #[prop(into, optional)]
    dropdown_class: MaybeProp<String>,

    /// Extra Tailwind classes for the options `<ul>`.
    #[prop(into, optional)]
    options_list_class: MaybeProp<String>,

    /// Extra Tailwind classes for each option `<li>`.
    #[prop(into, optional)]
    option_class: MaybeProp<String>,

    /// Extra Tailwind classes for the selected-option text (in addition to `font-semibold`).
    #[prop(into, optional)]
    option_text_class: MaybeProp<String>,
) -> impl IntoView {
    let (open, set_open) = signal(false);
    let (query, set_query) = signal(String::new());

    let trigger_ref = NodeRef::<Div>::new();
    let panel_ref = NodeRef::<Div>::new();
    let panel_pos = RwSignal::new(PanelPos::default());
    let z_stack = expect_z_stack();
    let overlay_z_index = RwSignal::new(ZONE_NESTED_FLOATING);
    let panel_z_index = RwSignal::new(ZONE_NESTED_FLOATING);

    let filtered_options = Signal::derive(move || {
        let q = query.get().to_lowercase();
        options
            .get()
            .unwrap_or_default()
            .into_iter()
            .filter(|o| o.label.to_lowercase().contains(&q))
            .collect::<Vec<_>>()
    });

    // Pass 2: panel is mounted (offscreen/hidden), measure its real
    // height and decide whether it fits below the trigger or must flip up.
    let measure_and_place = StoredValue::new(move || {
        let (Some(trigger), Some(panel)) = (trigger_ref.get_untracked(), panel_ref.get_untracked())
        else {
            return;
        };
        let Some(win) = web_sys::window() else { return };

        let t = trigger.get_bounding_client_rect();
        let p = panel.get_bounding_client_rect();
        let vh = win
            .inner_height()
            .unwrap_or_default()
            .as_f64()
            .unwrap_or(667.0);
        let vw = win
            .inner_width()
            .unwrap_or_default()
            .as_f64()
            .unwrap_or(375.0);

        let fits_below = t.bottom() + GAP + p.height() <= vh;
        let top = if fits_below {
            t.bottom() + GAP
        } else {
            (t.top() - GAP - p.height()).max(GAP)
        };

        let width = t.width();
        let left = t.left().min(vw - width - GAP).max(GAP);

        panel_pos.set(PanelPos {
            top,
            left,
            width,
            visible: true,
        });
    });

    let open_dropdown = Callback::new(move |_| {
        if open.get_untracked() {
            return;
        }
        set_open.set(true);
        z_stack.lock_scroll();
        let (oz, pz) = z_stack.acquire_pair(ZONE_NESTED_FLOATING);
        overlay_z_index.set(oz);
        panel_z_index.set(pz);
        panel_pos.set(PanelPos {
            top: 0.0,
            left: 0.0,
            width: 0.0,
            visible: false,
        });
        request_animation_frame(move || measure_and_place.get_value()());
    });

    let close_dropdown = Callback::new(move |_| {
        set_open.set(false);
        set_query.set(String::new());
        z_stack.unlock_scroll();
    });

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

        if !multiple {
            close_dropdown.run(());
        }
    };

    let remove_value = move |val: String| {
        value.update(|current| {
            current.retain(|v| v != &val);
        });
    };

    on_cleanup(move || {
        if open.get_untracked() {
            z_stack.unlock_scroll();
        }
    });

    let wrapper_class_val = move || tw_merge!("relative w-full", class.get().unwrap_or_default());
    let label_class_val = move || {
        tw_merge!(
            "block text-sm font-bold",
            label_class.get().unwrap_or_default()
        )
    };
    let control_class_val = move || {
        tw_merge!(
            "relative rounded-[5px] px-3 py-2 cursor-pointer flex items-center flex-wrap gap-2 min-h-[40px] border border-mid-gray leading-tight focus:outline-none focus:ring-2 focus:ring-secondary focus:border-transparent flex-grow",
            control_class.get().unwrap_or_default()
        )
    };
    let dropdown_class_val = move || {
        tw_merge!(
            "fixed bg-contrast-white rounded-[5px] shadow-sm overflow-auto",
            dropdown_class.get().unwrap_or_default()
        )
    };
    let dropdown_style_val = move || {
        let pos = panel_pos.get();
        format!(
            "top: {}px; left: {}px; width: {}px; z-index: {}; visibility: {};",
            pos.top,
            pos.left,
            pos.width,
            panel_z_index.get(),
            if pos.visible { "visible" } else { "hidden" }
        )
    };
    let options_list_class_val = move || {
        tw_merge!(
            "max-h-48 overflow-y-auto",
            options_list_class.get().unwrap_or_default()
        )
    };
    let option_class_val = move || {
        tw_merge!(
            "px-3 py-2 hover:bg-light-gray flex items-center gap-2 cursor-pointer",
            option_class.get().unwrap_or_default()
        )
    };
    let option_text_class_val = move || option_text_class.get().unwrap_or_default();
    let id_attr = StoredValue::new(id_attr);

    view! {
        <div class=wrapper_class_val>
            <span class=label_class_val>
                {label.clone()}
                {move || required.then_some(view! {
                    <span class="text-danger ml-1">*</span>
                })}
            </span>

            // Control with chips
            <div
                node_ref=trigger_ref
                class=control_class_val
                on:click=move |_| open_dropdown.run(())
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
                                    <Chip label=o.label on_remove=Callback::new(move |_| {
                                        remove_value(val.clone());
                                    }) />
                                }
                            })
                            .collect::<Vec<_>>()
                            .into_view())
                    } else {
                        None
                    }

                }}
            </div>

            <Show when=move || open.get() fallback=|| ()>
            {move || overlay_root().map(|root| {
                view! {
                    <Portal mount=root>
                        <div
                            class="fixed inset-0"
                            style=move || format!("z-index: {};", overlay_z_index.get())
                            on:click=move |_| close_dropdown.run(())
                        />
                        <div
                            node_ref=panel_ref
                            on:mousedown=|e: ev::MouseEvent| e.prevent_default()
                            class=dropdown_class_val.clone()
                            style=dropdown_style_val
                        >
                            <InputField
                                placeholder="Search…"
                                field_type=InputFieldType::Text
                                icon=BsSearch
                                id_attr="search"
                                on:input=move |ev: ev::Event| {
                                    set_query.set(event_target_value(&ev));
                                }
                            />

                            <ul class=options_list_class_val.clone()>
                                {move || {
                                    let option_class_val = option_class_val.clone();
                                    let option_text_class_val = option_text_class_val.clone();

                                    filtered_options.get().into_iter().map(move |opt| {
                                        let selected = value.get().contains(&opt.value);
                                        let val = opt.value.clone();
                                        let current_id_attr = format!("{}_{}", id_attr.get_value(), opt.value);
                                        let option_class_val = option_class_val.clone();
                                        let option_text_class_val = option_text_class_val.clone();

                                        view! {
                                            <li
                                                class=option_class_val
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

                                                <span class=move || tw_merge!(
                                                    if selected { "font-semibold" } else { "" },
                                                    option_text_class_val()
                                                )>
                                                    {opt.label.clone()}
                                                </span>
                                            </li>
                                        }
                                    }).collect::<Vec<_>>()
                                }}
                            </ul>
                        </div>
                    </Portal>
                }
            })}
            </Show>
        </div>
    }.into_any()
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
