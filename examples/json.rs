//! JSON output at DEBUG level with source file locations enabled.
//!
//! Run with: `cargo run --example json`

use tiny_tracing::{Level, LogFormat, Logger, info, warn};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::new()
        .with_level(Level::DEBUG)
        .with_format(LogFormat::Json)
        .with_file(true)
        .init()?;

    info!(user_id = 42, action = "login", "structured event");
    warn!("something worth a second look");
    Ok(())
}
