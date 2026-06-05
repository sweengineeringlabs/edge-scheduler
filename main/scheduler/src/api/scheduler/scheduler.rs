//! [`Scheduler`] — runtime-agnostic contract for driving an async future.

use std::future::Future;

use crate::api::error::SchedulerError;

/// Drives an async future to completion on the caller's chosen async runtime.
///
/// Implement this trait to plug in any runtime — tokio, async-std, smol, or a
/// custom executor.  The crate ships a ready-made tokio implementation behind
/// the `tokio-rt` feature (enabled by default).
pub trait Scheduler {
    /// Block the calling thread until `fut` completes and return its result.
    fn run<F>(&self, fut: F) -> Result<(), SchedulerError>
    where
        F: Future<Output = Result<(), SchedulerError>> + Send + 'static;
}
