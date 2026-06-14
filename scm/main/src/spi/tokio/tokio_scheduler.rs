//! Tokio-backed [`Scheduler`](crate::api::traits::Scheduler) implementation.

use std::future::Future;
use std::sync::OnceLock;

use crate::api::error::SchedulerError;
use crate::api::traits::{Configurable, Scheduler};
use crate::api::types::application_config_builder::ApplicationConfigBuilder;
use crate::api::types::scheduler_svc::SchedulerSvc;
use crate::api::types::tokio_scheduler_config::TokioSchedulerConfig;
use crate::api::types::tokio_scheduler_config_builder::TokioSchedulerConfigBuilder;

pub(crate) struct TokioScheduler {
    config: TokioSchedulerConfig,
    thread_name: String,
}

impl TokioScheduler {
    pub(crate) fn new(config: TokioSchedulerConfig, thread_name: impl Into<String>) -> Self {
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

        let mut builder = ::tokio::runtime::Builder::new_multi_thread();
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

impl Configurable for TokioScheduler {
    fn config_builder() -> ApplicationConfigBuilder {
        ApplicationConfigBuilder::new()
    }

    fn service() -> SchedulerSvc {
        SchedulerSvc
    }

    fn default_tokio_config() -> TokioSchedulerConfig {
        TokioSchedulerConfig::default()
    }

    fn tokio_config_builder() -> TokioSchedulerConfigBuilder {
        TokioSchedulerConfigBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_uses_config_thread_name_when_set() {
        let config = TokioSchedulerConfig {
            thread_name: Some("from-config".to_string()),
            ..Default::default()
        };
        let s = TokioScheduler::new(config, "from-param");
        assert_eq!(s.thread_name, "from-config");
    }

    #[test]
    fn test_new_uses_param_thread_name_when_config_has_none() {
        let config = TokioSchedulerConfig::default();
        let s = TokioScheduler::new(config, "from-param");
        assert_eq!(s.thread_name, "from-param");
    }
}
