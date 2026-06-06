//! SPI layer — implementations of `api/` contracts backed by external libraries.
//!
//! Downstream consumers may add a sibling `spi/{technology}/` directory plus a
//! `saf/` factory arm to plug a non-tokio async executor without forking the
//! crate.
#[cfg(feature = "tokio-rt")]
pub(crate) mod tokio;
