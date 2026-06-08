use crate::components::schemas::props::StringVec;
use icondata::BiChevronRightRegular;

use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::{components::A, hooks::use_location};

/// A breadcrumb navigation component that derives crumbs from the current route path.
///
/// The first crumb is always a home link (`/`). Each subsequent path segment becomes
/// a crumb linking to its cumulative path. Custom names can be provided for all crumbs
/// including the home crumb; if fewer names than segments are provided, the raw segment
/// string is used as the label.
///
/// # Props
///
/// - `custom_route_names` – `StringVec` of display names in order of appearance,
///   starting with the home crumb. Defaults to `["Home"]`.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::navigation::breadcrumbs::Breadcrumbs;
///
/// #[component]
/// fn Example() -> impl IntoView {
///     view! {
///         <Breadcrumbs custom_route_names=["Home", "Dashboard", "Settings"] />
///     }
/// }
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

#[cfg(test)]
mod tests {
    // path segment parsing

    fn parse_segments(path: &str) -> Vec<String> {
        path.split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn root_path_produces_no_segments() {
        assert_eq!(parse_segments("/"), Vec::<String>::new());
    }

    #[test]
    fn single_segment_path() {
        assert_eq!(parse_segments("/dashboard"), vec!["dashboard"]);
    }

    #[test]
    fn nested_path_produces_multiple_segments() {
        assert_eq!(
            parse_segments("/dashboard/settings/profile"),
            vec!["dashboard", "settings", "profile"]
        );
    }

    #[test]
    fn trailing_slash_ignored() {
        assert_eq!(parse_segments("/dashboard/"), vec!["dashboard"]);
    }

    // cumulative path building

    fn cumulative_paths(segments: &[&str]) -> Vec<String> {
        let mut cumulative = String::new();
        segments
            .iter()
            .map(|s| {
                cumulative.push('/');
                cumulative.push_str(s);
                cumulative.clone()
            })
            .collect()
    }

    #[test]
    fn cumulative_paths_build_correctly() {
        let paths = cumulative_paths(&["dashboard", "settings", "profile"]);
        assert_eq!(paths[0], "/dashboard");
        assert_eq!(paths[1], "/dashboard/settings");
        assert_eq!(paths[2], "/dashboard/settings/profile");
    }

    #[test]
    fn single_segment_cumulative_path() {
        assert_eq!(cumulative_paths(&["about"]), vec!["/about"]);
    }

    // segment name resolution

    fn resolve_name<'a>(
        custom_names: &'a [String],
        route_name_index: usize,
        segment: &'a str,
    ) -> &'a str {
        custom_names
            .get(route_name_index)
            .map(|s| s.as_str())
            .unwrap_or(segment)
    }

    #[test]
    fn custom_name_used_when_available() {
        let names = vec!["Home".to_string(), "Dashboard".to_string()];
        assert_eq!(resolve_name(&names, 1, "dashboard"), "Dashboard");
    }

    #[test]
    fn raw_segment_used_when_no_custom_name() {
        let names = vec!["Home".to_string()];
        assert_eq!(resolve_name(&names, 1, "settings"), "settings");
    }

    #[test]
    fn home_name_comes_from_index_zero() {
        let names = vec!["Start".to_string()];
        assert_eq!(resolve_name(&names, 0, ""), "Start");
    }

    // home crumb always prepended

    #[test]
    fn home_crumb_is_first() {
        // After building crumbs, home is inserted at index 0.
        // We verify the count: 1 home + n segments.
        let segments = parse_segments("/dashboard/settings");
        let total_crumbs = 1 + segments.len();
        assert_eq!(total_crumbs, 3);
    }

    // separator visibility

    fn show_separator(index: usize, total: usize) -> bool {
        index < total - 1
    }

    #[test]
    fn separator_shown_between_crumbs() {
        assert!(show_separator(0, 3));
        assert!(show_separator(1, 3));
    }

    #[test]
    fn separator_not_shown_after_last_crumb() {
        assert!(!show_separator(2, 3));
    }

    #[test]
    fn single_crumb_has_no_separator() {
        assert!(!show_separator(0, 1));
    }
}
