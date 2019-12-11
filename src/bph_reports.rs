//! # BizPharma Reports
//!
//! This module contains functions that take care of connecting to BizPharma databases and creating
//! reports using SQL queries.

use csv::{QuoteStyle, WriterBuilder};
use futures::prelude::*;
use futures_state_stream::StateStream;
use tiberius::query::QueryRow;
use tiberius::{BoxableIo, SqlConnection};
use tracing::{event, span};
use tracing_core::metadata::Level;

/// Gets the value of the `BPH_SQL_SERVER_CONNECTION_STRING` env variable.
///
/// __WARNING__: This function will panic if the environment variable can't be retrieved!
pub fn get_connection_string_from_env() -> String {
    if cfg!(debug_assertions) {
        event!(
            Level::WARN,
            "Using a hard-coded value for `BPH_SQL_SERVER_CONNECTION_STRING` \
             because this is a debugging build!"
        );
        String::from(
            "Server=tcp:10.0.0.140,1433;\
             IntegratedSecurity=false;\
             TrustServerCertificate=true;\
             UserName=sa;\
             Password=REPLACE_ME",
        )
    } else {
        std::env::var("BPH_SQL_SERVER_CONNECTION_STRING")
            .expect("BPH_SQL_SERVER_CONNECTION_STRING environment variable is not defined!")
    }
}

/// This structure's implementation exposes functions that generate reports required by IQVIA.
pub struct IqviaReports;

impl IqviaReports {
    /// Returns a future that generates an IQVIA Pharmacies report saved to a CSV file on disk.
    pub fn generate_pharmacies_report_to_file(
        filepath: &str,
    ) -> impl Future<Item = SqlConnection<Box<dyn BoxableIo>>, Error = String> + '_ {
        let s = span!(Level::INFO, "generate_pharmacies_report_to_file()");
        let _guard = s.enter();
        event!(Level::INFO, "Generating report to file {}", filepath);

        let connection_string = get_connection_string_from_env();

        SqlConnection::connect(connection_string.as_str())
            .and_then(move |connection| {
                let mut csv_writer = WriterBuilder::new()
                    .delimiter(b',')
                    .quote_style(QuoteStyle::Always)
                    .from_path(filepath)
                    .unwrap();
                let mut current_row_idx: usize = 1;

                // execute a stored procedure that generates the data for the report:
                let sql_future = connection
                    .query(
                        "exec [BizPharmaHO].[dbo].[spIMSCreatePharmaciesReport];",
                        &[],
                    )
                    .for_each(move |row: QueryRow| {
                        // create a CSV record from current query row:
                        let mut csv_record: Vec<String> = vec![];

                        for column_idx in 0..row.len() {
                            let value = match row.try_get::<usize, &str>(column_idx) {
                                // unwrap the value to a string slice and replace NULLs with "":
                                Ok(v) => v.unwrap_or(""),
                                // the column value couldn't be converted to a Rust datatype,
                                // replace with a generic value denoting such an error occurred:
                                Err(err) => {
                                    event!(
                                        Level::ERROR,
                                        "row.try_get::<usize, &str> failed for row={}, col={} with: {:?}",
                                        current_row_idx,
                                        column_idx,
                                        err
                                    );
                                    "ERROR_PARSING_VALUE"
                                }
                            };
                            csv_record.push(value.to_string());
                        }

                        // write CSV record to disk:
                        match csv_writer.write_record(&csv_record) {
                            Ok(_) => {},
                            Err(err) => event!(Level::ERROR, "Error writing CSV record: {:?}", err),
                        }

                        current_row_idx += 1;
                        Ok(())
                    });

                event!(Level::INFO, "Report saved to file {}", filepath);
                sql_future
            })
            // stringify tiberius errors:
            .map_err(|err| format!("{:?}", err))
    }

    /// Returns a future that generates an IQVIA CAP_IES report saved to a CSV file on disk.
    pub fn generate_cap_ies_report_to_file(_from_date: &str, _to_date: &str, _filepath: &str) {
        unimplemented!()
    }

    /// Returns a future that generates an IQVIA POZ_IES report saved to a CSV file on disk.
    pub fn generate_poz_ies_report_to_file(_from_date: &str, _to_date: &str, _filepath: &str) {
        unimplemented!()
    }

    /// Returns a future that generates an IQVIA PM report saved to a CSV file on disk.
    pub fn generate_pm_report_to_file(_from_date: &str, _to_date: &str, _filepath: &str) {
        unimplemented!()
    }

    /// Returns a future that generates an IQVIA TM report saved to a CSV file on disk.
    pub fn generate_tm_report_to_file(_from_date: &str, _to_date: &str, _filepath: &str) {
        unimplemented!()
    }

    /// Returns a future that generates an IQVIA CM report saved to a CSV file on disk.
    pub fn generate_cm_report_to_file(_from_date: &str, _to_date: &str, _filepath: &str) {
        unimplemented!()
    }
}
