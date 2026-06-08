use chrono::{DateTime, Local};

/// For Plain T
pub trait Pipe {
    fn text(&self, fallback: Option<&str>) -> Option<String>;
    fn float(&self, precision: Option<usize>, fallback: Option<&str>) -> Option<String>;
    fn int(&self, fallback: Option<&str>) -> Option<String>;
    fn date(&self, fmt_out: &str, fallback: Option<&str>) -> Option<String>;
}
/// For Option<T>
pub trait PipeOption {
    fn text(&self, fallback: Option<&str>) -> Option<String>;
    fn float(&self, precision: Option<usize>, fallback: Option<&str>) -> Option<String>;
    fn int(&self, fallback: Option<&str>) -> Option<String>;
    fn date(&self, fmt_out: &str, fallback: Option<&str>) -> Option<String>;
}

fn format_with_commas(f: f64, precision: Option<usize>) -> String {
    let precision = precision.unwrap_or_else(|| {
        let s = f.to_string();
        s.find('.').map(|i| s.len() - i - 1).unwrap_or(0)
    });

    let formatted = format!("{:.prec$}", f, prec = precision);
    let (integer_part, decimal_part) = match formatted.split_once('.') {
        Some((i, d)) => (i, Some(d)),
        None => (formatted.as_str(), None),
    };

    let (sign, digits) = if integer_part.starts_with('-') {
        ("-", &integer_part[1..])
    } else {
        ("", integer_part)
    };

    let with_commas = digits
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .join(",");

    match decimal_part {
        Some(d) if precision > 0 => format!("{sign}{with_commas}.{d}"),
        _ => format!("{sign}{with_commas}"),
    }
}

fn format_int_with_commas(n: i64) -> String {
    let (sign, digits) = if n < 0 {
        ("-", format!("{}", n.unsigned_abs()))
    } else {
        ("", format!("{}", n))
    };

    let with_commas = digits
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .join(",");

    format!("{sign}{with_commas}")
}

/// For Option<T>
impl<T: std::fmt::Display> PipeOption for Option<T> {
    /// Formats any value that can be displayed as a string.
    ///
    /// Handles Option<String>, Option<&str>, Option<i32>, Option<u32>, etc.
    ///
    /// Falls back to provided default or "N/A" if None or empty.
    fn text(&self, fallback: Option<&str>) -> Option<String> {
        match &self {
            Some(v) => {
                let s = v.to_string();
                if s.trim().is_empty() {
                    Some(fallback.unwrap_or("N/A").to_string())
                } else {
                    Some(s)
                }
            }
            None => Some(fallback.unwrap_or("N/A").to_string()),
        }
    }

    /// Formats a float value from anything parseable as f64 (including strings from APIs).
    ///
    /// Optional precision controls decimal places. Outputs comma-separated thousands.
    ///
    /// Falls back to provided default or "N/A" if None, empty, or unparseable.
    fn float(&self, precision: Option<usize>, fallback: Option<&str>) -> Option<String> {
        match &self {
            Some(v) => match v.to_string().trim().parse::<f64>() {
                Ok(f) => Some(match precision {
                    Some(p) => format_with_commas(f, Some(p)),
                    None => {
                        if f.fract() == 0.0 {
                            format_with_commas(f, Some(0))
                        } else {
                            format_with_commas(f, None)
                        }
                    }
                }),
                Err(_) => Some(fallback.unwrap_or("N/A").to_string()),
            },
            None => Some(fallback.unwrap_or("N/A").to_string()),
        }
    }

    /// Formats an integer value from anything parseable as i64 (including strings from APIs).
    ///
    /// Truncates any decimal portion before formatting. Outputs comma-separated thousands.
    ///
    /// Falls back to provided default or "N/A" if None, empty, or unparseable.
    fn int(&self, fallback: Option<&str>) -> Option<String> {
        match &self {
            Some(v) => {
                let s = v.to_string();
                let trimmed = s.trim();
                // Accept both "42" and "42.0" style strings from APIs
                let parsed = trimmed
                    .parse::<i64>()
                    .or_else(|_| trimmed.parse::<f64>().map(|f| f.trunc() as i64));
                match parsed {
                    Ok(n) => Some(format_int_with_commas(n)),
                    Err(_) => Some(fallback.unwrap_or("N/A").to_string()),
                }
            }
            None => Some(fallback.unwrap_or("N/A").to_string()),
        }
    }

    /// Formats a date string (ISO 8601 / RFC 3339) parseable as a datetime.
    ///
    /// Uses `fmt_out` as the output format. It must be a valid ISO 8601 / RFC 3339 string.
    ///
    /// Falls back to provided default or "N/A" if empty or unparseable.
    ///
    /// Usage:
    /// ```
    /// use detaxine_ui::utils::formatters::Pipe;
    ///
    /// "2026-04-22T00:00:00+03:00".date("%b %e %Y", None);
    /// // Some("Apr 22 2026")
    /// ```
    ///
    fn date(&self, fmt_out: &str, fallback: Option<&str>) -> Option<String> {
        match &self {
            Some(v) => {
                let s = v.to_string();
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    return Some(fallback.unwrap_or("N/A").to_string());
                }
                match DateTime::parse_from_rfc3339(trimmed) {
                    Ok(dt) => Some(dt.with_timezone(&Local).format(fmt_out).to_string()),
                    Err(_) => Some(fallback.unwrap_or("N/A").to_string()),
                }
            }
            None => Some(fallback.unwrap_or("N/A").to_string()),
        }
    }
}

/// For plain T — wraps in Some and delegates
impl<T: std::fmt::Display> Pipe for T {
    /// Formats any value that can be displayed as a string.
    ///
    /// Handles String, &str, i32, u32, etc.
    ///
    /// Falls back to provided default or "N/A" if empty.
    fn text(&self, fallback: Option<&str>) -> Option<String> {
        Some(self).text(fallback)
    }

    /// Formats a float value from anything parseable as f64 (including strings from APIs).
    ///
    /// Optional precision controls decimal places. Outputs comma-separated thousands.
    ///
    /// Falls back to provided default or "N/A" if empty or unparseable.
    fn float(&self, precision: Option<usize>, fallback: Option<&str>) -> Option<String> {
        Some(self).float(precision, fallback)
    }

    /// Formats an integer value from anything parseable as i64 (including strings from APIs).
    ///
    /// Truncates any decimal portion before formatting. Outputs comma-separated thousands.
    ///
    /// Falls back to provided default or "N/A" if empty or unparseable.
    fn int(&self, fallback: Option<&str>) -> Option<String> {
        Some(self).int(fallback)
    }

    /// Formats a date string (ISO 8601 / RFC 3339) parseable as a datetime.
    ///
    /// Uses `fmt_out` as the output format. It must be a valid ISO 8601 / RFC 3339 string.
    ///
    /// Falls back to provided default or "N/A" if empty or unparseable.
    ///
    /// Usage:
    /// ```
    /// use detaxine_ui::utils::formatters::PipeOption;
    ///
    /// Some("2026-04-22T00:00:00+03:00").date("%b %e %Y", None);
    /// // Some("Apr 22 2026")
    /// ```
    ///
    fn date(&self, fmt_out: &str, fallback: Option<&str>) -> Option<String> {
        Some(self).date(fmt_out, fallback)
    }
}
