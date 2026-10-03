//! Logging to stderr so stdout stays free for program output.
//!
//! Run with: `cargo run --example stderr`
//! Try:      `cargo run --example stderr 2>/dev/null` (only the data is printed)

use tiny_tracing::{Logger, Output, info};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = Logger::new().with_output(Output::Stderr).init()?;

    info!("this log line goes to stderr");
    println!("this is program output, on stdout");

    Ok(())
}
