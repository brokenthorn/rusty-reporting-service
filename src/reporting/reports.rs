/// A report that's generated from the results of a SQL query.
pub struct SQLQueryReport<'a> {
    /// A unique identifier used when scheduling multiple reports by the same scheduler instance.
    pub id: i32,
    /// A friendly, even descriptive name to give the report. Useful when logging.
    pub friendly_name: &'a str,
    /// Used to disable running a report for any reason (like temporarily stopping reporting of a
    /// bad SQL query until it's fixed, without having to stop the reporter instance from finishing
    /// other reports).
    pub active: bool,
    /// SQL query to run to produce the report results.
    pub sql_query: &'a str,
    /// Should SQL query headers be included in the results, as the first row?
    /// Useful for exporting to CSV with headers included.
    pub include_sql_headers: bool,
    /// Path to a folder on disk where the report will be saved.
    pub file_save_path: &'a str,
}
