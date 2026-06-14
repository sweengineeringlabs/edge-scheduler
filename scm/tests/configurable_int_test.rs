//! Integration tests for the [`Configurable`] trait — concrete scheduler coverage.

use swe_edge_runtime_scheduler::{
    Configurable, Scheduler, SchedulerSvc, TokioSchedulerConfig, TokioSchedulerConfigBuilder,
};

struct AlwaysDefault;

impl Configurable for AlwaysDefault {
    fn config_builder() -> swe_edge_runtime_scheduler::ApplicationConfigBuilder {
        SchedulerSvc::create_config_builder()
    }

    fn service() -> SchedulerSvc {
        SchedulerSvc
    }

    fn default_tokio_config() -> TokioSchedulerConfig {
        TokioSchedulerConfig::default()
    }

    fn tokio_config_builder() -> TokioSchedulerConfigBuilder {
        TokioSchedulerConfigBuilder::new()
    }
}

/// @covers: Configurable::default_tokio_config
#[test]
fn test_configurable_trait_default_tokio_config_produces_working_scheduler() {
    let cfg = AlwaysDefault::default_tokio_config();
    let s = SchedulerSvc::tokio_scheduler(cfg, "test-configurable-int");
    assert!(s.run(async { Ok(()) }).is_ok());
}

/// @covers: Configurable::config_builder
#[test]
fn test_configurable_trait_config_builder_produces_valid_config() {
    let _ = AlwaysDefault::config_builder().build();
}

/// @covers: Configurable::service
#[test]
fn test_configurable_trait_service_returns_scheduler_svc() {
    let svc = AlwaysDefault::service();
    let _ = svc;
}

/// @covers: Configurable::tokio_config_builder
#[test]
fn test_configurable_trait_tokio_config_builder_builds_default_config() {
    let cfg = AlwaysDefault::tokio_config_builder().build();
    assert!(cfg.workers.is_none());
}
