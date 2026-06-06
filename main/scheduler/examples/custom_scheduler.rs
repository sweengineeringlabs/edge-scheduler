//! Example: bring your own scheduler implementation.
//!
//! Run with: `cargo run --example custom_scheduler`

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::future::Future;

use swe_edge_runtime_scheduler::{Scheduler, SchedulerError};

/// A scheduler backed by a single-threaded tokio runtime.
struct SingleThreadScheduler;

impl Scheduler for SingleThreadScheduler {
    fn run<F>(&self, fut: F) -> Result<(), SchedulerError>
    where
        F: Future<Output = Result<(), SchedulerError>> + Send + 'static,
    {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| SchedulerError::StartFailed(format!("scheduler: {e}")))?
            .block_on(fut)
    }
}

fn main() {
    let scheduler = SingleThreadScheduler;
    let result = scheduler.run(async {
        println!("running on a custom single-thread scheduler");
        Ok(())
    });
    eprintln!("result: {result:?}");
}
