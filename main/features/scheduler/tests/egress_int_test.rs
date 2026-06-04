//! Integration tests for the scheduler egress SPI extension point.
//!
//! The egress SPI provides hooks for downstream consumers to plug in custom
//! async executor backends via the [`Scheduler`] trait.

// @allow: no_mocks_in_integration — StubScheduler is a real minimal Scheduler implementation
// used to prove the SPI extension pattern works end-to-end; it is not a mock framework type.

use std::future::Future;

use swe_edge_runtime::{Runtime, RuntimeError, RuntimeResult};
use swe_edge_runtime_scheduler::{Scheduler, SchedulerSvc};

/// A minimal [`Scheduler`] implementation used to verify the SPI extension pattern.
struct StubScheduler;

impl Scheduler for StubScheduler {
    fn run<F>(&self, fut: F) -> RuntimeResult<()>
    where
        F: Future<Output = RuntimeResult<()>> + Send + 'static,
    {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| RuntimeError::StartFailed(e.to_string()))?
            .block_on(fut)
    }
}

/// @covers: spi/egress — downstream extension via custom Scheduler
#[test]
fn test_egress_spi_custom_scheduler_runs_future_successfully() {
    // Prove that a downstream-provided Scheduler impl is accepted by the SAF.
    let result = StubScheduler.run(async { Ok(()) });
    assert!(result.is_ok());
}

/// @covers: spi/egress — custom Scheduler passed to SchedulerSvc
#[test]
fn test_egress_spi_scheduler_svc_accepts_custom_impl() {
    let result = SchedulerSvc::run_with_scheduler(Runtime::builder(), StubScheduler);
    assert!(matches!(result, Err(RuntimeError::StartFailed(_))));
}
