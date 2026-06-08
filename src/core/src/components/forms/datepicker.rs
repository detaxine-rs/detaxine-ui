use crate::components::actions::button::BasicButton;
use crate::components::forms::input::{InputField, InputFieldType};
use crate::utils::forms::fire_bubbled_and_cancelable_event;
use chrono::Local;
use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Weekday};
use icondata::BsCalendar2Date;
use icondata::{BiChevronLeftRegular, BiChevronRightRegular};
use leptos::ev;
use leptos::html::*;
use leptos::prelude::*;
use web_sys::HtmlInputElement;

/// A date picker with a calendar popup, supporting min/max constraints and explicitly disabled dates.
///
/// Renders a read-only display input that opens a calendar on click. The selected date is
/// written as an RFC3339 string to a hidden input for form submission.
///
/// # Props
///
/// - `label` – Label displayed above the visible input.
/// - `name` – `name` attribute on the hidden form input.
/// - `id_attr` – `id` attribute base; the display input receives `"{id_attr}-display"`.
/// - `required` – Marks the field as required. Defaults to `false`.
/// - `initial_value` – `RwSignal<Option<DateTime<Local>>>` pre-selecting a date. Defaults to `None`.
/// - `input_node_ref` – `NodeRef<Input>` for the hidden input, useful for programmatic access.
/// - `min` – Earliest selectable date (inclusive). Dates before this are disabled.
/// - `max` – Latest selectable date (inclusive). Dates after this are disabled.
/// - `disabled_dates` – Explicit list of dates to disable regardless of `min`/`max`.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use chrono::{Local, Duration};
/// use detaxine_ui::components::forms::datepicker::DatePicker;
///
/// #[component]
/// fn Example() -> impl IntoView {
///     view! {
///         <DatePicker
///             label="Appointment"
///             name="appointment"
///             min=Local::now()
///             max=Local::now() + Duration::days(30)
///         />
///     }
/// }
/// ```
#[component]
pub fn DatePicker(
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] name: String,
    #[prop(default = false, optional)] required: bool,
    #[prop(into, default = MaybeProp::derive(move || None), optional)] initial_value: MaybeProp<
        Option<DateTime<Local>>,
    >,
    #[prop(into, optional)] id_attr: String,
    #[prop(optional)] input_node_ref: NodeRef<Input>,
    /// Earliest selectable date (inclusive). Dates before this are disabled.
    #[prop(optional)]
    min: Option<DateTime<Local>>,
    /// Latest selectable date (inclusive). Dates after this are disabled.
    #[prop(optional)]
    max: Option<DateTime<Local>>,
    /// Explicit list of dates to disable regardless of min/max.
    #[prop(optional)]
    disabled_dates: Vec<DateTime<Local>>,
) -> impl IntoView {
    let (show_calendar, set_show_calendar) = signal(false);
    let (selected_date, set_selected_date) = signal(None);

    let selected_date_value = Memo::new(move |_| {
        selected_date
            .get()
            .map(|dt: DateTime<Local>| dt.to_rfc3339())
            .unwrap_or_default()
    });

    let selected_date_display_value = Memo::new(move |_| {
        selected_date
            .get()
            .map(|dt| dt.format("%b %0e %Y").to_string())
            .unwrap_or(String::from("Select Date"))
    });

    Effect::new(move |_| {
        let Some(provided_initial_date) = initial_value.get() else {
            return;
        };
        set_selected_date.set(provided_initial_date);
    });

    let toggle_calendar = Callback::new(move |_| {
        set_show_calendar.update(|val| *val = !*val);
    });

    let select_date = Callback::new(move |date: DateTime<Local>| {
        set_selected_date.set(Some(date));
        set_show_calendar.set(false);

        let date_str = date.to_rfc3339();

        if let Some(el) = input_node_ref.get() as Option<HtmlInputElement> {
            el.set_value(&date_str);
            fire_bubbled_and_cancelable_event("input", true, true, &el);
            fire_bubbled_and_cancelable_event("change", true, true, &el);
        }
    });

    view! {
        <div class="relative">
            <InputField
                initial_value=selected_date_value
                name=name
                field_type=InputFieldType::Text
                required=required
                ext_wrapper_styles="sr-only"
                id_attr=id_attr.clone()
                input_node_ref=input_node_ref
            />
            <InputField
                readonly=true
                required=required
                label=label
                on:click=move |ev: ev::MouseEvent| toggle_calendar.run(ev)
                initial_value=selected_date_display_value
                field_type=InputFieldType::Text
                id_attr=format!("{id_attr}-display")
                onblur=Callback::new(move |_| set_show_calendar.set(false))
                icon=BsCalendar2Date
                icon_is_leading=false
            />
            {move || show_calendar.get().then(|| view! {
                <div
                    on:mousedown=|e: ev::MouseEvent| e.prevent_default()
                    class="absolute bg-slate-50 rounded shadow-lg z-10 w-[300px] max-h-[400px] overflow-auto"
                >
                    <Calendar
                        select_date=select_date
                        initial_selected=selected_date.get()
                        min=min
                        max=max
                        disabled_dates=disabled_dates.clone()
                    />
                </div>
            })}
        </div>
    }
}

#[component]
fn Calendar(
    #[prop(into)] select_date: Callback<DateTime<Local>>,
    #[prop(into)] initial_selected: Option<DateTime<Local>>,
    min: Option<DateTime<Local>>,
    max: Option<DateTime<Local>>,
    #[prop(optional)] disabled_dates: Vec<DateTime<Local>>,
) -> impl IntoView {
    let today: DateTime<Local> = Local::now();
    let default_year = today.year();

    let start_month = initial_selected.map(|d| d.month()).unwrap_or(today.month());
    let start_year = initial_selected.map(|d| d.year()).unwrap_or(today.year());

    let (current_month, set_current_month) = signal(start_month);
    let (current_year, set_current_year) = signal(start_year);
    let (viewing_years, set_viewing_years) = signal(false);
    let (year_page, set_year_page) = signal(0usize);
    let (highlighted, set_highlighted) = signal(initial_selected);

    Effect::new(move |_| {
        if let Some(date) = initial_selected {
            set_current_month.set(date.month());
            set_current_year.set(date.year());
            set_highlighted.set(Some(date));
        }
    });

    // Returns true if the given date should not be selectable.
    let is_disabled = StoredValue::new(move |date: DateTime<Local>| -> bool {
        let date_naive = date.date_naive();
        if min.is_some_and(|m| date_naive < m.date_naive()) {
            return true;
        }
        if max.is_some_and(|m| date_naive > m.date_naive()) {
            return true;
        }
        disabled_dates.iter().any(|d| d.date_naive() == date_naive)
    });

    // Whether the user can navigate to the previous month (blocked if it would
    // go entirely before `min`).
    let can_go_prev = move || {
        min.map(|m| {
            let (prev_year, prev_month) = if current_month.get() == 1 {
                (current_year.get() - 1, 12u32)
            } else {
                (current_year.get(), current_month.get() - 1)
            };
            // The last day of the candidate prev-month must be >= min date.
            NaiveDate::from_ymd_opt(prev_year, prev_month, 1)
                .and_then(|_first| {
                    // last day of that month
                    let (ny, nm) = if prev_month == 12 {
                        (prev_year + 1, 1u32)
                    } else {
                        (prev_year, prev_month + 1)
                    };
                    NaiveDate::from_ymd_opt(ny, nm, 1)
                        .map(|next_first| next_first - Duration::days(1))
                })
                .is_some_and(|last_day| last_day >= m.date_naive())
        })
        .unwrap_or(true)
    };

    // Whether the user can navigate to the next month (blocked if it would
    // go entirely after `max`).
    let can_go_next = move || {
        max.map(|m| {
            let (next_year, next_month) = if current_month.get() == 12 {
                (current_year.get() + 1, 1u32)
            } else {
                (current_year.get(), current_month.get() + 1)
            };
            // The first day of the candidate next-month must be <= max date.
            NaiveDate::from_ymd_opt(next_year, next_month, 1)
                .is_some_and(|first_day| first_day <= m.date_naive())
        })
        .unwrap_or(true)
    };

    let years_per_page = 16usize;

    let toggle_viewing_years = Callback::new(move |_| {
        set_viewing_years.update(|val| *val = !*val);
    });

    let change_year = Callback::new(move |year: i32| {
        set_current_year.set(year);
        set_viewing_years.set(false);
    });

    let render_years = move || {
        let start_year = (default_year - 60).max(1);
        let end_year = default_year + 12;
        let total_years: Vec<i32> = (start_year..end_year).collect();
        let total_pages = (total_years.len() + years_per_page - 1) / years_per_page;

        if total_pages == 0 {
            return vec![];
        }

        let current_page = year_page.get() % total_pages;
        let start = current_page * years_per_page;
        let end = (start + years_per_page).min(total_years.len());

        // Optionally grey out years that are entirely out of range.
        let min_year = min.map(|m| m.year());
        let max_year = max.map(|m| m.year());

        total_years[start..end]
            .iter()
            .map(|&year| {
                let out_of_range =
                    min_year.is_some_and(|my| year < my) || max_year.is_some_and(|my| year > my);

                view! {
                    <BasicButton
                        onclick=Callback::new(move |_| {
                            if !out_of_range {
                                change_year.run(year);
                            }
                        })
                        style_ext=if out_of_range {
                            "flex text-xs border-none rounded m-1 text-gray-300 cursor-not-allowed"
                        } else {
                            "flex text-xs border-none rounded m-1 hover:bg-blue-200 cursor-pointer"
                        }
                        button_text=year.to_string()
                    />
                }
            })
            .collect::<Vec<_>>()
    };

    let next_year_page = Callback::new(move |_| set_year_page.update(|val| *val += 1));
    let prev_year_page = Callback::new(move |_| {
        set_year_page.update(|val| {
            if *val > 0 {
                *val -= 1
            }
        })
    });

    fn last_day_of_month(date: NaiveDate) -> Option<NaiveDate> {
        let (year, month) = if date.month() == 12 {
            (date.year() + 1, 1)
        } else {
            (date.year(), date.month() + 1)
        };
        NaiveDate::from_ymd_opt(year, month, 1)
            .map(|first_of_next_month| first_of_next_month - Duration::days(1))
    }

    let days_in_month = move || {
        let first_date = NaiveDate::from_ymd_opt(current_year.get(), current_month.get(), 1)
            .unwrap_or_else(|| today.date_naive());
        last_day_of_month(first_date).map(|last_day| last_day.day())
    };

    let render_days = StoredValue::new(move || {
        let days_in_month = days_in_month();
        let first_date = NaiveDate::from_ymd_opt(current_year.get(), current_month.get(), 1)
            .unwrap_or_else(|| today.date_naive());

        let calendar_adjustment = match first_date.weekday() {
            Weekday::Sun => 0u32,
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
        };

        if let Some(days_in_month) = days_in_month {
            Some(view! {
                <For
                    each=move || (0..(calendar_adjustment + days_in_month)).enumerate()
                    key=|&(i, _)| i
                    children=move |(i, _)| {
                        let is_blank = (i as u32) < calendar_adjustment;
                        let day = if is_blank { 0 } else { (i as u32) - calendar_adjustment + 1 };

                        let date = if is_blank {
                            None
                        } else {
                            NaiveDate::from_ymd_opt(current_year.get(), current_month.get(), day)
                                .and_then(|naive| {
                                    naive.and_hms_opt(0, 0, 0)
                                        .and_then(|dt| Local.from_local_datetime(&dt).single())
                                })
                        };

                        // Pre-compute disabled so it's available in both the
                        // click handler and the reactive style memo.
                        let disabled = date.map(is_disabled.get_value()).unwrap_or(false);

                        view! {
                            <BasicButton
                                onclick=Callback::new(move |_| {
                                    if let Some(d) = date {
                                        if !disabled {
                                            set_highlighted.set(Some(d));
                                            select_date.run(d);
                                        }
                                    }
                                })
                                style_ext=MaybeProp::derive(move || {
                                    let base = "flex text-xs items-center justify-center border-none rounded m-1";

                                    let variant = if is_blank {
                                        ""
                                    } else if disabled {
                                        "text-gray-300 cursor-not-allowed"
                                    } else if highlighted.get().map(|h| {
                                        h.day() == day
                                            && h.month() == current_month.get()
                                            && h.year() == current_year.get()
                                    }).unwrap_or(false) {
                                        "bg-primary text-contrast-white cursor-pointer"
                                    } else {
                                        "hover:bg-blue-200 cursor-pointer"
                                    };

                                    Some(format!("{base} {variant}"))
                                })
                                button_text={if is_blank { "".to_string() } else { day.to_string() }}
                            />
                        }
                    }
                />
            })
        } else {
            None
        }
    });

    view! {
        <div class="w-full max-w-md bg-contrast-white border-none rounded">
            {move || viewing_years.get().then(|| view! {
                <div>
                    <div class="flex justify-between items-center mb-2">
                        <BasicButton onclick=prev_year_page icon=Some(BiChevronLeftRegular) />
                        <span class="cursor-pointer">"Years"</span>
                        <BasicButton onclick=next_year_page icon=Some(BiChevronRightRegular) />
                    </div>
                    <div class="grid grid-cols-4 gap-1 bg-contrast-white rounded p-2">
                        {move || render_years()}
                    </div>
                </div>
            })}
            {move || (!viewing_years.get()).then(|| {
                let days_of_week = ["S", "M", "T", "W", "T", "F", "S"];
                view! {
                    <div>
                        <div class="flex justify-between items-center mb-2">
                            <BasicButton
                                onclick=Callback::new(move |_| {
                                    if can_go_prev() {
                                        set_current_month.update(|m| {
                                            if *m == 1 {
                                                set_current_year.update(|y| *y -= 1);
                                                *m = 12;
                                            } else {
                                                *m -= 1
                                            }
                                        });
                                    }
                                })
                                style_ext=if can_go_prev() { "cursor-pointer" } else { "opacity-30 cursor-not-allowed" }
                                icon=Some(BiChevronLeftRegular)
                            />
                            <span
                                on:click=move |_| toggle_viewing_years.run(())
                                class="cursor-pointer"
                            >
                                {move || {
                                    u8::try_from(current_month.get())
                                        .ok()
                                        .and_then(|m| chrono::Month::try_from(m).ok())
                                        .map(|month| format!("{:?} {:?}", current_year.get(), month))
                                }}
                            </span>
                            <BasicButton
                                onclick=Callback::new(move |_| {
                                    if can_go_next() {
                                        set_current_month.update(|m| {
                                            if *m == 12 {
                                                set_current_year.update(|y| *y += 1);
                                                *m = 1;
                                            } else {
                                                *m += 1
                                            }
                                        });
                                    }
                                })
                                style_ext=if can_go_next() { "cursor-pointer" } else { "opacity-30 cursor-not-allowed" }
                                icon=Some(BiChevronRightRegular)
                            />
                        </div>
                        <div class="grid grid-cols-7 gap-1 bg-contrast-white border-none rounded p-2">
                            {days_of_week.iter().map(|&day| view! {
                                <div class="font-bold text-center text-sm">{day}</div>
                            }).collect::<Vec<_>>()}
                            {render_days.get_value()()}
                        </div>
                    </div>
                }
            })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Local, NaiveDate, TimeZone};

    // selected_date_display_value

    fn display_value(date: Option<chrono::DateTime<Local>>) -> String {
        date.map(|dt| dt.format("%b %0e %Y").to_string())
            .unwrap_or("Select Date".to_string())
    }

    #[test]
    fn display_value_none_shows_placeholder() {
        assert_eq!(display_value(None), "Select Date");
    }

    #[test]
    fn display_value_some_formats_date() {
        let date = Local.with_ymd_and_hms(2024, 6, 15, 0, 0, 0).unwrap();
        let result = display_value(Some(date));
        assert!(result.contains("2024"));
        assert!(result.contains("Jun"));
    }

    // selected_date_value (RFC3339)

    fn rfc3339_value(date: Option<chrono::DateTime<Local>>) -> String {
        date.map(|dt| dt.to_rfc3339()).unwrap_or_default()
    }

    #[test]
    fn rfc3339_value_none_is_empty() {
        assert_eq!(rfc3339_value(None), "");
    }

    #[test]
    fn rfc3339_value_some_is_parseable() {
        let date = Local.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let result = rfc3339_value(Some(date));
        assert!(chrono::DateTime::parse_from_rfc3339(&result).is_ok());
    }

    // is_disabled logic

    fn is_disabled(
        date: NaiveDate,
        min: Option<NaiveDate>,
        max: Option<NaiveDate>,
        disabled: &[NaiveDate],
    ) -> bool {
        if min.is_some_and(|m| date < m) {
            return true;
        }
        if max.is_some_and(|m| date > m) {
            return true;
        }
        disabled.iter().any(|d| *d == date)
    }

    #[test]
    fn date_before_min_is_disabled() {
        let min = NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
        let date = NaiveDate::from_ymd_opt(2024, 5, 31).unwrap();
        assert!(is_disabled(date, Some(min), None, &[]));
    }

    #[test]
    fn date_on_min_is_not_disabled() {
        let min = NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
        assert!(!is_disabled(min, Some(min), None, &[]));
    }

    #[test]
    fn date_after_max_is_disabled() {
        let max = NaiveDate::from_ymd_opt(2024, 6, 30).unwrap();
        let date = NaiveDate::from_ymd_opt(2024, 7, 1).unwrap();
        assert!(is_disabled(date, None, Some(max), &[]));
    }

    #[test]
    fn date_on_max_is_not_disabled() {
        let max = NaiveDate::from_ymd_opt(2024, 6, 30).unwrap();
        assert!(!is_disabled(max, None, Some(max), &[]));
    }

    #[test]
    fn explicitly_disabled_date_is_disabled() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        assert!(is_disabled(date, None, None, &[date]));
    }

    #[test]
    fn date_within_range_and_not_listed_is_enabled() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let min = NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
        let max = NaiveDate::from_ymd_opt(2024, 6, 30).unwrap();
        assert!(!is_disabled(date, Some(min), Some(max), &[]));
    }

    // last_day_of_month

    fn last_day_of_month(date: NaiveDate) -> Option<NaiveDate> {
        let (year, month) = if date.month() == 12 {
            (date.year() + 1, 1)
        } else {
            (date.year(), date.month() + 1)
        };
        NaiveDate::from_ymd_opt(year, month, 1).map(|first| first - Duration::days(1))
    }

    #[test]
    fn last_day_january() {
        let d = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        assert_eq!(last_day_of_month(d).unwrap().day(), 31);
    }

    #[test]
    fn last_day_february_leap_year() {
        let d = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        assert_eq!(last_day_of_month(d).unwrap().day(), 29);
    }

    #[test]
    fn last_day_february_non_leap_year() {
        let d = NaiveDate::from_ymd_opt(2023, 2, 1).unwrap();
        assert_eq!(last_day_of_month(d).unwrap().day(), 28);
    }

    #[test]
    fn last_day_december_wraps_to_next_year() {
        let d = NaiveDate::from_ymd_opt(2024, 12, 1).unwrap();
        assert_eq!(last_day_of_month(d).unwrap().day(), 31);
    }

    // calendar_adjustment (weekday offset)

    fn calendar_adjustment(weekday: chrono::Weekday) -> u32 {
        match weekday {
            chrono::Weekday::Sun => 0,
            chrono::Weekday::Mon => 1,
            chrono::Weekday::Tue => 2,
            chrono::Weekday::Wed => 3,
            chrono::Weekday::Thu => 4,
            chrono::Weekday::Fri => 5,
            chrono::Weekday::Sat => 6,
        }
    }

    #[test]
    fn sunday_needs_no_offset() {
        assert_eq!(calendar_adjustment(chrono::Weekday::Sun), 0);
    }

    #[test]
    fn saturday_needs_six_blank_cells() {
        assert_eq!(calendar_adjustment(chrono::Weekday::Sat), 6);
    }

    #[test]
    fn wednesday_needs_three_blank_cells() {
        assert_eq!(calendar_adjustment(chrono::Weekday::Wed), 3);
    }

    // can_go_prev / can_go_next logic

    fn prev_month(month: u32, year: i32) -> (u32, i32) {
        if month == 1 {
            (12, year - 1)
        } else {
            (month - 1, year)
        }
    }

    fn next_month(month: u32, year: i32) -> (u32, i32) {
        if month == 12 {
            (1, year + 1)
        } else {
            (month + 1, year)
        }
    }

    #[test]
    fn prev_month_wraps_january_to_december() {
        assert_eq!(prev_month(1, 2024), (12, 2023));
    }

    #[test]
    fn next_month_wraps_december_to_january() {
        assert_eq!(next_month(12, 2024), (1, 2025));
    }

    #[test]
    fn prev_month_normal() {
        assert_eq!(prev_month(6, 2024), (5, 2024));
    }

    #[test]
    fn next_month_normal() {
        assert_eq!(next_month(6, 2024), (7, 2024));
    }

    // toggle_calendar reactive signal

    #[test]
    fn toggle_calendar_opens_and_closes() {
        let owner = Owner::new();
        owner.with(|| {
            let (show, set_show) = signal(false);
            set_show.update(|v| *v = !*v);
            assert!(show.get());
            set_show.update(|v| *v = !*v);
            assert!(!show.get());
        });
    }

    // year_page navigation

    #[test]
    fn year_page_increments() {
        let owner = Owner::new();
        owner.with(|| {
            let (year_page, set_year_page) = signal(0usize);
            set_year_page.update(|v| *v += 1);
            assert_eq!(year_page.get(), 1);
        });
    }

    #[test]
    fn year_page_does_not_go_below_zero() {
        let owner = Owner::new();
        owner.with(|| {
            let (year_page, set_year_page) = signal(0usize);
            set_year_page.update(|v| {
                if *v > 0 {
                    *v -= 1
                }
            });
            assert_eq!(year_page.get(), 0);
        });
    }
}
