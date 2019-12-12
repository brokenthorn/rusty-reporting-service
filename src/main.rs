//! # Rusty Reporting Service (RRS)
//!
//! A small special purpose reporting service that builds reports periodically on a schedule.

use std::time::Duration;

use clokwerk::{Interval, TimeUnits};
use tracing::{info, span, warn};
use tracing_core::Level;
use tracing_subscriber::FmtSubscriber;

use task_managers::SimpleTaskScheduler;

use crate::tasks::{
    generate_iqvia_cap_ies_report_to_file, generate_iqvia_pharmacies_report_to_file,
};

pub mod bph_reports;
pub mod task_managers;
pub mod tasks;

fn main() {
    // build a custom stdout formatter for log messages:
    use tracing_subscriber::field::MakeExt;
    use tracing_subscriber::fmt::format;
    let formatter = format::debug_fn(|writer, field, value| {
        write!(
            writer,
            "\r\n                         {} = {:?}",
            field, value
        )
    })
    // Use the `tracing_subscriber::MakeFmtExt` trait to wrap the
    // formatter so that a delimiter is added between fields.
    .delimited(", ");

    let fmt_subscriber = FmtSubscriber::builder()
        .with_ansi(true) // disable if using .json()
        .fmt_fields(formatter) // use the above custom formatter
        .with_target(true) // outputs module path (or custom target name if that was set)
        // .json() // enable in production
        .finish();

    tracing::subscriber::set_global_default(fmt_subscriber)
        .expect("Unable to set global default subscriber!");

    let s = span!(Level::INFO, "main()");
    let _guard = s.enter();

    if cfg!(debug_assertions) {
        warn!("RUNNING A DEBUG BUILD!!!");
    }

    info!("Application starting.");

    let mut manager = SimpleTaskScheduler::new();
    // TODO: Change interval to 1.day() and remove comment on code below for check if day = 1.
    let every_10_seconds: Interval = 10.second();
    let every_2_seconds: Interval = 2.second();

    manager.add_task(every_10_seconds, || {
        // if chrono::Utc::now().day() != 1 {
        //     return;
        // }
        generate_iqvia_pharmacies_report_to_file(None);
    });

    manager.add_task(every_2_seconds, || {
        generate_iqvia_cap_ies_report_to_file(None, None, None);
    });

    // the interval set here basically sets the fidelity of the task scheduler used within the manager
    manager.start(Duration::from_millis(500));

    info!("Application exited.");
}
