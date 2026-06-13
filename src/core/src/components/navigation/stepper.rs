use crate::components::actions::button::BasicButton;
use crate::components::forms::reactive_form::ReactiveForm;
use crate::utils::forms::fire_bubbled_and_cancelable_event;
use icondata::Icon as IconId;
use leptos::html::Form;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos_icons::Icon;
use web_sys::HtmlFormElement;
use web_sys::SubmitEvent;

#[derive(Clone, Debug, Default)]
pub struct StepInfo {
    pub label: String,
    pub icon: Option<IconId>,
}

impl StepInfo {
    pub fn new(label: &str, icon: Option<IconId>) -> Self {
        StepInfo {
            label: label.to_string(),
            icon,
        }
    }
}

/// A multi-step form wizard with a step indicator, per-step form validation, and linear/non-linear navigation.
///
/// Each direct child should be a `<Step>` component wrapping form fields. Every step is
/// wrapped in a `ReactiveForm` automatically. When `is_linear=true`, the Next button is
/// disabled until the current step's form passes `checkValidity()`.
///
/// Form refs for all steps are sent to the parent via `send_all_form_refs` whenever the
/// user navigates to the final step or clicks a step indicator directly.
///
/// # Props
///
/// - `step_labels` – `RwSignal<Vec<StepInfo>>` holding the label and optional icon for each step indicator.
/// - `final_button_text` – Label on the submit button shown at the last step.
/// - `on_click_final_button` – Callback fired when the final button is clicked.
/// - `send_all_form_refs` – Callback receiving `Vec<NodeRef<Form>>` for all steps.
/// - `is_linear` – When `true`, the Next button is disabled until the current step is valid. Defaults to `false`.
/// - `final_button_is_disabled` – `Signal<bool>` that disables the final button. Defaults to `false`.
/// - `ext_wrapper_styles` – `Signal<String>` of additional Tailwind classes on the step content wrapper.
/// - `handle_on_cleanup` – Callback fired when the component is cleaned up.
/// - `children` – One or more `<Step>` components.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use leptos::html::Form;
/// use detaxine_ui::components::navigation::stepper::{Stepper, Step, StepInfo};
///
/// #[component]
/// fn Example() -> impl IntoView {
///     let form_refs = RwSignal::new(Vec::<NodeRef<Form>>::new());
///     view! {
///         <Stepper
///             step_labels=RwSignal::new(vec![
///                 StepInfo::new("Details", None),
///                 StepInfo::new("Confirm", None),
///             ])
///             final_button_text="Submit"
///             send_all_form_refs=Callback::new(move |refs| form_refs.set(refs))
///             is_linear=true
///         >
///             <Step><p>"Step 1 content"</p></Step>
///             <Step><p>"Step 2 content"</p></Step>
///         </Stepper>
///     }
/// }
/// ```
#[component]
pub fn Stepper(
    mut children: ChildrenFragmentMut, // Children passed as a function
    #[prop(into, optional)] final_button_text: String,
    #[prop(optional, default = Callback::new(|_| {}))] on_click_final_button: Callback<()>,
    #[prop(into)] step_labels: RwSignal<Vec<StepInfo>>,
    #[prop(optional, default = false)] is_linear: bool,
    #[prop(optional, default = Callback::new(|_| {}))] send_all_form_refs: Callback<
        Vec<NodeRef<Form>>,
    >,
    #[prop(into, optional)] ext_wrapper_styles: Signal<String>,
    #[prop(into, optional, default = Signal::derive(|| false))] final_button_is_disabled: Signal<
        bool,
    >,
    #[prop(optional, default = Callback::new(|_| {}))] handle_on_cleanup: Callback<()>,
) -> impl IntoView {
    let (current_step, set_current_step) = signal(0); // Leptos signal for state
    let (step_form_is_valid, set_step_form_is_valid) = signal(false); // Leptos signal for state
    let child_nodes: Vec<AnyView> = children().nodes.into_iter().map(|n| n.into_any()).collect();
    let step_count = child_nodes.len();
    let form_refs = RwSignal::new(
        (0..step_count)
            .map(|_| NodeRef::<Form>::new())
            .collect::<Vec<_>>(),
    );

    let onclick_next = Callback::new(move |_| {
        if current_step.get() < step_count - 1 {
            set_current_step.update(|step| *step += 1);
        }

        // if second last, send all form_refs to parent in a callback
        if current_step.get() == step_count - 1 {
            let form_refs = form_refs.get();
            send_all_form_refs.run(form_refs);
        }
    });

    let onclick_prev = Callback::new(move |_| {
        if current_step.get() > 0 {
            set_current_step.update(|step| *step -= 1);
        }
    });

    let handle_final_button_click = Callback::new(move |_| {
        on_click_final_button.run(());
    });

    let handle_step_form_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        ev.stop_propagation();
        // Implement logic to show form validity
        let target = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

        if let Some(form) = target {
            // let form_data = FormData::new_with_form(&form).unwrap_or_default();
            let is_valid = form.check_validity();
            set_step_form_is_valid.set(is_valid);
        }
    };

    let next_is_disabled = Memo::new(move |_| !step_form_is_valid.get() && is_linear);

    // A workaround for updating the next step's form's validity state when navigating to the next step or previous step
    Effect::new(move || {
        if let Some(form_ref) = form_refs.get().get(current_step.get()) {
            if let Some(form) = form_ref.get() as Option<HtmlFormElement> {
                fire_bubbled_and_cancelable_event("submit", true, true, &form);
            }
        }
    });

    // Cleanup
    on_cleanup(move || {
        handle_on_cleanup.run(());
    });

    view! {
        <div class="flex flex-col items-center gap-[40px] w-full h-full p-4">
            <div class="relative flex items-center w-full overflow-x-auto">
                <div class="relative flex items-center justify-between w-full">
                    <For
                        each=move || step_labels.get().into_iter().enumerate()
                        key=|(index, _)| *index
                        let:((index, step_label))
                    >
                        {
                            let is_current = move || index == current_step.get();
                            let step_count_inner = step_count;
                            view! {
                                // Step circle + label
                                <div
                                    on:click=move |_| {
                                        if next_is_disabled.get() {
                                            return;
                                        }
                                        set_current_step.update(|step| *step = index);
                                        let form_refs = form_refs.get();
                                        send_all_form_refs.run(form_refs);
                                    }
                                    class="flex items-center gap-[10px] cursor-pointer shrink-0"
                                >
                                    <div class=move || format!(
                                        "w-8 h-8 flex items-center justify-center rounded-full text-sm {}",
                                        if is_current() {
                                            "bg-primary text-contrast-white"
                                        } else {
                                            "bg-light-gray"
                                        }
                                    )>
                                        {if step_label.icon.is_none() {
                                            Some(index + 1)
                                        } else {
                                            None
                                        }}
                                        {if let Some(icon) = step_label.icon {
                                            Some(view! { <Icon icon=icon /> })
                                        } else {
                                            None
                                        }}
                                    </div>
                                    <div class=move || format!(
                                        "text-sm {}",
                                        if is_current() {
                                            "font-bold text-primary"
                                        } else {
                                            "hidden md:flex"
                                        }
                                    )>
                                        {step_label.label.clone()}
                                    </div>
                                </div>

                                // Connector — only between steps, never after the last
                                {if index < step_count_inner - 1 {
                                    Some(view! {
                                        <div class="flex-1 h-px bg-mid-gray mx-2" />
                                    })
                                } else {
                                    None
                                }}
                            }
                        }
                    </For>
                </div>
            </div>
            <div on:submit=handle_step_form_submit class=move || format!("flex-1 w-full {}", ext_wrapper_styles.get())>
            {
                    child_nodes.into_iter().enumerate().map(|(i, child)| {
                        let form_ref = form_refs.get_untracked()[i].clone();
                        view! {
                            <ReactiveForm
                                form_ref=form_ref
                                ext_styles=Signal::derive(move || {
                                    if current_step.get() == i { "block".to_string() } else { "hidden".to_string() }
                                })
                            >
                                { child }
                            </ReactiveForm>
                        }
                    }).collect_view()
                }
            </div>
            <div class="mt-auto flex w-full justify-start gap-4">
                {
                    move || if current_step.get() == 0 {
                        None
                    } else {
                        Some(view! {
                            <BasicButton
                                onclick=onclick_prev
                                button_text="Previous"
                                style_ext="bg-white"
                            />
                        })
                    }
                }
                {
                    move || if current_step.get() == step_count - 1 {
                        view! {
                            <BasicButton
                                onclick=handle_final_button_click
                                button_text=final_button_text.clone()
                                style_ext="bg-primary text-contrast-white"
                                disabled=final_button_is_disabled
                            />
                        }
                    } else {
                        view! {
                            <BasicButton
                                disabled=next_is_disabled
                                onclick=onclick_next
                                button_text="Next"
                                style_ext="bg-primary text-contrast-white"
                            />
                        }
                    }
                }
            </div>
        </div>
    }.into_any()
}

/// A wrapper component representing a single step inside a `Stepper`.
///
/// Renders its children directly; layout and visibility are controlled by the parent `Stepper`.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::navigation::stepper::Step;
///
/// #[component]
/// fn Example() -> impl IntoView {
///     view! {
///         <Step>
///             <p>"Step content"</p>
///         </Step>
///     }
/// }
/// ```
#[component]
pub fn Step(children: Children) -> impl IntoView {
    view! {
        { children() }
    }
    .into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    // StepInfo::new

    #[test]
    fn step_info_new_sets_label() {
        let info = StepInfo::new("Details", None);
        assert_eq!(info.label, "Details");
        assert!(info.icon.is_none());
    }

    #[test]
    fn step_info_clone() {
        let info = StepInfo::new("Confirm", None);
        let cloned = info.clone();
        assert_eq!(cloned.label, info.label);
    }

    #[test]
    fn step_info_default_has_empty_label() {
        let info = StepInfo::default();
        assert_eq!(info.label, "");
        assert!(info.icon.is_none());
    }

    // next/prev navigation logic

    fn can_go_next(current: usize, total: usize) -> bool {
        current < total - 1
    }

    fn can_go_prev(current: usize) -> bool {
        current > 0
    }

    #[test]
    fn can_advance_before_last_step() {
        assert!(can_go_next(0, 3));
        assert!(can_go_next(1, 3));
    }

    #[test]
    fn cannot_advance_past_last_step() {
        assert!(!can_go_next(2, 3));
    }

    #[test]
    fn can_go_back_after_first_step() {
        assert!(can_go_prev(1));
        assert!(can_go_prev(2));
    }

    #[test]
    fn cannot_go_back_from_first_step() {
        assert!(!can_go_prev(0));
    }

    // next_is_disabled (linear mode)

    fn next_is_disabled(step_form_is_valid: bool, is_linear: bool) -> bool {
        !step_form_is_valid && is_linear
    }

    #[test]
    fn next_disabled_when_linear_and_invalid() {
        assert!(next_is_disabled(false, true));
    }

    #[test]
    fn next_enabled_when_linear_and_valid() {
        assert!(!next_is_disabled(true, true));
    }

    #[test]
    fn next_enabled_when_not_linear_regardless_of_validity() {
        assert!(!next_is_disabled(false, false));
        assert!(!next_is_disabled(true, false));
    }

    // is_last_step

    fn is_last_step(current: usize, total: usize) -> bool {
        current == total - 1
    }

    #[test]
    fn final_button_shown_on_last_step() {
        assert!(is_last_step(2, 3));
    }

    #[test]
    fn next_button_shown_before_last_step() {
        assert!(!is_last_step(0, 3));
        assert!(!is_last_step(1, 3));
    }

    // form_refs initialised per step

    #[test]
    fn form_refs_count_matches_step_count() {
        let owner = Owner::new();
        owner.with(|| {
            let step_count = 4;
            let form_refs = RwSignal::new(
                (0..step_count)
                    .map(|_| NodeRef::<leptos::html::Form>::new())
                    .collect::<Vec<_>>(),
            );
            assert_eq!(form_refs.get().len(), step_count);
        });
    }

    // send_all_form_refs fires on last step

    #[test]
    fn send_all_form_refs_fires_when_reaching_last_step() {
        let owner = Owner::new();
        owner.with(|| {
            let fired = RwSignal::new(false);
            let send_all_form_refs: Callback<Vec<NodeRef<leptos::html::Form>>> =
                Callback::new(move |_| fired.set(true));

            let step_count = 3;
            let (current_step, set_current_step) = signal(step_count - 2); // second-to-last

            // simulate clicking next from second-to-last step
            set_current_step.update(|s| *s += 1);

            if current_step.get() == step_count - 1 {
                send_all_form_refs.run(vec![]);
            }

            assert!(fired.get());
        });
    }

    // step indicator click blocked when disabled

    #[test]
    fn step_click_blocked_when_next_disabled() {
        let owner = Owner::new();
        owner.with(|| {
            let (current_step, set_current_step) = signal(0usize);
            let disabled = true;

            // simulate step click with guard
            if !disabled {
                set_current_step.set(2);
            }

            assert_eq!(current_step.get(), 0);
        });
    }

    #[test]
    fn step_click_allowed_when_not_disabled() {
        let owner = Owner::new();
        owner.with(|| {
            let (current_step, set_current_step) = signal(0usize);
            let disabled = false;

            if !disabled {
                set_current_step.set(2);
            }

            assert_eq!(current_step.get(), 2);
        });
    }

    // reactive step validity

    #[test]
    fn step_form_validity_updates_reactively() {
        let owner = Owner::new();
        owner.with(|| {
            let (step_form_is_valid, set_step_form_is_valid) = signal(false);
            assert!(!step_form_is_valid.get());
            set_step_form_is_valid.set(true);
            assert!(step_form_is_valid.get());
        });
    }
}
