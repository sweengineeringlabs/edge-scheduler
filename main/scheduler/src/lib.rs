//! `swe_edge_runtime_scheduler` — runtime-agnostic async scheduler.

mod api;
mod core;
mod gateway;
mod saf;
mod spi;

pub use gateway::*;
