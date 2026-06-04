//! Integration tests for the [`Scheduler`] trait contract.

use std::future::Future;

use swe_edge_runtime::{RuntimeError, RuntimeResult};
use swe_edge_runtime_scheduler::Scheduler;

struct OkScheduler;

impl Scheduler for OkScheduler {
    fn run<F>(&self, fut: F) -> RuntimeResult<()>
    where
        F: Future<Output = RuntimeResult<()>> + Send + 'static,
    {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| RuntimeError::StartFailed(format!("{e}")))?
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
    let result = s.run(async { Err(RuntimeError::StartFailed("x".into())) });
    assert!(result.is_err());
}
