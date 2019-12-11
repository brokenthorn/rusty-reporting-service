//! # Tasks
//!
//! This module contains higher level functions for generating reports.
//! The functions wrap the more complex and lower-level functions in the `*_reports` modules.

use tracing::{event, span};
use tracing_core::metadata::Level;

use crate::bph_reports;

/// This function generates an IQVIA Pharmacies report to a CSV file.
///
/// By default, the report is saved to a CSV file in the current directory. The file name contains
/// the text `iqvia_pharmacies_report_` followed by a UNIX timestamp generated when the function was
/// called.
pub fn generate_iqvia_pharmacies_report_to_file<'a>(filepath: impl Into<Option<&'a str>>) {
    let s = span!(Level::INFO, "generate_iqvia_pharmacies_report_to_file()");
    let _guard = s.enter();
    event!(Level::INFO, "Started.");

    let default_filepath = format!(
        "iqvia_pharmacies_report_{}.csv",
        chrono::Local::now().timestamp()
    );
    let my_filepath = filepath.into().unwrap_or(default_filepath.as_str());
    let sql_future = bph_reports::IqviaReports::generate_pharmacies_report_to_file(my_filepath);

    // pool the above future in the current thread since this will run in clokwerk anyway:
    match tokio::runtime::current_thread::block_on_all(sql_future) {
        Ok(_result) => event!(Level::INFO, "Finished.",),
        Err(err) => event!(Level::ERROR, "Error: {:?}", err),
    }
}
