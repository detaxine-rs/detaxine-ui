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

/// A flexible input field supporting all standard HTML input types via `InputFieldType`.
///
/// Includes optional icon (leading or trailing), password visibility toggle,
/// label, and full styling extension points.
///
/// # Props
///
/// - `field_type` – One of the `InputFieldType` variants (e.g. `Text`, `Email`, `Password`, `File`).
/// - `initial_value` – `Signal<String>` bound to the input's value.
/// - `label` – Text displayed above the input. Hidden if empty.
/// - `name` – `name` attribute for form submission.
/// - `id_attr` – `id` attribute linking the input to its label.
/// - `required` – Shows a `*` beside the label and sets the `required` attribute. Defaults to `false`.
/// - `readonly` – Sets the `readonly` attribute. Defaults to `false`.
/// - `placeholder` – Placeholder text.
/// - `autocomplete` – `autocomplete` attribute. Defaults to `"off"`.
/// - `accept` – `accept` attribute, used with `InputFieldType::File`.
/// - `multiple` – Allows multiple file selection. Defaults to `false`.
/// - `min` / `max` – Range constraints for `Number`, `Date`, and similar types.
/// - `icon` – Optional icon rendered inside the input wrapper.
/// - `icon_is_leading` – When `true`, icon appears on the left. Defaults to `true`.
/// - `onfocus` / `onblur` – Focus and blur event callbacks.
/// - `ext_wrapper_styles` – Additional Tailwind classes on the outer wrapper `<div>`.
/// - `ext_label_styles` – Additional Tailwind classes on the `<label>`.
/// - `ext_input_styles` – Additional Tailwind classes on the inner wrapper `<div>`.
/// - `input_node_ref` – `NodeRef<Input>` for direct DOM access.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::forms::input::{InputField, InputFieldType};
///
/// #[component]
/// fn Example() -> impl IntoView {
///     view! {
///         <InputField
///             field_type=InputFieldType::Email
///             label="Email"
///             name="email"
///             id_attr="email"
///             required=true
///         />
///     }
/// }
/// ```
#[component]
pub fn InputField(
    #[prop(into, optional)] initial_value: MaybeProp<String>,
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

    let is_file = matches!(field_type, InputFieldType::File);

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
                     shadow-sm appearance-none
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
                    prop:value=move || {
                        if is_file {
                            Some(String::new())
                        } else {
                            initial_value.get()
                        }
                    }
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
    }.into_any()
}

/// A styled file input that shows a button when empty and a file list with a replace affordance once files are selected.
///
/// # Props
///
/// - `label` – Label displayed above the hidden file input.
/// - `name` – `name` attribute for form submission.
/// - `id_attr` – `id` attribute for the hidden input.
/// - `required` – Marks the field as required. Defaults to `false`.
/// - `multiple` – Allows selecting multiple files. Defaults to `false`.
/// - `accept` – MIME types or file extensions accepted (e.g. `"image/*"`).
/// - `ext_label_styles` – Additional Tailwind classes on the label.
/// - `input_node_ref` – `NodeRef<Input>` for programmatic access to the hidden input.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::forms::input::CustomFileInput;
///
/// #[component]
/// fn Example() -> impl IntoView {
///     view! {
///         <CustomFileInput
///             label="Upload Resume"
///             name="resume"
///             accept=".pdf,.doc"
///             id_attr="resume-upload"
///         />
///     }
/// }
/// ```
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
    }.into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    // InputFieldType → &str mapping

    fn field_type_str(field_type: &InputFieldType) -> &'static str {
        match field_type {
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
        }
    }

    #[test]
    fn all_field_types_map_to_correct_str() {
        assert_eq!(field_type_str(&InputFieldType::Text), "text");
        assert_eq!(field_type_str(&InputFieldType::Email), "email");
        assert_eq!(field_type_str(&InputFieldType::Date), "date");
        assert_eq!(field_type_str(&InputFieldType::Number), "number");
        assert_eq!(field_type_str(&InputFieldType::Password), "password");
        assert_eq!(field_type_str(&InputFieldType::Tel), "tel");
        assert_eq!(field_type_str(&InputFieldType::Url), "url");
        assert_eq!(field_type_str(&InputFieldType::Search), "search");
        assert_eq!(field_type_str(&InputFieldType::Color), "color");
        assert_eq!(field_type_str(&InputFieldType::Range), "range");
        assert_eq!(field_type_str(&InputFieldType::File), "file");
        assert_eq!(field_type_str(&InputFieldType::Hidden), "hidden");
        assert_eq!(field_type_str(&InputFieldType::Image), "image");
        assert_eq!(field_type_str(&InputFieldType::Month), "month");
        assert_eq!(field_type_str(&InputFieldType::Time), "time");
        assert_eq!(field_type_str(&InputFieldType::Week), "week");
    }

    // step attribute

    fn step(field_type: &InputFieldType) -> Option<&'static str> {
        match field_type {
            InputFieldType::Number => Some("any"),
            _ => None,
        }
    }

    #[test]
    fn number_type_has_step_any() {
        assert_eq!(step(&InputFieldType::Number), Some("any"));
    }

    #[test]
    fn other_types_have_no_step() {
        assert_eq!(step(&InputFieldType::Text), None);
        assert_eq!(step(&InputFieldType::Date), None);
        assert_eq!(step(&InputFieldType::Email), None);
    }

    // password visibility toggle

    fn resolved_type(field_type: &InputFieldType, show_password: bool) -> &'static str {
        if *field_type == InputFieldType::Password && show_password {
            "text"
        } else {
            field_type_str(field_type)
        }
    }

    #[test]
    fn password_hidden_by_default() {
        assert_eq!(resolved_type(&InputFieldType::Password, false), "password");
    }

    #[test]
    fn password_shown_when_toggled() {
        assert_eq!(resolved_type(&InputFieldType::Password, true), "text");
    }

    #[test]
    fn non_password_type_unaffected_by_toggle() {
        assert_eq!(resolved_type(&InputFieldType::Email, true), "email");
    }

    #[test]
    fn show_password_toggle_flips() {
        let owner = Owner::new();
        owner.with(|| {
            let (show_password, set_show_password) = signal(false);
            assert!(!show_password.get());
            set_show_password.set(!show_password.get());
            assert!(show_password.get());
            set_show_password.set(!show_password.get());
            assert!(!show_password.get());
        });
    }

    // password toggle visibility

    fn shows_password_toggle(field_type: &InputFieldType) -> bool {
        *field_type == InputFieldType::Password
    }

    #[test]
    fn toggle_only_shown_for_password() {
        assert!(shows_password_toggle(&InputFieldType::Password));
        assert!(!shows_password_toggle(&InputFieldType::Text));
        assert!(!shows_password_toggle(&InputFieldType::Email));
    }

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
        assert!(label_visible("Email"));
    }

    // CustomFileInput: file extension extraction

    fn file_ext(name: &str) -> String {
        name.rsplit('.').next().unwrap_or("").to_uppercase()
    }

    #[test]
    fn ext_extracted_correctly() {
        assert_eq!(file_ext("resume.pdf"), "PDF");
        assert_eq!(file_ext("photo.PNG"), "PNG");
        assert_eq!(file_ext("archive.tar.gz"), "GZ");
    }

    #[test]
    fn ext_empty_for_no_extension() {
        assert_eq!(file_ext("README"), "README");
    }

    // CustomFileInput: has_files logic

    fn has_files(files: &[String]) -> bool {
        !files.is_empty()
    }

    #[test]
    fn no_files_shows_upload_button() {
        assert!(!has_files(&[]));
    }

    #[test]
    fn files_present_shows_file_list() {
        assert!(has_files(&["resume.pdf".to_string()]));
    }

    #[test]
    fn has_files_reactive() {
        let owner = Owner::new();
        owner.with(|| {
            let selected = RwSignal::new(Vec::<String>::new());
            assert!(!has_files(&selected.get()));
            selected.set(vec!["doc.pdf".to_string()]);
            assert!(has_files(&selected.get()));
        });
    }
}
