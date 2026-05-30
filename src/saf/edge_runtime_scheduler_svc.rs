//! SAF — scheduler public factory surface.

use swe_edge_runtime::{RuntimeBuilder, RuntimeResult};

use crate::api::runtime_builder_ext::RuntimeBuilderExt as _;
use crate::api::scheduler::Scheduler;
#[cfg(feature = "tokio-rt")]
use crate::api::traits::Validator;
#[cfg(feature = "tokio-rt")]
use crate::api::scheduler::tokio_scheduler_config::TokioSchedulerConfig;
#[cfg(feature = "tokio-rt")]
use crate::core::scheduler::TokioScheduler;

/// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
    swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// Drive the runtime with a custom [`Scheduler`] implementation.
pub fn run_with_scheduler<S: Scheduler>(
    builder: RuntimeBuilder,
    scheduler: S,
) -> RuntimeResult<()> {
    builder.run_with_scheduler(scheduler)
}

/// Drive the runtime with the tokio scheduler and default config.
///
/// Requires the `tokio-rt` feature (enabled by default).
#[cfg(feature = "tokio-rt")]
pub fn run(builder: RuntimeBuilder) -> RuntimeResult<()> {
    builder.run()
}

/// Drive the runtime with the tokio scheduler and the supplied config.
///
/// Requires the `tokio-rt` feature (enabled by default).
#[cfg(feature = "tokio-rt")]
pub fn run_with_config(builder: RuntimeBuilder, config: TokioSchedulerConfig) -> RuntimeResult<()> {
    builder.run_with_config(config)
}

/// Validate a value that implements [`Validator`].
#[cfg(feature = "tokio-rt")]
pub fn validate<V: Validator>(v: &V) -> Result<(), String> {
    v.validate()
}

/// Construct a tokio-backed scheduler with the given config and thread name prefix.
///
/// Pass the result to [`run_with_scheduler`].
#[cfg(feature = "tokio-rt")]
pub fn tokio_scheduler(
    config: TokioSchedulerConfig,
    thread_name: impl Into<String>,
) -> impl Scheduler {
    TokioScheduler::new(config, thread_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use swe_edge_runtime::{Runtime, RuntimeError};

    /// @covers: run_with_scheduler
    #[test]
    fn test_run_with_scheduler_returns_start_failed_for_empty_builder() {
        use std::future::Future;
        struct NoopScheduler;
        impl Scheduler for NoopScheduler {
            fn run<F>(&self, fut: F) -> RuntimeResult<()>
            where
                F: Future<Output = RuntimeResult<()>> + Send + 'static,
            {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(fut)
            }
        }
        let result = run_with_scheduler(Runtime::builder(), NoopScheduler);
        assert!(matches!(result, Err(RuntimeError::StartFailed(_))));
    }

    /// @covers: run
    #[cfg(feature = "tokio-rt")]
    #[test]
    fn test_run_returns_start_failed_for_empty_builder() {
        assert!(matches!(run(Runtime::builder()), Err(RuntimeError::StartFailed(_))));
    }

    /// @covers: run_with_config
    #[cfg(feature = "tokio-rt")]
    #[test]
    fn test_run_with_config_returns_start_failed_for_empty_builder() {
        let result = run_with_config(Runtime::builder(), TokioSchedulerConfig::default());
        assert!(matches!(result, Err(RuntimeError::StartFailed(_))));
    }

    /// @covers: validate
    #[cfg(feature = "tokio-rt")]
    #[test]
    fn test_validate_returns_ok_for_valid_config() {
        assert!(validate(&TokioSchedulerConfig::default()).is_ok());
    }

    /// @covers: tokio_scheduler
    #[cfg(feature = "tokio-rt")]
    #[test]
    fn test_tokio_scheduler_factory_produces_working_scheduler() {
        let s = tokio_scheduler(TokioSchedulerConfig::default(), "test");
        assert!(s.run(async { Ok(()) }).is_ok());
    }
}
