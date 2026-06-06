//! [`SchedulerError`] — domain error type for the scheduler crate.

/// Errors that can occur during scheduler construction or execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulerError {
    /// The async runtime failed to start.
    StartFailed(String),
    /// The scheduler configuration is invalid.
    InvalidConfig(String),
}

impl std::fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StartFailed(msg) => write!(f, "scheduler start failed: {msg}"),
            Self::InvalidConfig(msg) => write!(f, "invalid scheduler config: {msg}"),
        }
    }
}

impl std::error::Error for SchedulerError {}
