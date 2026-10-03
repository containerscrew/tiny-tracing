//! Reading the filter from the `RUST_LOG` environment variable.
//!
//! Without `RUST_LOG` the level set with `with_level` applies (DEBUG here).
//!
//! Run with: `cargo run --example env_filter_from_env`
//! Or:       `RUST_LOG=trace cargo run --example env_filter_from_env`
//! Or:       `RUST_LOG=warn cargo run --example env_filter_from_env`

use tiny_tracing::{Level, Logger, debug, info, trace, warn};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = Logger::new()
        .with_level(Level::DEBUG)
        .with_env_filter_from_env()
        .init()?;

    warn!("warn line — visible unless RUST_LOG is set to error");
    info!("info line — visible at INFO or lower");
    debug!("debug line — visible by default, hidden by RUST_LOG=info");
    trace!("trace line — needs RUST_LOG=trace");
    Ok(())
}
