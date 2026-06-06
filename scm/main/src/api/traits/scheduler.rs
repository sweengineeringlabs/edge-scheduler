//! [`Scheduler`] — runtime-agnostic contract for driving an async future.

use std::future::Future;

use crate::api::error::SchedulerError;

/// Drives an async future to completion on the caller's chosen async runtime.
///
/// Implement this trait to plug in any async executor — a custom runtime, or
/// any third-party executor.  The crate ships a ready-made implementation
/// behind its default feature; alternative backends are added under `spi/`.
pub trait Scheduler {
    /// Block the calling thread until `fut` completes and return its result.
    fn run<F>(&self, fut: F) -> Result<(), SchedulerError>
    where
        F: Future<Output = Result<(), SchedulerError>> + Send + 'static;
}
