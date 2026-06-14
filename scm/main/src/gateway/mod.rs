//! Gateway layer — I/O boundary adapters for `swe-edge-runtime-scheduler`.
//!
//! This module is private to the crate; external consumers use the `saf/` facade.

pub(crate) mod egress;
pub(crate) mod ingress;

pub use crate::api::traits::Configurable;
pub use crate::api::traits::Scheduler;
pub use crate::api::traits::Validator;
#[cfg(feature = "tokio-rt")]
pub use crate::api::types::TokioSchedulerConfig;
#[cfg(feature = "tokio-rt")]
pub use crate::api::types::TokioSchedulerConfigBuilder;
pub use crate::saf::*;
