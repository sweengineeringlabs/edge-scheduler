//! [`TokioSchedulerConfig`] — tokio-specific tuning knobs for the async executor.

use std::num::NonZeroUsize;

use serde::{Deserialize, Serialize};

/// Tuning configuration for the tokio-backed scheduler.
///
/// All fields are optional; absent fields let tokio pick its own defaults.
/// Deserializes from the `[scheduler]` section of your application's TOML config.
/// Use [`TokioSchedulerConfigBuilder`](crate::TokioSchedulerConfigBuilder) for
/// programmatic construction.
///
/// # Examples
///
/// ```rust
/// use swe_edge_runtime_scheduler::TokioSchedulerConfig;
/// use std::num::NonZeroUsize;
///
/// // All-default: tokio chooses worker count, stack size, blocking pool size.
/// let cfg = TokioSchedulerConfig::default();
/// assert!(cfg.workers.is_none());
/// assert!(cfg.thread_stack_kib.is_none());
///
/// // Pin to 4 workers for a latency-sensitive service.
/// let cfg = TokioSchedulerConfig {
///     workers: NonZeroUsize::new(4),
///     thread_name: Some("swe-worker".to_string()),
///     ..Default::default()
/// };
/// assert_eq!(cfg.workers.map(|n| n.get()), Some(4));
/// assert_eq!(cfg.thread_name.as_deref(), Some("swe-worker"));
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TokioSchedulerConfig {
    /// Number of worker threads (default: logical CPU count).
    pub workers: Option<NonZeroUsize>,
    /// Stack size per worker thread in KiB (default: OS default, ~8 MiB).
    pub thread_stack_kib: Option<usize>,
    /// Maximum threads in the blocking pool (default: 512).
    pub max_blocking_threads: Option<usize>,
    /// Worker thread name prefix (visible in `ps` and profilers).
    pub thread_name: Option<String>,
}
