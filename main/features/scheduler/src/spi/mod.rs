//! SPI (Service Provider Interface) — extension hooks for downstream consumers.
//!
//! Implement the traits in this module to plug a custom async runtime into the
//! scheduler crate without forking the crate itself.

pub mod egress;
