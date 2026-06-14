//! Integration tests for [`TokioSchedulerConfigBuilder`].

use std::num::NonZeroUsize;

use swe_edge_runtime_scheduler::{
    Scheduler, SchedulerSvc, TokioSchedulerConfig, TokioSchedulerConfigBuilder,
};

/// @covers: TokioSchedulerConfigBuilder::new
#[test]
fn test_tokio_scheduler_config_builder_struct_new_produces_default_config() {
    let cfg = TokioSchedulerConfigBuilder::new().build();
    assert!(cfg.workers.is_none());
    assert!(cfg.thread_stack_kib.is_none());
    assert!(cfg.max_blocking_threads.is_none());
    assert!(cfg.thread_name.is_none());
}

/// @covers: TokioSchedulerConfigBuilder::workers
#[test]
fn test_tokio_scheduler_config_builder_struct_workers_sets_worker_count() {
    const FOUR: NonZeroUsize = match NonZeroUsize::new(4) {
        Some(n) => n,
        None => panic!("4 is nonzero"),
    };
    let cfg = TokioSchedulerConfigBuilder::new().workers(FOUR).build();
    assert_eq!(cfg.workers.map(|n| n.get()), Some(4));
}

/// @covers: TokioSchedulerConfigBuilder::thread_stack_kib
#[test]
fn test_tokio_scheduler_config_builder_struct_thread_stack_kib_sets_stack_size() {
    let cfg = TokioSchedulerConfigBuilder::new()
        .thread_stack_kib(256)
        .build();
    assert_eq!(cfg.thread_stack_kib, Some(256));
}

/// @covers: TokioSchedulerConfigBuilder::max_blocking_threads
#[test]
fn test_tokio_scheduler_config_builder_struct_max_blocking_threads_sets_pool() {
    let cfg = TokioSchedulerConfigBuilder::new()
        .max_blocking_threads(64)
        .build();
    assert_eq!(cfg.max_blocking_threads, Some(64));
}

/// @covers: TokioSchedulerConfigBuilder::thread_name
#[test]
fn test_tokio_scheduler_config_builder_struct_thread_name_sets_name() {
    let cfg = TokioSchedulerConfigBuilder::new()
        .thread_name("worker")
        .build();
    assert_eq!(cfg.thread_name.as_deref(), Some("worker"));
}

/// @covers: TokioSchedulerConfigBuilder::build
#[test]
fn test_tokio_scheduler_config_builder_struct_build_produces_working_scheduler() {
    let cfg: TokioSchedulerConfig = TokioSchedulerConfigBuilder::new().build();
    let s = SchedulerSvc::tokio_scheduler(cfg, "test");
    assert!(s.run(async { Ok(()) }).is_ok());
}
