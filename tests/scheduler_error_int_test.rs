//! Integration tests for [`SchedulerError`].

use swe_edge_runtime_scheduler::SchedulerError;

/// @covers: SchedulerError::StartFailed
#[test]
fn test_scheduler_error_start_failed_displays_message() {
    let err = SchedulerError::StartFailed("runtime init failed".into());
    let msg = err.to_string();
    assert!(msg.contains("runtime init failed"), "got: {msg}");
}

/// @covers: SchedulerError::InvalidConfig
#[test]
fn test_scheduler_error_invalid_config_displays_message() {
    let err = SchedulerError::InvalidConfig("stack too small".into());
    let msg = err.to_string();
    assert!(msg.contains("stack too small"), "got: {msg}");
}

/// @covers: SchedulerError
#[test]
fn test_scheduler_error_variants_are_distinct() {
    let a = SchedulerError::StartFailed("x".into());
    let b = SchedulerError::InvalidConfig("x".into());
    assert_ne!(a, b);
}

/// @covers: SchedulerError
#[test]
fn test_scheduler_error_implements_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(SchedulerError::StartFailed("e".into()));
    assert!(err.to_string().contains("e"));
}
