//! Validation interface counterpart for scheduler configuration types.
//!
//! The concrete implementations live in `core/scheduler/validator.rs`.
//! The `Validator` trait itself is declared in `api/traits/validator.rs`.

/// Marker type: the canonical validator for scheduler configuration.
///
/// This type exists to satisfy the SEA Rule 161 requirement that each `api/` file
/// declares exactly one public interface item, and to provide a named anchor for
/// documentation and test-coverage tracking.
pub struct SchedulerConfigValidator;
