use icondata::{BiChevronLeftRegular, BiChevronRightRegular};
use leptos::prelude::*;
use leptos_icons::Icon;

/// Carousel component for displaying a series of items in a sliding manner.
///
/// # Example
/// ```rust
/// use leptos::prelude::*;
///
/// view! {
///     <Carousel>
///         <div>"Slide 1"</div>
///         <div>"Slide 2"</div>
///         <div>"Slide 3"</div>
///     </Carousel>
/// }
/// ```
#[component]
pub fn Carousel(mut children: ChildrenFragmentMut) -> impl IntoView {
    let children_vec = children()
        .nodes
        .into_iter()
        .map(|n| n.into_view())
        .collect::<Vec<_>>();
    let total_slides = children_vec.len();
    if total_slides == 0 {
        return view! { <div></div> }.into_any();
    }

    let (current_index, set_current_index) = signal(0);
    let current_index_read = current_index.clone();

    let next_slide = move || {
        set_current_index.update(|idx| *idx = (*idx + 1) % total_slides);
    };

    let prev_slide = move || {
        set_current_index.update(|idx| {
            *idx = if *idx == 0 {
                total_slides - 1
            } else {
                *idx - 1
            }
        });
    };

    view! {
        <div class="relative overflow-hidden">
            // Slides container
            <div
                class="flex transition-transform duration-500 ease-in-out"
                style:transform=move || format!("translateX(-{}%)", current_index_read.get() * 100)
            >

            {children_vec.into_iter().map(|slide| view! {
                <div class="shrink-0 w-full">
                    {slide}
                </div>
            }).collect::<Vec<_>>()}

            </div>

            // Previous button
            <button
                class="absolute left-0 top-1/2 transform -translate-y-1/2 bg-transparent text-white hover:bg-opacity-75 transition-opacity z-10 h-full cursor-pointer"
                on:click=move |_| prev_slide()
            >
                <Icon width="1.5em" height="1.5em" icon=BiChevronLeftRegular />
            </button>

            // Next button
            <button
                class="absolute right-0 top-1/2 transform -translate-y-1/2 bg-transparent text-white hover:bg-opacity-75 transition-opacity z-10 h-full cursor-pointer"
                on:click=move |_| next_slide()
            >
                <Icon width="1.5em" height="1.5em" icon=BiChevronRightRegular />
            </button>

            // Indicators
            <div class="absolute bottom-4 left-1/2 transform -translate-x-1/2 flex space-x-2">
                           {move || (0..total_slides).map(|i| view! {
                               <button
                                   class=move || format!("w-6 h-[2.5px] rounded-[5px] {}", if current_index_read.get() == i {
                                       "bg-mid-gray"
                                   } else {
                                       "bg-contrast-white hover:bg-light-gray"
                                   })
                                   on:click=move |_| set_current_index.set(i)
                               ></button>
                           }).collect::<Vec<_>>()}
                       </div>
        </div>
    }.into_any()
}
