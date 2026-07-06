use crate::{errors::LoggerError, time::LocalTimer};
use tracing::Level;
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

/// Builder for initializing the global tracing subscriber.
///
/// Uses a fluent builder pattern to configure log level, output format,
/// and optional environment-based filtering before calling [`init`](Logger::init).
///
/// # Examples
///
/// Minimal setup (text output at INFO level):
///
/// ```rust
/// use tiny_tracing::Logger;
///
/// Logger::new().init().unwrap();
/// tiny_tracing::info!("Ready");
/// ```
///
/// JSON output with environment filter:
///
/// ```rust
/// use tiny_tracing::Logger;
///
/// Logger::new()
///     .with_level("debug")
///     .with_format("json")
///     .with_env_filter("info,my_crate=trace")
///     .init()
///     .unwrap();
/// ```
pub struct Logger {
    level: String,
    format: String,
    env_filter: Option<String>,
    with_file: bool,
    with_target: bool,
}

impl Logger {
    /// Creates a new [`Logger`] with default values.
    ///
    /// Defaults: `info` level, `text` format, no env filter,
    /// file location off, target on.
    #[must_use]
    pub fn new() -> Self {
        Self {
            level: "info".to_string(),
            format: "text".to_string(),
            env_filter: None,
            with_file: false,
            with_target: true,
        }
    }

    /// Sets the log level.
    ///
    /// Accepted values: `"trace"`, `"debug"`, `"info"`, `"warn"`, `"error"`.
    #[must_use]
    pub fn with_level(mut self, level: &str) -> Self {
        self.level = level.to_string();
        self
    }

    /// Sets the output format.
    ///
    /// Accepted values: `"text"` (default) or `"json"`.
    #[must_use]
    pub fn with_format(mut self, format: &str) -> Self {
        self.format = format.to_string();
        self
    }

    /// Enables environment-based filtering via [`EnvFilter`].
    ///
    /// When set, the filter string is used instead of the static level.
    /// Supported syntax: `"info"`, `"info,my_crate=debug"`, etc.
    ///
    /// If not called, only the static [`with_level`](Logger::with_level) value applies.
    #[must_use]
    pub fn with_env_filter(mut self, filter: &str) -> Self {
        self.env_filter = Some(filter.to_string());
        self
    }

    /// Controls whether the source file name appears in log lines.
    ///
    /// Disabled by default.
    #[must_use]
    pub fn with_file(mut self, enabled: bool) -> Self {
        self.with_file = enabled;
        self
    }

    /// Controls whether the module path (target) appears in log lines.
    ///
    /// Enabled by default.
    #[must_use]
    pub fn with_target(mut self, enabled: bool) -> Self {
        self.with_target = enabled;
        self
    }

    /// Returns the configured log level.
    #[must_use]
    pub fn level(&self) -> &str {
        &self.level
    }

    /// Returns the configured output format.
    #[must_use]
    pub fn format(&self) -> &str {
        &self.format
    }

    /// Initializes the global tracing subscriber.
    ///
    /// Consumes the builder. Must be called only once per process;
    /// subsequent calls return [`LoggerError::TryInitError`].
    ///
    /// # Errors
    ///
    /// - [`LoggerError::InvalidLevel`] if the level string is unrecognised.
    /// - [`LoggerError::InvalidFormat`] if the format string is unrecognised.
    /// - [`LoggerError::TryInitError`] if a global subscriber is already set.
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
            .with_thread_names(false)
            .with_span_events(FmtSpan::FULL)
            .with_file(self.with_file)
            .with_target(self.with_target)
            .with_timer(LocalTimer);

        match self.format.to_lowercase().as_str() {
            "json" => {
                let base = base.json();
                if let Some(filter) = &self.env_filter {
                    let env_filter = EnvFilter::try_new(filter)
                        .map_err(|e| LoggerError::InvalidLevel(e.to_string()))?;
                    base.with_env_filter(env_filter)
                        .try_init()
                        .map_err(|e| LoggerError::TryInitError(e.to_string()))
                } else {
                    base.with_max_level(level)
                        .try_init()
                        .map_err(|e| LoggerError::TryInitError(e.to_string()))
                }
            }
            "text" => {
                if let Some(filter) = &self.env_filter {
                    let env_filter = EnvFilter::try_new(filter)
                        .map_err(|e| LoggerError::InvalidLevel(e.to_string()))?;
                    base.with_env_filter(env_filter)
                        .try_init()
                        .map_err(|e| LoggerError::TryInitError(e.to_string()))
                } else {
                    base.with_max_level(level)
                        .try_init()
                        .map_err(|e| LoggerError::TryInitError(e.to_string()))
                }
            }
            other => Err(LoggerError::InvalidFormat(other.to_string())),
        }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}
