//! `swe_edge_runtime_scheduler` — runtime-agnostic async scheduler.

mod api;
mod saf;
mod spi;

pub use crate::api::traits::Configurable;
pub use crate::api::traits::Scheduler;
pub use crate::api::traits::Validator;
#[cfg(feature = "tokio-rt")]
pub use crate::api::types::TokioSchedulerConfig;
#[cfg(feature = "tokio-rt")]
pub use crate::api::types::TokioSchedulerConfigBuilder;
pub use crate::saf::*;
