pub mod sql {
    use chrono::offset::Local;
    use chrono::DateTime;

    use crate::reporting::reports::SQLQueryReport;
    use crate::reporting::schedulers::SQLQueryReportScheduler;
    use crate::reporting::schedules::{MonthlyReportSchedule, OneTimeReportSchedule};
    use crate::reporting::util::get_monthly_report_next_run_datetime;

    /// An implementation of the `Scheduler` trait for scheduling and executing reports generated
    /// from SQL query results.
    ///
    /// Works with MSSQL Server.
    pub struct SQLQueryReporter<'a> {
        /// Reports that are managed by this reporter.
        reports: Vec<Box<SQLQueryReport<'a>>>,
        /// A vector of report schedules. Contains the report ID and the instant in time to run at.
        schedules: Vec<OneTimeReportSchedule>,
        /// A vector of monthly report schedules.
        /// Contains the report ID and the day, hour and minute when each report should be run at,
        /// every month.
        monthly_schedules: Vec<MonthlyReportSchedule>,
    }

    impl<'a> SQLQueryReportScheduler<'a> for SQLQueryReporter<'a> {
        fn add_report(mut self, report: Box<SQLQueryReport<'a>>) -> Result<(), String> {
            let find_pos_result = self.reports.iter().position(|r| r.id == report.id);
            match find_pos_result {
                None => {
                    self.reports.push(report);
                    Ok(())
                }
                Some(_idx) => Err("This report ID already exists with this scheduler.".into()),
            }
        }

        fn schedule(mut self, id: i32, when: DateTime<Local>) -> Result<DateTime<Local>, String> {
            let find_report_pos_result = self.reports.iter().position(|r| r.id == id);
            match find_report_pos_result {
                None => Err("No report with that ID exists with this scheduler.".into()),
                Some(_idx) => {
                    let find_existing_schedule_pos_result =
                        self.schedules.iter().position(|s| s.id == id);
                    match find_existing_schedule_pos_result {
                        Some(_idx) => Err("That report is already scheduled to run.".into()),
                        None => {
                            let schedule = OneTimeReportSchedule { id, when };
                            self.schedules.push(schedule);
                            Ok(when)
                        }
                    }
                }
            }
        }

        fn schedule_monthly(
            mut self,
            id: i32,
            day: u32,
            hour: u32,
            minute: u32,
        ) -> Result<DateTime<Local>, String> {
            let scheduled_at = get_monthly_report_next_run_datetime(day, hour, minute)?;

            let find_report_pos_result = self.reports.iter().position(|r| r.id == id);
            match find_report_pos_result {
                None => Err("No report with that ID exists with this scheduler.".into()),
                Some(_idx) => {
                    let find_existing_schedule_pos_result =
                        self.monthly_schedules.iter().position(|s| s.id == id);
                    match find_existing_schedule_pos_result {
                        Some(_idx) => Err("That report is already scheduled to run.".into()),
                        None => {
                            let schedule = MonthlyReportSchedule {
                                id,
                                day,
                                hour,
                                minute,
                            };
                            self.monthly_schedules.push(schedule);
                            Ok(scheduled_at)
                        }
                    }
                }
            }
        }

        fn get_next_instant(self, id: i32) -> Result<DateTime<Local>, String> {
            // find report:
            let find_report_pos_result = self.reports.iter().position(|r| r.id == id);
            match find_report_pos_result {
                None => Err("No report with that ID exists with this scheduler.".into()),
                Some(_idx) => {
                    // and now find schedules, starting with the one-offs:
                    let schedule_result = self.schedules.iter().find(|s| s.id == id);
                    match schedule_result {
                        Some(schedule) => {
                            return Ok(schedule.when);
                        }
                        _ => {}
                    }

                    // then the monthlies:
                    let monthly_schedule_result =
                        self.monthly_schedules.iter().find(|s| s.id == id);
                    match monthly_schedule_result {
                        Some(schedule) => {
                            return get_monthly_report_next_run_datetime(
                                schedule.day,
                                schedule.hour,
                                schedule.minute,
                            );
                        }
                        _ => {}
                    }

                    // but if no schedule was found, the report isn't scheduled yet:
                    return Err("That report hasn't been scheduled yet.".into());
                }
            }
        }
    }
}
