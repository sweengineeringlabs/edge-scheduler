//! [`Scheduler`] — runtime-agnostic contract for driving an async future.

use std::future::Future;

use swe_edge_runtime::RuntimeResult;

/// Drives an async future to completion on the caller's chosen async runtime.
///
/// Implement this trait to plug in any runtime — tokio, async-std, smol, or a
/// custom executor.  The crate ships a ready-made tokio implementation behind
/// the `tokio-rt` feature (enabled by default).
///
/// # Note
///
/// [`RuntimeBuilder::serve`] uses tokio I/O internally, so a custom scheduler
/// must still provide a tokio runtime context (e.g. via
/// `tokio::runtime::Runtime::block_on`) to drive the serve future correctly.
///
/// [`RuntimeBuilder::serve`]: swe_edge_runtime::RuntimeBuilder::serve
pub trait Scheduler {
    /// Block the calling thread until `fut` completes and return its result.
    fn run<F>(&self, fut: F) -> RuntimeResult<()>
    where
        F: Future<Output = RuntimeResult<()>> + Send + 'static;
}
