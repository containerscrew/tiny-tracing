use chrono::Local;
use tracing_subscriber::fmt::{
    format::Writer,
    time::FormatTime,
};

/// MyTimer is a custom implementation of FormatTime using chrono::Local.
pub struct MyTimer;

impl FormatTime for MyTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        let now = Local::now();
        write!(w, "{}", now.format("%Y-%m-%d %H:%M:%S"))
    }
}
