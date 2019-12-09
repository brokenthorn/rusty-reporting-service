//! # ⏱ Rusty Reporting Service
//! A tiny special purpose reporting service built initially for Mini-Farm S.R.L.
//!
//! It schedules and builds reports to be sent via email.

use std::time::Duration;

use clokwerk::{Interval, TimeUnits};
//use futures::Future;
//use futures_state_stream::StateStream;
use tracing::{event, span};
use tracing_core::metadata::Level;
use tracing_subscriber::FmtSubscriber;

use managers::SimpleManager;

pub mod iqvia_reporting_jobs;
pub mod managers;

/// Log span name for events emitted from `main()`.
pub const LOG_SPAN_NAME: &'static str = "main()";

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
    let i: Interval = 5.second();

    manager.add_task(i, move || {
        let s = span!(Level::INFO, "IqviaPharmaciesReportTask");
        let _guard = s.enter();
        event!(Level::INFO, msg = "SQL Server Report Job 1 was triggered.");

        // TODO: Here check if current day is day 1 of Month,
        //  because we want to run once every Month on day 1.

        event!(
            Level::INFO,
            msg = "Spinning up a Tokio executor on the current thread in order to run SQL query."
        );

        let sql_future =
            iqvia_reporting_jobs::iqvia_pharmacies_report("./test-pharmacies-report.csv");

        match tokio::runtime::current_thread::block_on_all(sql_future) {
            Ok(_result) => event!(
                Level::INFO,
                msg = "Tokio executor finished running all futures successfully.",
            ),
            Err(err) => event!(
                Level::ERROR,
                "Tokio executor finished with an error: {:?}",
                err
            ),
        }
    });

    // the interval set here basically sets the fidelity of the task scheduler used within the manager
    manager.start(Duration::from_millis(500));

    event!(Level::INFO, "Application exited.");
}
