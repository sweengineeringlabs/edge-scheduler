//! SAF — scheduler factory methods on [`SchedulerSvc`].

#[cfg(feature = "tokio-rt")]
use crate::api::traits::Validator;
use crate::api::types::ApplicationConfigBuilder;
use crate::api::types::SchedulerSvc;
#[cfg(feature = "tokio-rt")]
use crate::spi::tokio::tokio_scheduler_config::TokioSchedulerConfig;
#[cfg(feature = "tokio-rt")]
use crate::spi::tokio::TokioScheduler;

impl SchedulerSvc {
    /// Return an [`ApplicationConfigBuilder`] pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> ApplicationConfigBuilder {
        ApplicationConfigBuilder::new()
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
    /// Requires the `tokio-rt` feature (enabled by default).
    #[cfg(feature = "tokio-rt")]
    pub fn tokio_scheduler(
        config: TokioSchedulerConfig,
        thread_name: impl Into<String>,
    ) -> TokioScheduler {
        TokioScheduler::new(config, thread_name)
    }
}
