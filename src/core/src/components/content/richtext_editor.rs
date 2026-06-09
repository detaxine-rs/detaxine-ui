use icondata::{
    BiStrikethroughRegular, BsBraces, BsCode, BsImage, BsListOl, BsListUl, BsTypeItalic,
    BsTypeUnderline, FiBold, VsMarkdown,
};
use js_sys::wasm_bindgen::prelude::Closure;
use leptos::html::Div;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos::{ev, prelude::*};
use markdown;
use std::pin::Pin;
use web_sys::{Element, HtmlDivElement, HtmlInputElement, Node, window};

use crate::components::actions::button::BasicButton;
use crate::components::forms::input::{InputField, InputFieldType};
use crate::components::forms::select::{SelectInput, SelectOption};
use crate::components::forms::textarea::Textarea;
use crate::utils::forms::fire_bubbled_and_cancelable_event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtraFormatingOption {
    MarkdownUpload,
    ImageUpload,
    Heading,
    InlineCode,
    CodeBlock,
    Lists,
}

pub type InsertImageCallback =
    Callback<web_sys::File, Pin<Box<dyn Future<Output = Option<String>>>>>;

/// A rich text editor with a formatting toolbar supporting bold, italic, underline,
/// strikethrough, and optional extras like headings, inline code, code blocks, lists,
/// image upload, and markdown upload.
///
/// # Props
///
/// - `initial_content` – `RwSignal<String>` containing the initial HTML content. Defaults to `<p><br></p>`.
/// - `id_attr` – HTML `id` applied to the editor and its hidden inputs.
/// - `name` – `name` attribute forwarded to the hidden textarea for form submission.
/// - `placeholder` – Placeholder text shown when the editor is empty.
/// - `extra_formating_options` – Opt-in toolbar features via `Vec<ExtraFormatingOption>`:
///   `MarkdownUpload`, `ImageUpload`, `Heading`, `InlineCode`, `CodeBlock`, `Lists`.
/// - `on_image_insert` – Async callback receiving a `web_sys::File` and returning `Option<String>` (the URL to insert). Defaults to a base64 data-URL reader.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::content::richtext_editor::{ExtraFormatingOption, RichTextEditor};
///
/// #[component]
/// fn Example() -> impl IntoView {
///     let content = RwSignal::new("<p><br></p>".to_string());
///     view! {
///         <RichTextEditor
///             initial_content=content
///             id_attr="my-editor"
///             name="body"
///             extra_formating_options=vec![
///                 ExtraFormatingOption::Heading,
///                 ExtraFormatingOption::CodeBlock,
///                 ExtraFormatingOption::Lists,
///             ]
///         />
///     }
/// }
/// ```
#[component]
pub fn RichTextEditor(
    #[prop(into, optional, default = RwSignal::new("<p><br></p>".into()))]
    initial_content: RwSignal<String>,
    #[prop(into, optional)] id_attr: String,
    #[prop(into, optional)] name: String,
    #[prop(into, optional)] placeholder: String,
    #[prop(optional, default = Vec::new())] extra_formating_options: Vec<ExtraFormatingOption>,
    #[prop(optional, default = Callback::new(move |file: web_sys::File| {
        Box::pin(async move {
            gloo_file::futures::read_as_data_url(&file.into())
                .await
                .ok()
        }) as Pin<Box<dyn Future<Output = Option<String>>>>
    }))]
    on_image_insert: InsertImageCallback,
) -> impl IntoView {
    let editor_ref = NodeRef::new();
    let file_input_ref = NodeRef::new();
    let font_options = RwSignal::new(
        [
            ("p", "Normal"),
            ("h1", "H1"),
            ("h2", "H2"),
            ("h3", "H3"),
            ("h4", "H4"),
            ("h5", "H5"),
            ("h6", "H6"),
        ]
        .into_iter()
        .map(|(value, label)| SelectOption::new(value, label))
        .collect::<Vec<_>>(),
    );
    let language_options = RwSignal::new(
        [
            ("plaintext", "Plain Text"),
            ("rust", "Rust"),
            ("surql", "SurrealQL"),
            ("javascript", "JavaScript"),
            ("typescript", "TypeScript"),
            ("python", "Python"),
            ("java", "Java"),
            ("cpp", "C++"),
            ("c", "C"),
            ("csharp", "C#"),
            ("go", "Go"),
            ("ruby", "Ruby"),
            ("php", "PHP"),
            ("html", "HTML"),
            ("css", "CSS"),
            ("json", "JSON"),
            ("sql", "SQL"),
            ("bash", "Bash"),
            ("yaml", "YAML"),
            ("markdown", "Markdown"),
        ]
        .into_iter()
        .map(|(value, label)| SelectOption::new(value, label))
        .collect::<Vec<_>>(),
    );
    let last_enter_empty = RwSignal::new(false);
    let show_language_picker = RwSignal::new(false);
    let (tracked_content, set_tracked_content) = signal(String::new());
    let md_file_input_ref = NodeRef::new();

    // Track active formatting states
    let is_bold = RwSignal::new(false);
    let is_italic = RwSignal::new(false);
    let is_underline = RwSignal::new(false);
    let is_strikethrough = RwSignal::new(false);
    let is_inline_code = RwSignal::new(false);
    let is_code_block = RwSignal::new(false);
    let is_ordered_list = RwSignal::new(false);
    let is_unordered_list = RwSignal::new(false);

    // Create style functions for buttons
    let active_style = move |signal: RwSignal<bool>| {
        Memo::new(move |_| {
            if signal.get() {
                "bg-primary text-contrast-white"
            } else {
                "hover:bg-light-gray"
            }
            .into()
        })
    };

    let bold_style: Memo<String> = active_style(is_bold);
    let italic_style = active_style(is_italic);
    let underline_style = active_style(is_underline);
    let strikethrough_style = active_style(is_strikethrough);
    let inline_code_style = active_style(is_inline_code);
    let code_block_style = active_style(is_code_block);
    let ordered_list_style = active_style(is_ordered_list);
    let unordered_list_style = active_style(is_unordered_list);

    let update_button_states = move || {
        is_bold.set(cursor_inside("b").is_some());
        is_italic.set(cursor_inside("i").is_some());
        is_underline.set(cursor_inside("u").is_some());
        is_strikethrough.set(cursor_inside("s").is_some());
        is_inline_code.set(cursor_inside("code").is_some() && current_code_block().is_none());
        is_code_block.set(current_code_block().is_some());

        if let Some((list, _)) = current_list_item() {
            let tag = list.tag_name().to_lowercase();
            is_ordered_list.set(tag == "ol");
            is_unordered_list.set(tag == "ul");
        } else {
            is_ordered_list.set(false);
            is_unordered_list.set(false);
        }
    };

    let toggle_style = move |tag_name: &'static str| {
        if let Some(el) = cursor_inside(tag_name) {
            // Toggle off: insert a zero-width space after the element and move cursor there
            if let Some(doc) = window().and_then(|w| w.document()) {
                if let Ok(Some(selection)) = doc.get_selection() {
                    // Create a zero-width space to break out of the formatting
                    let space = doc.create_text_node("\u{200B}");

                    // Insert it after the styled element
                    if let Some(parent) = el.parent_node() {
                        if let Some(next_sibling) = el.next_sibling() {
                            parent.insert_before(&space, Some(&next_sibling)).ok();
                        } else {
                            parent.append_child(&space).ok();
                        }

                        // Move cursor after the zero-width space
                        if let Ok(new_range) = doc.create_range() {
                            new_range.set_start(&space, 1).ok();
                            new_range.set_end(&space, 1).ok();
                            selection.remove_all_ranges().ok();
                            selection.add_range(&new_range).ok();
                        }
                    }
                }
            }
        } else {
            // Toggle on: Wrap selection or insert at cursor
            if let Some(doc) = window().and_then(|w| w.document()) {
                if let Ok(Some(selection)) = doc.get_selection() {
                    if let Ok(range) = selection.get_range_at(0) {
                        if let Ok(element) = doc.create_element(tag_name) {
                            if range.collapsed() {
                                // Insert zero-width space so caret can live inside the tag
                                let text = doc.create_text_node("\u{200B}");
                                let _ = element.append_child(&text);
                                let _ = range.insert_node(&element);
                                if let Ok(new_range) = doc.create_range() {
                                    // Place cursor AFTER the zero-width space (inside tag)
                                    let _ = new_range.set_start(&text, 1);
                                    let _ = new_range.set_end(&text, 1);
                                    let _ = selection.remove_all_ranges();
                                    let _ = selection.add_range(&new_range);
                                }
                            } else {
                                // Wrap selected text
                                let contents = range.clone_contents().ok();
                                range.delete_contents().ok();
                                if let Some(contents) = contents {
                                    element.append_child(&contents).ok();
                                }
                                range.insert_node(&element).ok();

                                // Move cursor to the end of the new element
                                if let Ok(new_range) = doc.create_range() {
                                    new_range.select_node_contents(&element).ok();
                                    new_range.collapse_with_to_start(false);
                                    let _ = selection.remove_all_ranges();
                                    let _ = selection.add_range(&new_range);
                                }
                            }
                        }
                    }
                }
            }
        }
        update_button_states();
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() != "Enter" {
            return;
        }

        // CODE BLOCK
        if let Some((pre, code)) = current_code_block() {
            ev.prevent_default();
            handle_code_enter(&pre, &code, &last_enter_empty);
            update_button_states();
            return;
        }

        // INLINE CODE
        if let Some(code) = current_inline_code() {
            ev.prevent_default();
            handle_inline_code_enter(&code);
            update_button_states();
            return;
        }

        // LIST ITEM
        if let Some((list, li)) = current_list_item() {
            ev.prevent_default();
            handle_list_enter(&list, &li);
            update_button_states();
            return;
        }

        // DEFAULT
        // Let browser handle paragraphs, headings, etc.

        if let Some(window) = window() {
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                Closure::once_into_js(move || {
                    update_button_states();
                })
                .as_ref()
                .unchecked_ref(),
                0,
            );
        }
    };

    // Update button states on selection change
    let on_selection_change = move |_: ev::MouseEvent| {
        update_button_states();
    };

    let on_keyup = move |_: ev::KeyboardEvent| {
        update_button_states();
    };

    let bold = Callback::new(move |_| toggle_style("b"));
    let italic = Callback::new(move |_| toggle_style("i"));
    let underline = Callback::new(move |_| toggle_style("u"));
    let strikethrough = Callback::new(move |_| toggle_style("s"));

    // Fixed inline code handler - check if we're in a code block first
    let inline_code = Callback::new(move |_| {
        // Don't toggle inline code if we're in a code block
        if current_code_block().is_none() {
            toggle_style("code");
        }
    });

    let code_block = Callback::new(move |_| {
        // If we're already in a code block, exit it
        if let Some((pre, _code)) = current_code_block() {
            exit_code_block(&pre);
            show_language_picker.set(false);
        } else {
            insert_code_block();
            show_language_picker.set(true);
        }
        update_button_states();
    });

    // Insert image callback
    let insert_image = Callback::new(move |_| {
        if let Some(input) = file_input_ref.get() as Option<HtmlInputElement> {
            let _ = input.click();
        }
    });

    // Handle file selection
    let on_file_change = move |_ev: ev::Event| {
        let Some(file_input) = file_input_ref.get() as Option<HtmlInputElement> else {
            return;
        };
        let Some(files) = file_input.files() else {
            return;
        };

        for i in 0..files.length() {
            let Some(file) = files.item(i) else {
                continue;
            };
            let editor_ref = editor_ref.clone();
            spawn_local(async move {
                if let Some(url) = on_image_insert.run(file).await {
                    insert_image_at_cursor(&url, &editor_ref);
                }
            });
        }
    };

    // Add this effect to position the cursor
    Effect::new(move |_| {
        if let Some(editor) = editor_ref.get() as Option<HtmlDivElement> {
            if let Some(doc) = window().and_then(|w| w.document()) {
                if let Ok(Some(selection)) = doc.get_selection() {
                    if let Some(p) = editor.first_element_child() {
                        if let Ok(range) = doc.create_range() {
                            range.set_start(&p, 0).ok();
                            range.set_end(&p, 0).ok();
                            selection.remove_all_ranges().ok();
                            selection.add_range(&range).ok();
                        };
                    }
                }
            }
        }
    });

    let apply_heading = move |ev: ev::Event| {
        let tag = event_target_value(&ev);
        if let Some(editor) = editor_ref.get() as Option<HtmlDivElement> {
            if let Some(doc) = window().and_then(|w| w.document()) {
                if let Ok(Some(selection)) = doc.get_selection() {
                    if let Ok(range) = selection.get_range_at(0) {
                        if let Ok(mut node) = range.start_container() {
                            let editor_node: &Node = editor.as_ref();
                            loop {
                                if let Some(el) = node.dyn_ref::<Element>() {
                                    if let Some(parent) = el.parent_element() {
                                        if parent.is_same_node(Some(editor_node)) {
                                            if ["p", "h1", "h2", "h3", "h4", "h5", "h6"]
                                                .contains(&el.tag_name().to_lowercase().as_str())
                                            {
                                                // let new_el =
                                                //     doc.create_element(&tag).unwrap_or_default();
                                                if let (Ok(new_el), Ok(new_range)) =
                                                    (doc.create_element(&tag), doc.create_range())
                                                {
                                                    while let Some(child) = el.first_child() {
                                                        new_el.append_child(&child).ok();
                                                    }
                                                    parent.replace_child(&new_el, el).ok();
                                                    new_range.select_node_contents(&new_el).ok();
                                                    new_range.collapse();
                                                    selection.remove_all_ranges().ok();
                                                    selection.add_range(&new_range).ok();
                                                    break;
                                                };
                                            }
                                        }
                                    }
                                }
                                match node.parent_node() {
                                    Some(p) => node = p,
                                    None => break,
                                }
                            }
                        };
                    }
                }
            }
        }
    };

    let apply_language = move |ev: ev::Event| {
        let lang = event_target_value(&ev);
        if let Some((_pre, code)) = current_code_block() {
            code.set_attribute("class", &format!("language-{}", lang))
                .ok();
        }
        show_language_picker.set(false);
    };

    let handle_on_input = move |_: ev::Event| {
        if let Some(editor) = editor_ref.get() as Option<HtmlDivElement> {
            set_tracked_content.set(editor.inner_html());
        }
    };

    Effect::new(move |_| {
        let changed_value = initial_content.get();
        set_tracked_content.set(changed_value);
    });

    let upload_md = Callback::new(move |_| {
        if let Some(input) = md_file_input_ref.get() as Option<HtmlInputElement> {
            let _ = input.click();
        }
    });

    let on_md_file_change = move |_ev: ev::Event| {
        if let Some(file_input) = md_file_input_ref.get() as Option<HtmlInputElement> {
            if let Some(files) = file_input.files() {
                if let Some(file) = files.item(0) {
                    spawn_local(async move {
                        match gloo_file::futures::read_as_text(&file.into()).await {
                            Ok(markdown_content) => {
                                // Parse markdown to HTML using markdown crate
                                // let html_output = markdown::to_html(&markdown_content);
                                if let Ok(html_output) = markdown::to_html_with_options(
                                    &markdown_content,
                                    &markdown::Options::gfm(),
                                ) {
                                    // Also update the initial content
                                    initial_content.set(html_output);
                                };
                            }
                            Err(err) => {
                                leptos::logging::error!("Failed to read markdown file: {:?}", err);
                            }
                        }
                    });
                }
            }
        }
    };

    // Ordered list
    let ordered_list = Callback::new(move |_| {
        if let Some(editor) = editor_ref.get() {
            if let Some((list, _)) = current_list_item() {
                // If we're in a list, exit it
                exit_list(&editor, &list);
            } else {
                // Otherwise, insert a new ordered list
                insert_list(&editor, "ol");
            }
        }
        update_button_states();
    });

    // Unordered list
    let unordered_list = Callback::new(move |_| {
        if let Some(editor) = editor_ref.get() {
            if let Some((list, _)) = current_list_item() {
                // If we're in a list, exit it
                exit_list(&editor, &list);
            } else {
                // Otherwise, insert a new unordered list
                insert_list(&editor, "ul");
            }
        }
        update_button_states();
    });

    view! {
        <div class="border-[1px] border-light-gray rounded-[5px]">
            // Toolbar
            <div class="flex gap-2 items-center flex-wrap border-b-[1px] border-light-gray p-[10px]">
                {
                    extra_formating_options.contains(&ExtraFormatingOption::Heading).then(|| view!{
                        <SelectInput initial_value="p" id_attr="font-sizes" options=font_options on:change=apply_heading />
                    })
                }
                <BasicButton
                    icon=Some(FiBold)
                    icon_before=true
                    onclick=bold
                    style_ext=bold_style
                />
                <BasicButton
                    icon=Some(BsTypeItalic)
                    icon_before=true
                    onclick=italic
                    style_ext=italic_style
                />
                <BasicButton
                    icon=Some(BsTypeUnderline)
                    icon_before=true
                    onclick=underline
                    style_ext=underline_style
                />
                <BasicButton
                    icon=Some(BiStrikethroughRegular)
                    icon_before=true
                    onclick=strikethrough
                    style_ext=strikethrough_style
                />
                {
                    extra_formating_options.contains(&ExtraFormatingOption::ImageUpload).then(|| view!{
                        <BasicButton icon=Some(BsImage) icon_before=true onclick=insert_image style_ext="hover:bg-light-gray" />
                    })
                }
                {
                    extra_formating_options.contains(&ExtraFormatingOption::InlineCode).then(|| view!{
                        <BasicButton
                            icon=Some(BsCode)
                            onclick=inline_code
                            style_ext=inline_code_style
                        />
                    })
                }
                {
                    extra_formating_options.contains(&ExtraFormatingOption::CodeBlock).then(|| view!{
                        <BasicButton
                            icon=Some(BsBraces)
                            onclick=code_block
                            style_ext=code_block_style
                        />

                        <Show when=move || show_language_picker.get()>
                            <div class="ml-2">
                                <SelectInput initial_value="plaintext" id_attr="code-language" options=language_options on:change=apply_language />
                            </div>
                        </Show>
                    })
                }
                {
                    extra_formating_options.contains(&ExtraFormatingOption::MarkdownUpload).then(|| view!{
                        <BasicButton
                            icon=Some(VsMarkdown)
                            icon_before=true
                            onclick=upload_md
                            style_ext="hover:bg-light-gray"
                        />
                    })
                }


                {
                    extra_formating_options.contains(&ExtraFormatingOption::Lists).then(|| view!{
                        <BasicButton
                            icon=Some(BsListOl)
                            icon_before=true
                            onclick=ordered_list
                            style_ext=ordered_list_style
                        />
                        <BasicButton
                            icon=Some(BsListUl)
                            icon_before=true
                            onclick=unordered_list
                            style_ext=unordered_list_style
                        />
                    })
                }
            </div>

            // Editor
            <div
                node_ref=editor_ref
                contenteditable="true"
                on:keydown=on_keydown
                on:click=on_selection_change
                on:keyup=on_keyup
                class="min-h-[200px] max-h-[45svh] overflow-y-auto p-3 prose focus:rounded-b-none outline-secondary"
                inner_html=move || initial_content.get()
                on:input=handle_on_input
            />
            <InputField field_type=InputFieldType::File input_node_ref=file_input_ref accept="image/*" on:change=on_file_change ext_input_styles="hidden" id_attr=format!("{}-file-input", id_attr) />


            <Textarea id_attr=format!("{}-text-input", id_attr) ext_input_styles="hidden" initial_value=tracked_content name=name />

            <InputField
                field_type=InputFieldType::File
                input_node_ref=md_file_input_ref
                accept=".md,.markdown"
                on:change=on_md_file_change
                ext_input_styles="hidden"
                id_attr=format!("{}-md-file-input", id_attr)
            />
        </div>
    }
}

fn cursor_inside(tag: &str) -> Option<Element> {
    let doc = window()?.document()?;
    let selection = doc.get_selection().ok()??;
    let range = selection.get_range_at(0).ok()?;

    let container = range.start_container().ok()?;

    let mut node = if let Some(text) = container.dyn_ref::<web_sys::Text>() {
        text.parent_element()?
    } else if let Some(el) = container.dyn_ref::<Element>() {
        el.clone()
    } else {
        return None;
    };

    loop {
        if node.tag_name().eq_ignore_ascii_case(tag) {
            return Some(node);
        }
        match node.parent_element() {
            Some(p) => node = p,
            None => break,
        }
    }

    None
}

fn exit_code_block(pre: &Element) {
    if let (Some(doc),) = (window().and_then(|w| w.document()),) {
        if let (Ok(p), Ok(br), Ok(new_range), Ok(Some(sel))) = (
            doc.create_element("p"),
            doc.create_element("br"),
            doc.create_range(),
            doc.get_selection(),
        ) {
            p.append_child(&br).ok();
            pre.after_with_node_1(&p).ok();
            new_range.set_start(&p, 0).ok();
            new_range.set_end(&p, 0).ok();
            sel.remove_all_ranges().ok();
            sel.add_range(&new_range).ok();
        }
    }
}

fn insert_code_block() {
    if let Some(doc) = window().and_then(|w| w.document()) {
        if let (Ok(Some(selection)), Ok(pre), Ok(code), Ok(new_range)) = (
            doc.get_selection(),
            doc.create_element("pre"),
            doc.create_element("code"),
            doc.create_range(),
        ) {
            if let Ok(range) = selection.get_range_at(0) {
                let text = doc.create_text_node("\n");
                pre.set_attribute("data-block", "code").ok();
                code.set_attribute("class", "language-plaintext").ok();
                code.append_child(&text).ok();
                pre.append_child(&code).ok();
                range.delete_contents().ok();
                range.insert_node(&pre).ok();
                new_range.set_start(&text, 1).ok();
                new_range.set_end(&text, 1).ok();
                selection.remove_all_ranges().ok();
                selection.add_range(&new_range).ok();
            }
        }
    }
}

fn current_code_block() -> Option<(web_sys::Element, web_sys::Element)> {
    let doc = window()?.document()?;
    let sel = doc.get_selection().ok()??;
    let range = sel.get_range_at(0).ok()?;

    let mut node = range.start_container().ok()?;

    loop {
        if let Some(code) = node.dyn_ref::<Element>() {
            if code.tag_name().eq_ignore_ascii_case("code") {
                if let Some(pre) = code.parent_element() {
                    if pre.tag_name().eq_ignore_ascii_case("pre") {
                        return Some((pre, code.clone()));
                    }
                }
            }
        }
        node = node.parent_node()?;
    }
}

fn is_current_line_empty(_code: &Element) -> bool {
    let doc = match window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return false,
    };

    let sel = match doc.get_selection().ok().flatten() {
        Some(s) if s.range_count() > 0 => s,
        _ => return false,
    };

    let range = match sel.get_range_at(0) {
        Ok(r) => r,
        Err(_) => return false,
    };

    let container = match range.start_container() {
        Ok(n) => n,
        Err(_) => return false,
    };

    if container.node_type() != Node::TEXT_NODE {
        return false;
    }

    let text_node: web_sys::Text = container.unchecked_into();
    let value = text_node.data();

    let offset = range.start_offset().unwrap_or(0) as usize;
    let offset = offset.min(value.len());

    let before = &value[..offset];
    let line_start = before.rfind('\n').map(|i| i + 1).unwrap_or(0);
    let line = &before[line_start..];

    line.trim().is_empty()
}

fn handle_code_enter(pre: &Element, code: &Element, last_enter_empty: &RwSignal<bool>) {
    let Some(doc) = window().and_then(|w| w.document()) else {
        return;
    };

    let Ok(Some(sel)) = doc.get_selection() else {
        return;
    };

    if sel.range_count() == 0 {
        return;
    }

    let Ok(range) = sel.get_range_at(0) else {
        return;
    };

    let Ok(container) = range.start_container() else {
        return;
    };

    if !code.contains(Some(&container)) {
        return;
    }

    let empty = is_current_line_empty(code);

    // -------- Empty line logic --------
    if empty {
        if last_enter_empty.get() {
            // Second empty line -> exit code block
            last_enter_empty.set(false);

            if let (Ok(p), Ok(br), Ok(new_range)) = (
                doc.create_element("p"),
                doc.create_element("br"),
                doc.create_range(),
            ) {
                p.append_child(&br).ok();

                // Append paragraph AFTER the code block
                pre.after_with_node_1(&p).ok();

                // Move cursor into the new paragraph
                new_range.set_start(&p, 0).ok();
                new_range.set_end(&p, 0).ok();

                sel.remove_all_ranges().ok();
                sel.add_range(&new_range).ok();
            }

            return;
        } else {
            // Allow first empty line
            last_enter_empty.set(true);
        }
    } else {
        // Reset state if line has content
        last_enter_empty.set(false);
    }

    // Delete selection if not collapsed
    if !range.collapsed() {
        range.delete_contents().ok();
    }

    match container.node_type() {
        Node::TEXT_NODE => {
            let text_node: web_sys::Text = container.unchecked_into();

            let Ok(offset) = range.start_offset() else {
                return;
            };

            let offset = offset as usize;
            let value = text_node.data();
            let offset = offset.min(value.len());

            let (before, after) = value.split_at(offset);

            let new_data = if after.is_empty() {
                format!("{before}\n\u{200B}")
            } else {
                format!("{before}\n{after}")
            };

            text_node.set_data(&new_data);

            if let Ok(new_range) = doc.create_range() {
                new_range.set_start(&text_node, (offset + 1) as u32).ok();
                new_range.set_end(&text_node, (offset + 1) as u32).ok();

                sel.remove_all_ranges().ok();
                sel.add_range(&new_range).ok();
            }
        }

        Node::ELEMENT_NODE => {
            let new_text = doc.create_text_node("\n\u{200B}");
            range.insert_node(&new_text).ok();

            if let Ok(new_range) = doc.create_range() {
                new_range.set_start(&new_text, 1).ok();
                new_range.set_end(&new_text, 1).ok();

                sel.remove_all_ranges().ok();
                sel.add_range(&new_range).ok();
            }
        }

        _ => {}
    }
}

fn current_inline_code() -> Option<Element> {
    let doc = window()?.document()?;
    let sel = doc.get_selection().ok()??;

    if sel.range_count() == 0 {
        return None;
    }

    let range = sel.get_range_at(0).ok()?;
    let container = range.start_container().ok()?;

    let element = match container.node_type() {
        Node::ELEMENT_NODE => container.unchecked_into::<Element>(),
        _ => container.parent_element()?,
    };

    let code = element.closest("code").ok()??;

    // Reject <pre><code>
    if code.closest("pre").ok()?.is_some() {
        return None;
    }

    Some(code)
}

fn handle_inline_code_enter(code: &Element) {
    let Some(doc) = window().and_then(|w| w.document()) else {
        return;
    };

    let Ok(Some(sel)) = doc.get_selection() else {
        return;
    };

    let Some(parent_p) = code.closest("p").ok().flatten() else {
        return;
    };

    let Ok(p) = doc.create_element("p") else {
        return;
    };

    let Ok(br) = doc.create_element("br") else {
        return;
    };

    p.append_child(&br).ok();

    parent_p.after_with_node_1(&p).ok();

    if let Ok(range) = doc.create_range() {
        range.set_start(&p, 0).ok();
        range.set_end(&p, 0).ok();

        sel.remove_all_ranges().ok();
        sel.add_range(&range).ok();
    }
}

fn insert_list(editor: &HtmlDivElement, list_type: &str) {
    let Some(doc) = window().and_then(|w| w.document()) else {
        return;
    };
    let Ok(Some(selection)) = doc.get_selection() else {
        return;
    };
    let Ok(range) = selection.get_range_at(0) else {
        return;
    };
    let Ok(list) = doc.create_element(list_type) else {
        return;
    };

    let has_selection = !range.collapsed();

    if has_selection {
        if let Ok(contents) = range.clone_contents() {
            let mut has_blocks = false;
            let mut current_child = contents.first_child();

            while let Some(node) = current_child {
                if let Some(el) = node.dyn_ref::<Element>() {
                    let tag = el.tag_name().to_lowercase();
                    if ["p", "h1", "h2", "h3", "h4", "h5", "h6", "div"].contains(&tag.as_str()) {
                        has_blocks = true;
                        break;
                    }
                }
                current_child = node.next_sibling();
            }

            if has_blocks {
                current_child = contents.first_child();

                while let Some(node) = current_child {
                    let next = node.next_sibling();

                    if let Some(el) = node.dyn_ref::<Element>() {
                        let tag = el.tag_name().to_lowercase();
                        if ["p", "h1", "h2", "h3", "h4", "h5", "h6", "div"].contains(&tag.as_str())
                        {
                            if let Ok(li) = doc.create_element("li") {
                                while let Some(child) = el.first_child() {
                                    li.append_child(&child).ok();
                                }
                                if li.first_child().is_none() {
                                    if let Ok(br) = doc.create_element("br") {
                                        li.append_child(&br).ok();
                                    }
                                }
                                list.append_child(&li).ok();
                            }
                        }
                    } else if node.node_type() == 3 {
                        if let Some(text) = node.text_content() {
                            if !text.trim().is_empty() {
                                if let (Ok(li), Ok(cloned)) =
                                    (doc.create_element("li"), node.clone_node())
                                {
                                    li.append_child(&cloned).ok();
                                    list.append_child(&li).ok();
                                }
                            }
                        }
                    }

                    current_child = next;
                }
            } else {
                if let Ok(li) = doc.create_element("li") {
                    li.append_child(&contents).ok();
                    list.append_child(&li).ok();
                }
            }
        }
    } else {
        if let (Ok(li), Ok(br)) = (doc.create_element("li"), doc.create_element("br")) {
            li.append_child(&br).ok();
            list.append_child(&li).ok();
        }
    }

    let Ok(mut node) = range.start_container() else {
        return;
    };
    let editor_as_node: &web_sys::Node = editor.as_ref();

    loop {
        if let Some(el) = node.dyn_ref::<web_sys::Element>() {
            let tag = el.tag_name().to_lowercase();
            if ["p", "h1", "h2", "h3", "h4", "h5", "h6"].contains(&tag.as_str()) {
                if let Some(parent) = el.parent_element() {
                    if parent.is_same_node(Some(editor_as_node)) {
                        el.after_with_node_1(&list).ok();
                        if let (Some(first_li), Ok(new_range)) =
                            (list.first_element_child(), doc.create_range())
                        {
                            new_range.select_node_contents(&first_li).ok();
                            new_range.collapse_with_to_start(false);
                            selection.remove_all_ranges().ok();
                            selection.add_range(&new_range).ok();
                        }
                        return;
                    }
                }
            }
        }
        match node.parent_node() {
            Some(parent) => node = parent,
            None => break,
        }
    }

    // Fallback: append to editor if no block found
    editor.append_child(&list).ok();
    if let (Some(first_li), Ok(new_range)) = (list.first_element_child(), doc.create_range()) {
        new_range.select_node_contents(&first_li).ok();
        new_range.collapse_with_to_start(false);
        selection.remove_all_ranges().ok();
        selection.add_range(&new_range).ok();
    }
}

fn current_list_item() -> Option<(web_sys::Element, web_sys::Element)> {
    let doc = window()?.document()?;
    let sel = doc.get_selection().ok()??;
    let range = sel.get_range_at(0).ok()?;

    let mut node = range.start_container().ok()?;

    loop {
        if let Some(li) = node.dyn_ref::<Element>() {
            if li.tag_name().eq_ignore_ascii_case("li") {
                if let Some(list) = li.parent_element() {
                    let tag = list.tag_name().to_lowercase();
                    if tag == "ol" || tag == "ul" {
                        return Some((list, li.clone()));
                    }
                }
            }
        }
        node = node.parent_node()?;
    }
}

fn handle_list_enter(list: &Element, li: &Element) {
    let Some(doc) = window().and_then(|w| w.document()) else {
        return;
    };
    let Ok(Some(sel)) = doc.get_selection() else {
        return;
    };

    let is_empty = li
        .text_content()
        .map(|t| t.trim().is_empty())
        .unwrap_or(true);

    if is_empty {
        if let (Ok(p), Ok(br), Ok(new_range)) = (
            doc.create_element("p"),
            doc.create_element("br"),
            doc.create_range(),
        ) {
            p.append_child(&br).ok();
            list.after_with_node_1(&p).ok();
            li.remove();
            if list.children().length() == 0 {
                list.remove();
            }
            new_range.set_start(&p, 0).ok();
            new_range.set_end(&p, 0).ok();
            sel.remove_all_ranges().ok();
            sel.add_range(&new_range).ok();
        }
    } else {
        if let (Ok(new_li), Ok(br), Ok(new_range)) = (
            doc.create_element("li"),
            doc.create_element("br"),
            doc.create_range(),
        ) {
            new_li.append_child(&br).ok();
            match li.next_sibling() {
                Some(next_sibling) => {
                    list.insert_before(&new_li, Some(&next_sibling)).ok();
                }
                None => {
                    list.append_child(&new_li).ok();
                }
            }
            new_range.set_start(&new_li, 0).ok();
            new_range.set_end(&new_li, 0).ok();
            sel.remove_all_ranges().ok();
            sel.add_range(&new_range).ok();
        }
    }
}

fn exit_list(_editor: &HtmlDivElement, list: &Element) {
    let Some(doc) = window().and_then(|w| w.document()) else {
        return;
    };
    let Ok(Some(sel)) = doc.get_selection() else {
        return;
    };

    if let (Ok(p), Ok(br), Ok(new_range)) = (
        doc.create_element("p"),
        doc.create_element("br"),
        doc.create_range(),
    ) {
        p.append_child(&br).ok();
        list.after_with_node_1(&p).ok();
        new_range.set_start(&p, 0).ok();
        new_range.set_end(&p, 0).ok();
        sel.remove_all_ranges().ok();
        sel.add_range(&new_range).ok();
    }
}

fn insert_image_at_cursor(src: &str, editor_ref: &NodeRef<Div>) {
    let Some(doc) = window().and_then(|w| w.document()) else {
        return;
    };
    let Ok(Some(selection)) = doc.get_selection() else {
        return;
    };
    let Ok(range) = selection.get_range_at(0) else {
        return;
    };
    let Ok(img) = doc.create_element("img") else {
        return;
    };

    img.set_attribute("src", src).unwrap_or_default();
    img.set_attribute("style", "max-width: 100%; height: auto;")
        .unwrap_or_default();

    range.delete_contents().ok();
    range.insert_node(&img).ok();

    if let Ok(new_range) = doc.create_range() {
        new_range.set_start_after(&img).ok();
        new_range.set_end_after(&img).ok();
        selection.remove_all_ranges().ok();
        selection.add_range(&new_range).ok();
    }

    if let Some(editor) = editor_ref.get_untracked() as Option<HtmlDivElement> {
        fire_bubbled_and_cancelable_event("input", true, true, &editor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ExtraFormatingOption

    #[test]
    fn extra_formatting_option_eq() {
        assert_eq!(
            ExtraFormatingOption::CodeBlock,
            ExtraFormatingOption::CodeBlock
        );
        assert_ne!(
            ExtraFormatingOption::CodeBlock,
            ExtraFormatingOption::InlineCode
        );
    }

    #[test]
    fn extra_formatting_option_clone() {
        let opt = ExtraFormatingOption::Lists;
        assert_eq!(opt.clone(), ExtraFormatingOption::Lists);
    }

    #[test]
    fn extra_formatting_option_hash() {
        use std::collections::HashSet;
        let set: HashSet<ExtraFormatingOption> = [
            ExtraFormatingOption::Heading,
            ExtraFormatingOption::Lists,
            ExtraFormatingOption::Heading, // duplicate
        ]
        .into_iter()
        .collect();
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn toolbar_contains_check_positive() {
        let opts = vec![
            ExtraFormatingOption::Heading,
            ExtraFormatingOption::CodeBlock,
        ];
        assert!(opts.contains(&ExtraFormatingOption::Heading));
        assert!(opts.contains(&ExtraFormatingOption::CodeBlock));
    }

    #[test]
    fn toolbar_contains_check_negative() {
        let opts = vec![ExtraFormatingOption::Heading];
        assert!(!opts.contains(&ExtraFormatingOption::Lists));
        assert!(!opts.contains(&ExtraFormatingOption::ImageUpload));
        assert!(!opts.contains(&ExtraFormatingOption::InlineCode));
        assert!(!opts.contains(&ExtraFormatingOption::MarkdownUpload));
    }

    #[test]
    fn empty_extra_options_contains_nothing() {
        let opts: Vec<ExtraFormatingOption> = vec![];
        assert!(!opts.contains(&ExtraFormatingOption::CodeBlock));
    }

    // Active style logic

    fn active_style(is_active: bool) -> &'static str {
        if is_active {
            "bg-primary text-contrast-white"
        } else {
            "hover:bg-light-gray"
        }
    }

    #[test]
    fn active_style_when_true() {
        assert_eq!(active_style(true), "bg-primary text-contrast-white");
    }

    #[test]
    fn active_style_when_false() {
        assert_eq!(active_style(false), "hover:bg-light-gray");
    }

    // Reactive active style

    #[test]
    fn active_style_signal_reflects_state() {
        let owner = Owner::new();
        owner.with(|| {
            let is_bold = RwSignal::new(false);
            assert_eq!(active_style(is_bold.get()), "hover:bg-light-gray");

            is_bold.set(true);
            assert_eq!(
                active_style(is_bold.get()),
                "bg-primary text-contrast-white"
            );
        });
    }

    // is_current_line_empty logic

    fn line_before_cursor(text: &str, offset: usize) -> &str {
        let before = &text[..offset.min(text.len())];
        let line_start = before.rfind('\n').map(|i| i + 1).unwrap_or(0);
        &before[line_start..]
    }

    #[test]
    fn empty_line_detected_at_start() {
        assert!(line_before_cursor("", 0).trim().is_empty());
    }

    #[test]
    fn empty_line_detected_after_newline() {
        assert!(line_before_cursor("code\n", 5).trim().is_empty());
    }

    #[test]
    fn non_empty_line_not_detected_as_empty() {
        assert!(!line_before_cursor("let x = 1;", 10).trim().is_empty());
    }

    #[test]
    fn line_before_cursor_splits_on_last_newline() {
        let text = "line1\nline2\n";
        assert_eq!(line_before_cursor(text, 11), "line2");
    }

    // handle_list_enter empty-li logic

    fn li_is_empty(text_content: Option<&str>) -> bool {
        text_content.map(|t| t.trim().is_empty()).unwrap_or(true)
    }

    #[test]
    fn empty_li_exits_list() {
        assert!(li_is_empty(Some("")));
        assert!(li_is_empty(Some("   ")));
        assert!(li_is_empty(None));
    }

    #[test]
    fn non_empty_li_appends_new_item() {
        assert!(!li_is_empty(Some("some text")));
    }

    // last_enter_empty double-enter logic

    #[test]
    fn first_empty_enter_sets_flag() {
        let owner = Owner::new();
        owner.with(|| {
            let last_enter_empty = RwSignal::new(false);
            // simulate first empty-line enter
            last_enter_empty.set(true);
            assert!(last_enter_empty.get());
        });
    }

    #[test]
    fn second_empty_enter_exits_and_resets_flag() {
        let owner = Owner::new();
        owner.with(|| {
            let last_enter_empty = RwSignal::new(true);
            // simulate second empty-line enter
            last_enter_empty.set(false);
            assert!(!last_enter_empty.get());
        });
    }

    #[test]
    fn non_empty_line_resets_flag() {
        let owner = Owner::new();
        owner.with(|| {
            let last_enter_empty = RwSignal::new(true);
            last_enter_empty.set(false);
            assert!(!last_enter_empty.get());
        });
    }
}
