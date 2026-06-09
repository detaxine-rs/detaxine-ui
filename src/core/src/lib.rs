//! # detaxine-ui
//!
//! `detaxine-ui` is a Leptos + Tailwind CSS component library compiled to WebAssembly.
//! It provides a full set of accessible, themeable UI components for building modern
//! web applications with [Leptos](https://leptos.dev).
//!
//! ## Live Demo
//!
//! [https://elonaire.github.io/detaxine-ui/](https://elonaire.github.io/detaxine-ui/)
//!
//! ## Installation
//!
//! Add the crate to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! detaxine-ui = "0.8.21"
//! ```
//!
//! Then run the CLI tool to copy the required CSS into your project:
//!
//! ```bash
//! cargo install detaxine-ui-cli
//! dtx init
//! ```
//!
//! ## Available Components
//!
//! ### Actions
//!
//! | Component | Description |
//! |---|---|
//! | [`BasicButton`] | Button with optional icon, disabled state, and custom styles |
//! | [`ButtonGroup`] | Inline group of buttons with shared styles and rounded ends |
//! | [`Carousel`] | Sliding panel carousel with navigation and dot indicators |
//!
//! ### Data Display
//!
//! | Component | Description |
//! |---|---|
//! | [`Badge`] | Overlaid count or status indicator anchored to a child element |
//! | [`Chip`] | Removable tag with color temperature variants |
//! | [`LabelTag`] | Static color-coded label |
//! | [`Timeline`] | Vertical list of timestamped steps with icon/image/circle heads |
//! | [`DataTable`] | Sortable, paginated table with optional row actions |
//! | [`Pagination`] | Page navigation control |
//!
//! ### Feedback
//!
//! | Component | Description |
//! |---|---|
//! | [`BasicModal`] | Portal-based dialog with contextual icons and footer actions |
//! | [`Popover`] | Viewport-aware floating panel with auto-alignment |
//! | [`ProgressBar`] | Horizontal progress bar with determinate and indeterminate modes |
//! | [`CircularProgress`] | Circular progress ring with optional percentage label |
//! | [`Spinner`] | Animated SVG loading spinner with optional backdrop |
//!
//! ### Forms
//!
//! | Component | Description |
//! |---|---|
//! | [`InputField`] | Text, email, password, number, file and all other HTML input types |
//! | [`CustomFileInput`] | Styled file picker with chip-based selected file list |
//! | [`Textarea`] | Multi-line text input |
//! | [`SelectInput`] | Native `<select>` dropdown |
//! | [`CustomSelectInput`] | Searchable, chip-based single or multi-select |
//! | [`CheckboxInputField`] | Single checkbox input |
//! | [`CheckboxGroup`] | Grouped checkboxes with shared selection state |
//! | [`RadioInputField`] | Single radio input |
//! | [`RadioInputGroup`] | Grouped radio inputs with shared selection state |
//! | [`ToggleSwitch`] | Boolean toggle built on a hidden checkbox |
//! | [`DatePicker`] | Calendar date picker with min/max and disabled date support |
//! | [`RichTextEditor`] | Contenteditable rich text editor with formatting toolbar |
//! | [`ReactiveForm`] | Form wrapper that auto-submits on valid input/change |
//!
//! ### Navigation
//!
//! | Component | Description |
//! |---|---|
//! | [`Breadcrumbs`] | Route-derived breadcrumb trail |
//! | [`Panel`] | Collapsible panel with clickable title |
//! | [`Collapse`] | Accordion group of panels |
//! | [`Tabs`] | Scrollable tabbed view with slot-based content |
//! | [`Stepper`] | Multi-step form wizard with per-step validation |
//!
//! ## Quick Start
//!
//! ```rust
//! use detaxine_ui::leptos::prelude::*;
//! use detaxine_ui::components::actions::button::BasicButton;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <BasicButton
//!             button_text="Click me"
//!             style_ext="bg-primary text-white hover:bg-secondary"
//!             onclick=Callback::new(move |_| leptos::logging::log!("clicked"))
//!         />
//!     }
//! }
//! ```
//!
//! ## Component Examples
//!
//! ### [`BasicButton`] and [`ButtonGroup`]
//!
//! ```rust
//! use detaxine_ui::leptos::prelude::*;
//! use detaxine_ui::components::actions::button::{BasicButton, ButtonGroup};
//!
//! #[component]
//! fn Example() -> impl IntoView {
//!     view! {
//!         <ButtonGroup style_ext="bg-primary text-white hover:bg-secondary">
//!             <BasicButton button_text="Save" />
//!             <BasicButton button_text="Cancel" />
//!         </ButtonGroup>
//!     }
//! }
//! ```
//!
//! ### [`BasicModal`]
//!
//! ```rust
//! use detaxine_ui::leptos::prelude::*;
//! use detaxine_ui::components::feedback::modal::modal::{BasicModal, UseCase};
//!
//! #[component]
//! fn Example() -> impl IntoView {
//!     let is_open = RwSignal::new(false);
//!     view! {
//!         <BasicModal
//!             title="Confirm"
//!             is_open=is_open
//!             use_case=UseCase::Confirmation
//!             on_click_primary=Callback::new(|_| {})
//!             on_cancel=Callback::new(|_| {})
//!         >
//!             <p>"Are you sure?"</p>
//!         </BasicModal>
//!     }
//! }
//! ```
//!
//! ### [`DataTable`]
//!
//! ```rust
//! use detaxine_ui::leptos::prelude::*;
//! use std::collections::HashMap;
//! use detaxine_ui::components::data_display::table::data_table::{Column, DataTable, TableCellData};
//!
//! #[component]
//! fn Example() -> impl IntoView {
//!     let columns = vec![
//!         Column::new("Name", true),
//!         Column::new("Role", false),
//!     ];
//!     let rows = vec![{
//!         let mut row = HashMap::new();
//!         row.insert("id".to_string(),   TableCellData::String("1".into()));
//!         row.insert("Name".to_string(), TableCellData::String("Alice".into()));
//!         row.insert("Role".to_string(), TableCellData::String("Engineer".into()));
//!         row
//!     }];
//!     let data = RwSignal::new((columns, rows));
//!     view! {
//!         <DataTable data=data editable=true deletable=true />
//!     }
//! }
//! ```
//!
//! ### [`Stepper`]
//!
//! ```rust
//! use detaxine_ui::leptos::prelude::*;
//! use detaxine_ui::leptos::html::Form;
//! use detaxine_ui::components::navigation::stepper::{Step, Stepper, StepInfo};
//! use detaxine_ui::components::forms::input::{InputField, InputFieldType};
//!
//! #[component]
//! fn Example() -> impl IntoView {
//!     view! {
//!         <Stepper
//!             step_labels=RwSignal::new(vec![
//!                 StepInfo::new("Account", None),
//!                 StepInfo::new("Confirm", None),
//!             ])
//!             final_button_text="Finish"
//!             send_all_form_refs=Callback::new(|_| {})
//!             is_linear=true
//!         >
//!             <Step>
//!                 <InputField
//!                     field_type=InputFieldType::Email
//!                     label="Email"
//!                     name="email"
//!                     id_attr="email"
//!                     required=true
//!                 />
//!             </Step>
//!             <Step>
//!                 <p>"Review and submit."</p>
//!             </Step>
//!         </Stepper>
//!     }
//! }
//! ```
//!
//! ### [`Tabs`]
//!
//! ```rust
//! use detaxine_ui::leptos::prelude::*;
//! use detaxine_ui::components::navigation::tabs::{Tab, TabLabel, Tabs};
//!
//! #[component]
//! fn Example() -> impl IntoView {
//!     let labels = RwSignal::new(vec![
//!         TabLabel::new(ViewFn::from(|| view! { <span>"First"</span> })),
//!         TabLabel::new(ViewFn::from(|| view! { <span>"Second"</span> })),
//!     ]);
//!     view! {
//!         <Tabs tab_labels=labels>
//!             <Tab slot><p>"First tab content"</p></Tab>
//!             <Tab slot><p>"Second tab content"</p></Tab>
//!         </Tabs>
//!     }
//! }
//! ```
//!
//! ### [`Form Handling`]
//!
//! `ReactiveForm` wraps your fields and fires a native `submit` event
//! automatically whenever `checkValidity()` returns true, on every
//! `input` or `change` event. Read form data with the standard
//! `FormData` API:
//!
//! ```rust
//! use detaxine_ui::{
//!     components::forms::{
//!         checkbox::{CheckboxGroup, CheckboxOption},
//!         reactive_form::ReactiveForm,
//!         input::{InputField, InputFieldType}
//!     },
//!     leptos::prelude::*,
//!     web_sys::{HtmlFormElement, SubmitEvent},
//!     utils::forms::deserialize_form,
//!     serde::{Deserialize, Serialize}
//! };
//! use detaxine_ui::leptos::wasm_bindgen::JsCast;
//! use std::collections::HashSet;
//!
//! #[derive(Debug, Serialize, Deserialize)]
//! struct RegistrationForm {
//!     interests: Vec<String>, // note that this matches the form field name attribute
//!     email: String, // note that this matches the form field name attribute
//! }
//!
//! #[component]
//! fn Example() -> impl IntoView {
//!     let selected = RwSignal::new(HashSet::new());
//!     let form_ref = NodeRef::new();
//!     let (registration_form_is_valid, set_registration_form_is_valid) = signal(false);
//!
//!     let handle_submit = move |ev: SubmitEvent| {
//!         ev.prevent_default();
//!
//!         let target = ev.target()
//!             .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());
//!
//!         if let Some(form) = target {
//!             set_registration_form_is_valid.set(form.check_validity());
//!
//!             // You can use this in case you want to perform an action
//!             // based on the condition that the submit event was triggered
//!             // by, let's say a submit button.
//!             if let Some(_submitter) = ev.submitter() {
//!
//!             }
//!         }
//!
//!         // You might also put this into its own Effect for guaranteed reactivity.
//!         if registration_form_is_valid.get() {
//!             // This is how you deserialize a form's value.
//!             let deserialized_registration_form = deserialize_form::<RegistrationForm>(
//!                 &form_ref,
//!                 false,
//!                 Some(&["interests"]), // note that this matches the form field name
//!             );
//!         }
//!     };
//!
//!     view! {
//!         <ReactiveForm form_ref=form_ref on:submit=handle_submit>
//!             <CheckboxGroup
//!                 legend="Interests"
//!                 name="interests"
//!                 options=RwSignal::new(vec![
//!                     CheckboxOption::new("rust", "Rust", None),
//!                     CheckboxOption::new("leptos", "Leptos", None),
//!                 ])
//!                 selected_values=selected
//!             />
//!             <InputField
//!                 field_type=InputFieldType::Email
//!                 name="email"
//!                 label="Email"
//!                 required=true
//!             />
//!         </ReactiveForm>
//!     }
//! }
//! ```
//!
//! ### [`RichTextEditor`]
//!
//! ```rust
//! use detaxine_ui::leptos::prelude::*;
//! use detaxine_ui::components::content::richtext_editor::{ExtraFormatingOption, RichTextEditor};
//!
//! #[component]
//! fn Example() -> impl IntoView {
//!     let content = RwSignal::new("<p><br></p>".to_string());
//!     view! {
//!         <RichTextEditor
//!             initial_content=content
//!             id_attr="editor"
//!             name="body"
//!             extra_formating_options=vec![
//!                 ExtraFormatingOption::Heading,
//!                 ExtraFormatingOption::CodeBlock,
//!                 ExtraFormatingOption::Lists,
//!                 ExtraFormatingOption::ImageUpload,
//!             ]
//!         />
//!     }
//! }
//! ```
//!
//! ## Design Notes
//!
//! - **WebAssembly-first** — compiled to WASM via `wasm-pack` / `trunk`, with full
//!   access to `web_sys` APIs for DOM interaction.
//! - **Tailwind CSS v4** — styles are distributed via `input.css` using `@apply`
//!   and `@source inline()` so the library's classes are always included in the
//!   consumer's build regardless of scanning boundaries.
//! - **Reactive by default** — all stateful props accept `Signal`, `RwSignal`, or
//!   `MaybeProp` so components integrate naturally into any Leptos reactive graph.
//! - **Accessible** — form components forward `id`, `name`, `required`, `readonly`,
//!   and `aria-*` attributes to the underlying HTML elements.
//! - **Zero opinion on layout** — components expose `style_ext`, `ext_input_styles`,
//!   `ext_wrapper_styles`, and similar props so consumers can apply any Tailwind
//!   utility without forking the library.

pub mod components;
pub mod utils;

// Re-exports
// Consumers import everything they need directly from detaxine_ui,
// eliminating the need to add these crates individually to their Cargo.toml and avoiding version conflicts.

pub use chrono;
pub use gloo;
pub use gloo_file;
pub use icondata;
pub use js_sys;
pub use leptos;
pub use leptos_icons;
pub use leptos_meta;
pub use leptos_router;
pub use serde;
pub use serde_json;
pub use wasm_bindgen_futures;
pub use web_sys;
