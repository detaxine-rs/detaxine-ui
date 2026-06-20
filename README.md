# detaxine-ui

A Leptos + Tailwind CSS component library compiled to WebAssembly. Build modern, reactive web UIs with a full set of accessible, themeable components - no JavaScript framework required.

[![Crates.io](https://img.shields.io/crates/v/detaxine-ui.svg)](https://crates.io/crates/detaxine-ui)
[![Docs.rs](https://docs.rs/detaxine-ui/badge.svg)](https://docs.rs/detaxine-ui)
![Detaxine UI CI](https://github.com/elonaire/detaxine-ui/actions/workflows/main.yml/badge.svg?branch=)

## Live Demo

[https://elonaire.github.io/detaxine-ui/](https://elonaire.github.io/detaxine-ui/)

---
## Limitation ⚠️

The library currently only supports CSR but, SSR support is in the pipeline.

---

## Installation

### New Project

Install the CLI and let it scaffold everything for you:

```bash
cargo install detaxine-ui-cli
dtx init my-app
cd my-app
trunk serve
```

`dtx init` creates a ready-to-run Leptos + Trunk project with `detaxine-ui`
as a dependency, `input.css` configured, and a Tailwind binary in place.

### Existing Project

`Coming soon`

---

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Trunk](https://trunkrs.dev/) - WASM bundler for Leptos

```bash
cargo install trunk
rustup target add wasm32-unknown-unknown
```

---

## Quick Start

```rust
use leptos::prelude::*;
use detaxine_ui::{components::actions::button::BasicButton, icondata::AiCheckCircleOutlined};

#[component]
fn App() -> impl IntoView {
    view! {
        <BasicButton
            button_text="Confirm"
            icon=Some(AiCheckCircleOutlined)
            icon_before=true
            style_ext="bg-primary text-white hover:bg-secondary px-6 py-2.5"
            onclick=Callback::new(|_| leptos::logging::log!("clicked"))
        />
    }
}

fn main() {
    mount_to_body(App)
}
```

---

## Components

### Actions

| Component | Description |
|---|---|
| `BasicButton` | Button with optional icon, disabled state, and custom styles |
| `ButtonGroup` | Inline group of buttons with shared styles and rounded ends |
| `Carousel` | Sliding panel carousel with navigation and indicators |

### Data Display

| Component | Description |
|---|---|
| `Badge` | Overlaid count or status indicator anchored to a child element |
| `Chip` | Removable tag with color temperature variants |
| `LabelTag` | Static color-coded label |
| `Timeline` | Vertical list of timestamped steps with icon, image, or circle heads |
| `DataTable` | Sortable, paginated table with optional row-level edit and delete actions |
| `Pagination` | Page navigation control emitting page-change callbacks |

### Feedback

| Component | Description |
|---|---|
| `BasicModal` | Portal-based dialog with contextual icons and configurable footer actions |
| `Popover` | Viewport-aware floating panel with auto-alignment and route-change close |
| `ProgressBar` | Horizontal progress bar with determinate and indeterminate modes |
| `CircularProgress` | Circular progress ring with optional percentage label |
| `Spinner` | Animated SVG loading spinner with optional full-screen backdrop |

### Forms

| Component | Description |
|---|---|
| `InputField` | All standard HTML input types via the `InputFieldType` enum |
| `CustomFileInput` | Styled file picker with chip-based selected file list |
| `Textarea` | Multi-line text input |
| `SelectInput` | Native select dropdown |
| `CustomSelectInput` | Searchable, chip-based single or multi-select |
| `CheckboxInputField` | Single checkbox input |
| `CheckboxGroup` | Grouped checkboxes with shared HashSet selection state |
| `RadioInputField` | Single radio input |
| `RadioInputGroup` | Grouped radio inputs with shared selection state |
| `ToggleSwitch` | Boolean toggle built on a hidden checkbox |
| `DatePicker` | Calendar date picker with min/max and disabled date support |
| `RichTextEditor` | Contenteditable rich text editor with formatting toolbar |
| `ReactiveForm` | Form wrapper that auto-submits whenever all fields are valid |

### Navigation

| Component | Description |
|---|---|
| `Breadcrumbs` | Route-derived breadcrumb trail with custom label support |
| `Panel` | Collapsible panel with a clickable title bar |
| `Collapse` | Accordion group of panels with optional single-open enforcement |
| `Tabs` | Scrollable tabbed view with slot-based content |
| `Stepper` | Multi-step form wizard with per-step validation and linear mode |

---

## Theming

`detaxine-ui` uses CSS custom properties for its color system. Override them in your own CSS to match your brand:

```css
@theme {
    --color-primary:        #3b82f6;
    --color-secondary:      #6366f1;
    --color-danger:         #ef4444;
    --color-success:        #22c55e;
    --color-warning:        #f59e0b;
    --color-info:           #0ea5e9;
    --color-light-gray:     #e5e7eb;
    --color-mid-gray:       #9ca3af;
    --color-contrast-white: #ffffff;
    --color-navy:           #0f172a;
}
```

All components reference these variables via Tailwind utilities (`bg-primary`, `text-danger`, etc.) so a single override updates the entire library.

---

## Style Extension

Every component exposes one or more style extension props so you can apply any Tailwind utility without forking the library:

| Prop | Applies to |
|---|---|
| `style_ext` | The root or primary element |
| `ext_input_styles` | The input or select element |
| `ext_wrapper_styles` | The outer wrapper div |
| `ext_label_styles` | The label element |
| `container_style_ext` | The modal panel |

Example - making a full-width danger button:

```rust
use leptos::prelude::*;
use detaxine_ui::{components::actions::button::BasicButton, icondata::AiCheckCircleOutlined};

#[component]
fn Example() -> impl IntoView {
    view! {
        <BasicButton
            button_text="Delete"
            style_ext="w-full bg-danger text-white hover:bg-danger/80"
        />
    }
}
```

---

## Form Handling

`ReactiveForm` wraps your fields and fires a native `submit` event automatically whenever `checkValidity()` returns true, on every `input` or `change` event. Read form data with the standard `FormData` API:

```rust
use leptos::prelude::*;
use web_sys::{HtmlFormElement, SubmitEvent};
use serde::{Deserialize, Serialize};
use detaxine_ui::{
    components::forms::{
        checkbox::{CheckboxGroup, CheckboxOption},
        reactive_form::ReactiveForm,
        input::{InputField, InputFieldType}
    },
    utils::forms::deserialize_form,
};
use leptos::wasm_bindgen::JsCast;
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
struct RegistrationForm {
    interests: Vec<String>, // note that this matches the form field name attribute
    email: String, // note that this matches the form field name attribute
}

#[component]
fn Example() -> impl IntoView {
    let selected = RwSignal::new(HashSet::new());
    let form_ref = NodeRef::new();
    let (registration_form_is_valid, set_registration_form_is_valid) = signal(false);

    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let target = ev.target()
            .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());
    
        if let Some(form) = target {
            set_registration_form_is_valid.set(form.check_validity());

            // You can use this in case you want to perform an action based on the condition that the submit event was triggered by, let's say a submit button
            if let Some(_submitter) = ev.submitter() {
                
            }
        }

        // You might also put this into it's own Effect for guaranteed reactivity
        if registration_form_is_valid.get() {
            // This is how you deserialize a form's value
            let deserialized_registration_form = deserialize_form::<RegistrationForm>(
                &form_ref,
                false,
                Some(&["interests"]), // note that this matches the form field name
            );
        }
    };

    view! {
        <ReactiveForm form_ref=form_ref on:submit=handle_submit>
            <CheckboxGroup
                legend="Interests"
                name="interests"
                options=RwSignal::new(vec![
                    CheckboxOption::new("rust", "Rust", None),
                    CheckboxOption::new("leptos", "Leptos", None),
                ])
                selected_values=selected
            />
            <InputField field_type=InputFieldType::Email name="email" label="Email" required=true />
        </ReactiveForm>
    }
}
```

---

## Modal Setup

`BasicModal` portals its content into a `#modal-root` element. Add this to your `index.html` before the closing `</body>` tag or to your root component:

```html
<div id="modal-root"></div>
```

---

## Router Requirement ⚠️

`Popover`, `DataTable` (which uses `Popover` internally), and `Breadcrumbs` all call `use_location()` from `leptos_router`. Wrap your app in a `<Router>` or else these components will panic at runtime:

```rust
use leptos_router::components::Router;

fn main() {
    mount_to_body(|| view! {
        <Router>
            // ... your root component
        </Router>
    })
}
```

---

## Rich Text Editor - Image Upload

The `RichTextEditor` defaults to base64 data URLs for image insertion. Supply a custom `on_image_insert` callback to upload to your own storage:

```rust
use leptos::prelude::*;
use detaxine_ui::components::content::richtext_editor::{ExtraFormatingOption, RichTextEditor};
use web_sys::File;
use std::pin::Pin;

#[component]
fn Example() -> impl IntoView {
    let content = RwSignal::new("<p>Hello world!</p>".to_string());

    view! {
        <RichTextEditor
            initial_content=content
            id_attr="editor"
            name="body"
            placeholder="Write something..."
            extra_formating_options=vec![
                ExtraFormatingOption::Heading,
                ExtraFormatingOption::Lists,
                ExtraFormatingOption::CodeBlock,
                ExtraFormatingOption::ImageUpload,
            ]
            on_image_insert=Callback::new(move |file: File| {
                Box::pin(async move {
                    // upload to S3, Cloudflare R2, etc. and return the URL
                    Some("https://cdn.example.com/image.png".to_string())
                }) as Pin<Box<dyn Future<Output = Option<String>>>>
            })
        />
    }
}
```

---

## Contributing

Contributions are welcome. Please open an issue before submitting a pull request for significant changes.

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/my-component`
3. Commit your changes: `git commit -m "feat: add MyComponent"`
4. Push and open a pull request

---

## License

This project is licensed under both the MIT license and the Apache License (Version 2.0).
MIT - see [LICENSE-MIT](LICENSE-MIT) for details.
APACHE - see [LICENSE-APACHE](LICENSE-APACHE) for details.
