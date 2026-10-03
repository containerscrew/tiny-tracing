//! Minimal setup: text output at INFO with no color
//!
//! Run with: `cargo run --example colored`

use tiny_tracing::{Logger, info};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = Logger::new().colored(false).init()?;

    info!("hello from tiny-tracing");

    Ok(())
}
