//! SAF layer — scheduler public facade.

mod edge_runtime_scheduler_svc;

pub use crate::api::application_config_builder::ApplicationConfigBuilder;
pub use crate::api::architecture_config_builder::ArchitectureConfigBuilder;
#[cfg(feature = "tokio-rt")]
pub use crate::api::scheduler::TokioSchedulerConfig;

pub use edge_runtime_scheduler_svc::run_with_scheduler;
#[cfg(feature = "tokio-rt")]
pub use edge_runtime_scheduler_svc::{run, run_with_config, tokio_scheduler, validate};
