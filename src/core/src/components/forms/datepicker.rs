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

/// This is a custom date picker component that allows users to select a date from a calendar.
///
/// It provides a user-friendly interface for selecting dates, with features such as:
/// - Displaying the current date and time
/// - Navigating between months and years
/// - Selecting a specific date by clicking on the calendar grid
/// - Validating user input and displaying error messages if necessary
/// - Supporting min/max date constraints
/// - Supporting explicitly disabled dates
///
/// The component is designed to be easily integrated into existing forms and can be customized to fit specific design requirements.
/// Example usage:
/// ```
/// <DatePicker
///     label="Appointment"
///     name="appointment"
///     min=Local::now()
///     max=Local::now() + Duration::days(30)
///     disabled_dates=vec![some_holiday]
/// />
/// ```
#[component]
pub fn DatePicker(
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] name: String,
    #[prop(default = false, optional)] required: bool,
    #[prop(into, default = RwSignal::new(None), optional)] initial_value: RwSignal<
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
        set_selected_date.set(initial_value.get());
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
                                style_ext_reactive=Memo::new(move |_| {
                                    let is_selected = highlighted.get().map(|h| {
                                        !is_blank
                                            && h.day() == day
                                            && h.month() == current_month.get()
                                            && h.year() == current_year.get()
                                    }).unwrap_or(false);

                                    if is_blank {
                                        "".into()
                                    } else if disabled {
                                        "flex text-xs items-center justify-center border-none rounded m-1 text-gray-300 cursor-not-allowed".into()
                                    } else if is_selected {
                                        "flex text-xs items-center justify-center border-none rounded m-1 bg-primary text-contrast-white cursor-pointer".into()
                                    } else {
                                        "flex text-xs items-center justify-center border-none rounded m-1 hover:bg-blue-200 cursor-pointer".into()
                                    }
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
