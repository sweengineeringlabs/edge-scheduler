//! [`TokioSchedulerConfigBuilder`] — fluent builder for [`TokioSchedulerConfig`].

use std::num::NonZeroUsize;

use super::tokio_scheduler_config::TokioSchedulerConfig;

/// Fluent builder for [`TokioSchedulerConfig`].
pub struct TokioSchedulerConfigBuilder {
    inner: TokioSchedulerConfig,
}

impl TokioSchedulerConfigBuilder {
    /// Create a new builder with all fields unset (defaults).
    pub(crate) fn new() -> Self {
        Self {
            inner: TokioSchedulerConfig::default(),
        }
    }

    /// Set the number of worker threads.
    pub(crate) fn workers(mut self, n: NonZeroUsize) -> Self {
        self.inner.workers = Some(n);
        self
    }

    /// Set the stack size per worker thread in KiB.
    pub(crate) fn thread_stack_kib(mut self, kib: usize) -> Self {
        self.inner.thread_stack_kib = Some(kib);
        self
    }

    /// Set the maximum number of threads in the blocking pool.
    pub(crate) fn max_blocking_threads(mut self, n: usize) -> Self {
        self.inner.max_blocking_threads = Some(n);
        self
    }

    /// Set the worker thread name prefix.
    pub(crate) fn thread_name(mut self, name: impl Into<String>) -> Self {
        self.inner.thread_name = Some(name.into());
        self
    }

    /// Consume the builder and return the configured [`TokioSchedulerConfig`].
    pub(crate) fn build(self) -> TokioSchedulerConfig {
        self.inner
    }
}

impl Default for TokioSchedulerConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
