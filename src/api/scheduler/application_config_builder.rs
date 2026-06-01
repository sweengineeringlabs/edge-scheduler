//! [`ApplicationConfigBuilder`] — fluent builder for scheduler application configuration.
//!
//! Maps to `config/application.toml` per SEA Rule 170.

use swe_edge_configbuilder::ConfigBuilderImpl;

/// Fluent builder for the scheduler application configuration.
///
/// Wraps [`ConfigBuilderImpl`] pre-seeded with this crate's package name and version.
///
/// # Usage
///
/// ```rust,ignore
/// use swe_edge_runtime_scheduler::ApplicationConfigBuilder;
///
/// let builder = ApplicationConfigBuilder::new().build();
/// ```
pub struct ApplicationConfigBuilder {
    inner: ConfigBuilderImpl,
}

impl ApplicationConfigBuilder {
    /// Create a new builder pre-seeded with the crate's package name and version.
    pub fn new() -> Self {
        Self {
            inner: swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
                .with_name(env!("CARGO_PKG_NAME"))
                .with_version(env!("CARGO_PKG_VERSION")),
        }
    }

    /// Consume the builder and return the underlying [`ConfigBuilderImpl`].
    pub fn build(self) -> ConfigBuilderImpl {
        self.inner
    }
}

impl Default for ApplicationConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
