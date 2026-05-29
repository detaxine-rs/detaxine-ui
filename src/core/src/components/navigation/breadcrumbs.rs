use crate::components::schemas::props::StringVec;
use icondata::BiChevronRightRegular;

use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::{components::A, hooks::use_location};

/// This component renders a breadcrumb navigation based on the current route.
/// Example usage:
/// ```
/// // The custom_route_names prop is optional and defaults to ["Home"].
/// <Breadcrumbs custom_route_names=["Home", "Sign In"] />
/// ```
#[component]
pub fn Breadcrumbs(
    /// These routes are named in order of appearance and if the `custom_route_names` prop is not specified, the first route is by default named "Home". If specified, you need to provide a name for each route, including the first route.
    #[prop(into, default = StringVec(vec!["Home".to_string()]), optional)]
    custom_route_names: StringVec,
) -> impl IntoView {
    let location = use_location();
    let (breadcrumbs, set_breadcrumbs) = signal(vec![] as Vec<ViewFn>);

    Effect::new(move |_| {
        let path_segments = location
            .pathname
            .get()
            .split('/')
            .map(|val| val.to_owned())
            .collect::<Vec<_>>();

        let mut cumulative_path = String::new();
        // let mut new_crumbs = vec![] as Vec<ViewFn>;

        let mut new_crumbs: Vec<ViewFn> = path_segments
            .into_iter()
            .filter(|segment| !segment.is_empty())
            .enumerate()
            .map(|(i, segment)| {
                cumulative_path.push('/');
                cumulative_path.push_str(segment.as_str());
                let route_name_index = i + 1;
                let segment_text = if custom_route_names.0.get(route_name_index).is_some() {
                    custom_route_names
                        .0
                        .get(route_name_index)
                        .unwrap_or(&String::new())
                        .to_owned()
                } else {
                    segment.clone()
                };
                let link_path = cumulative_path.clone();

                let link = ViewFn::from(move || {
                    let segment_text = segment_text.clone();
                    let link_path = link_path.clone();
                    view! { <A href=link_path>{segment_text}</A> }
                });

                link
            })
            .collect();

        // Append the home link
        let home_route_name = custom_route_names.0[0].clone();
        let home_link = ViewFn::from(move || {
            let home_route_name = home_route_name.clone();
            view! { <A href="/">{home_route_name}</A> }
        });
        new_crumbs.insert(0, home_link);

        set_breadcrumbs.set(new_crumbs);
    });

    view! {
        <nav class="rounded">
            <ul class="flex items-center space-x-2">
                {move || {
                    breadcrumbs
                        .get()
                        .into_iter()
                        .enumerate()
                        .map(|(i, item)| {
                            view! {
                                <li class="flex flex-row gap-2 items-center">
                                    {
                                        item.run()
                                    }
                                    {if i < breadcrumbs.get().len() - 1 {
                                        Some(view! {
                                            <span class="text-xs mx-2">
                                                <Icon icon=BiChevronRightRegular />
                                            </span>
                                        })
                                    } else {
                                        None
                                    }}
                                </li>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </ul>
        </nav>
    }
}
