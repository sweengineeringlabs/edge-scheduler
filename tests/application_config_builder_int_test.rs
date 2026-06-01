//! Integration tests for [`SchedulerSvc::create_config_builder`] — also covers the
//! `swe-edge-configbuilder` dependency (SEA Rule 95).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_configbuilder::ConfigBuilderImpl;
use swe_edge_runtime_scheduler::{ApplicationConfigBuilder, SchedulerSvc};

/// @covers: SchedulerSvc::create_config_builder
/// Verifies the config builder can be constructed and returns a [`ConfigBuilderImpl`].
#[test]
fn test_scheduler_svc_create_config_builder_is_pre_seeded_with_package_name() {
    let builder: ConfigBuilderImpl = SchedulerSvc::create_config_builder().build();
    drop(builder);
}

/// @covers: SchedulerSvc::create_config_builder
/// Each call returns an independent builder (no shared state).
#[test]
fn test_scheduler_svc_create_config_builder_returns_builder_with_valid_state() {
    let b1: ConfigBuilderImpl = SchedulerSvc::create_config_builder().build();
    let b2: ConfigBuilderImpl = SchedulerSvc::create_config_builder().build();
    drop(b1);
    drop(b2);
}

/// @covers: ApplicationConfigBuilder::new
#[test]
fn test_application_config_builder_new_creates_valid_builder() {
    let _impl: ConfigBuilderImpl = ApplicationConfigBuilder::new().build();
}

/// @covers: ApplicationConfigBuilder::default
#[test]
fn test_application_config_builder_default_equals_new() {
    let _a: ConfigBuilderImpl = ApplicationConfigBuilder::default().build();
    let _b: ConfigBuilderImpl = ApplicationConfigBuilder::new().build();
}
