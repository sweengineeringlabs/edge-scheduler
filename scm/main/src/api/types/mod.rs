//! Public types for `swe-edge-runtime-scheduler`.

pub use application_config_builder::ApplicationConfigBuilder;
pub use scheduler_svc::SchedulerSvc;

pub mod application_config_builder;
pub mod scheduler_svc;

#[cfg(feature = "tokio-rt")]
pub use tokio_scheduler_config::TokioSchedulerConfig;
#[cfg(feature = "tokio-rt")]
pub use tokio_scheduler_config_builder::TokioSchedulerConfigBuilder;

#[cfg(feature = "tokio-rt")]
pub mod tokio_scheduler_config;
#[cfg(feature = "tokio-rt")]
pub mod tokio_scheduler_config_builder;
