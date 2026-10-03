//! Omitting the timestamp, for platforms that already add their own (journald,
//! container runtimes, …).
//!
//! Run with: `cargo run --example no_timestamp`

use tiny_tracing::{Logger, info};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = Logger::new().with_timestamp(false).init()?;

    info!("this line has no timestamp");
    Ok(())
}
