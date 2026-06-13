use detaxine_ui::components::actions::button::{BasicButton, ButtonGroup};
use detaxine_ui::components::content::carousel::Carousel;
use detaxine_ui::components::content::collapse::{Collapse, Panel, PanelInfo};
use detaxine_ui::components::content::richtext_editor::{ExtraFormatingOption, RichTextEditor};
use detaxine_ui::components::data_display::badge::Badge;
use detaxine_ui::components::data_display::chip::Chip;
use detaxine_ui::components::data_display::table::data_table::{Column, DataTable, TableCellData};
use detaxine_ui::components::data_display::tag::LabelTag;
use detaxine_ui::components::data_display::timeline::{Timeline, TimelineItem, TimelineStatus};
use detaxine_ui::components::feedback::modal::modal::{BasicModal, UseCase};
use detaxine_ui::components::feedback::popover::Popover;
use detaxine_ui::components::feedback::progress::{
    CircularProgress, ProgressBar, ProgressComponentSize,
};
use detaxine_ui::components::feedback::spinner::Spinner;
use detaxine_ui::components::feedback::spinner::SpinnerSize;
use detaxine_ui::components::forms::checkbox::{CheckboxGroup, CheckboxInputField, CheckboxOption};
use detaxine_ui::components::forms::datepicker::DatePicker;
use detaxine_ui::components::forms::input::{CustomFileInput, InputField, InputFieldType};
use detaxine_ui::components::forms::radio_input::{RadioInputField, RadioInputGroup, RadioOption};
use detaxine_ui::components::forms::select::{CustomSelectInput, SelectInput, SelectOption};
use detaxine_ui::components::forms::textarea::Textarea;
use detaxine_ui::components::forms::toggle_switch::ToggleSwitch;
use detaxine_ui::components::navigation::breadcrumbs::Breadcrumbs;
use detaxine_ui::components::navigation::stepper::{Step, StepInfo, Stepper};
use detaxine_ui::components::navigation::tabs::{Tab, TabLabel, Tabs};
use detaxine_ui::components::schemas::props::ColorTemperature;
use icondata::{AiCheckCircleOutlined, BsXCircle};
use leptos::html::*;
use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_router::components::Router;
use std::collections::HashMap;
use std::collections::HashSet;

#[component]
pub fn App() -> impl IntoView {
    // ── Section refs ─────────────────────────────────────────────
    let buttons_ref: NodeRef<Div> = NodeRef::new();
    let carousel_ref: NodeRef<Div> = NodeRef::new();
    let badges_ref: NodeRef<Div> = NodeRef::new();
    let chips_ref: NodeRef<Div> = NodeRef::new();
    let labels_ref: NodeRef<Div> = NodeRef::new();
    let timeline_ref: NodeRef<Div> = NodeRef::new();
    let table_ref: NodeRef<Div> = NodeRef::new();
    let modal_ref: NodeRef<Div> = NodeRef::new();
    let popover_ref: NodeRef<Div> = NodeRef::new();
    let progress_ref: NodeRef<Div> = NodeRef::new();
    let spinner_ref: NodeRef<Div> = NodeRef::new();
    let inputs_ref: NodeRef<Div> = NodeRef::new();
    let selects_ref: NodeRef<Div> = NodeRef::new();
    let checks_ref: NodeRef<Div> = NodeRef::new();
    let radios_ref: NodeRef<Div> = NodeRef::new();
    let textarea_ref: NodeRef<Div> = NodeRef::new();
    let toggle_ref: NodeRef<Div> = NodeRef::new();
    let datepick_ref: NodeRef<Div> = NodeRef::new();
    let panel_ref: NodeRef<Div> = NodeRef::new();
    let tabs_ref: NodeRef<Div> = NodeRef::new();
    let stepper_ref: NodeRef<Div> = NodeRef::new();
    let breadcrumb_ref: NodeRef<Div> = NodeRef::new();
    let richtext_ref: NodeRef<Div> = NodeRef::new();

    let drawer_open = RwSignal::new(false);

    let scroll_to = |node_ref: NodeRef<Div>, close_drawer: RwSignal<bool>| {
        move |_| {
            close_drawer.set(false);
            if let Some(el) = node_ref.get() {
                el.scroll_into_view_with_bool(true);
            }
        }
    };

    // ── Component state ──────────────────────────────────────────
    let modal_open = RwSignal::new(false);
    let confirm_open = RwSignal::new(false);
    let popover_showing = RwSignal::new(false);
    let progress_val = RwSignal::new(65.0f64);
    let circular_val = RwSignal::new(42.0f64);
    let checkbox_selected = RwSignal::new(HashSet::<String>::new());
    let radio_selected = Signal::derive(move || "option1".to_string());
    let custom_select_val = RwSignal::new(Vec::<String>::new());

    let table_data = RwSignal::new({
        let columns = vec![
            Column::new("Name", true),
            Column::new("Role", true),
            Column::new("Status", false),
            Column::new("Joined", true),
        ];
        let rows: Vec<HashMap<String, TableCellData>> = vec![
            [
                ("id", TableCellData::String("1".into())),
                ("Name", TableCellData::String("Alice Mwangi".into())),
                ("Role", TableCellData::String("Engineer".into())),
                ("Status", TableCellData::Bool(true)),
                (
                    "Joined",
                    TableCellData::DateTime("2023-03-15T00:00:00Z".into()),
                ),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect(),
            [
                ("id", TableCellData::String("2".into())),
                ("Name", TableCellData::String("Bob Otieno".into())),
                ("Role", TableCellData::String("Designer".into())),
                ("Status", TableCellData::Bool(false)),
                (
                    "Joined",
                    TableCellData::DateTime("2022-11-01T00:00:00Z".into()),
                ),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect(),
            [
                ("id", TableCellData::String("3".into())),
                ("Name", TableCellData::String("Carol Njeri".into())),
                ("Role", TableCellData::String("Product".into())),
                ("Status", TableCellData::Bool(true)),
                (
                    "Joined",
                    TableCellData::DateTime("2024-01-20T00:00:00Z".into()),
                ),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect(),
        ];
        (columns, rows)
    });

    let timeline_steps = RwSignal::new(vec![
        TimelineItem::builder(
            "3 mins ago",
            "Project created",
            false,
            ViewFn::from(
                || view! { <p class="text-sm">"Initialized with Leptos and Tailwind."</p> },
            ),
        )
        .completed()
        .build(),
        TimelineItem::builder(
            "2 mins ago",
            "Components built",
            false,
            ViewFn::from(|| view! { <p class="text-sm">"UI components added to the library."</p> }),
        )
        .completed()
        .build(),
        TimelineItem::builder(
            "1 min ago",
            "Deploy pending",
            true,
            ViewFn::from(|| view! { <p class="text-sm">"Awaiting CI pipeline."</p> }),
        )
        .status(TimelineStatus::Warning)
        .build(),
    ]);

    let collapse_panels = RwSignal::new(vec![
        PanelInfo::builder(
            ViewFn::from(|| view! { <span>"What is detaxine-ui?"</span> }),
            ViewFn::from(
                || view! { <p class="text-sm">"A Leptos + Tailwind component library."</p> },
            ),
        )
        .build(),
        PanelInfo::builder(
            ViewFn::from(|| view! { <span>"Is it production ready?"</span> }),
            ViewFn::from(|| view! { <p class="text-sm">"Approaching v1. Use with care."</p> }),
        )
        .build(),
        PanelInfo::builder(
            ViewFn::from(|| view! { <span>"How do I install it?"</span> }),
            ViewFn::from(
                || view! { <p class="text-sm">"Add it as a dependency and run dtx init."</p> },
            ),
        )
        .build(),
    ]);

    let tab_labels = RwSignal::new(vec![
        TabLabel::new(ViewFn::from(|| view! { <span>"Overview"</span> })),
        TabLabel::new(ViewFn::from(|| view! { <span>"Usage"</span> })),
        TabLabel::new(ViewFn::from(|| view! { <span>"API"</span> })),
    ]);

    let nav_items = vec![
        ("Buttons", buttons_ref),
        ("Carousel", carousel_ref),
        ("Badge", badges_ref),
        ("Chip", chips_ref),
        ("Label", labels_ref),
        ("Timeline", timeline_ref),
        ("Table", table_ref),
        ("Modal", modal_ref),
        ("Popover", popover_ref),
        ("Progress", progress_ref),
        ("Spinner", spinner_ref),
        ("Input", inputs_ref),
        ("Select", selects_ref),
        ("Checkbox", checks_ref),
        ("Radio", radios_ref),
        ("Textarea", textarea_ref),
        ("Toggle", toggle_ref),
        ("Date Picker", datepick_ref),
        ("Panel", panel_ref),
        ("Tabs", tabs_ref),
        ("Stepper", stepper_ref),
        ("Breadcrumbs", breadcrumb_ref),
        ("Rich Text", richtext_ref),
    ];

    view! {
        <div class="min-h-screen bg-white text-gray-900 font-sans">

            // ── Mobile topbar ────────────────────────────────────
            <header class="md:hidden sticky top-0 z-20 bg-white border-b border-gray-200
                            flex items-center gap-3 px-4 h-14">
                <BasicButton
                    style_ext="p-1.5 rounded-md text-gray-500 hover:bg-gray-100 transition-colors"
                    on:click=move |_| drawer_open.set(true)
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor"
                         stroke-width="2" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round"
                              d="M4 6h16M4 12h16M4 18h16"/>
                    </svg>
                </BasicButton>
                <span class="text-sm font-semibold text-gray-700">"detaxine-ui"</span>
            </header>

            // ── Drawer backdrop ──────────────────────────────────
            <div
                class="md:hidden fixed inset-0 z-30 bg-black/40 transition-opacity duration-200"
                class:opacity-0=move || !drawer_open.get()
                class:pointer-events-none=move || !drawer_open.get()
                on:click=move |_| drawer_open.set(false)
            />

            // ── Drawer panel ─────────────────────────────────────
            <div
                class="md:hidden fixed top-0 left-0 z-40 h-full w-56 bg-white
                        shadow-xl flex flex-col py-6 transition-transform duration-200"
                class:-translate-x-full=move || !drawer_open.get()
            >
                <div class="flex items-center justify-between px-4 mb-4">
                    <span class="text-[11px] font-semibold uppercase tracking-widest text-gray-400">
                        "Components"
                    </span>
                    <BasicButton
                        style_ext="p-1 rounded text-gray-400 hover:text-gray-700 hover:bg-gray-100 transition-colors"
                        on:click=move |_| drawer_open.set(false)
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor"
                             stroke-width="2" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/>
                        </svg>
                    </BasicButton>
                </div>
                <nav class="flex flex-col gap-0.5 overflow-y-auto">
                    {nav_items.clone().into_iter().map(|(label, node_ref)| {

                        view! {
                            <NavButton on:click=scroll_to(node_ref, drawer_open)>
                                {label}
                            </NavButton>
                        }
                    }).collect::<Vec<_>>()}
                </nav>
            </div>

            // ── Page body ────────────────────────────────────────
            <div class="flex">

                // ── Desktop sidebar ──────────────────────────────
                <nav class="hidden md:flex shrink-0 w-48 sticky top-0 h-screen
                             overflow-y-auto border-r border-gray-200
                             flex-col gap-0.5 py-6">
                    <p class="text-[11px] font-semibold uppercase tracking-widest
                               text-gray-400 px-4 pb-2">
                        "Components"
                    </p>
                    {nav_items.into_iter().map(|(label, node_ref)| {

                        view! {
                            <NavButton on:click=scroll_to(node_ref, drawer_open)>
                                {label}
                            </NavButton>
                        }
                    }).collect::<Vec<_>>()}
                </nav>

                // ── Main content ─────────────────────────────────
                <main class="flex-1 min-w-0 px-4 py-8 sm:px-8 md:px-10 md:py-10
                              flex flex-col gap-16 max-w-4xl">

                    // ── Buttons ──────────────────────────────────
                    <Section section_ref=buttons_ref label="Buttons">
                        <div class="flex flex-col gap-6 w-full">
                            <div class="flex flex-wrap gap-3 items-center">
                                <BasicButton button_text="Default" />
                                <BasicButton button_text="Primary"
                                    style_ext="bg-primary text-white hover:bg-secondary" />
                                <BasicButton button_text="Danger"
                                    style_ext="bg-danger text-white hover:bg-danger/80" />
                                <BasicButton button_text="Disabled"
                                    style_ext="bg-primary text-white"
                                    disabled=Signal::derive(move || true) />
                            </div>
                            <div class="flex flex-wrap gap-3 items-center">
                                <BasicButton button_text="Icon Left"
                                    style_ext="bg-primary text-white hover:bg-secondary"
                                    icon=Some(AiCheckCircleOutlined) icon_before=true />
                                <BasicButton button_text="Icon Right"
                                    style_ext="bg-primary text-white hover:bg-secondary"
                                    icon=Some(BsXCircle) icon_before=false />
                            </div>
                            <ButtonGroup style_ext="bg-primary text-white hover:bg-secondary">
                                <BasicButton button_text="First"
                                    icon=Some(AiCheckCircleOutlined) icon_before=true />
                                <BasicButton button_text="Second"
                                    icon=Some(BsXCircle) icon_before=false />
                                <BasicButton button_text="Third"
                                    disabled=Signal::derive(move || true) />
                            </ButtonGroup>
                        </div>
                    </Section>

                    // ── Carousel ─────────────────────────────────
                    <Section section_ref=carousel_ref label="Carousel">
                        <div class="w-full rounded overflow-hidden">
                            <Carousel>
                                <div class="h-48 bg-primary/20 flex items-center justify-center rounded">
                                    <span class="text-primary font-semibold">"Slide 1"</span>
                                </div>
                                <div class="h-48 bg-secondary/20 flex items-center justify-center rounded">
                                    <span class="text-secondary font-semibold">"Slide 2"</span>
                                </div>
                                <div class="h-48 bg-success/20 flex items-center justify-center rounded">
                                    <span class="text-success font-semibold">"Slide 3"</span>
                                </div>
                            </Carousel>
                        </div>
                    </Section>

                    // ── Badge ────────────────────────────────────
                    <Section section_ref=badges_ref label="Badge">
                        <div class="flex flex-wrap gap-8 items-center">
                            <Badge text="5" color=ColorTemperature::Primary>
                                <BasicButton button_text="Notifications" />
                            </Badge>
                            <Badge text="2" color=ColorTemperature::Danger>
                                <BasicButton button_text="Messages" />
                            </Badge>
                            <Badge text="" color=ColorTemperature::Success>
                                <BasicButton button_text="Online" />
                            </Badge>
                        </div>
                    </Section>

                    // ── Chip ─────────────────────────────────────
                    <Section section_ref=chips_ref label="Chip">
                        <div class="flex flex-wrap gap-3">
                            <Chip label="Rust" color=ColorTemperature::Primary
                                on_remove=Callback::new(|_| {}) />
                            <Chip label="Leptos" color=ColorTemperature::Success
                                on_remove=Callback::new(|_| {}) />
                            <Chip label="Tailwind" color=ColorTemperature::Info
                                on_remove=Callback::new(|_| {}) />
                            <Chip label="Static" color=ColorTemperature::Warning removable=false />
                            <Chip label="Error" color=ColorTemperature::Danger
                                on_remove=Callback::new(|_| {}) />
                        </div>
                    </Section>

                    // ── LabelTag ─────────────────────────────────
                    <Section section_ref=labels_ref label="Label Tag">
                        <div class="flex flex-wrap gap-3">
                            <LabelTag label="Primary" color=ColorTemperature::Primary />
                            <LabelTag label="Success" color=ColorTemperature::Success />
                            <LabelTag label="Warning" color=ColorTemperature::Warning />
                            <LabelTag label="Info"    color=ColorTemperature::Info />
                            <LabelTag label="Danger"  color=ColorTemperature::Danger />
                        </div>
                    </Section>

                    // ── Timeline ─────────────────────────────────
                    <Section section_ref=timeline_ref label="Timeline">
                        <Timeline steps=timeline_steps />
                    </Section>

                    // ── DataTable ────────────────────────────────
                    <Section section_ref=table_ref label="Table">
                        <DataTable
                            data=table_data
                            page_size=5
                            editable=true
                            deletable=true
                            on_row_action=Callback::new(move |(_, action): (HashMap<String, TableCellData>, String)| {
                                leptos::logging::log!("action: {}", action);
                            })
                        />
                    </Section>

                    // ── Modal ────────────────────────────────────
                    <Section section_ref=modal_ref label="Modal">
                        <div class="flex flex-wrap gap-3">
                            <BasicButton
                                button_text="Open Info Modal"
                                style_ext="bg-primary text-white"
                                onclick=Callback::new(move |_| modal_open.set(true))
                            />
                            <BasicButton
                                button_text="Open Confirm Modal"
                                style_ext="bg-warning text-white"
                                onclick=Callback::new(move |_| confirm_open.set(true))
                            />
                        </div>
                        <BasicModal
                            title="Information"
                            is_open=modal_open
                            use_case=UseCase::Info
                            primary_button_text="Got it"
                            on_click_primary=Callback::new(move |_| {})
                        >
                            <div class="p-4">
                                <p>"This is an informational modal."</p>
                            </div>
                        </BasicModal>
                        <BasicModal
                            title="Are you sure?"
                            is_open=confirm_open
                            use_case=UseCase::Confirmation
                            primary_button_text="Yes, confirm"
                            on_click_primary=Callback::new(|_| {})
                            on_cancel=Callback::new(move |_| {})
                        >
                            <div class="p-4">
                                <p>"This action cannot be undone."</p>
                            </div>
                        </BasicModal>
                    </Section>

                    // ── Popover ──────────────────────────────────
                    <Section section_ref=popover_ref label="Popover">
                        <Popover
                            showing=popover_showing
                            display_item=move || view! {
                                <BasicButton
                                    button_text="Open Popover"
                                    style_ext="bg-primary text-white"
                                />
                            }
                        >
                            <div class="p-3 flex flex-col gap-2">
                                <p class="font-semibold text-sm">"Quick Actions"</p>
                                <BasicButton button_text="Edit"   style_ext="w-full text-left" />
                                <BasicButton button_text="Delete" style_ext="w-full text-left text-danger" />
                            </div>
                        </Popover>
                    </Section>

                    // ── Progress ─────────────────────────────────
                    <Section section_ref=progress_ref label="Progress">
                        <div class="flex flex-col gap-6 w-full">
                            <div class="flex flex-col gap-2 w-full">
                                <p class="text-xs text-gray-500">"Determinate — 65%"</p>
                                <ProgressBar progress=progress_val show_percentage=true />
                            </div>
                            <div class="flex flex-col gap-2 w-full">
                                <p class="text-xs text-gray-500">"Indeterminate"</p>
                                <ProgressBar indeterminate=true color="bg-primary" />
                            </div>
                            <div class="flex flex-col gap-2 w-full">
                                <p class="text-xs text-gray-500">"Sizes"</p>
                                <ProgressBar progress=progress_val size=ProgressComponentSize::Sm />
                                <ProgressBar progress=progress_val size=ProgressComponentSize::Md />
                                <ProgressBar progress=progress_val size=ProgressComponentSize::Lg />
                            </div>
                            <div class="flex flex-wrap gap-8 items-center">
                                <div class="flex flex-col items-center gap-1">
                                    <p class="text-xs text-gray-500">"Sm"</p>
                                    <CircularProgress
                                        progress_percentage=circular_val
                                        size=ProgressComponentSize::Sm
                                    />
                                </div>
                                <div class="flex flex-col items-center gap-1">
                                    <p class="text-xs text-gray-500">"Md"</p>
                                    <CircularProgress
                                        progress_percentage=circular_val
                                        size=ProgressComponentSize::Md
                                    />
                                </div>
                                <div class="flex flex-col items-center gap-1">
                                    <p class="text-xs text-gray-500">"Lg"</p>
                                    <CircularProgress
                                        progress_percentage=circular_val
                                        size=ProgressComponentSize::Lg
                                    />
                                </div>
                            </div>
                        </div>
                    </Section>

                    // ── Spinner ──────────────────────────────────
                    <Section section_ref=spinner_ref label="Spinner">
                        <div class="flex flex-wrap gap-8 items-center">
                            <div class="flex flex-col items-center gap-1">
                                <p class="text-xs text-gray-500">"Sm"</p>
                                <Spinner size=SpinnerSize::Sm with_backdrop=false />
                            </div>
                            <div class="flex flex-col items-center gap-1">
                                <p class="text-xs text-gray-500">"Md"</p>
                                <Spinner size=SpinnerSize::Md with_backdrop=false />
                            </div>
                            <div class="flex flex-col items-center gap-1">
                                <p class="text-xs text-gray-500">"Lg"</p>
                                <Spinner size=SpinnerSize::Lg with_backdrop=false />
                            </div>
                        </div>
                    </Section>

                    // ── Input ────────────────────────────────────
                    <Section section_ref=inputs_ref label="Input">
                        <div class="flex flex-col gap-4 w-full max-w-md">
                            <InputField
                                field_type=InputFieldType::Text
                                label="Text"
                                name="demo_text"
                                id_attr="demo-text"
                                placeholder="Enter text…"
                            />
                            <InputField
                                field_type=InputFieldType::Email
                                label="Email"
                                name="demo_email"
                                id_attr="demo-email"
                                placeholder="you@example.com"
                                required=true
                            />
                            <InputField
                                field_type=InputFieldType::Password
                                label="Password"
                                name="demo_password"
                                id_attr="demo-password"
                                placeholder="••••••••"
                            />
                            <InputField
                                field_type=InputFieldType::Number
                                label="Number"
                                name="demo_number"
                                id_attr="demo-number"
                                placeholder="0"
                            />
                            <CustomFileInput
                                label="File Upload"
                                name="demo_file"
                                id_attr="demo-file"
                                accept="image/*"
                            />
                        </div>
                    </Section>

                    // ── Select ───────────────────────────────────
                    <Section section_ref=selects_ref label="Select">
                        <div class="flex flex-col gap-4 w-full max-w-md">
                            <SelectInput
                                label="Native Select"
                                name="demo_select"
                                id_attr="demo-select"
                                options=RwSignal::new(vec![
                                    SelectOption::new("", "-- Choose one --"),
                                    SelectOption::new("rust", "Rust"),
                                    SelectOption::new("leptos", "Leptos"),
                                    SelectOption::new("tailwind", "Tailwind"),
                                ])
                            />
                            <CustomSelectInput
                                label="Custom Select (single)"
                                id_attr="demo-custom-select"
                                options=RwSignal::new(vec![
                                    SelectOption::new("rust", "Rust"),
                                    SelectOption::new("leptos", "Leptos"),
                                    SelectOption::new("tailwind", "Tailwind"),
                                ])
                                value=custom_select_val
                            />
                            <CustomSelectInput
                                label="Custom Select (multi)"
                                id_attr="demo-custom-select-multi"
                                options=RwSignal::new(vec![
                                    SelectOption::new("rust", "Rust"),
                                    SelectOption::new("leptos", "Leptos"),
                                    SelectOption::new("tailwind", "Tailwind"),
                                    SelectOption::new("wasm", "WebAssembly"),
                                ])
                                value=RwSignal::new(vec![])
                                multiple=true
                            />
                        </div>
                    </Section>

                    // ── Checkbox ─────────────────────────────────
                    <Section section_ref=checks_ref label="Checkbox">
                        <div class="flex flex-col gap-4 w-full max-w-md">
                            <CheckboxInputField
                                label="Standalone checkbox"
                                name="demo_check"
                                id_attr="demo-check"
                            />
                            <CheckboxGroup
                                legend="Interests"
                                name="interests"
                                options=RwSignal::new(vec![
                                    CheckboxOption::new("rust",    "Rust",    None),
                                    CheckboxOption::new("leptos",  "Leptos",  None),
                                    CheckboxOption::new("wasm",    "WASM",    None),
                                ])
                                selected_values=checkbox_selected
                            />
                        </div>
                    </Section>

                    // ── Radio ────────────────────────────────────
                    <Section section_ref=radios_ref label="Radio">
                        <div class="flex flex-col gap-4 w-full max-w-md">
                            <RadioInputField
                                label="Standalone option"
                                name="demo_radio_lone"
                                id_attr="demo-radio-lone"
                            />
                            <RadioInputGroup
                                legend="Pick one"
                                name="demo_radio_group"
                                initial_value=radio_selected
                                options=vec![
                                    RadioOption::new("option1", "Option 1", None),
                                    RadioOption::new("option2", "Option 2", None),
                                    RadioOption::new("option3", "Option 3", None),
                                ]
                            />
                        </div>
                    </Section>

                    // ── Textarea ─────────────────────────────────
                    <Section section_ref=textarea_ref label="Textarea">
                        <div class="w-full max-w-md">
                            <Textarea
                                label="Description"
                                name="demo_textarea"
                                id_attr="demo-textarea"
                                placeholder="Write something…"
                                required=true
                            />
                        </div>
                    </Section>

                    // ── Toggle ───────────────────────────────────
                    <Section section_ref=toggle_ref label="Toggle Switch">
                        <div class="flex flex-col gap-4">
                            <ToggleSwitch
                                name="demo_toggle"
                                id_attr="demo-toggle"
                                label_active="Enabled"
                                label_inactive="Disabled"
                                initial_active_state=false
                            />
                            <ToggleSwitch
                                name="demo_toggle_on"
                                id_attr="demo-toggle-on"
                                label_active="On"
                                label_inactive="Off"
                                initial_active_state=true
                            />
                            <ToggleSwitch
                                name="demo_toggle_readonly"
                                id_attr="demo-toggle-readonly"
                                label_active="Locked on"
                                label_inactive="Locked off"
                                initial_active_state=true
                                readonly=true
                            />
                        </div>
                    </Section>

                    // ── Date Picker ──────────────────────────────
                    <Section section_ref=datepick_ref label="Date Picker">
                        <div class="w-full max-w-xs">
                            <DatePicker
                                label="Pick a date"
                                name="demo_date"
                                id_attr="demo-date"
                            />
                        </div>
                    </Section>

                    // ── Panel / Collapse ─────────────────────────
                    <Section section_ref=panel_ref label="Panel / Collapse">
                        <div class="flex flex-col gap-4 w-full max-w-lg">
                            <p class="text-xs text-gray-500">"Standalone panel"</p>
                            <Panel
                                title=ViewFn::from(|| view! { <span>"Standalone Panel"</span> })
                                is_open=RwSignal::new(false)
                            >
                                <p class="text-sm">"Panel body content goes here."</p>
                            </Panel>
                            <p class="text-xs text-gray-500">"Accordion (one open at a time)"</p>
                            <Collapse panel_items=collapse_panels is_accordion=true />
                        </div>
                    </Section>

                    // ── Tabs ─────────────────────────────────────
                    <Section section_ref=tabs_ref label="Tabs">
                        <div class="w-full">
                            <Tabs tab_labels=tab_labels>
                                <Tab slot>
                                    <p class="text-sm p-2">
                                        "detaxine-ui is a Leptos + Tailwind CSS component library
                                         built with WebAssembly."
                                    </p>
                                </Tab>
                                <Tab slot>
                                    <p class="text-sm p-2">
                                        "Add the crate as a dependency and run "
                                        <code class="bg-gray-100 px-1 rounded">"dtx init"</code>
                                        " to get started."
                                    </p>
                                </Tab>
                                <Tab slot>
                                    <p class="text-sm p-2">
                                        "Full API documentation is available in each component's
                                         doc comment."
                                    </p>
                                </Tab>
                            </Tabs>
                        </div>
                    </Section>

                    // ── Stepper ──────────────────────────────────
                    <Section section_ref=stepper_ref label="Stepper">
                        <div class="w-full border border-gray-100 rounded-lg">
                            <Stepper
                                step_labels=RwSignal::new(vec![
                                    StepInfo::new("Account and Personal Info",  None),
                                    StepInfo::new("Profile",  None),
                                    StepInfo::new("Confirm",  None),
                                ])
                                final_button_text="Finish"
                                is_linear=true
                                send_all_form_refs=Callback::new(|_| {})
                            >
                                <Step>
                                    <div class="flex flex-col gap-3 p-2">
                                        <InputField
                                            field_type=InputFieldType::Email
                                            label="Email"
                                            name="step_email"
                                            id_attr="step-email"
                                            required=true
                                        />
                                        <InputField
                                            field_type=InputFieldType::Password
                                            label="Password"
                                            name="step_password"
                                            id_attr="step-password"
                                            required=true
                                        />
                                    </div>
                                </Step>
                                <Step>
                                    <div class="flex flex-col gap-3 p-2">
                                        <InputField
                                            field_type=InputFieldType::Text
                                            label="Full Name"
                                            name="step_name"
                                            id_attr="step-name"
                                            required=true
                                        />
                                        <ToggleSwitch
                                            name="step_tos"
                                            id_attr="step-tos"
                                            label_active="Terms accepted"
                                            label_inactive="Accept terms"
                                        />
                                    </div>
                                </Step>
                                <Step>
                                    <div class="p-2">
                                        <p class="text-sm">"Review your details and click Finish."</p>
                                    </div>
                                </Step>
                            </Stepper>
                        </div>
                    </Section>

                    // ── Breadcrumbs ──────────────────────────────
                    <Section section_ref=breadcrumb_ref label="Breadcrumbs">
                        <Breadcrumbs
                            custom_route_names=["Home", "Components", "Breadcrumbs"]
                        />
                    </Section>

                    // ── Rich Text Editor ─────────────────────────────────────
                    <Section section_ref=richtext_ref label="Rich Text Editor">
                        <div class="w-full flex flex-col gap-8">

                            // Basic — just formatting toolbar
                            <div class="flex flex-col gap-2 w-full">
                                <p class="text-xs text-gray-500">"Basic (bold, italic, underline, strikethrough)"</p>
                                <RichTextEditor
                                    id_attr="demo-rte-basic"
                                    name="demo_rte_basic"
                                    placeholder="Start typing…"
                                />
                            </div>

                            // Full featured
                            <div class="flex flex-col gap-2 w-full">
                                <p class="text-xs text-gray-500">"Full featured"</p>
                                <RichTextEditor
                                    id_attr="demo-rte-full"
                                    name="demo_rte_full"
                                    placeholder="Write something rich…"
                                    extra_formating_options=vec![
                                        ExtraFormatingOption::Heading,
                                        ExtraFormatingOption::InlineCode,
                                        ExtraFormatingOption::CodeBlock,
                                        ExtraFormatingOption::Lists,
                                        ExtraFormatingOption::ImageUpload,
                                        ExtraFormatingOption::MarkdownUpload,
                                    ]
                                    // on_image_insert=Callback::new(move |file: File| {
                                    //     Box::pin(async move {
                                    //         // upload to S3, Cloudflare R2, etc. and return the URL
                                    //         Some("https://cdn.example.com/image.png".to_string())
                                    //     }) as Pin<Box<dyn Future<Output = Option<String>>>>
                                    // })
                                />
                            </div>

                        </div>
                    </Section>
                </main>
            </div>
        </div>
    }
}

// ── Nav button ───────────────────────────────────────────────────
#[component]
fn NavButton(children: Children) -> impl IntoView {
    view! {
        <button class="text-left px-4 py-2 text-sm text-gray-500
                        hover:text-gray-900 hover:bg-gray-50
                        focus:outline-none transition-colors duration-100">
            {children()}
        </button>
    }
}

// ── Section wrapper ──────────────────────────────────────────────
#[component]
fn Section(section_ref: NodeRef<Div>, label: &'static str, children: Children) -> impl IntoView {
    view! {
        <div node_ref=section_ref class="flex flex-col gap-4 scroll-mt-8">
            <div>
                <p class="text-[11px] font-semibold uppercase tracking-widest
                           text-gray-400 mb-1">
                    {label}
                </p>
                <div class="h-px bg-gray-100" />
            </div>
            <div class="flex flex-wrap gap-3 items-start">
                {children()}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(|| {
        view! {
            <Stylesheet id="leptos" href="/style/output.css"/>
            <Router>
                <App />
            </Router>
        }
    })
}
