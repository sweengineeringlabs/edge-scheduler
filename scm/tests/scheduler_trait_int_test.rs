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

// ── Scheduler::run ────────────────────────────────────────────────────────────

/// @covers: Scheduler::run
#[test]
fn test_run_drives_ok_future_to_completion_happy() {
    let s = OkScheduler;
    assert!(s.run(async { Ok(()) }).is_ok());
}

/// @covers: Scheduler::run
#[test]
fn test_run_returns_err_when_future_yields_error_error() {
    let s = OkScheduler;
    let result = s.run(async { Err(SchedulerError::StartFailed("x".into())) });
    assert!(result.is_err());
}

/// @covers: Scheduler::run
#[test]
fn test_run_accepts_immediately_resolving_future_edge() {
    let s = OkScheduler;
    assert!(s.run(std::future::ready(Ok(()))).is_ok());
}
