//! SAF layer — scheduler public facade.

mod scheduler_svc;

pub use crate::api::error::SchedulerError;
pub use crate::api::scheduler::ApplicationConfigBuilder;
#[cfg(feature = "tokio-rt")]
pub use crate::api::scheduler::TokioSchedulerConfig;
#[cfg(feature = "tokio-rt")]
pub use crate::api::scheduler::TokioSchedulerConfigBuilder;
pub use crate::api::types::SchedulerSvc;
#[cfg(feature = "tokio-rt")]
pub use crate::api::types::TokioScheduler;
