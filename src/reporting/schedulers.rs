use chrono::offset::Local;
use chrono::DateTime;

use crate::reporting::reports::SQLQueryReport;

/// A trait for a scheduler that reports from SQL query results.
pub trait SQLQueryReportScheduler<'a> {
    /// Adds a report to the scheduler.
    ///
    /// Note: This does not actually schedule a report to be run.
    /// Use one of the `schedule_*` functions to schedule when to run a report.
    fn add_report(self, report: Box<SQLQueryReport<'a>>) -> Result<(), String>;

    /// Schedule a job represented by an ID, to run at a specified instant in time.
    ///
    /// Returns the next instant in time when the job will be run.
    fn schedule(self, id: i32, when: DateTime<Local>) -> Result<DateTime<Local>, String>;

    /// Schedule a job represented by an ID, to be run repeatedly each month, at the specified
    /// day, hour and minute. If the day is 31, it will automatically adjust for day 30 in 30
    /// day months or day 28 (or 29) in February. Day 30 will also auto-adjust to run in the
    /// last day of February.
    ///
    /// Returns the next instant in time when the job will be run.
    fn schedule_monthly(
        self,
        id: i32,
        day: u32,
        hour: u32,
        minute: u32,
    ) -> Result<DateTime<Local>, String>;

    /// Returns the next instant in time when the specified job will be run.
    fn get_next_instant(self, id: i32) -> Result<DateTime<Local>, String>;
}
