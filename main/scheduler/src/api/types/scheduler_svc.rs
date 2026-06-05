//! [`SchedulerSvc`] — the primary public facade type for `swe-edge-runtime-scheduler`.
//!
//! All factory methods for constructing and running schedulers are associated
//! functions on this type.  Implementations are added in `saf/scheduler_svc.rs`.

/// Stateless factory type for scheduler construction.
///
/// Use the associated functions declared in this type to build schedulers
/// and validate configuration.
pub struct SchedulerSvc;
