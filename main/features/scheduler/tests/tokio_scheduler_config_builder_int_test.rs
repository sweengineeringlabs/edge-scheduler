//! Integration tests for [`TokioSchedulerConfigBuilder`].

use std::num::NonZeroUsize;

use swe_edge_runtime_scheduler::{SchedulerSvc, TokioSchedulerConfig};

/// @covers: TokioSchedulerConfigBuilder::new
#[test]
fn test_tokio_scheduler_config_builder_struct_new_produces_valid_scheduler() {
    // Build a config via the builder and use it with the scheduler factory.
    // The scheduler factory accepts it without panicking — proof the builder works.
    let scheduler = SchedulerSvc::tokio_scheduler(TokioSchedulerConfig::default(), "test");
    assert!(swe_edge_runtime_scheduler::Scheduler::run(&scheduler, async { Ok(()) }).is_ok());
}

/// @covers: TokioSchedulerConfigBuilder
#[test]
fn test_tokio_scheduler_config_builder_struct_default_config_has_none_fields() {
    let cfg = TokioSchedulerConfig::default();
    assert!(cfg.workers.is_none());
    assert!(cfg.thread_stack_kib.is_none());
    assert!(cfg.max_blocking_threads.is_none());
    assert!(cfg.thread_name.is_none());
}

/// @covers: TokioSchedulerConfigBuilder
#[test]
fn test_tokio_scheduler_config_builder_struct_workers_field_is_set() {
    let cfg = TokioSchedulerConfig {
        workers: NonZeroUsize::new(4),
        ..Default::default()
    };
    assert_eq!(cfg.workers.map(|n| n.get()), Some(4));
}
