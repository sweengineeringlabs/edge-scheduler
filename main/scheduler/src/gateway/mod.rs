//! Gateway layer — I/O boundary adapters for `swe-edge-runtime-scheduler`.
//!
//! This module is private to the crate; external consumers use the `saf/` facade.

pub(crate) mod egress;
pub(crate) mod ingress;

pub use crate::api::scheduler::Scheduler;
pub use crate::api::traits::Validator;
pub use crate::saf::*;
