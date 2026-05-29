use icondata as IconData;
use icondata::Icon as IconId;
use leptos::ev;
use leptos::html::*;
use leptos::prelude::*;
use leptos_icons::Icon;

use crate::components::actions::button::BasicButton;

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum InputFieldType {
    Text,
    Email,
    Date,
    Number,
    Password,
    Tel,
    Url,
    Search,
    Color,
    Range,
    File,
    Hidden,
    Image,
    Month,
    Time,
    Week,
}

/// This is a component for creating input fields with various types.
/// It provides a flexible way to create input fields with different types, such as text, email, date, number, password, tel, url, search, color, range, file, hidden, image, month, time and week by using the `InputFieldType` enum.
/// The component also supports various attributes such as label, name, placeholder, and event handlers.
/// Examples:
/// ```
/// <InputField field_type=InputFieldType::Text name="name" />
/// ```

#[component]
pub fn InputField(
    #[prop(into, optional)] initial_value: Signal<String>,
    #[prop(into, optional)] label: String,
    field_type: InputFieldType,
    #[prop(into, optional)] name: String,
    #[prop(optional)] input_node_ref: NodeRef<Input>,
    #[prop(default = false)] readonly: bool,
    #[prop(default = false)] required: bool,
    #[prop(into, optional)] placeholder: String,
    #[prop(into, optional)] ext_wrapper_styles: String,
    #[prop(into, optional)] ext_label_styles: String,
    #[prop(into, optional)] ext_input_styles: String,
    #[prop(into, optional, default = "off".to_string())] autocomplete: String,
    #[prop(into, optional)] id_attr: String,
    #[prop(into, optional)] accept: String,
    #[prop(into, optional)] multiple: bool,
    #[prop(optional, default = None)] icon: Option<IconId>,
    #[prop(optional, default = true)] icon_is_leading: bool,
    #[prop(into, optional)] min: String,
    #[prop(into, optional)] max: String,
    #[prop(optional, default = Callback::new(|_| {}))] onfocus: Callback<ev::FocusEvent>,
    #[prop(optional, default = Callback::new(|_| {}))] onblur: Callback<ev::FocusEvent>,
) -> impl IntoView {
    let input_field_type_str = match field_type {
        InputFieldType::Text => "text",
        InputFieldType::Email => "email",
        InputFieldType::Date => "date",
        InputFieldType::Number => "number",
        InputFieldType::Password => "password",
        InputFieldType::Tel => "tel",
        InputFieldType::Url => "url",
        InputFieldType::Search => "search",
        InputFieldType::Color => "color",
        InputFieldType::Range => "range",
        InputFieldType::File => "file",
        InputFieldType::Hidden => "hidden",
        InputFieldType::Image => "image",
        InputFieldType::Month => "month",
        InputFieldType::Time => "time",
        InputFieldType::Week => "week",
    };
    let (show_password, set_show_password) = signal(false);

    let step = match field_type {
        InputFieldType::Number => Some("any"),
        _ => None,
    };

    view! {
        <div class=move || format!("box-border {}", ext_wrapper_styles)>
            {
                if label.is_empty() {
                    None
                } else {
                    Some(
                        view! {
                            <label
                                class={format!("block text-sm font-bold {}", ext_label_styles)}
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
            <div
            class=move || format!(
                    "h-[45px] flex items-center border border-mid-gray rounded-[5px]
                     shadow appearance-none
                     focus-within:ring-2 focus-within:ring-secondary
                     focus-within:border-transparent
                     {} {}",
                    if icon_is_leading { "" } else { "flex-row-reverse" },
                    ext_input_styles
                )
                >
                {
                    icon.map(|icon_id| view!{
                        <div class=format!("h-full flex items-center px-3 justify-center")>
                            <Icon icon=icon_id width="1rem" height="1rem" />
                        </div>
                    })
                }
                <input
                    class=format!(
                        "w-full h-full py-2 px-3 leading-tight flex-grow focus:outline-none"
                    )
                    type=move || if show_password.get() { "text" } else { input_field_type_str }
                    prop:value=initial_value
                    name=name
                    node_ref=input_node_ref
                    readonly=readonly
                    placeholder=placeholder
                    autocomplete=autocomplete
                    id=id_attr.clone()
                    required=required
                    accept=accept
                    multiple=multiple
                    step=step
                    on:focus=move |e| onfocus.run(e)
                    on:blur=move |e| onblur.run(e)
                    min=min
                    max=max
                />
                {move ||
                    {
                        let show_password_val = show_password.get();

                        if field_type == InputFieldType::Password {
                            Some(
                                view!{
                                    <div on:click=move |_e| set_show_password.set(!show_password.get()) class=format!("h-full flex items-center px-3 justify-center cursor-pointer")>
                                        <Icon icon={if show_password_val { IconData::BsEyeSlash } else { IconData::BsEye }} width="1rem" height="1rem" />
                                    </div>
                                }
                            )
                        } else {
                            None
                        }
                    }
                }
            </div>
        </div>
    }
}

#[component]
pub fn CustomFileInput(
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] name: String,
    #[prop(default = false, optional)] required: bool,
    #[prop(into, optional)] id_attr: String,
    #[prop(into, optional)] ext_label_styles: String,
    #[prop(optional)] input_node_ref: NodeRef<Input>,
    #[prop(into, optional)] multiple: bool,
    #[prop(into, optional)] accept: String,
) -> impl IntoView {
    let selected_files = RwSignal::new(Vec::<String>::new());

    let has_files = move || !selected_files.get().is_empty();

    let on_change = move |_| {
        if let Some(input) = input_node_ref.get() {
            if let Some(files) = input.files() {
                let mut names = Vec::new();
                for i in 0..files.length() {
                    if let Some(file) = files.get(i) {
                        names.push(file.name());
                    }
                }
                selected_files.set(names);
            }
        }
    };

    let name_c = name.clone();
    let label_c = label.clone();
    let id_attr_c = id_attr.clone();
    let accept_c = accept.clone();

    view! {
        <div class="relative flex flex-col gap-2 box-border">
            <InputField
                name=name_c
                label=label_c
                required=required
                field_type=InputFieldType::File
                ext_input_styles="sr-only"
                id_attr=id_attr_c
                input_node_ref=input_node_ref
                multiple=multiple
                accept=accept_c
                on:change=on_change
            />

            // Upload button — shown when no files selected
            <Show when=move || !has_files()>
                <BasicButton
                    button_text="Choose File"
                    icon=Some(IconData::FiUpload)
                    icon_before=true
                    style_ext="w-full bg-primary text-contrast-white"
                    on:click=move |_| {
                        if let Some(ref input) = input_node_ref.get() {
                            input.click();
                        }
                    }
                />
            </Show>

            // File list + replace affordance — shown when files are selected
            <Show when=has_files>
                <div class="flex flex-col gap-2">
                    <For
                        each=move || {
                            selected_files
                                .get()
                                .iter()
                                .cloned()
                                .enumerate()
                                .collect::<Vec<_>>()
                        }
                        key=|(i, name)| format!("{i}-{name}")
                        children=move |(_, name)| {
                            let ext = name
                                .rsplit('.')
                                .next()
                                .unwrap_or("")
                                .to_uppercase();
                            let display_name = name.clone();

                            view! {
                                <div class="flex items-center gap-3 rounded-lg border border-border bg-surface px-3 py-2.5 text-sm shadow-sm">
                                    <div class="flex h-8 w-8 shrink-0 items-center justify-center rounded-md bg-primary/10 text-primary">
                                        <span class="h-4 w-4"><Icon icon=IconData::FiFile /></span>
                                    </div>
                                    <div class="flex flex-col min-w-0">
                                        <span class="truncate font-medium text-foreground leading-tight">
                                            {display_name}
                                        </span>
                                        <span class="text-xs text-muted uppercase tracking-wide">
                                            {ext}
                                        </span>
                                    </div>
                                </div>
                            }
                        }
                    />

                    <BasicButton
                        style_ext="mt-1 flex items-center gap-1.5 self-start text-xs text-muted underline-offset-2 hover:text-primary hover:underline transition-colors focus:outline-none cursor-pointer"
                        on:click=move |_| {
                            if let Some(ref input) = input_node_ref.get() {
                                input.click();
                            }
                        }
                    >
                        <span class="h-3 w-3"><Icon icon=IconData::FiUpload /></span>
                        "Choose different file(s)"
                    </BasicButton>
                </div>
            </Show>
        </div>
    }
}
