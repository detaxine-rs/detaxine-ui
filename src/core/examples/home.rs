use detaxine_ui::components::actions::button::{BasicButton, ButtonGroup};
use icondata::{AiCheckCircleOutlined, BsXCircle};
use leptos::html::*;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let buttons_ref: NodeRef<Div> = NodeRef::new();
    let badges_ref: NodeRef<Div> = NodeRef::new();
    let inputs_ref: NodeRef<Div> = NodeRef::new();
    let cards_ref: NodeRef<Div> = NodeRef::new();
    let alerts_ref: NodeRef<Div> = NodeRef::new();
    let avatars_ref: NodeRef<Div> = NodeRef::new();
    let toggles_ref: NodeRef<Div> = NodeRef::new();

    let drawer_open = RwSignal::new(false);

    let scroll_to = |node_ref: NodeRef<Div>, close_drawer: RwSignal<bool>| {
        move |_| {
            close_drawer.set(false);
            if let Some(el) = node_ref.get() {
                el.scroll_into_view_with_bool(true);
            }
        }
    };

    view! {
        <div class="min-h-screen bg-white text-gray-900 font-sans">

            // ── Mobile topbar ────────────────────────────────────
            <header class="md:hidden sticky top-0 z-20 bg-white border-b
                            border-gray-200 flex items-center gap-3 px-4 h-14">
                <button
                    class="p-1.5 rounded-md text-gray-500 hover:bg-gray-100
                            transition-colors"
                    on:click=move |_| drawer_open.set(true)
                >
                    // hamburger icon
                    <svg class="w-5 h-5" fill="none" stroke="currentColor"
                         stroke-width="2" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round"
                              d="M4 6h16M4 12h16M4 18h16"/>
                    </svg>
                </button>
                <span class="text-sm font-semibold text-gray-700">
                    "detaxine-ui"
                </span>
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
                class="md:hidden fixed top-0 left-0 z-40 h-full w-64 bg-white
                        shadow-xl flex flex-col py-6 transition-transform duration-200"
                class:-translate-x-full=move || !drawer_open.get()
            >
                <div class="flex items-center justify-between px-4 mb-4">
                    <span class="text-[11px] font-semibold uppercase tracking-widest
                                  text-gray-400">
                        "Components"
                    </span>
                    <button
                        class="p-1 rounded text-gray-400 hover:text-gray-700
                                hover:bg-gray-100 transition-colors"
                        on:click=move |_| drawer_open.set(false)
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor"
                             stroke-width="2" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round"
                                  d="M6 18L18 6M6 6l12 12"/>
                        </svg>
                    </button>
                </div>
                <nav class="flex flex-col gap-0.5">
                    <NavButton on:click=scroll_to(buttons_ref, drawer_open)>"Buttons"</NavButton>
                    <NavButton on:click=scroll_to(badges_ref, drawer_open)>"Badges"</NavButton>
                    <NavButton on:click=scroll_to(inputs_ref, drawer_open)>"Inputs"</NavButton>
                    <NavButton on:click=scroll_to(cards_ref, drawer_open)>"Cards"</NavButton>
                    <NavButton on:click=scroll_to(alerts_ref, drawer_open)>"Alerts"</NavButton>
                    <hr class="my-2 border-gray-100" />
                    <NavButton on:click=scroll_to(avatars_ref, drawer_open)>"Avatars"</NavButton>
                    <NavButton on:click=scroll_to(toggles_ref, drawer_open)>"Toggles"</NavButton>
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
                    <NavButton on:click=scroll_to(buttons_ref, drawer_open)>"Buttons"</NavButton>
                    <NavButton on:click=scroll_to(badges_ref, drawer_open)>"Badges"</NavButton>
                    <NavButton on:click=scroll_to(inputs_ref, drawer_open)>"Inputs"</NavButton>
                    <NavButton on:click=scroll_to(cards_ref, drawer_open)>"Cards"</NavButton>
                    <NavButton on:click=scroll_to(alerts_ref, drawer_open)>"Alerts"</NavButton>
                    <hr class="my-2 border-gray-100" />
                    <NavButton on:click=scroll_to(avatars_ref, drawer_open)>"Avatars"</NavButton>
                    <NavButton on:click=scroll_to(toggles_ref, drawer_open)>"Toggles"</NavButton>
                </nav>

                // ── Main content ─────────────────────────────────
                <main class="flex-1 min-w-0 px-4 py-8 sm:px-8 md:px-10 md:py-10
                              flex flex-col gap-12 max-w-4xl">

                    <Section section_ref=buttons_ref label="Buttons">
                        <ButtonGroup style_ext="font-bold bg-primary text-white hover:bg-secondary".to_string()>
                            <BasicButton
                                button_text="First"
                                icon=Some(AiCheckCircleOutlined)
                                icon_before=true
                            />
                            <BasicButton
                                button_text="Second"
                                icon=Some(BsXCircle)
                                icon_before=false
                            />
                            <BasicButton
                                button_text="Third"
                                disabled=true
                            />
                        </ButtonGroup>
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
    mount_to_body(App)
}
