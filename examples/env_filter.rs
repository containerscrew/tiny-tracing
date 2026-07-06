//! Per-target filtering via `EnvFilter` (RUST_LOG-style directives).
//!
//! Run with: `cargo run --example env_filter`
//! Or:       `RUST_LOG=trace cargo run --example env_filter`

use tiny_tracing::{Logger, debug, info, trace};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info,env_filter=debug".to_string());

    Logger::new().with_env_filter(filter).init()?;

    info!("info line — always visible");
    debug!("debug line — visible when this crate is at DEBUG or lower");
    trace!("trace line — needs RUST_LOG=trace or similar");
    Ok(())
}
