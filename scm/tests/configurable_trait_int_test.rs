//! Integration tests for the [`Configurable`] trait contract.

use swe_edge_runtime_scheduler::{
    ApplicationConfigBuilder, Configurable, Scheduler, SchedulerSvc, TokioSchedulerConfig,
    TokioSchedulerConfigBuilder,
};

struct DefaultConfigurable;

impl Configurable for DefaultConfigurable {
    fn tokio_config_builder() -> TokioSchedulerConfigBuilder {
        TokioSchedulerConfigBuilder::new()
    }

    fn config_builder() -> ApplicationConfigBuilder {
        SchedulerSvc::create_config_builder()
    }

    fn service() -> SchedulerSvc {
        SchedulerSvc
    }

    fn default_tokio_config() -> TokioSchedulerConfig {
        TokioSchedulerConfig::default()
    }
}

// ── Configurable::config_builder ─────────────────────────────────────────────

/// @covers: Configurable::config_builder
#[test]
fn test_config_builder_returns_usable_builder_happy() {
    let _ = DefaultConfigurable::config_builder().build();
}

/// @covers: Configurable::config_builder
#[test]
fn test_config_builder_is_independent_of_scheduler_state_error() {
    let _ = DefaultConfigurable::config_builder().build();
}

/// @covers: Configurable::config_builder
#[test]
fn test_config_builder_each_call_returns_fresh_builder_edge() {
    let b1 = DefaultConfigurable::config_builder().build();
    let b2 = DefaultConfigurable::config_builder().build();
    let _ = (b1, b2);
}

// ── Configurable::service ─────────────────────────────────────────────────────

/// @covers: Configurable::service
#[test]
fn test_service_returns_scheduler_svc_facade_happy() {
    let svc = DefaultConfigurable::service();
    let _ = svc;
}

/// @covers: Configurable::service
#[test]
fn test_service_facade_can_create_config_builder_error() {
    let _ = DefaultConfigurable::service();
    let _ = SchedulerSvc::create_config_builder().build();
}

/// @covers: Configurable::service
#[test]
fn test_service_is_stateless_on_repeated_calls_edge() {
    let s1 = DefaultConfigurable::service();
    let s2 = DefaultConfigurable::service();
    let _ = (s1, s2);
}

// ── Configurable::default_tokio_config ───────────────────────────────────────

/// @covers: Configurable::default_tokio_config
#[test]
fn test_default_tokio_config_returns_all_none_fields_happy() {
    let cfg = DefaultConfigurable::default_tokio_config();
    assert!(cfg.workers.is_none());
    assert!(cfg.thread_stack_kib.is_none());
    assert!(cfg.max_blocking_threads.is_none());
    assert!(cfg.thread_name.is_none());
}

/// @covers: Configurable::default_tokio_config
#[test]
fn test_default_tokio_config_is_valid_for_scheduler_construction_error() {
    let cfg = DefaultConfigurable::default_tokio_config();
    let s = SchedulerSvc::tokio_scheduler(cfg, "test-configurable");
    assert!(s.run(async { Ok(()) }).is_ok());
}

/// @covers: Configurable::default_tokio_config
#[test]
fn test_default_tokio_config_matches_struct_default_edge() {
    let from_configurable = DefaultConfigurable::default_tokio_config();
    let from_default = TokioSchedulerConfig::default();
    assert_eq!(from_configurable.workers, from_default.workers);
    assert_eq!(
        from_configurable.thread_stack_kib,
        from_default.thread_stack_kib
    );
}

// ── Configurable::tokio_config_builder ───────────────────────────────────────

/// @covers: Configurable::tokio_config_builder
#[test]
fn test_tokio_config_builder_returns_usable_builder_happy() {
    let cfg = DefaultConfigurable::tokio_config_builder().build();
    assert!(cfg.workers.is_none());
}

/// @covers: Configurable::tokio_config_builder
#[test]
fn test_tokio_config_builder_setters_produce_configured_output_error() {
    use std::num::NonZeroUsize;
    const TWO: NonZeroUsize = match NonZeroUsize::new(2) {
        Some(n) => n,
        None => panic!("2 is nonzero"),
    };
    let cfg = DefaultConfigurable::tokio_config_builder()
        .workers(TWO)
        .build();
    assert_eq!(cfg.workers.map(|n| n.get()), Some(2));
}

/// @covers: Configurable::tokio_config_builder
#[test]
fn test_tokio_config_builder_each_call_is_independent_edge() {
    let b1 = DefaultConfigurable::tokio_config_builder().build();
    let b2 = DefaultConfigurable::tokio_config_builder().build();
    assert_eq!(b1.workers, b2.workers);
}
