//! Integration tests for [`SchedulerSvc`] — the primary facade type.

use swe_edge_runtime::{Runtime, RuntimeError, RuntimeResult};
use swe_edge_runtime_scheduler::{Scheduler, SchedulerSvc};

use std::future::Future;

struct InfallibleScheduler;

impl Scheduler for InfallibleScheduler {
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

/// @covers: SchedulerSvc::run_with_scheduler
#[test]
fn test_scheduler_svc_struct_run_with_scheduler_returns_err_for_empty_builder() {
    let result = SchedulerSvc::run_with_scheduler(Runtime::builder(), InfallibleScheduler);
    assert!(matches!(result, Err(RuntimeError::StartFailed(_))));
}

/// @covers: SchedulerSvc::create_config_builder
#[test]
fn test_scheduler_svc_struct_create_config_builder_produces_usable_builder() {
    let _b = SchedulerSvc::create_config_builder().build();
}

#[cfg(feature = "tokio-rt")]
mod tokio_tests {
    use swe_edge_runtime::{Runtime, RuntimeError};
    use swe_edge_runtime_scheduler::{SchedulerSvc, TokioSchedulerConfig};

    /// @covers: SchedulerSvc::run
    #[test]
    fn test_scheduler_svc_struct_run_returns_err_for_empty_builder() {
        let result = SchedulerSvc::run(Runtime::builder());
        assert!(matches!(result, Err(RuntimeError::StartFailed(_))));
    }

    /// @covers: SchedulerSvc::run_with_config
    #[test]
    fn test_scheduler_svc_struct_run_with_config_returns_err_for_empty_builder() {
        let result =
            SchedulerSvc::run_with_config(Runtime::builder(), TokioSchedulerConfig::default());
        assert!(matches!(result, Err(RuntimeError::StartFailed(_))));
    }

    /// @covers: SchedulerSvc::validate
    #[test]
    fn test_scheduler_svc_struct_validate_returns_ok_for_default_config() {
        assert!(SchedulerSvc::validate(&TokioSchedulerConfig::default()).is_ok());
    }

    /// @covers: SchedulerSvc::tokio_scheduler
    #[test]
    fn test_scheduler_svc_struct_tokio_scheduler_produces_working_scheduler() {
        use swe_edge_runtime_scheduler::Scheduler;
        let s = SchedulerSvc::tokio_scheduler(TokioSchedulerConfig::default(), "test");
        assert!(s.run(async { Ok(()) }).is_ok());
    }
}
