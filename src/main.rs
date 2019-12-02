//! # ⏱ Rusty Reporting Service
//! A tiny special purpose reporting service built for Mini-Farm S.R.L.
//!
//! It schedules and builds reports to be sent via email.

use std::time::Duration;

use clokwerk::{Interval, TimeUnits};
use std::thread;
use tracing::{event, span};
use tracing_core::metadata::Level;
use tracing_subscriber::FmtSubscriber;

pub mod manager;

/// Log span name used in this module, in conjunction with facilities from the `tracing` crate.
pub const LOG_SPAN_NAME: &'static str = "MAIN";

fn main() {
    let fmt_subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(fmt_subscriber)
        .expect("Setting global default tracing subscriber failed.");

    let s = span!(Level::INFO, LOG_SPAN_NAME);
    let _guard = s.enter();
    event!(Level::INFO, msg = "Application starting.");

    let mut manager = manager::Manager::new();
    let i: Interval = 1.second();

    manager.add_task(i, move || {
        let s = span!(Level::INFO, LOG_SPAN_NAME);
        let _guard = s.enter();
        // TODO: Here check if current day is day 1 of Month,
        //  because we want to run once every Month on day 1.
        event!(Level::INFO, msg = "A job was triggered.")
    });

    manager.start(Duration::from_millis(1000));

    //    use futures::Future;
    //    use futures_state_stream::StateStream;
    //    use tiberius::SqlConnection;
    //    use tokio::executor::current_thread;
    //
    //    let conn_str = if cfg!(windows) {
    //        "server=tcp:localhost,1433;integratedSecurity=true;TrustServerCertificate=true;".to_owned()
    //    } else {
    //        ::std::env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap()
    //    };
    //
    //    let future = SqlConnection::connect(conn_str.as_str()).and_then(|conn| {
    //        conn.query(
    //            "SELECT x FROM (VALUES (1),(2),(3),(4)) numbers(x) WHERE x%@P1=@P2",
    //            &[&2i32, &0i32],
    //        )
    //        .for_each(|row| {
    //            let val: i32 = row.get(0);
    //            assert_eq!(val % 2, 0i32);
    //            Ok(())
    //        })
    //    });
    //    current_thread::block_on_all(future).unwrap();

    event!(Level::INFO, "Application exited.");
}
