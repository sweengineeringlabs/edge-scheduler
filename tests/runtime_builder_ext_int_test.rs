//! Integration tests for the [`RuntimeBuilderExt`] trait contract.

use swe_edge_runtime::{Runtime, RuntimeError};
use swe_edge_runtime_scheduler::RuntimeBuilderExt;

/// @covers: RuntimeBuilderExt::run
#[cfg(feature = "tokio-rt")]
#[test]
fn test_runtime_builder_ext_trait_run_returns_start_failed_for_empty_builder() {
    assert!(matches!(Runtime::builder().run(), Err(RuntimeError::StartFailed(_))));
}

/// @covers: RuntimeBuilderExt::run_with_config
#[cfg(feature = "tokio-rt")]
#[test]
fn test_runtime_builder_ext_trait_run_with_config_returns_start_failed_for_empty_builder() {
    use swe_edge_runtime_scheduler::TokioSchedulerConfig;
    let result = Runtime::builder().run_with_config(TokioSchedulerConfig::default());
    assert!(matches!(result, Err(RuntimeError::StartFailed(_))));
}
