//! [`Configurable`] — trait for types that expose a scheduler configuration surface.

use crate::api::types::application_config_builder::ApplicationConfigBuilder;
use crate::api::types::scheduler_svc::SchedulerSvc;
#[cfg(feature = "tokio-rt")]
use crate::api::types::tokio_scheduler_config::TokioSchedulerConfig;
#[cfg(feature = "tokio-rt")]
use crate::api::types::tokio_scheduler_config_builder::TokioSchedulerConfigBuilder;

/// Implemented by scheduler types that expose application-level configuration.
pub trait Configurable {
    /// Return a pre-seeded [`ApplicationConfigBuilder`] for this scheduler.
    fn config_builder() -> ApplicationConfigBuilder
    where
        Self: Sized;

    /// Return the stateless [`SchedulerSvc`] facade associated with this scheduler.
    fn service() -> SchedulerSvc
    where
        Self: Sized;

    /// Return a default [`TokioSchedulerConfig`] for this scheduler.
    #[cfg(feature = "tokio-rt")]
    fn default_tokio_config() -> TokioSchedulerConfig
    where
        Self: Sized;

    /// Return a fresh [`TokioSchedulerConfigBuilder`] for programmatic config construction.
    #[cfg(feature = "tokio-rt")]
    fn tokio_config_builder() -> TokioSchedulerConfigBuilder
    where
        Self: Sized;
}
