//! Integration tests for the scheduler crate.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::future::Future;
use std::num::NonZeroUsize;

use swe_edge_runtime_scheduler::{Scheduler, SchedulerError, SchedulerSvc};

// ── Custom Scheduler impl ─────────────────────────────────────────────────────

struct SingleThreadScheduler;

impl Scheduler for SingleThreadScheduler {
    fn run<F>(&self, fut: F) -> Result<(), SchedulerError>
    where
        F: Future<Output = Result<(), SchedulerError>> + Send + 'static,
    {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| SchedulerError::StartFailed(format!("single-thread: {e}")))?
            .block_on(fut)
    }
}

// ── Custom scheduler drives futures ──────────────────────────────────────────

#[test]
fn test_scheduler_struct_custom_impl_runs_future_successfully() {
    assert!(SingleThreadScheduler.run(async { Ok(()) }).is_ok());
}

#[test]
fn test_scheduler_struct_custom_impl_propagates_error_from_future() {
    let result =
        SingleThreadScheduler.run(async { Err(SchedulerError::StartFailed("boom".into())) });
    assert!(matches!(result, Err(SchedulerError::StartFailed(_))));
}

// ── TokioSchedulerConfig + tokio factory (tokio-rt) ──────────────────────────

#[cfg(feature = "tokio-rt")]
mod tokio_tests {
    use super::*;
    use swe_edge_runtime_scheduler::TokioSchedulerConfig;

    #[test]
    fn test_scheduler_struct_tokio_config_default_has_all_fields_none() {
        let cfg = TokioSchedulerConfig::default();
        assert!(cfg.workers.is_none());
        assert!(cfg.thread_stack_kib.is_none());
        assert!(cfg.max_blocking_threads.is_none());
        assert!(cfg.thread_name.is_none());
    }

    #[test]
    fn test_scheduler_struct_tokio_config_roundtrips_through_toml() {
        let cfg = TokioSchedulerConfig {
            workers: NonZeroUsize::new(2),
            thread_name: Some("svc".into()),
            ..Default::default()
        };
        let s = toml::to_string(&cfg).expect("serialize");
        let back: TokioSchedulerConfig = toml::from_str(&s).expect("deserialize");
        assert_eq!(back.workers, cfg.workers);
        assert_eq!(back.thread_name.as_deref(), Some("svc"));
    }

    #[test]
    fn test_scheduler_struct_tokio_scheduler_factory_drives_future_successfully() {
        let s = SchedulerSvc::tokio_scheduler(TokioSchedulerConfig::default(), "test");
        assert!(s.run(async { Ok(()) }).is_ok());
    }
}
