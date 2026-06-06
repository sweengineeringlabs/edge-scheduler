//! SAF layer — scheduler public facade.

mod scheduler_svc;

pub use crate::api::error::SchedulerError;
pub use crate::api::types::ApplicationConfigBuilder;
pub use crate::api::types::SchedulerSvc;
#[cfg(feature = "tokio-rt")]
pub use crate::spi::tokio::TokioScheduler;
#[cfg(feature = "tokio-rt")]
pub use crate::spi::tokio::TokioSchedulerConfig;
#[cfg(feature = "tokio-rt")]
pub use crate::spi::tokio::TokioSchedulerConfigBuilder;
