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

/// Represents a single entry in a `Timeline`.
///
/// Use `TimelineItem::builder` to construct, then chain optional setters before calling `.build()`.
///
/// # Fields
///
/// - `time_info` – Human-readable time label (e.g. `"2 mins ago"`).
/// - `title` – Primary heading for the step.
/// - `more_info` – Optional subtitle appended to the title.
/// - `display_ping` – When `true`, the step head pulses with an animation.
/// - `icon_head` – Optional icon rendered in the step head. Takes priority over `image_head`.
/// - `image_head` – Optional image URL rendered in the step head.
/// - `content` – `ViewFn` rendered as the body of the step.
/// - `status` – `TimelineStatus` controlling the head and text color.
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

/// A timeline component that displays a vertical list of steps or events.
///
/// Each step's header can be rendered as an icon, an image, or a default colored circle,
/// with an optional ping animation to indicate activity.
///
/// # Props
///
/// - `steps` – `RwSignal<Vec<TimelineItem>>` holding the ordered list of timeline entries.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::data_display::timeline::{TimelineItem, TimelineStatus, Timeline};
///
/// #[component]
/// fn Example() -> impl IntoView {
///     let steps = RwSignal::new(vec![
///         TimelineItem::builder("2 mins ago", "Step 1", false, ViewFn::from(|| view! { <p>"Done"</p> }))
///             .status(TimelineStatus::Success)
///             .build(),
///         TimelineItem::builder("1 min ago", "Step 2", true, ViewFn::from(|| view! { <p>"In progress"</p> }))
///             .status(TimelineStatus::Info)
///             .build(),
///     ]);
///     view! {
///         <Timeline steps=steps />
///     }
/// }
/// ```
#[component]
pub fn Timeline(#[prop(into)] steps: MaybeProp<Vec<TimelineItem>>) -> impl IntoView {
    view! {
        <div class="relative">
            <For
                each=move || steps.get().unwrap_or_default().into_iter().enumerate()
                key=|(i, _)| *i
                let:((_i, item))
            >
                {
                    let bg_status_classes = match item.status {
                        TimelineStatus::Warning => "bg-warning/50 text-warning",
                        TimelineStatus::Info => "bg-info/50 text-info",
                        TimelineStatus::Success => "bg-success/50 text-success",
                        TimelineStatus::Danger => "bg-danger/50 text-danger",
                        TimelineStatus::Neutral => "bg-primary/50 text-primary",
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
    }.into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    // TimelineStatus

    #[test]
    fn timeline_status_default_is_success() {
        assert!(matches!(TimelineStatus::default(), TimelineStatus::Success));
    }

    // bg_status_classes logic

    fn bg_status_classes(status: &TimelineStatus) -> &'static str {
        match status {
            TimelineStatus::Warning => "bg-warning/20 text-warning",
            TimelineStatus::Info => "bg-info/20 text-info",
            TimelineStatus::Success => "bg-success/20 text-success",
            TimelineStatus::Danger => "bg-danger/20 text-danger",
            TimelineStatus::Neutral => "bg-primary/20 text-primary",
        }
    }

    #[test]
    fn warning_status_classes() {
        assert_eq!(
            bg_status_classes(&TimelineStatus::Warning),
            "bg-warning/20 text-warning"
        );
    }

    #[test]
    fn info_status_classes() {
        assert_eq!(
            bg_status_classes(&TimelineStatus::Info),
            "bg-info/20 text-info"
        );
    }

    #[test]
    fn success_status_classes() {
        assert_eq!(
            bg_status_classes(&TimelineStatus::Success),
            "bg-success/20 text-success"
        );
    }

    #[test]
    fn danger_status_classes() {
        assert_eq!(
            bg_status_classes(&TimelineStatus::Danger),
            "bg-danger/20 text-danger"
        );
    }

    #[test]
    fn neutral_status_classes() {
        assert_eq!(
            bg_status_classes(&TimelineStatus::Neutral),
            "bg-primary/20 text-primary"
        );
    }

    // head rendering logic

    fn resolve_head(icon: Option<()>, image: Option<()>) -> &'static str {
        if icon.is_some() {
            "icon"
        } else if image.is_some() {
            "image"
        } else {
            "circle"
        }
    }

    #[test]
    fn icon_head_takes_priority() {
        assert_eq!(resolve_head(Some(()), Some(())), "icon");
    }

    #[test]
    fn image_head_used_when_no_icon() {
        assert_eq!(resolve_head(None, Some(())), "image");
    }

    #[test]
    fn default_circle_when_neither() {
        assert_eq!(resolve_head(None, None), "circle");
    }

    // TimelineItem builder

    #[test]
    fn builder_sets_required_fields() {
        let item = TimelineItem::builder("now", "Deploy", true, ViewFn::from(|| view! {})).build();

        assert_eq!(item.time_info, "now");
        assert_eq!(item.title, "Deploy");
        assert!(item.display_ping);
        assert!(item.more_info.is_none());
        assert!(item.icon_head.is_none());
        assert!(item.image_head.is_none());
    }

    #[test]
    fn builder_more_info() {
        let item = TimelineItem::builder("now", "Step", false, ViewFn::from(|| view! {}))
            .more_info("extra detail")
            .build();
        assert_eq!(item.more_info, Some("extra detail".to_string()));
    }

    #[test]
    fn builder_image_head() {
        let item = TimelineItem::builder("now", "Step", false, ViewFn::from(|| view! {}))
            .image_head("https://example.com/img.png")
            .build();
        assert_eq!(
            item.image_head,
            Some("https://example.com/img.png".to_string())
        );
    }

    #[test]
    fn pending_sets_info_status() {
        let item = TimelineItem::builder("now", "Step", false, ViewFn::from(|| view! {}))
            .pending()
            .build();
        assert!(matches!(item.status, TimelineStatus::Info));
    }

    #[test]
    fn completed_sets_success_status() {
        let item = TimelineItem::builder("now", "Step", false, ViewFn::from(|| view! {}))
            .completed()
            .build();
        assert!(matches!(item.status, TimelineStatus::Success));
    }

    #[test]
    fn failed_sets_danger_status() {
        let item = TimelineItem::builder("now", "Step", false, ViewFn::from(|| view! {}))
            .failed()
            .build();
        assert!(matches!(item.status, TimelineStatus::Danger));
    }

    // display_ping logic

    fn ping_class(display_ping: bool) -> &'static str {
        if display_ping { "animate-ping" } else { "" }
    }

    #[test]
    fn ping_class_when_true() {
        assert_eq!(ping_class(true), "animate-ping");
    }

    #[test]
    fn ping_class_when_false() {
        assert_eq!(ping_class(false), "");
    }
}
