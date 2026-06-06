//! Integration tests for [`SchedulerSvc`] — the primary facade type.

use swe_edge_runtime_scheduler::SchedulerSvc;

/// @covers: SchedulerSvc::create_config_builder
#[test]
fn test_scheduler_svc_struct_create_config_builder_produces_usable_builder() {
    let _b = SchedulerSvc::create_config_builder().build();
}

#[cfg(feature = "tokio-rt")]
mod tokio_tests {
    use swe_edge_runtime_scheduler::{SchedulerSvc, TokioSchedulerConfig};

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
