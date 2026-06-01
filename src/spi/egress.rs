//! Egress SPI extension point — downstream consumers plug custom scheduler backends here.

pub(crate) use crate::api::scheduler::Scheduler;
pub(crate) use crate::api::traits::Validator;

/// Marker extension point for downstream crates that provide custom async executors.
///
/// Implement [`Scheduler`] on a zero-size struct and pass it to
/// [`crate::SchedulerSvc::run_with_scheduler`] to drive the runtime with a
/// non-tokio async executor.
pub(crate) struct SchedulerEgress;
