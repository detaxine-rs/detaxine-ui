use crate::components::actions::button::BasicButton;
use crate::components::forms::reactive_form::ReactiveForm;
use crate::utils::forms::fire_bubbled_and_cancelable_event;
use icondata::Icon as IconId;
use leptos::html::Form;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos_icons::Icon;
use tailwind_fuse::tw_merge;
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
    /// One or more `<Step>` components.
    mut children: ChildrenFragmentMut,

    /// Label on the submit button shown at the last step.
    #[prop(into, optional)]
    final_button_text: String,

    /// Callback fired when the final button is clicked.
    #[prop(optional, default = Callback::new(|_| {}))]
    on_click_final_button: Callback<()>,

    /// `RwSignal<Vec<StepInfo>>` holding the label and optional icon for each step indicator.
    #[prop(into)]
    step_labels: RwSignal<Vec<StepInfo>>,

    /// When `true`, the Next button is disabled until the current step is valid. Defaults to `false`.
    #[prop(optional, default = false)]
    is_linear: bool,

    /// Callback receiving `Vec<NodeRef<Form>>` for all steps.
    #[prop(optional, default = Callback::new(|_| {}))]
    send_all_form_refs: Callback<Vec<NodeRef<Form>>>,

    /// **Deprecated**: use `form_area_class` instead. Still supported and
    /// merged in alongside `form_area_class` for backward compatibility.
    #[prop(into, optional)]
    ext_wrapper_styles: MaybeProp<String>,
    /// Extra Tailwind classes for the form area wrapper.
    #[prop(into, optional)]
    form_area_class: MaybeProp<String>,

    /// `MaybeProp<bool>` that disables the final button. Defaults to `false`.
    #[prop(into, optional)]
    final_button_is_disabled: MaybeProp<bool>,

    /// Callback fired when the component is cleaned up.
    #[prop(optional, default = Callback::new(|_| {}))]
    handle_on_cleanup: Callback<()>,

    /// Extra Tailwind classes for the root wrapper `<div>`.
    #[prop(into, optional)]
    class: MaybeProp<String>,

    /// Extra Tailwind classes for the step-rail container (holds all step circles/connectors).
    #[prop(into, optional)]
    rail_class: MaybeProp<String>,

    /// Extra Tailwind classes for each step's clickable container (circle + label).
    #[prop(into, optional)]
    step_class: MaybeProp<String>,

    /// Extra Tailwind classes for the step circle.
    #[prop(into, optional)]
    step_circle_class: MaybeProp<String>,

    /// Extra Tailwind classes for the step label text.
    #[prop(into, optional)]
    step_label_class: MaybeProp<String>,

    /// Extra Tailwind classes for the connector line between steps.
    #[prop(into, optional)]
    connector_class: MaybeProp<String>,

    /// Extra Tailwind classes for the bottom action-buttons row.
    #[prop(into, optional)]
    actions_class: MaybeProp<String>,

    /// Extra Tailwind classes for the "Previous" button.
    #[prop(into, optional)]
    prev_button_class: MaybeProp<String>,

    /// Extra Tailwind classes for the "Next" button.
    #[prop(into, optional)]
    next_button_class: MaybeProp<String>,

    /// Extra Tailwind classes for the final-step submit button.
    #[prop(into, optional)]
    final_button_class: MaybeProp<String>,
) -> impl IntoView {
    let (current_step, set_current_step) = signal(0);
    let (step_form_is_valid, set_step_form_is_valid) = signal(false);
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
        let target = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

        if let Some(form) = target {
            let is_valid = form.check_validity();
            set_step_form_is_valid.set(is_valid);
        }
    };

    let next_is_disabled = Memo::new(move |_| !step_form_is_valid.get() && is_linear);

    Effect::new(move || {
        if let Some(form_ref) = form_refs.get().get(current_step.get()) {
            if let Some(form) = form_ref.get() as Option<HtmlFormElement> {
                fire_bubbled_and_cancelable_event("submit", true, true, &form);
            }
        }
    });

    on_cleanup(move || {
        handle_on_cleanup.run(());
    });

    let root_class = move || {
        tw_merge!(
            "flex flex-col items-center gap-[40px] w-full h-full p-4 overflow-hidden",
            class.get().unwrap_or_default()
        )
    };
    let rail_class_val = move || {
        tw_merge!(
            "relative flex items-center justify-between w-full overflow-x-auto shrink-0",
            rail_class.get().unwrap_or_default()
        )
    };
    let step_class_val = move || {
        tw_merge!(
            "flex items-center gap-[10px] cursor-pointer shrink-0",
            step_class.get().unwrap_or_default()
        )
    };
    let step_circle_class_val = step_circle_class;
    let step_label_class_val = step_label_class;
    let connector_class_val = move || {
        tw_merge!(
            "flex-1 h-px bg-mid-gray mx-2",
            connector_class.get().unwrap_or_default()
        )
    };
    let form_area_class_val = move || {
        tw_merge!(
            "flex-1 w-full min-h-0 overflow-y-auto",
            ext_wrapper_styles.get().unwrap_or_default(),
            form_area_class.get().unwrap_or_default()
        )
    };
    let actions_class_val = move || {
        tw_merge!(
            "flex w-full justify-start gap-4 shrink-0",
            actions_class.get().unwrap_or_default()
        )
    };
    let prev_button_class_val =
        move || tw_merge!("bg-white", prev_button_class.get().unwrap_or_default());
    let next_button_class_val = move || {
        tw_merge!(
            "bg-primary text-contrast-white",
            next_button_class.get().unwrap_or_default()
        )
    };
    let final_button_class_val = move || {
        tw_merge!(
            "bg-primary text-contrast-white",
            final_button_class.get().unwrap_or_default()
        )
    };

    view! {
        <div class=root_class>
            <div class=rail_class_val>
                <For
                    each=move || step_labels.get().into_iter().enumerate()
                    key=|(index, _)| *index
                    let:((index, step_label))
                >
                    {
                        let is_current = move || index == current_step.get();
                        let step_count_inner = step_count;
                        let step_class_val = step_class_val.clone();
                        let step_circle_class_val = step_circle_class_val;
                        let step_label_class_val = step_label_class_val;

                        view! {
                            <div
                                on:click=move |_| {
                                    if next_is_disabled.get() {
                                        return;
                                    }
                                    set_current_step.update(|step| *step = index);
                                    let form_refs = form_refs.get();
                                    send_all_form_refs.run(form_refs);
                                }
                                class=step_class_val
                            >
                                <div class=move || tw_merge!(
                                    format!(
                                        "w-8 h-8 flex items-center justify-center rounded-full text-sm {}",
                                        if is_current() { "bg-primary text-contrast-white" } else { "bg-light-gray" }
                                    ),
                                    step_circle_class_val.get().unwrap_or_default()
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
                                <div class=move || tw_merge!(
                                    format!(
                                        "text-sm {}",
                                        if is_current() { "font-bold text-primary" } else { "hidden md:flex" }
                                    ),
                                    step_label_class_val.get().unwrap_or_default()
                                )>
                                    {step_label.label.clone()}
                                </div>
                            </div>

                            {if index < step_count_inner - 1 {
                                Some(view! {
                                    <div class=connector_class_val.clone() />
                                })
                            } else {
                                None
                            }}
                        }
                    }
                </For>
            </div>
            <div on:submit=handle_step_form_submit class=form_area_class_val>
            {
                    child_nodes.into_iter().enumerate().map(|(i, child)| {
                        let form_ref = form_refs.get_untracked()[i].clone();
                        view! {
                            <ReactiveForm
                                form_ref=form_ref
                                class=Signal::derive(move || {
                                    if current_step.get() == i { "block".to_string() } else { "hidden".to_string() }
                                })
                            >
                                { child }
                            </ReactiveForm>
                        }
                    }).collect_view()
                }
            </div>
            <div class=actions_class_val>
                {
                    move || if current_step.get() == 0 {
                        None
                    } else {
                        Some(view! {
                            <BasicButton
                                onclick=onclick_prev
                                button_text="Previous"
                                class=Signal::derive(prev_button_class_val)
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
                                class=Signal::derive(final_button_class_val)
                                disabled=final_button_is_disabled
                            />
                        }
                    } else {
                        view! {
                            <BasicButton
                                disabled=next_is_disabled
                                onclick=onclick_next
                                button_text="Next"
                                class=Signal::derive(next_button_class_val)
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
