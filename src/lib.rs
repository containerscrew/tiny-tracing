// ! # slog-rs
// !
// ! `slog-rs` is a simple logger configuration crate for Rust,
// ! using `tracing` and `chrono`.
// !
// ! # Example
// !
// ! ```rust
// ! use slogrs::Logger;
// !
// ! Logger::new()
// !     .with_level("debug")
// !     .with_format("json")
// !     .init();
// ! ```

pub mod config;
pub mod errors;
pub mod time;

pub use tracing::{debug, error, info, trace, warn};

pub use config::Logger;
