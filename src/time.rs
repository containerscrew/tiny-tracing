use chrono::Local;
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

/// Formats timestamps using the local system time via [`chrono::Local`].
pub struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        let now = Local::now();
        write!(w, "{}", now.format("%Y-%m-%d %H:%M:%S"))
    }
}
