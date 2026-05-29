use icondata::Icon as IconId;
use leptos::prelude::*;
use leptos_icons::Icon;

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub enum TimelineStatus {
    Warning,
    Info,
    Danger,
    #[default]
    Success,
    Neutral,
}

#[derive(Clone)]
pub struct TimelineItem {
    pub time_info: String,
    pub title: String,
    pub display_ping: bool,
    pub more_info: Option<String>,
    pub icon_head: Option<IconId>,
    pub image_head: Option<String>,
    pub content: ViewFn,
    pub status: TimelineStatus,
}

impl Default for TimelineItem {
    fn default() -> Self {
        Self {
            time_info: String::new(),
            title: String::new(),
            display_ping: false,
            more_info: None,
            icon_head: None,
            image_head: None,
            content: ViewFn::from(|| view! {}), // or however you default it
            status: TimelineStatus::default(),
        }
    }
}

#[allow(dead_code)]
impl TimelineItem {
    pub fn builder(
        time_info: impl Into<String>,
        title: impl Into<String>,
        display_ping: bool,
        content: ViewFn,
    ) -> TimelineItem {
        TimelineItem {
            time_info: time_info.into(),
            title: title.into(),
            display_ping,
            content,
            ..Default::default()
        }
    }

    pub fn more_info(mut self, s: impl Into<String>) -> Self {
        self.more_info = Some(s.into());
        self
    }

    pub fn icon_head(mut self, icon: IconId) -> Self {
        self.icon_head = Some(icon);
        self
    }

    pub fn image_head(mut self, url: impl Into<String>) -> Self {
        self.image_head = Some(url.into());
        self
    }

    pub fn status(mut self, status: TimelineStatus) -> Self {
        self.status = status;
        self
    }

    // Optional: shortcuts if some combinations are very common
    pub fn pending(mut self) -> Self {
        self.status = TimelineStatus::Info;
        self
    }
    pub fn completed(mut self) -> Self {
        self.status = TimelineStatus::Success;
        self
    }
    pub fn failed(mut self) -> Self {
        self.status = TimelineStatus::Danger;
        self
    }

    pub fn build(self) -> TimelineItem {
        TimelineItem {
            time_info: self.time_info,
            title: self.title,
            display_ping: self.display_ping,
            more_info: self.more_info,
            icon_head: self.icon_head,
            image_head: self.image_head,
            content: self.content,
            status: self.status,
        }
    }
}

/// This is a component that displays a timeline of steps.
/// There are 3 ways to display the header of each step.
/// One way is to use icons, another way is to use images, and a third way is to use the default circle.
/// Example usage:
/// ```
/// let steps = RwSignal::new(vec![
/// TimelineItem {
///     time_info: "2 mins ago".into(),
///     title: "Step 1".into(),
///     more_info: Some("Initialize project".into()),
///     status: TimelineStatus::Info,
///     image_head: Some("https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTiZ17_7VUm-SR8l9Z7ksl7n7SfjUTTNK5NWA&s".into()),
///     content: ViewFn::from(|| view! { <p>"Project created with Leptos!"</p> }),
///     ..Default::default()
/// },
/// TimelineItem {
///     time_info: "3 mins ago".into(),
///     title: "Step 2".into(),
///     more_info: Some("Build UI components".into()),
///     status: TimelineStatus::Info,
///     image_head: Some("https://img.icons8.com/?size=100&id=4PiNHtUJVbLs&format=png&color=000000".into()),
///     content: ViewFn::from(|| view! { <p>"Using Tailwind and component slots."</p> }),
///     ..Default::default()
/// },
/// TimelineItem {
///     time_info: "5 mins ago".into(),
///     title: "Step 3".into(),
///     more_info: Some("Deploy to production".into()),
///     status: TimelineStatus::Info,
///     image_head: Some("https://img.icons8.com/color/512/amazon-web-services.png".into()),
///     content: ViewFn::from(|| view! { <p>"Vercel, Deno or your own server."</p> }),
///     ..Default::default()
/// }
/// ]);
///
/// // The view is as shown below
/// <Timeline steps=steps />
///```

#[component]
pub fn Timeline(#[prop(into)] steps: RwSignal<Vec<TimelineItem>>) -> impl IntoView {
    view! {
        <div class="relative">
            <For
                each=move || steps.get().into_iter().enumerate()
                key=|(i, _)| *i
                let:((_i, item))
            >
                {
                    let bg_status_classes = match item.status {
                        TimelineStatus::Warning => "bg-warning/20 text-warning",
                        TimelineStatus::Info => "bg-info/20 text-info",
                        TimelineStatus::Success => "bg-success/20 text-success",
                        TimelineStatus::Danger => "bg-danger/20 text-danger",
                        TimelineStatus::Neutral => "bg-primary/20 text-primary",
                    };

                    view! {
                        <div class="relative flex">
                            <div class="flex flex-col">
                                {
                                    if let Some(icon_head) = &item.icon_head {
                                        Some(view!{
                                            <span class="relative flex size-6 cursor-pointer">
                                                <span class=format!("absolute inline-flex h-full w-full rounded-full {} {}", if item.display_ping { "animate-ping" } else { "" }, bg_status_classes)></span>
                                                <span class=format!("relative inline-flex items-center justify-center size-6 rounded-full {}", bg_status_classes)>
                                                    <Icon width="50%" height="50%" icon=icon_head.to_owned() />
                                                </span>
                                            </span>
                                        })
                                    } else {
                                        None
                                    }
                                }
                                {
                                    if let Some(image_head) = &item.image_head {
                                        Some(view!{
                                            <span class="relative flex size-6 cursor-pointer">
                                                <span class=format!("absolute inline-flex h-full w-full rounded-full {} {}", if item.display_ping { "animate-ping" } else { "" }, bg_status_classes)></span>
                                                <span class=format!("relative inline-flex items-center justify-center size-6 rounded-full {}", bg_status_classes)>
                                                    <img alt="timeline-head" src=image_head.to_owned() class="w-full h-full rounded-full object-contain saturate-200" />
                                                </span>
                                            </span>
                                        })
                                    } else {
                                        None
                                    }
                                }
                                {
                                    if item.image_head.is_none() && item.icon_head.is_none() {
                                        Some(view!{
                                            <span class="relative flex size-6 cursor-pointer">
                                                <span class=format!("absolute inline-flex h-full w-full rounded-full {} {}", if item.display_ping { "animate-ping" } else { "" }, bg_status_classes)></span>
                                                <span class=format!("relative inline-flex size-6 rounded-full {}", bg_status_classes)></span>
                                            </span>
                                        })
                                    } else {
                                        None
                                    }
                                }
                                <div class="flex justify-center flex-1">
                                    <div class="border-[1px] border-primary"></div>
                                </div>
                            </div>
                            <div class="ml-4 mb-4">
                                <p class="text-sm">{item.time_info}</p>
                                <div class="text-wrap">
                                    <h4 class="text-primary">{item.title}<span class="text-sm text-secondary">{
                                        item.more_info.as_ref().map(|info| format!(" - {}", info))
                                    }</span></h4>
                                </div>

                                <div class="mt-2">
                                    {item.content.run()}
                                </div>
                            </div>
                        </div>
                    }
                }
            </For>
        </div>
    }
}
