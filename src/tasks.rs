//! # Tasks
//!
//! This module contains higher level functions for generating reports.
//! The functions wrap the more complex and lower-level functions in the `*_reports` modules.

use chrono::{Datelike, NaiveDate};
use tracing::{error, info, span};
use tracing_core::Level;

use crate::bph_reports;

fn days_in_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or(NaiveDate::from_ymd(year + 1, 1, 1))
        .pred()
        .day()
}

/// This function generates an IQVIA Pharmacies report to a CSV file.
///
/// By default, the report is saved to a CSV file in the current directory. The file name contains
/// the text `iqvia_pharmacies_report_` followed by a UNIX timestamp generated when the function was
/// called.
pub fn generate_iqvia_pharmacies_report_to_file<'a>(filepath: impl Into<Option<&'a str>>) {
    let s = span!(Level::INFO, "generate_iqvia_pharmacies_report_to_file()");
    let _guard = s.enter();
    info!("Started.");

    let default_filepath = format!(
        "iqvia_pharmacies_report_{}.csv",
        chrono::Local::now().timestamp()
    );
    let my_filepath = filepath.into().unwrap_or(default_filepath.as_str());
    let sql_future = bph_reports::IqviaReports::generate_pharmacies_report_to_file(my_filepath);

    // pool the above future in the current thread since this will run in clokwerk anyway:
    match tokio::runtime::current_thread::block_on_all(sql_future) {
        Ok(_result) => info!("Finished.",),
        Err(err) => error!("Finished with error: {:?}", err),
    }
}

/// This function generates an IQVIA CAP_IES report to a CSV file.
///
/// By default, the report is saved to a CSV file in the current directory. The file name contains
/// the text `iqvia_cap_ies_report_` followed by a UNIX timestamp generated when the function was
/// called.
pub fn generate_iqvia_cap_ies_report_to_file<'a>(
    from_date: impl Into<Option<&'a str>>,
    to_date: impl Into<Option<&'a str>>,
    filepath: impl Into<Option<&'a str>>,
) {
    let s = span!(Level::INFO, "generate_iqvia_cap_ies_report_to_file()");
    let _guard = s.enter();
    info!("Started.");

    let now_datetime = chrono::Local::now();
    let last_day_of_month = days_in_month(now_datetime.year(), now_datetime.month());

    let start_of_month_datetime = now_datetime.with_day(1).unwrap().date().and_hms(0, 0, 0);
    let end_of_month_datetime = now_datetime
        .with_day(last_day_of_month)
        .unwrap()
        .date()
        .and_hms(23, 59, 59);

    let default_filepath = format!("iqvia_cap_ies_report_{}.csv", now_datetime.timestamp());
    // ISO 8601 / RFC 3339 date & time format.
    let default_from_date = format!("{}", start_of_month_datetime.format("%+"));
    let default_to_date = format!("{}", end_of_month_datetime.format("%+"));

    let my_filepath = filepath.into().unwrap_or(default_filepath.as_str());
    let my_from_date = from_date.into().unwrap_or(default_from_date.as_str());
    let my_to_date = to_date.into().unwrap_or(default_to_date.as_str());

    let sql_future = bph_reports::IqviaReports::generate_cap_ies_report_to_file(
        my_from_date,
        my_to_date,
        my_filepath,
    );

    // pool the above future in the current thread since this will run in clokwerk anyway:
    match tokio::runtime::current_thread::block_on_all(sql_future) {
        Ok(_result) => info!("Finished.",),
        Err(err) => error!("Finished with error: {:?}", err),
    }
}

/// This function generates an IQVIA POZ_IES report to a CSV file.
///
/// By default, the report is saved to a CSV file in the current directory. The file name contains
/// the text `iqvia_poz_ies_report_` followed by a UNIX timestamp generated when the function was
/// called.
pub fn generate_iqvia_poz_ies_report_to_file<'a>(
    from_date: impl Into<Option<&'a str>>,
    to_date: impl Into<Option<&'a str>>,
    filepath: impl Into<Option<&'a str>>,
) {
    let s = span!(Level::INFO, "generate_iqvia_poz_ies_report_to_file()");
    let _guard = s.enter();
    info!("Started.");

    let default_filepath = format!(
        "iqvia_poz_ies_report_{}.csv",
        chrono::Local::now().timestamp()
    );
    let my_filepath = filepath.into().unwrap_or(default_filepath.as_str());
    let sql_future = bph_reports::IqviaReports::generate_pharmacies_report_to_file(my_filepath);

    // pool the above future in the current thread since this will run in clokwerk anyway:
    match tokio::runtime::current_thread::block_on_all(sql_future) {
        Ok(_result) => info!("Finished.",),
        Err(err) => error!("Finished with error: {:?}", err),
    }
}

/// This function generates an IQVIA PM report to a CSV file.
///
/// By default, the report is saved to a CSV file in the current directory. The file name contains
/// the text `iqvia_pm_report_` followed by a UNIX timestamp generated when the function was
/// called.
pub fn generate_iqvia_pm_report_to_file<'a>(
    from_date: impl Into<Option<&'a str>>,
    to_date: impl Into<Option<&'a str>>,
    filepath: impl Into<Option<&'a str>>,
) {
    let s = span!(Level::INFO, "generate_iqvia_pm_report_to_file()");
    let _guard = s.enter();
    info!("Started.");

    let default_filepath = format!("iqvia_pm_report_{}.csv", chrono::Local::now().timestamp());
    let my_filepath = filepath.into().unwrap_or(default_filepath.as_str());
    let sql_future = bph_reports::IqviaReports::generate_pharmacies_report_to_file(my_filepath);

    // pool the above future in the current thread since this will run in clokwerk anyway:
    match tokio::runtime::current_thread::block_on_all(sql_future) {
        Ok(_result) => info!("Finished.",),
        Err(err) => error!("Finished with error: {:?}", err),
    }
}

/// This function generates an IQVIA TM report to a CSV file.
///
/// By default, the report is saved to a CSV file in the current directory. The file name contains
/// the text `iqvia_tm_report_` followed by a UNIX timestamp generated when the function was
/// called.
pub fn generate_iqvia_tm_report_to_file<'a>(
    from_date: impl Into<Option<&'a str>>,
    to_date: impl Into<Option<&'a str>>,
    filepath: impl Into<Option<&'a str>>,
) {
    let s = span!(Level::INFO, "generate_iqvia_tm_report_to_file()");
    let _guard = s.enter();
    info!("Started.");

    let default_filepath = format!("iqvia_tm_report_{}.csv", chrono::Local::now().timestamp());
    let my_filepath = filepath.into().unwrap_or(default_filepath.as_str());
    let sql_future = bph_reports::IqviaReports::generate_pharmacies_report_to_file(my_filepath);

    // pool the above future in the current thread since this will run in clokwerk anyway:
    match tokio::runtime::current_thread::block_on_all(sql_future) {
        Ok(_result) => info!("Finished.",),
        Err(err) => error!("Finished with error: {:?}", err),
    }
}

/// This function generates an IQVIA CM report to a CSV file.
///
/// By default, the report is saved to a CSV file in the current directory. The file name contains
/// the text `iqvia_cm_report_` followed by a UNIX timestamp generated when the function was
/// called.
pub fn generate_iqvia_cm_report_to_file<'a>(
    from_date: impl Into<Option<&'a str>>,
    to_date: impl Into<Option<&'a str>>,
    filepath: impl Into<Option<&'a str>>,
) {
    let s = span!(Level::INFO, "generate_iqvia_cm_report_to_file()");
    let _guard = s.enter();
    info!("Started.");

    let default_filepath = format!("iqvia_cm_report_{}.csv", chrono::Local::now().timestamp());
    let my_filepath = filepath.into().unwrap_or(default_filepath.as_str());
    let sql_future = bph_reports::IqviaReports::generate_pharmacies_report_to_file(my_filepath);

    // pool the above future in the current thread since this will run in clokwerk anyway:
    match tokio::runtime::current_thread::block_on_all(sql_future) {
        Ok(_result) => info!("Finished.",),
        Err(err) => error!("Finished with error: {:?}", err),
    }
}
