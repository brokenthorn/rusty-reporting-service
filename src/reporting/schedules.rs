use chrono::{DateTime, Local};

/// Represents a report scheduled to be run repeatedly each month, at the specified day,
/// hour and minute.
pub struct MonthlyReportSchedule {
    pub id: i32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
}

/// Represents a report scheduled to run once at a certain instant in time.
pub struct OneTimeReportSchedule {
    pub id: i32,
    pub when: DateTime<Local>,
}
