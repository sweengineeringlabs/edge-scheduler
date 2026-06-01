//! Public types for `swe-edge-runtime-scheduler`.

pub use scheduler_svc::SchedulerSvc;
#[cfg(feature = "tokio-rt")]
pub use tokio_scheduler::TokioScheduler;

pub mod scheduler_svc;
#[cfg(feature = "tokio-rt")]
pub mod tokio_scheduler;
