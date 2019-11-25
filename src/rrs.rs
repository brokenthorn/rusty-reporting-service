//! # rrs module
//! `rrs` is the main `rusty-reporting-service` crate module.

// Scheduler, and trait for .seconds(), .minutes(), etc.
use clokwerk::{ScheduleHandle, Scheduler, TimeUnits};
// Import week days and WeekDay
use clokwerk::Interval::*;

use std::thread;
use std::time::Duration;

use rand::Rng;
use tracing::{event, instrument, span, Level};

/// Manager for `clokwerk` schedulers.
/// You can add custom `clokwerk` schedulers, start them all and stop them all.
pub struct SchedulersManager {
    schedulers: Vec<Scheduler>,
    running_handles: Vec<ScheduleHandle>,
}

impl SchedulersManager {
    pub fn new() -> Self {
        let span = span!(Level::INFO, "SchedulersManager::new()");
        let _guard = span.enter();
        event!(Level::INFO, "Creating new instance of `SchedulersManager`.");
        SchedulersManager {
            schedulers: vec![],
            running_handles: vec![],
        }
    }

    /// Adds a new scheduler to this manager.
    /// Added `clokwerk` schedulers are not started automatically. You need to call `start()` for that.
    pub fn add_scheduler(&mut self, s: Scheduler) {
        let span = span!(Level::INFO, "SchedulersManager::add_scheduler()");
        let _guard = span.enter();
        event!(
            Level::INFO,
            "Adding new `clokwerk` scheduler to this manager."
        );
        self.schedulers.push(s);
    }

    /// Starts all schedulers in separate threads so this function does not block.
    /// All schedulers that have been started are consumed and cannot be stopped and then started again.
    /// To do that, you need to recreate the schedulers and then add them to this manager again.
    pub fn start(&mut self) {
        let span = span!(Level::INFO, "SchedulersManager::start()");
        let _guard = span.enter();
        event!(
            Level::INFO,
            "Starting all schedulers in separate threads watched each at 100ms intervals."
        );
        while let Some(scheduler) = self.schedulers.pop() {
            let handle = scheduler.watch_thread(Duration::from_millis(100));
            self.running_handles.push(handle);
        }
        event!(Level::INFO, "Done. All schedulers started.");
    }

    pub fn stop(&mut self) {
        let span = span!(Level::INFO, "SchedulersManager::stop()");
        let _guard = span.enter();
        event!(Level::INFO, "Stopping all schedulers.");
        while let Some(handle) = self.running_handles.pop() {
            handle.stop();
        }
        event!(Level::INFO, "Done.");
    }

    pub fn wait(&mut self) {
        let check_interval = 30000;

        let span = span!(Level::INFO, "SchedulersManager::wait()");
        let _guard = span.enter();
        let message_about_interval = format!(
            "Waiting for schedulers to finish. Checking state every {}ms.",
            check_interval
        );
        let msg_about_interval = message_about_interval.as_str();
        event!(Level::INFO, msg_about_interval);

        let mut rng = rand::thread_rng();

        loop {
            let cnt_running_handles = self.running_handles.len();
            if cnt_running_handles > 0 {
                let message = format!(
                    "{} schedulers are still running. Next check in {}ms.",
                    cnt_running_handles, check_interval
                );
                let msg = message.as_str();
                event!(Level::INFO, msg);

                thread::sleep(Duration::from_millis(check_interval));

                if rng.gen_ratio(1, 2) {
                    event!(
                        Level::INFO,
                        "A 1/2 chance has triggered a random 'finish' of all schedulers."
                    );
                    self.stop();
                }
            } else {
                event!(Level::INFO, "Done. No more schedulers running.");
                break;
            }
        }
    }
}
