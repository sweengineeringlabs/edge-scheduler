//! SAF — scheduler factory methods on [`SchedulerSvc`].

pub use crate::api::error::SchedulerError;
pub use crate::api::types::ApplicationConfigBuilder;
pub use crate::api::types::SchedulerSvc;

#[cfg(feature = "tokio-rt")]
use crate::api::traits::Validator;
#[cfg(feature = "tokio-rt")]
use crate::api::types::tokio_scheduler_config::TokioSchedulerConfig;
#[cfg(feature = "tokio-rt")]
use crate::spi::tokio::tokio_scheduler::TokioScheduler;

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

    /// Construct a tokio-backed scheduler with the given config and thread name prefix.
    ///
    /// Returns `impl Scheduler` — callers depend on the trait, not the concrete type.
    /// Requires the `tokio-rt` feature (enabled by default).
    #[cfg(feature = "tokio-rt")]
    pub fn tokio_scheduler(
        config: TokioSchedulerConfig,
        thread_name: impl Into<String>,
    ) -> impl crate::api::traits::Scheduler {
        TokioScheduler::new(config, thread_name)
    }
}
