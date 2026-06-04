//! Egress SPI extension point — downstream consumers plug custom scheduler backends here.

/// Marker extension point for downstream crates that provide custom async executors.
///
/// Implement [`crate::api::scheduler::Scheduler`] on a zero-size struct and pass it
/// to [`crate::SchedulerSvc::run_with_scheduler`] to drive the runtime with a
/// non-tokio async executor.
#[expect(
    dead_code,
    reason = "SEA spi/ anchor — its presence signals downstream scheduler-backend substitution; never constructed internally"
)]
pub(crate) struct SchedulerEgress;
