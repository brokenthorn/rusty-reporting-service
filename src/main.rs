//! # Rusty Reporting Service (RRS)
//!
//! A small special purpose reporting service that builds reports periodically on a schedule.

use std::time::Duration;

use clokwerk::{Interval, TimeUnits};
use tracing::{event, span};
use tracing_core::metadata::Level;
use tracing_subscriber::FmtSubscriber;

use task_managers::SimpleTaskScheduler;

use crate::tasks::generate_iqvia_pharmacies_report_to_file;

pub mod bph_reports;
pub mod task_managers;
pub mod tasks;

fn main() {
    let fmt_subscriber = FmtSubscriber::new();

    tracing::subscriber::set_global_default(fmt_subscriber)
        .expect("Setting global default tracing subscriber failed.");

    let s = span!(Level::INFO, "main()");
    let _guard = s.enter();

    if cfg!(debug_assertions) {
        event!(Level::WARN, "RUNNING A DEBUG BUILD!!!");
    }

    event!(Level::INFO, "Application starting.");

    let mut manager = SimpleTaskScheduler::new();
    let every_day: Interval = 10.second(); // TODO: Change interval to 1.day().

    manager.add_task(every_day, || {
        // if chrono::Utc::now().day() != 1 {
        //     return;
        // }
        generate_iqvia_pharmacies_report_to_file(None);
    });

    // the interval set here basically sets the fidelity of the task scheduler used within the manager
    manager.start(Duration::from_millis(500));

    event!(Level::INFO, "Application exited.");
}
