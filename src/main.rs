//! # ⏱ Rusty Reporting Service
//! A tiny special purpose reporting service built initially for Mini-Farm S.R.L.
//!
//! It schedules and builds reports to be sent via email.

use std::time::Duration;

use clokwerk::{Interval, TimeUnits};
use futures::Future;
use futures_state_stream::StateStream;
use tiberius::SqlConnection;
use tracing::{event, span};
use tracing_core::metadata::Level;
use tracing_subscriber::FmtSubscriber;

use managers::SimpleManager;

pub mod managers;

/// Log span name used in this module, in conjunction with facilities from the `tracing` crate.
pub const LOG_SPAN_NAME: &'static str = "MAIN";

fn main() {
    let fmt_subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(fmt_subscriber)
        .expect("Setting global default tracing subscriber failed.");

    let s = span!(Level::INFO, LOG_SPAN_NAME);
    let _guard = s.enter();
    event!(Level::INFO, msg = "Application starting.");

    if cfg!(debug_assertions) {
        event!(
            Level::WARN,
            msg = "WARNING! YOU ARE RUNNING A NON-OPTIMIZED DEBUG BUILD!!!"
        );
    }

    let mut manager = SimpleManager::new();
    let i: Interval = 1.second();

    manager.add_task(i, move || {
        let s = span!(Level::INFO, LOG_SPAN_NAME);
        let _guard = s.enter();
        // TODO: Here check if current day is day 1 of Month,
        //  because we want to run once every Month on day 1.
        event!(Level::INFO, msg = "A job was triggered.");

        let mut connection_string: String = String::new();

        if cfg!(debug_assertions) {
            connection_string.push_str("server=tcp:10.0.0.140,1433;integratedSecurity=false;TrustServerCertificate=true;username=sa;password=REPLACE_WITH_PASSWORD");
            event!(Level::WARN, msg = "USING A HARD-CODED `SQL_SERVER_CONNECTION_STRING`!");
        } else {
            // Get connection string from environment variable:
            event!(Level::INFO, msg="Using value from `SQL_SERVER_CONNECTION_STRING` environment variable.");
            connection_string = std::env::var("SQL_SERVER_CONNECTION_STRING").expect("SQL_SERVER_CONNECTION_STRING environment variable is missing!");
        }

        let sql_future = SqlConnection::connect(connection_string.as_str()).and_then(|conn| {
            let s = span!(Level::INFO, LOG_SPAN_NAME);
            let _guard = s.enter();

            conn
                .query("SELECT x FROM (VALUES (1), (2), (3), (4)) numbers(x) WHERE x % @P1 = @P2", &[&2i32, &0i32])
                .for_each(|row| {
                    let val: i32 = row.get(0);
                    event!(Level::INFO, msg = "Got value", val);
                    Ok(())
                })
        }).map_err(|e: tiberius::Error| {
            event!(Level::ERROR, error_msg = format!("{:?}", e).as_str());
        });

        event!(Level::INFO, msg = "Spinning up a Tokio executor on the current thread, to connect to SQL server and run queries.");

        match tokio::runtime::current_thread::block_on_all(sql_future) {
            Ok(_) => event!(Level::INFO, msg = "Tokio executor finished running all futures successfully."),
            Err(e) => event!(Level::ERROR, error_msg = format!("Tokio executor terminated with an error: {:?}", e).as_str()),
        }
    });

    manager.start(Duration::from_millis(2000));

    event!(Level::INFO, "Application exited.");
}
