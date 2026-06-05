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

/// This is a custom select input component that allows you to create a dropdown menu with custom options.
/// Example usage:
/// ```
/// <SelectInput
///    initial_value="est"
///    label="Time Zone"
///    name="timezone"
///    required=true
///    options=RwSignal::new(vec![
///       SelectOption::new("", "--Select Timezone"),
///       SelectOption::new("utc", "UTC"),
///       SelectOption::new("est", "EST"),
///    ])
/// />
/// ```
/// You may use the SelectOption struct to create custom options for the SelectInput component.
#[component]
pub fn SelectInput(
    #[prop(into, optional)] initial_value: Signal<String>,
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

/// Custom Select Input
#[component]
pub fn CustomSelectInput(
    #[prop(into)] label: String,
    #[prop(into)] options: RwSignal<Vec<SelectOption>>,
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
