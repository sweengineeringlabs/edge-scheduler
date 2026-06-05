//! Integration tests for [`TokioSchedulerConfig`].

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::num::NonZeroUsize;

use swe_edge_runtime_scheduler::TokioSchedulerConfig;

/// @covers: TokioSchedulerConfig::default
#[test]
fn test_tokio_scheduler_config_struct_default_has_all_fields_none() {
    let cfg = TokioSchedulerConfig::default();
    assert!(cfg.workers.is_none());
    assert!(cfg.thread_stack_kib.is_none());
    assert!(cfg.max_blocking_threads.is_none());
    assert!(cfg.thread_name.is_none());
}

/// @covers: TokioSchedulerConfig
#[test]
fn test_tokio_scheduler_config_struct_roundtrips_through_toml() {
    let cfg = TokioSchedulerConfig {
        workers: NonZeroUsize::new(4),
        thread_name: Some("svc".into()),
        ..Default::default()
    };
    let s = toml::to_string(&cfg).expect("serialize");
    let back: TokioSchedulerConfig = toml::from_str(&s).expect("deserialize");
    assert_eq!(back.workers, cfg.workers);
    assert_eq!(back.thread_name.as_deref(), Some("svc"));
}

/// @covers: TokioSchedulerConfig
#[test]
fn test_tokio_scheduler_config_struct_deserializes_from_empty_toml() {
    let cfg: TokioSchedulerConfig = toml::from_str("").expect("empty toml");
    assert!(cfg.workers.is_none());
}
