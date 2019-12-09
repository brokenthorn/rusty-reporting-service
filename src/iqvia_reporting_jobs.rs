//! # BizPharma Reporting Jobs for usually requested reports by IQVIA Romania
//! This module contains reporting jobs for BizPharma databases.
//! They jobs produce files with reporting data that is usually requested by
//! [IQVIA Romania](https://www.iqvia.com/ro-ro/locations/romania).

use csv::{QuoteStyle, WriterBuilder};
use futures::prelude::*;
use futures_state_stream::StateStream;
use tiberius::query::QueryRow;
use tiberius::{BoxableIo, SqlConnection};
use tracing::{event, span};
use tracing_core::metadata::Level;

/// Get SQL Server connection string from environment.
pub fn get_sql_server_connection_string() -> String {
    if cfg!(debug_assertions) {
        event!(
            Level::WARN,
            msg = "USING A HARD-CODED `SQL_SERVER_CONNECTION_STRING`!"
        );
        String::from("Server=tcp:10.0.0.140,1433;IntegratedSecurity=false;TrustServerCertificate=true;UserName=sa;Password=REPLACE_ME")
    } else {
        // Get connection string from environment variable:
        event!(
            Level::INFO,
            msg = "Using value from `SQL_SERVER_CONNECTION_STRING` environment variable."
        );
        std::env::var("SQL_SERVER_CONNECTION_STRING")
            .expect("SQL_SERVER_CONNECTION_STRING environment variable is missing!")
    }
}

/// Returns a `tiberius::SqlConnection` future chained with other operations for generating
/// IQVIA's `Pharmacies` report, saved to a file on disk.
pub fn iqvia_pharmacies_report(
    filepath: &str,
) -> impl Future<Item = SqlConnection<Box<dyn BoxableIo>>, Error = String> + '_ {
    let s = span!(Level::INFO, "iqvia_pharmacies_report()");
    let _guard = s.enter();
    event!(
        Level::INFO,
        "Producing IQVIA Pharmacies report in file {}",
        filepath
    );

    let connection_string = get_sql_server_connection_string();
    let connection_future = SqlConnection::connect(connection_string.as_str());

    connection_future
        .and_then(move |connection| {
            let mut csv_writer = WriterBuilder::new()
                .delimiter(b',')
                .quote_style(QuoteStyle::Always)
                .from_path(filepath)
                .unwrap();
            let mut current_row_idx: usize = 1;

            connection
                .query(
                    "exec [BizPharmaHO].[dbo].[spIMSCreatePharmaciesReport];",
                    &[],
                )
                .for_each(move |row: QueryRow| {
                    let s = span!(Level::INFO, "for_each_row()");
                    let _guard = s.enter();
                    event!(
                        Level::INFO,
                        "Fetching results from row {}.",
                        current_row_idx
                    );

                    let mut csv_record: Vec<String> = vec![];

                    for column_idx in 0..row.len() {
                        let column_value = match row.try_get::<usize, &str>(column_idx) {
                            Ok(v) => v.unwrap_or(""),
                            Err(err) => {
                                event!(
                                    Level::ERROR,
                                    "Error getting value from row {}, col {}: {:?}",
                                    current_row_idx,
                                    column_idx,
                                    err
                                );
                                "ERROR_GETTING_VALUE"
                            }
                        };
                        csv_record.push(column_value.to_string());
                    }

                    match csv_writer.write_record(&csv_record) {
                        Ok(_) => {}
                        Err(err) => {
                            event!(Level::ERROR, "Error writing CSV record: {:?}", err);
                        }
                    }

                    event!(Level::INFO, "Flushing file {}", filepath);
                    csv_writer.flush()?;

                    current_row_idx += 1;
                    Ok(())
                })
        })
        // turn errors into a String:
        .map_err(|err| format!("Query error: {:?}", err))
}

/// Returns a future for the completion of IQVIA's `CAP_IES` report saved to a file.
pub fn iqvia_cap_ies_report(_from_date: &str, _to_date: &str, _filepath: &str) {
    unimplemented!()
}

/// Returns a future for the completion of IQVIA's `POZ_IES` report saved to a file.
pub fn iqvia_poz_ies_report(_from_date: &str, _to_date: &str, _filepath: &str) {
    unimplemented!()
}

/// Returns a future for the completion of IQVIA's `PM` report saved to a file.
pub fn iqvia_pm_report(_from_date: &str, _to_date: &str, _filepath: &str) {
    unimplemented!()
}

/// Returns a future for the completion of IQVIA's `TM` report saved to a file.
pub fn iqvia_tm_report(_from_date: &str, _to_date: &str, _filepath: &str) {
    unimplemented!()
}

/// Returns a future for the completion of IQVIA's `CM` report saved to a file.
pub fn iqvia_cm_report(_from_date: &str, _to_date: &str, _filepath: &str) {
    unimplemented!()
}
