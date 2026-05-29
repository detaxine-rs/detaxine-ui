use icondata::Icon as IconId;
use leptos::ev;
use leptos::prelude::*;
use leptos_icons::Icon; // Adjust based on your icon set (e.g., icondata::BsIcon)

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub enum ButtonType {
    #[default]
    Button,
    Submit,
    Reset,
}

/// This is a reusable button component which also includes a button group component for creating grouped buttons
/// Example usage:
/// ```
/// <ButtonGroup style_ext="font-bold bg-primary text-white hover:bg-secondary".to_string()>
///    <BasicButton
///        button_text="First"
///        icon=Some(IconId::AiCheckCircleOutlined)
///        icon_before=true
///     />
///     <BasicButton
///        button_text="Second"
///        icon=Some(IconId::BsXCircle)
///        icon_before=false
///      />
///      <BasicButton
///        button_text="Third"
///        disabled=true
///      />
///   </ButtonGroup>
/// ```
#[component]
pub fn BasicButton(
    #[prop(into, optional)] button_text: String,
    #[prop(into, optional)] style_ext: String,
    #[prop(into, optional, default = Signal::derive(move || "".to_string()))]
    style_ext_reactive: Signal<String>,
    #[prop(into, optional)] children_style_ext: String,
    #[prop(default = Callback::new(|_| {}))] onclick: Callback<ev::MouseEvent>,
    #[prop(default = None)] icon: Option<IconId>,
    #[prop(into, default = Signal::derive(move || false))] disabled: Signal<bool>,
    #[prop(into, default = ButtonType::Button)] button_type: ButtonType,
    #[prop(default = false)] icon_before: bool,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let button_text_styles = button_text.clone();
    let button_content_styles = move || {
        if button_text_styles.is_empty() {
            ""
        } else if icon_before {
            "gap-2"
        } else {
            "gap-2 flex-row-reverse"
        }
    };

    view! {
        <button
            type={
                match button_type {
                    ButtonType::Button => "button",
                    ButtonType::Submit => "submit",
                    ButtonType::Reset => "reset"
                }
            }
            class=move || format!(
                "font-bold py-2 px-4 cursor-pointer rounded-[5px] disabled:opacity-50 disabled:cursor-not-allowed {} {}",
                style_ext,
                style_ext_reactive.get()
            )
            on:click=move |ev| onclick.run(ev)
            disabled={disabled}
        >
            {
                if let Some(children) = &children {
                    Some(children())
                } else {
                    None
                }
            }
            {
                if children.is_none() {
                    Some(view! {
                        <span class=move || format!("flex flex-row items-center justify-center {} {}", button_content_styles(), children_style_ext)>
                            {move || match icon {
                                Some(button_icon) => view! {
                                    <Icon width="1em" height="1em" icon=button_icon />
                                }.into(),
                                None => None,
                            }}
                            <span>{button_text}</span>
                        </span>
                    })
                } else {
                    None
                }
            }
        </button>
    }
}

// Define the ButtonGroup component
#[component]
pub fn ButtonGroup(
    /// `style_ext` property will extend styles for the buttons. `N/B:` All buttons share the same styles(These styles will affect all the buttons).
    #[prop(default = "".to_string())]
    style_ext: String,
    mut children: ChildrenFragmentMut,
) -> impl IntoView {
    view! {
        <div class="flex" role="group">
            {
                let style_ext_view = style_ext.clone();
                let children_len = children().nodes.iter().collect::<Vec<_>>().len();
                children()
                .nodes
                .into_iter()
                .enumerate()
                .map(|(index, child)| {
                    let style_ext_view = style_ext_view.clone();
                    let class_name = move || {
                        let mut base = format!(
                            "font-bold py-2 px-4 border border-light-gray border-l-0 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed {}",
                            style_ext_view
                        );
                        if index == 0 {
                            base.push_str(" rounded-l-[5px]");
                        }

                        if index == children_len - 1 {
                            base.push_str(" rounded-r-[5px]");
                        }
                        base
                    };
                    view! {
                        {child.attr("class", class_name())}
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
