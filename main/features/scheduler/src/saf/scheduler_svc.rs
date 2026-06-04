//! SAF — scheduler factory methods on [`SchedulerSvc`].

use swe_edge_runtime::{RuntimeBuilder, RuntimeResult};

use crate::api::runtime::RuntimeBuilderExt as _;
#[cfg(feature = "tokio-rt")]
use crate::api::scheduler::tokio_scheduler_config::TokioSchedulerConfig;
use crate::api::scheduler::ApplicationConfigBuilder;
use crate::api::scheduler::Scheduler;
#[cfg(feature = "tokio-rt")]
use crate::api::traits::Validator;
use crate::api::types::SchedulerSvc;
#[cfg(feature = "tokio-rt")]
use crate::api::types::TokioScheduler;

impl SchedulerSvc {
    /// Return an [`ApplicationConfigBuilder`] pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> ApplicationConfigBuilder {
        ApplicationConfigBuilder::new()
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
    pub fn run_with_config(
        builder: RuntimeBuilder,
        config: TokioSchedulerConfig,
    ) -> RuntimeResult<()> {
        builder.run_with_config(config)
    }

    /// Validate a value that implements [`Validator`].
    ///
    /// Requires the `tokio-rt` feature (enabled by default).
    #[cfg(feature = "tokio-rt")]
    pub fn validate<V: Validator>(v: &V) -> Result<(), String> {
        v.validate()
    }

    /// Construct a tokio-backed [`TokioScheduler`] with the given config and thread name prefix.
    ///
    /// Pass the result to [`SchedulerSvc::run_with_scheduler`].
    ///
    /// Requires the `tokio-rt` feature (enabled by default).
    #[cfg(feature = "tokio-rt")]
    pub fn tokio_scheduler(
        config: TokioSchedulerConfig,
        thread_name: impl Into<String>,
    ) -> TokioScheduler {
        TokioScheduler::new(config, thread_name)
    }
}
