use chrono::{DateTime, Datelike, Local, TimeZone};

/// A function that calculates the next date and time that a monthly report should run at.
pub fn get_monthly_report_next_run_datetime(
    day: u32,
    hour: u32,
    minute: u32,
) -> Result<DateTime<Local>, String> {
    let now = Local::now();

    let month = if day <= now.day() {
        (now.month() + 1).rem_euclid(12)
    } else {
        now.month()
    };

    let year = if month >= now.month() {
        now.year()
    } else {
        now.year() + 1
    };

    if day > 31 {
        return Err("Invalid day number. Can't be greater than 31.".into());
    }

    if hour > 23 {
        return Err("Invalid hour number. Can't be greater than 23.".into());
    }

    if minute > 59 {
        return Err("Invalid minute number. Can't be greater than 59.".into());
    }

    Ok(Local.ymd(year, month, day).and_hms(hour, minute, 0))
}
