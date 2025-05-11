use crate::{errors::LoggerError, time::MyTimer};
use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;

/// Logger is a builder-style logger initializer for slog-rs.
pub struct Logger {
    level: String,
    format: String,
}

impl Logger {
    /// Create a new Logger with default values (info level, text format).
    /// Create a new Logger with default values (info level, text format).
    pub fn new() -> Self {
        Self {
            level: "info".to_string(),
            format: "text".to_string(),
        }
    }

    /// Sets the log level (to be validated in `init()`).
    ///
    /// Accepts one of: `"trace"`, `"debug"`, `"info"`, `"warn"`, `"error"`.
    pub fn with_level(mut self, level: &str) -> Self {
        self.level = level.to_string();
        self
    }

    /// Sets the log output format (to be validated in `init()`).
    ///
    /// Accepts `"text"` (default) or `"json"`.
    pub fn with_format(mut self, format: &str) -> Self {
        self.format = format.to_string();
        self
    }

    /// Getter for the log level
    pub fn level(&self) -> &str {
        &self.level
    }

    /// Getter for the log format
    pub fn format(&self) -> &str {
        &self.format
    }

    /// Initializes the logger with the current configuration.
    ///
    /// This consumes the builder. Must be called only once in your program,
    /// usually at the beginning of `main()`.
    ///
    /// Internally uses `tracing_subscriber`, so calling this more than once
    /// will cause a panic.
    pub fn init(self) -> Result<(), LoggerError> {
        let level = match self.level.to_lowercase().as_str() {
            "trace" => Level::TRACE,
            "debug" => Level::DEBUG,
            "info" => Level::INFO,
            "warn" => Level::WARN,
            "error" => Level::ERROR,
            other => return Err(LoggerError::InvalidLevel(other.to_string())),
        };

        let base = tracing_subscriber::fmt()
            .with_max_level(level)
            .with_thread_names(false)
            .with_span_events(FmtSpan::FULL)
            .with_file(false)
            .with_target(true)
            .with_timer(MyTimer);

        match self.format.to_lowercase().as_str() {
            "json" => {
                base.json().init();
                Ok(())
            }
            "text" => {
                base.init();
                Ok(())
            }
            other => Err(LoggerError::InvalidFormat(other.to_string())),
        }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new().with_format("text").with_level("info")
    }
}