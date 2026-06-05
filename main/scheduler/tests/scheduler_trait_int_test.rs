//! Integration tests for the [`Scheduler`] trait contract.

use std::future::Future;

use swe_edge_runtime_scheduler::{Scheduler, SchedulerError};

struct OkScheduler;

impl Scheduler for OkScheduler {
    fn run<F>(&self, fut: F) -> Result<(), SchedulerError>
    where
        F: Future<Output = Result<(), SchedulerError>> + Send + 'static,
    {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| SchedulerError::StartFailed(format!("{e}")))?
            .block_on(fut)
    }
}

/// @covers: Scheduler::run
#[test]
fn test_scheduler_trait_run_succeeds_with_ok_future() {
    let s = OkScheduler;
    assert!(s.run(async { Ok(()) }).is_ok());
}

/// @covers: Scheduler::run
#[test]
fn test_scheduler_trait_run_propagates_error_from_future() {
    let s = OkScheduler;
    let result = s.run(async { Err(SchedulerError::StartFailed("x".into())) });
    assert!(result.is_err());
}
