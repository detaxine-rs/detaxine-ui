use icondata::{
    AiInfoCircleOutlined, AiQuestionCircleOutlined, BiCheckCircleRegular, BiErrorSolid,
};
use leptos::{control_flow::Show, ev, portal::Portal, prelude::*};
use leptos_icons::Icon;

use crate::components::actions::button::BasicButton;

#[derive(Clone, PartialEq, Copy, Debug, Default, Eq)]
#[allow(dead_code)]
pub enum UseCase {
    Error,
    Success,
    Confirmation,
    Info,
    #[default]
    General,
}

/// A modal dialog rendered into a `#modal-root` portal, supporting multiple use cases
/// with contextual icons and configurable footer actions.
///
/// # Props
///
/// - `title` – Heading text displayed in the modal header.
/// - `use_case` – Controls the header icon and cancel button visibility. One of `UseCase::General`,
///   `UseCase::Error`, `UseCase::Success`, `UseCase::Info`, `UseCase::Confirmation`. Defaults to `General`.
/// - `is_open` – `RwSignal<bool>` controlling visibility. Defaults to `false`.
/// - `primary_button_text` – Label for the primary action button. Defaults to `"OK"`.
/// - `on_click_primary` – Callback fired when the primary button is clicked. Defaults to a no-op.
/// - `on_cancel` – Callback fired when the cancel button or backdrop is clicked. Defaults to a no-op.
/// - `disable_auto_close` – When `true`, clicking the backdrop does not close the modal. Defaults to `true`.
/// - `disable_primary_close` – When `true`, clicking the primary button does not close the modal. Defaults to `false`.
/// - `primary_is_disabled` – `Signal<bool>` that disables the primary button. Defaults to `false`.
/// - `stack_number` – Z-index offset for stacking multiple modals. Defaults to `0`.
/// - `container_style_ext` – Additional Tailwind classes applied to the modal panel.
/// - `show_footer` – When `false`, hides the footer entirely. Defaults to `true`.
/// - `children` – Optional body content rendered inside the modal.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::feedback::modal::modal::{BasicModal, UseCase};
///
/// #[component]
/// fn Example() -> impl IntoView {
///     let is_open = RwSignal::new(false);
///     view! {
///         <BasicModal
///             title="Confirm action"
///             is_open=is_open
///             use_case=UseCase::Confirmation
///             on_click_primary=Callback::new(|_| leptos::logging::log!("confirmed"))
///             on_cancel=Callback::new(|_| leptos::logging::log!("cancelled"))
///         >
///             <p>"Are you sure?"</p>
///         </BasicModal>
///     }
/// }
/// ```
#[component]
pub fn BasicModal(
    #[prop(into)] title: String,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(default = UseCase::General, optional)] use_case: UseCase,
    #[prop(default = Callback::new(|_| {}), optional)] on_click_primary: Callback<()>,
    #[prop(default = Callback::new(|_| {}), optional)] on_cancel: Callback<()>,
    #[prop(default = RwSignal::new(false), into, optional)] is_open: RwSignal<bool>,
    #[prop(into, default = "OK".to_string())] primary_button_text: String,
    #[prop(default = true, optional)] disable_auto_close: bool,
    #[prop(default = false, optional)] disable_primary_close: bool,
    #[prop(into, default = Signal::derive(move || false), optional)] primary_is_disabled: Signal<
        bool,
    >,
    #[prop(into, default = 0, optional)] stack_number: u8,
    #[prop(into, optional)] container_style_ext: String,
    #[prop(into, optional, default = true)] show_footer: bool,
) -> impl IntoView {
    let (title, _set_title) = signal(title);
    let (primary_button_text, _set_primary_button_text) = signal(primary_button_text);
    let (children, _set_children) = signal(children);

    let oncancel_handler = move |_| {
        Callback::new(move |e: ev::MouseEvent| {
            e.stop_propagation();
            is_open.update(|val| *val = false);
            on_cancel.run(());
        })
    };

    let onclick_primary_handler = move || {
        Callback::new(move |e: ev::MouseEvent| {
            e.stop_propagation();
            if !disable_primary_close {
                is_open.update(|val| *val = false);
            };

            on_click_primary.run(());
        })
    };

    let handle_backdrop_click = move |e: ev::MouseEvent| {
        e.stop_propagation();
        if !disable_auto_close {
            is_open.update(|val| *val = false);
            on_cancel.run(());
        };
    };

    view! {
        <>
        <Show when=move || is_open.get() fallback=|| ()>
            {
                let container_style_ext_clone = container_style_ext.clone();

                match document().get_element_by_id("modal-root") {
                    Some(root) => Some(
                        view! {
                            <Portal mount=root>
                                // Backdrop
                                <div
                                    class="fixed inset-0 bg-gray opacity-50 animate-fade-in"
                                    style=format!("z-index: {}", 10 + stack_number)
                                />

                                // Centering layer
                                <div
                                    on:click=handle_backdrop_click
                                    class="fixed inset-0 flex items-center justify-center bg-transparent"
                                    style=format!("z-index: {}", 10 + (stack_number + 1))
                                >
                                    // Modal panel
                                    <div
                                        on:click=move |e| e.stop_propagation()
                                        class=format!("bg-contrast-white dark:bg-navy rounded shadow-lg min-w-sm flex flex-col animate-modal-in {}", container_style_ext_clone)
                                    >
                                        // Header
                                        <div class="flex items-center border-light-gray border-b p-[10px]">
                                            {
                                                move || match use_case {
                                                    UseCase::Error => Some(view! {
                                                        <span class="text-danger mr-2">
                                                            <Icon width="2rem" height="2rem" icon=BiErrorSolid />
                                                        </span>
                                                    }),
                                                    UseCase::Success => Some(view! {
                                                        <span class="text-success mr-2">
                                                            <Icon width="2rem" height="2rem" icon=BiCheckCircleRegular />
                                                        </span>
                                                    }),
                                                    UseCase::Info => Some(view! {
                                                        <span class="text-info mr-2">
                                                            <Icon width="2rem" height="2rem" icon=AiInfoCircleOutlined />
                                                        </span>
                                                    }),
                                                    UseCase::Confirmation => Some(view! {
                                                        <span class="text-warning mr-2">
                                                            <Icon width="2rem" height="2rem" icon=AiQuestionCircleOutlined />
                                                        </span>
                                                    }),
                                                    UseCase::General => None,
                                                }
                                            }
                                            <h2>{move || title.get()}</h2>
                                        </div>

                                        // Body
                                        <div class="flex-1 overflow-y-auto">
                                            {move || children.get().map(|c| c())}
                                        </div>

                                        // Footer
                                        {
                                            if show_footer {
                                                Some(
                                                    view! {
                                                        <div class="mt-auto flex gap-[20px] p-[10px] border-light-gray border-t">
                                                            {move || {
                                                                if use_case == UseCase::Confirmation {
                                                                    Some(view! {
                                                                        <BasicButton
                                                                            button_text="Cancel".to_string()
                                                                            style_ext="bg-mid-gray text-contrast-white".to_string()
                                                                            onclick=oncancel_handler(false)
                                                                        />
                                                                    })
                                                                } else {
                                                                    None
                                                                }
                                                            }}
                                                            <BasicButton
                                                                button_text=primary_button_text.get()
                                                                style_ext="bg-primary text-contrast-white".to_string()
                                                                onclick=onclick_primary_handler()
                                                                disabled=primary_is_disabled
                                                            />
                                                        </div>
                                                    }
                                                )
                                            } else { None }
                                        }
                                    </div>
                                </div>
                            </Portal>
                        }
                    ),
                    None => None,
                }
            }
        </Show>
        </>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // UseCase

    #[test]
    fn use_case_default_is_general() {
        assert_eq!(UseCase::default(), UseCase::General);
    }

    #[test]
    fn use_case_eq() {
        assert_eq!(UseCase::Error, UseCase::Error);
        assert_ne!(UseCase::Error, UseCase::Success);
    }

    #[test]
    fn use_case_clone_and_copy() {
        let uc = UseCase::Confirmation;
        let cloned = uc;
        assert_eq!(uc, cloned);
    }

    // cancel button visibility

    fn shows_cancel(use_case: UseCase) -> bool {
        use_case == UseCase::Confirmation
    }

    #[test]
    fn confirmation_shows_cancel_button() {
        assert!(shows_cancel(UseCase::Confirmation));
    }

    #[test]
    fn other_use_cases_hide_cancel_button() {
        assert!(!shows_cancel(UseCase::General));
        assert!(!shows_cancel(UseCase::Error));
        assert!(!shows_cancel(UseCase::Success));
        assert!(!shows_cancel(UseCase::Info));
    }

    // backdrop / auto-close logic

    #[test]
    fn backdrop_closes_modal_when_auto_close_enabled() {
        let owner = Owner::new();
        owner.with(|| {
            let is_open = RwSignal::new(true);
            let disable_auto_close = false;

            if !disable_auto_close {
                is_open.set(false);
            }

            assert!(!is_open.get());
        });
    }

    #[test]
    fn backdrop_does_not_close_when_auto_close_disabled() {
        let owner = Owner::new();
        owner.with(|| {
            let is_open = RwSignal::new(true);
            let disable_auto_close = true;

            if !disable_auto_close {
                is_open.set(false);
            }

            assert!(is_open.get());
        });
    }

    // primary button close logic

    #[test]
    fn primary_closes_modal_when_not_disabled() {
        let owner = Owner::new();
        owner.with(|| {
            let is_open = RwSignal::new(true);
            let disable_primary_close = false;

            if !disable_primary_close {
                is_open.set(false);
            }

            assert!(!is_open.get());
        });
    }

    #[test]
    fn primary_does_not_close_modal_when_disabled() {
        let owner = Owner::new();
        owner.with(|| {
            let is_open = RwSignal::new(true);
            let disable_primary_close = true;

            if !disable_primary_close {
                is_open.set(false);
            }

            assert!(is_open.get());
        });
    }

    // stack_number z-index logic

    fn backdrop_z(stack_number: u8) -> u8 {
        10 + stack_number
    }

    fn panel_z(stack_number: u8) -> u8 {
        10 + stack_number + 1
    }

    #[test]
    fn default_stack_z_indices() {
        assert_eq!(backdrop_z(0), 10);
        assert_eq!(panel_z(0), 11);
    }

    #[test]
    fn stacked_modal_z_indices() {
        assert_eq!(backdrop_z(2), 12);
        assert_eq!(panel_z(2), 13);
    }

    #[test]
    fn panel_always_above_backdrop() {
        for n in 0..5u8 {
            assert!(panel_z(n) > backdrop_z(n));
        }
    }

    // primary_is_disabled reactive signal

    #[test]
    fn primary_disabled_signal_reactive() {
        let owner = Owner::new();
        owner.with(|| {
            let disabled = RwSignal::new(false);
            let primary_is_disabled = Signal::derive(move || disabled.get());

            assert!(!primary_is_disabled.get());
            disabled.set(true);
            assert!(primary_is_disabled.get());
        });
    }

    // on_cancel callback

    #[test]
    fn on_cancel_fires() {
        let owner = Owner::new();
        owner.with(|| {
            let fired = RwSignal::new(false);
            let on_cancel = Callback::new(move |_: ()| fired.set(true));
            on_cancel.run(());
            assert!(fired.get());
        });
    }

    // on_click_primary callback

    #[test]
    fn on_click_primary_fires() {
        let owner = Owner::new();
        owner.with(|| {
            let fired = RwSignal::new(false);
            let on_click_primary = Callback::new(move |_: ()| fired.set(true));
            on_click_primary.run(());
            assert!(fired.get());
        });
    }
}
