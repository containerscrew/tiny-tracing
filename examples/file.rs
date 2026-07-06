//! Writing logs to a file and stdout at the same time.
//!
//! ANSI colours stay on stdout but are stripped from the file.
//!
//! Run with: `cargo run --example file`
//! Then inspect: `cat app.log`

use tiny_tracing::{Logger, Output, info, warn};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::new()
        .with_output(Output::Both("app.log".into()))
        .init()?;

    info!("this line goes to both stdout and app.log");
    warn!(file = "app.log", "check the file for a colour-free copy");
    Ok(())
}
