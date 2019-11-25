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

use futures::Future;
use futures_state_stream::StateStream;
use tiberius::SqlConnection;
use tokio::executor::current_thread;
use tracing::{info, span, Level};

mod rrs;

fn main() {
    #[derive(Debug)]
    struct FooStruct<'a> {
        a: &'a str,
    }

    let mut span = span!(Level::INFO, "Application start up");
    span.in_scope(|| {
        info!(target: "application_main_events", "Something has happened in {}!", "main function");
        let s = FooStruct { a: "bar!" };
        info!("{:?} got created.", s);
    });

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
}
