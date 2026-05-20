//! `swe_edge_runtime_scheduler` — async executor for standalone binaries.
//!
//! Consumers who embed `swe-edge-runtime` inside an existing tokio application
//! call [`RuntimeBuilder::serve`] directly from their own async context.
//!
//! Consumers who need a standalone binary with no tokio boilerplate depend on
//! this crate and call [`run`] or [`RuntimeBuilderExt::run`] instead.

mod api;
mod core;
mod saf;

pub use api::runtime_builder_ext::RuntimeBuilderExt;
pub use api::scheduler::Scheduler;
pub use api::traits::Validator;
pub use saf::*;
