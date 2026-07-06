//! # tiny-tracing
//!
//! A lightweight, builder-style logger for Rust projects that wraps
//! [`tracing`] and [`tracing-subscriber`].
//!
//! Ideal for small to medium applications that want structured or
//! text output with minimal ceremony.
//!
//! # Quick start
//!
//! ```rust
//! use tiny_tracing::Logger;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     Logger::new().init()?;
//!
//!     tiny_tracing::info!("Application started");
//!     Ok(())
//! }
//! ```
//!
//! # Features
//!
//! - Text and JSON output formats
//! - Environment-filter support via [`EnvFilter`](tracing_subscriber::EnvFilter)
//!   (`"info,my_crate=debug"`, `RUST_LOG`, etc.)
//! - Fluent builder API with sensible defaults
//! - Safe initialisation — never panics on double-init

#![deny(missing_docs)]

/// Logger builder and initialisation logic.
pub mod config;
/// Error types returned by the library.
pub mod errors;
/// Custom timestamp formatter for log lines.
pub mod time;

pub use tracing::{Level, debug, error, info, trace, warn};

pub use config::{LogFormat, Logger};
