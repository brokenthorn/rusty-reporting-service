//! # ⏱ Rusty Reporting Service
//! A special purpose reporting service built for Mini-Farm S.R.L.
//!
//! It can schedule tasks that generate reports and execute them at specified time intervals.
//! It then saves those reports to disk.
//!
//! It can also optionally:
//! * upload reports to FTP/SFTP
//! * send reports by email as attachments

//extern crate futures;
//extern crate futures_state_stream;
//extern crate tiberius;
//extern crate tokio;
//extern crate tracing;

//use futures::Future;
//use futures_state_stream::StateStream;
//use tiberius::SqlConnection;
//use tokio::executor::current_thread;

// Scheduler, and trait for .seconds(), .minutes(), etc.
use clokwerk::{Scheduler, TimeUnits};
// Import week days and WeekDay
//use clokwerk::Interval::*;

use tracing::{event, span};
use tracing_core::metadata::Level;
use tracing_subscriber::FmtSubscriber;

mod rrs;

fn main() {
    let fmt_subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(fmt_subscriber)
        .expect("Setting global default tracing subscriber failed.");

    // The main application tracing span. By default it only traces TRACE level events.
    let s = span!(Level::INFO, "Main");
    let _guard = s.enter();

    event!(Level::INFO, msg = "Application started.");

    let mut manager = rrs::SchedulersManager::new();

    let mut s1 = Scheduler::new();

    s1.every(5.seconds()).run(|| {
        let s = span!(target: "S1_Scheduler", Level::INFO, "S1 Scheduler");
        let _guard = s.enter();
        event!(Level::INFO, msg = "Triggered. Next trigger in 5s.");
    });

    manager.add_scheduler(s1);
    manager.start();
    manager.wait();

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
