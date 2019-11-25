//! # rrs module
//! `rrs` is the main `rusty-reporting-service` crate module.

// Scheduler, and trait for .seconds(), .minutes(), etc.
use clokwerk::{Scheduler, TimeUnits, ScheduleHandle};
// Import week days and WeekDay
use clokwerk::Interval::*;

use std::thread;
use std::time::Duration;

use tracing::{info, span, Level};

/// Manager for `clokwerk` schedulers.
/// You can add custom `clokwerk` schedulers,
pub struct SchedulersManager {
    schedulers: Vec<Scheduler>,
    running_handles: Vec<ScheduleHandle>,
    trace_span: span,
}

impl SchedulersManager {
    pub fn new() -> Self {
        SchedulersManager {
            schedulers: vec![],
            running_handles: vec![],
            trace_span: span!(Level::INFO, "SchedulerManager"),
        }
    }

    /// Starts
    pub fn start(&mut self) -> () {
        for scheduler in self.schedulers {
            self.running_handles.push(scheduler.watch_thread(Duration::from_millis(100)));
        }
    }

    pub fn stop(&mut self) -> () {
        for handle in self.running_handles {
            handle.stop();
        }
        self.running_handles.clear();
    }
}
