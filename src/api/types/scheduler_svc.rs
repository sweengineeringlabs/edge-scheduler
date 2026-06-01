//! [`SchedulerSvc`] — the primary public facade type for `swe-edge-runtime-scheduler`.
//!
//! All factory methods for constructing and running schedulers are associated
//! functions on this type.  Implementations are added in `saf/scheduler_svc.rs`.

/// Stateless factory type for scheduler construction and runtime dispatch.
///
/// Use the associated functions declared in this type to build schedulers,
/// validate configuration, and drive the async runtime.
///
/// # Example
///
/// ```rust,ignore
/// use swe_edge_runtime_scheduler::SchedulerSvc;
/// use swe_edge_runtime::Runtime;
///
/// fn main() {
///     SchedulerSvc::run(Runtime::builder()).expect("runtime failed");
/// }
/// ```
pub struct SchedulerSvc;
