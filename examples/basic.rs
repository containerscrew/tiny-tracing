//! Minimal setup: text output at INFO level.
//!
//! Run with: `cargo run --example basic`

use tiny_tracing::{Logger, info};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::new().init()?;

    info!("hello from tiny-tracing");
    Ok(())
}
