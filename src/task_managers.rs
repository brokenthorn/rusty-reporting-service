//! # Task Managers
//!
//! This module contains reporting task managers.

use std::thread;
use std::time::Duration;

use clokwerk::{Interval, ScheduleHandle, Scheduler};
use tracing::{event, span};
use tracing_core::metadata::Level;

/// A simple task scheduler.
///
/// Internally, it uses a `clokwerk::Scheduler` that does the actual work.
///
/// There are two ways to start the clokwerk `Scheduler`:
/// 1. By calling `watch_thread`, which moves the scheduler to a separate background thread and
/// pools it periodically.
/// 2. By calling `start`, which does the same thing, but keeps the scheduler in this thread.
pub struct SimpleTaskScheduler {
    /// `clokwerk` job scheduler.
    scheduler: Scheduler,

    /// A handle for the thread where the `clokwerk` job scheduler is running in, available after
    /// `watch_thread` has been called.
    handle: Option<ScheduleHandle>,
}

impl Default for SimpleTaskScheduler {
    /// Creates a new `SimpleTaskScheduler` using default values.
    fn default() -> Self {
        SimpleTaskScheduler {
            scheduler: Scheduler::new(),
            handle: None,
        }
    }
}

impl SimpleTaskScheduler {
    /// Creates a new `SimpleTaskScheduler`.
    pub fn new() -> Self {
        let s = span!(Level::INFO, "new()");
        let _guard = s.enter();
        event!(Level::INFO, "Creating new Task Manager.");
        SimpleTaskScheduler::default()
    }

    /// Add a new task to the scheduler to be run on `every_interval`.
    ///
    /// # Examples
    /// 1. to run every day, use `clokwerk::Interval::Days(1)`.
    /// 2. to run every day using `clokwerk::TimeUnits` trait, use `1.day()`.
    ///
    /// For more intervals, check out [docs.rs/clokwerk](https://docs.rs/clokwerk/).
    pub fn add_task<F>(&mut self, every_interval: Interval, f: F)
    where
        F: 'static + FnMut() + Sync + Send,
    {
        let s = span!(Level::INFO, "add_task()");
        let _guard = s.enter();
        event!(
            Level::INFO,
            "Adding task with interval {:?}.",
            every_interval
        );
        let _job = &mut self.scheduler.every(every_interval).run(f);
    }

    /// Start the clokwerk scheduler by using a background thread to call `Scheduler::run_pending()`
    /// with the specified frequency.
    ///
    /// The thread handle is stored in `Self::handle`. This function does not block.
    ///
    /// __WARNING__: If the thread handle goes out of scope, the background thread is terminated
    /// successfully.
    pub fn watch_thread(mut self, frequency: Duration) {
        let s = span!(Level::INFO, "watch_thread()");
        let _guard = s.enter();

        if let Some(_) = &self.handle {
            event!(
                Level::WARN,
                "Scheduler already started in background thread once before!"
            );
        } else {
            // `handle` is a thread handle that can be used to stop the thread.
            self.handle = Some(self.scheduler.watch_thread(frequency));
            event!(Level::INFO, "Scheduler started in background thread.");
        }
    }

    /// Halts the clokwerk scheduler background thread.
    pub fn stop_watch_thread(self) {
        if let Some(h) = self.handle {
            h.stop();
        }
    }

    /// Start the clokwerk scheduler by calling `Scheduler::run_pending()` with the specified
    /// frequency.
    ///
    /// This function blocks indefinitely.
    pub fn start(&mut self, interval: Duration) {
        let s = span!(Level::INFO, "start()");
        let _guard = s.enter();
        event!(Level::INFO, "Starting scheduler in the current thread.");
        loop {
            &mut self.scheduler.run_pending();
            thread::sleep(interval);
        }
    }
}
