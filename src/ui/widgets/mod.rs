use crate::i18n::*;
use bytesize::ByteSize;
use chrono::{DateTime, Local};

/// Format bytes into human-readable string
pub fn format_size(bytes: u64) -> String {
    ByteSize::b(bytes).to_string()
}

/// Format a datetime as relative age in Chinese (e.g., "3天前")
pub fn format_age(dt: &DateTime<Local>) -> String {
    let now = Local::now();
    let duration = now.signed_duration_since(*dt);
    let days = duration.num_days();

    if days > 365 {
        translate_format_age_years((days as f64 / 365.0).floor() as u64)
    } else if days > 30 {
        translate_format_age_months((days as f64 / 30.0).floor() as u64)
    } else if days > 0 {
        translate_format_age_days(days)
    } else if duration.num_hours() > 0 {
        translate_format_age_hours(duration.num_hours())
    } else {
        translate_format_age_just_now()
    }
}

/// Generate a simple bar chart string
pub fn bar_chart(ratio: f64, width: usize) -> String {
    let filled = (ratio * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);
    format!("{}{}", "\u{2588}".repeat(filled), "\u{2591}".repeat(empty))
}
