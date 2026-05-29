use chrono::{DateTime, Datelike, Utc};

/// The `start` argument to be a valid RFC3339 formatted string.
pub fn get_elapsed_time(start: &str, end: &DateTime<Utc>) -> String {
    match DateTime::parse_from_rfc3339(start) {
        Ok(start) => {
            // let start = start.with_timezone(&Utc);

            // Years (calendar-aware)
            let mut years = end.year() - start.year();
            if end.ordinal() < start.ordinal() {
                years -= 1;
            }

            match years {
                0 => {
                    // Months (calendar-aware)
                    let mut months = (end.year() - start.year()) * 12
                        + (end.month() as i32 - start.month() as i32);
                    if end.day() < start.day() {
                        months -= 1;
                    }

                    match months {
                        0 => match end.signed_duration_since(&start).num_weeks() {
                            0 => match end.signed_duration_since(&start).num_days() {
                                0 => match end.signed_duration_since(&start).num_hours() {
                                    0 => match end.signed_duration_since(&start).num_minutes() {
                                        0 => {
                                            match end.signed_duration_since(&start).num_seconds() {
                                                0 => "Just now".to_string(),
                                                val => format!(
                                                    "{val} {}",
                                                    if val == 1 { "second" } else { "seconds" }
                                                ),
                                            }
                                        }
                                        val => format!(
                                            "{val} {}",
                                            if val == 1 { "minute" } else { "minutes" }
                                        ),
                                    },
                                    val => {
                                        format!("{val} {}", if val == 1 { "hour" } else { "hours" })
                                    }
                                },
                                val => format!("{val} {}", if val == 1 { "day" } else { "days" }),
                            },
                            val => format!("{val} {}", if val == 1 { "week" } else { "weeks" }),
                        },
                        val => format!("{val} {}", if val == 1 { "month" } else { "months" }),
                    }
                }
                val => format!("{val} {}", if val == 1 { "year" } else { "years" }),
            }
        }
        Err(_e) => "Invalid Date".to_string(),
    }
}

pub fn convert_date_to_human_readable_format(date_string: &str) -> String {
    match DateTime::parse_from_rfc3339(date_string) {
        Ok(date_string) => date_string.format("%d %b %Y").to_string(),
        Err(_) => "Invalid Date".to_string(),
    }
}
