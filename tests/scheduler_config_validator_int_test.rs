//! Integration tests for the scheduler configuration validator.

use swe_edge_runtime_scheduler::{SchedulerSvc, TokioSchedulerConfig};

/// @covers: SchedulerSvc::validate
#[test]
fn test_scheduler_config_validator_default_config_is_valid() {
    assert!(SchedulerSvc::validate(&TokioSchedulerConfig::default()).is_ok());
}

/// @covers: SchedulerSvc::validate
#[test]
fn test_scheduler_config_validator_stack_below_64kib_is_invalid() {
    let cfg = TokioSchedulerConfig {
        thread_stack_kib: Some(32),
        ..Default::default()
    };
    let err = SchedulerSvc::validate(&cfg).unwrap_err();
    assert!(err.contains("64"), "expected 64 KiB threshold in error: {err}");
}

/// @covers: SchedulerSvc::validate
#[test]
fn test_scheduler_config_validator_stack_at_64kib_is_valid() {
    let cfg = TokioSchedulerConfig {
        thread_stack_kib: Some(64),
        ..Default::default()
    };
    assert!(SchedulerSvc::validate(&cfg).is_ok());
}

/// @covers: SchedulerSvc::validate
#[test]
fn test_scheduler_config_validator_stack_above_64kib_is_valid() {
    let cfg = TokioSchedulerConfig {
        thread_stack_kib: Some(1024),
        ..Default::default()
    };
    assert!(SchedulerSvc::validate(&cfg).is_ok());
}
