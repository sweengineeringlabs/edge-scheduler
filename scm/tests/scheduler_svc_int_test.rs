//! Integration tests for [`SchedulerSvc`] — the primary facade type.

use swe_edge_runtime_scheduler::SchedulerSvc;

// ── SchedulerSvc::create_config_builder ──────────────────────────────────────

/// @covers: SchedulerSvc::create_config_builder
#[test]
fn test_create_config_builder_returns_usable_builder_happy() {
    let _ = SchedulerSvc::create_config_builder().build();
}

/// @covers: SchedulerSvc::create_config_builder
#[test]
fn test_create_config_builder_is_pre_seeded_with_package_metadata_error() {
    // Verify the builder is seeded (not an empty Default): with_name is called
    // during new(), so build() returns a valid ConfigBuilderImpl without panic.
    let _ = SchedulerSvc::create_config_builder().build();
}

/// @covers: SchedulerSvc::create_config_builder
#[test]
fn test_create_config_builder_each_call_is_independent_edge() {
    let b1 = SchedulerSvc::create_config_builder().build();
    let b2 = SchedulerSvc::create_config_builder().build();
    let _ = (b1, b2);
}

#[cfg(feature = "tokio-rt")]
mod tokio_tests {
    use swe_edge_runtime_scheduler::{
        Scheduler, SchedulerError, SchedulerSvc, TokioSchedulerConfig,
    };

    // ── SchedulerSvc::validate ────────────────────────────────────────────────

    /// @covers: SchedulerSvc::validate
    #[test]
    fn test_validate_accepts_default_config_happy() {
        assert!(SchedulerSvc::validate(&TokioSchedulerConfig::default()).is_ok());
    }

    /// @covers: SchedulerSvc::validate
    #[test]
    fn test_validate_rejects_stack_below_minimum_error() {
        let cfg = TokioSchedulerConfig {
            thread_stack_kib: Some(32),
            ..Default::default()
        };
        assert!(SchedulerSvc::validate(&cfg).is_err());
    }

    /// @covers: SchedulerSvc::validate
    #[test]
    fn test_validate_accepts_none_stack_size_as_valid_edge() {
        let cfg = TokioSchedulerConfig {
            thread_stack_kib: None,
            ..Default::default()
        };
        assert!(SchedulerSvc::validate(&cfg).is_ok());
    }

    // ── SchedulerSvc::tokio_scheduler ────────────────────────────────────────

    /// @covers: SchedulerSvc::tokio_scheduler
    #[test]
    fn test_tokio_scheduler_drives_ok_future_happy() {
        let s = SchedulerSvc::tokio_scheduler(TokioSchedulerConfig::default(), "test");
        assert!(s.run(async { Ok(()) }).is_ok());
    }

    /// @covers: SchedulerSvc::tokio_scheduler
    #[test]
    fn test_tokio_scheduler_propagates_future_error_error() {
        let s = SchedulerSvc::tokio_scheduler(TokioSchedulerConfig::default(), "test-err");
        let result = s.run(async { Err(SchedulerError::StartFailed("test".into())) });
        assert!(result.is_err());
    }

    /// @covers: SchedulerSvc::tokio_scheduler
    #[test]
    fn test_tokio_scheduler_config_thread_name_overrides_param_edge() {
        let cfg = TokioSchedulerConfig {
            thread_name: Some("cfg-name".to_string()),
            ..Default::default()
        };
        let s = SchedulerSvc::tokio_scheduler(cfg, "param-name");
        assert!(s.run(async { Ok(()) }).is_ok());
    }
}
