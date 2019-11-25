//! # rrs module
//! `rrs` is the main `rusty-reporting-service` crate module.

use std::thread;
use std::time::Duration;

// Scheduler, and trait for .seconds(), .minutes(), etc.
use clokwerk::{ScheduleHandle, Scheduler};
use rand::Rng;
use tracing::{event, span};
use tracing_core::metadata::Level;

// Import week days and WeekDay
//use clokwerk::Interval::*;

/// Manager for `clokwerk` schedulers.
/// You can add custom `clokwerk` schedulers, start them all and stop them all.
pub struct SchedulersManager {
    schedulers: Vec<Scheduler>,
    running_handles: Vec<ScheduleHandle>,
}

impl SchedulersManager {
    pub fn new() -> Self {
        let s = span!(Level::INFO, "SchedulersManager");
        let _guard = s.enter();
        event!(Level::INFO, msg = "New SchedulersManager.");

        SchedulersManager {
            schedulers: vec![],
            running_handles: vec![],
        }
    }

    /// Adds a new scheduler to this manager.
    /// Added `clokwerk` schedulers are not started automatically. You need to call `start()` for that.
    pub fn add_scheduler(&mut self, scheduler: Scheduler) {
        let s = span!(Level::INFO, "SchedulersManager");
        let _guard = s.enter();
        event!(
            Level::INFO,
            msg = "Adding new `clokwerk` scheduler to this manager."
        );

        self.schedulers.push(scheduler);
    }

    /// Starts all schedulers in separate threads so this function does not block.
    /// All schedulers that have been started are consumed and cannot be stopped and then started again.
    /// To do that, you need to recreate the schedulers and then add them to this manager again.
    pub fn start(&mut self) {
        let s = span!(Level::INFO, "SchedulersManager");
        let _guard = s.enter();
        event!(
            Level::INFO,
            msg = "Starting all schedulers in separate threads watched each at 100ms intervals."
        );

        while let Some(scheduler) = self.schedulers.pop() {
            let handle = scheduler.watch_thread(Duration::from_millis(100));
            self.running_handles.push(handle);
        }

        event!(Level::INFO, msg = "Done. All schedulers started.");
    }

    pub fn stop(&mut self) {
        let s = span!(Level::INFO, "SchedulersManager");
        let _guard = s.enter();
        event!(Level::INFO, msg = "Stopping all schedulers.");

        while let Some(handle) = self.running_handles.pop() {
            handle.stop();
        }

        event!(Level::INFO, msg = "Done.");
    }

    pub fn wait(&mut self) {
        let check_interval = 10000;

        let s = span!(Level::INFO, "SchedulersManager");
        let _guard = s.enter();
        event!(
            Level::INFO,
            msg = format!(
                "Waiting for schedulers to finish. Checking state every {}ms.",
                check_interval
            )
            .as_str()
        );

        let mut rng = rand::thread_rng();

        loop {
            let cnt_running_handles = self.running_handles.len();
            if cnt_running_handles > 0 {
                event!(
                    Level::INFO,
                    msg = format!(
                        "{} scheduler(s) are still running. Next check in {}ms.",
                        cnt_running_handles, check_interval
                    )
                    .as_str()
                );

                thread::sleep(Duration::from_millis(check_interval));

                if rng.gen_ratio(2, 3) {
                    event!(
                        Level::WARN,
                        msg = "A 2/3 chance RNG triggered a finish of all schedulers."
                    );
                    self.stop();
                }
            } else {
                event!(Level::INFO, msg = "Done. No more schedulers running.");
                break;
            }
        }
    }
}
