//! [`TokioScheduler`] — public-facing tokio-backed [`Scheduler`] type.

use std::future::Future;
use std::sync::OnceLock;

use crate::api::error::SchedulerError;
use crate::api::scheduler::tokio_scheduler_config::TokioSchedulerConfig;
use crate::api::scheduler::Scheduler;

/// Tokio-backed async scheduler.
///
/// Construct via [`crate::SchedulerSvc::tokio_scheduler`].  Pass the result to
/// [`crate::SchedulerSvc::run_with_scheduler`] to drive the runtime.
pub struct TokioScheduler {
    config: TokioSchedulerConfig,
    thread_name: String,
}

impl TokioScheduler {
    /// Create a new tokio scheduler with the given config and thread name prefix.
    pub fn new(config: TokioSchedulerConfig, thread_name: impl Into<String>) -> Self {
        let thread_name = config
            .thread_name
            .clone()
            .unwrap_or_else(|| thread_name.into());
        Self {
            config,
            thread_name,
        }
    }

    fn install_panic_hook() {
        static PANIC_HOOK: OnceLock<()> = OnceLock::new();
        PANIC_HOOK.get_or_init(|| {
            let prev = std::panic::take_hook();
            std::panic::set_hook(Box::new(move |info| {
                tracing::error!(panic = %info, "worker thread panicked — process will abort");
                prev(info);
            }));
        });
    }
}

impl Scheduler for TokioScheduler {
    fn run<F>(&self, fut: F) -> Result<(), SchedulerError>
    where
        F: Future<Output = Result<(), SchedulerError>> + Send + 'static,
    {
        Self::install_panic_hook();

        let mut builder = tokio::runtime::Builder::new_multi_thread();
        builder.enable_all();
        builder.thread_name(&self.thread_name);

        if let Some(workers) = self.config.workers {
            builder.worker_threads(workers.get());
        }
        if let Some(stack_kib) = self.config.thread_stack_kib {
            builder.thread_stack_size(stack_kib * 1024);
        }
        if let Some(max_blocking) = self.config.max_blocking_threads {
            builder.max_blocking_threads(max_blocking);
        }

        let rt = builder
            .build()
            .map_err(|e| SchedulerError::StartFailed(format!("scheduler: {e}")))?;

        rt.block_on(fut)
    }
}
