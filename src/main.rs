extern crate futures;
extern crate futures_state_stream;
extern crate tiberius;
extern crate tokio;

use futures::Future;
use futures_state_stream::StateStream;
use tiberius::SqlConnection;
use tokio::executor::current_thread;

mod reporting;

fn main() {
    let conn_str = if cfg!(windows) {
        "server=tcp:localhost,1433;integratedSecurity=true;TrustServerCertificate=true;".to_owned()
    } else {
        ::std::env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap()
    };

    let future = SqlConnection::connect(conn_str.as_str()).and_then(|conn| {
        conn.query(
            "SELECT x FROM (VALUES (1),(2),(3),(4)) numbers(x) WHERE x%@P1=@P2",
            &[&2i32, &0i32],
        )
        .for_each(|row| {
            let val: i32 = row.get(0);
            assert_eq!(val % 2, 0i32);
            Ok(())
        })
    });
    current_thread::block_on_all(future).unwrap();
}
