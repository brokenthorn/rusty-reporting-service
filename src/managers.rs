//! # Job Managers
//! This module contains job managers.
//!
//! Currently, only a simple manager for a job scheduler is available.

use std::thread;
use std::time::Duration;

use clokwerk::{Interval, ScheduleHandle, Scheduler};
use tracing::{event, span};
use tracing_core::metadata::Level;

/// Log span name used in this module, in conjunction with facilities from the `tracing` crate.
pub const LOG_SPAN_NAME: &'static str = "MANAGER";

/// A simple manager for a job scheduler.
///
/// Internally, it contains a scheduler.
/// You can add jobs to this scheduler, to be run at specified intervals.
///
/// After adding jobs, there are two ways to start scheduling the jobs to be executed.
/// One is to call `watch_thread`, which moves the scheduler to a separate background thread and
/// pools it periodically.
/// The other way is to call `start`, which does the same thing, but keeps the scheduler in the main
/// thread.
pub struct SimpleManager {
    /// Job scheduler.
    scheduler: Option<Scheduler>,

    /// Is `Some(ScheduleHandle)` if `Self::scheduler` has not been consumed yet.
    /// Is `None` if `Self::watch_thread` has been called.
    handle: Option<ScheduleHandle>,
}

impl Default for SimpleManager {
    fn default() -> Self {
        SimpleManager {
            scheduler: Some(Scheduler::new()),
            handle: None,
        }
    }
}

impl SimpleManager {
    /// Create a new Manager.
    pub fn new() -> Self {
        let s = span!(Level::INFO, LOG_SPAN_NAME);
        let _guard = s.enter();
        event!(Level::INFO, msg = "Creating new Manager.");
        SimpleManager::default()
    }

    /// Add a new job to the scheduler to be run on every given interval.
    /// For example to run every day use `Interval::Days(1)` or `1.day()` (if using `TimeUnits` trait).
    pub fn add_task<F>(&mut self, every_interval: Interval, f: F)
    where
        F: 'static + FnMut() + Sync + Send,
    {
        let s = span!(Level::INFO, LOG_SPAN_NAME);
        let _guard = s.enter();
        event!(Level::INFO, msg = "Adding task.");
        if let Some(scheduler) = &mut self.scheduler {
            let _job = scheduler.every(every_interval).run(f);
        } else {
            event!(
                Level::ERROR,
                msg = "The task cannot be scheduled because the scheduler has been started."
            );
        }
    }

    /// Start the scheduler by using a background thread to call `Scheduler::run_pending()`
    /// with the specified frequency.
    ///
    /// The thread handle is stored in `Self::handle`. This function does not block. If the thread
    /// handle goes out of scope, the background thread is terminated successfully.
    pub fn watch_thread(mut self, frequency: Duration) {
        let s = span!(Level::INFO, LOG_SPAN_NAME);
        let _guard = s.enter();
        event!(
            Level::INFO,
            msg = "Moving scheduler to a background thread to watch."
        );
        if let Some(_) = &self.handle {
            event!(
                Level::WARN,
                msg = "Scheduler is already being watched in a background thread!"
            );
        } else {
            // `handle` is a thread handle that can be used to stop the background watcher thread.
            self.handle = Some(
                self.scheduler
                    .expect("`self.scheduler` should have not been None!")
                    .watch_thread(frequency),
            );
            event!(
                Level::INFO,
                msg = "Scheduler is now being watched in a background thread."
            );
        }
    }

    /// Start the scheduler by calling `Scheduler::run_pending()` with the specified frequency.
    pub fn start(&mut self, interval: Duration) {
        let s = span!(Level::INFO, LOG_SPAN_NAME);
        let _guard = s.enter();
        event!(
            Level::INFO,
            msg = "Starting scheduler in the current thread."
        );
        if let Some(scheduler) = &mut self.scheduler {
            loop {
                scheduler.run_pending();
                thread::sleep(interval);
            }
        } else {
            event!(
                Level::ERROR,
                msg = "The scheduler cannot be started because it has been consumed."
            );
        }
    }
}
