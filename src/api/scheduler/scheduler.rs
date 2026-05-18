//! [`Scheduler`] — runtime-agnostic contract for driving an async future.

use std::future::Future;

use swe_edge_runtime::RuntimeResult;

/// Drives an async future to completion on the caller's chosen async runtime.
///
/// Implement this trait to plug in any runtime — tokio, async-std, smol, or a
/// custom executor.  The crate ships [`crate::TokioScheduler`] as a ready-made
/// implementation behind the `tokio-rt` feature (enabled by default).
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

#[cfg(test)]
mod tests {
    use super::*;
    use swe_edge_runtime::RuntimeError;

    struct OkScheduler;
    impl Scheduler for OkScheduler {
        fn run<F>(&self, fut: F) -> RuntimeResult<()>
        where
            F: Future<Output = RuntimeResult<()>> + Send + 'static,
        {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(fut)
        }
    }

    #[test]
    fn test_scheduler_is_object_safe_via_blanket_impl() {
        let s = OkScheduler;
        assert!(s.run(async { Ok(()) }).is_ok());
    }

    #[test]
    fn test_scheduler_propagates_error() {
        let s = OkScheduler;
        let result = s.run(async { Err(RuntimeError::StartFailed("x".into())) });
        assert!(result.is_err());
    }
}
