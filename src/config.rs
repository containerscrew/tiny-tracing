use std::str::FromStr;

use crate::{errors::LoggerError, time::LocalTimer};
use tracing::Level;
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

/// Output format for log lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LogFormat {
    /// Human-readable single-line text output.
    #[default]
    Text,
    /// One JSON object per log line.
    Json,
}

impl LogFormat {
    fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Json => "json",
        }
    }
}

impl std::fmt::Display for LogFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for LogFormat {
    type Err = LoggerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(Self::Text),
            "json" => Ok(Self::Json),
            other => Err(LoggerError::InvalidFormat(other.to_string())),
        }
    }
}

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
/// use tiny_tracing::{Logger, LogFormat, Level};
///
/// Logger::new()
///     .with_level(Level::DEBUG)
///     .with_format(LogFormat::Json)
///     .with_env_filter("info,my_crate=trace")
///     .init()
///     .unwrap();
/// ```
pub struct Logger {
    level: Level,
    format: LogFormat,
    env_filter: Option<String>,
    with_file: bool,
    with_target: bool,
}

impl Logger {
    /// Creates a new [`Logger`] with default values.
    ///
    /// Defaults: [`Level::INFO`], [`LogFormat::Text`], no env filter,
    /// file location off, target on.
    #[must_use]
    pub fn new() -> Self {
        Self {
            level: Level::INFO,
            format: LogFormat::Text,
            env_filter: None,
            with_file: false,
            with_target: true,
        }
    }

    /// Sets the maximum log level.
    ///
    /// Ignored if [`with_env_filter`](Self::with_env_filter) is also set.
    #[must_use]
    pub fn with_level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    /// Sets the output format.
    #[must_use]
    pub fn with_format(mut self, format: LogFormat) -> Self {
        self.format = format;
        self
    }

    /// Enables environment-based filtering via [`EnvFilter`].
    ///
    /// When set, the filter string is used instead of the static level.
    /// Supported syntax: `"info"`, `"info,my_crate=debug"`, etc.
    ///
    /// If not called, only the static [`with_level`](Logger::with_level) value applies.
    #[must_use]
    pub fn with_env_filter(mut self, filter: impl Into<String>) -> Self {
        self.env_filter = Some(filter.into());
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
    pub fn level(&self) -> Level {
        self.level
    }

    /// Returns the configured output format.
    #[must_use]
    pub fn format(&self) -> LogFormat {
        self.format
    }

    /// Initializes the global tracing subscriber.
    ///
    /// Consumes the builder. Must be called only once per process;
    /// subsequent calls return [`LoggerError::TryInitError`].
    ///
    /// # Errors
    ///
    /// - [`LoggerError::InvalidEnvFilter`] if an env filter was set and
    ///   [`EnvFilter`] rejects it.
    /// - [`LoggerError::TryInitError`] if a global subscriber is already set.
    pub fn init(self) -> Result<(), LoggerError> {
        let base = tracing_subscriber::fmt()
            .with_thread_names(false)
            .with_span_events(FmtSpan::FULL)
            .with_file(self.with_file)
            .with_target(self.with_target)
            .with_timer(LocalTimer);

        let env_filter = self
            .env_filter
            .as_deref()
            .map(EnvFilter::try_new)
            .transpose()
            .map_err(|e| LoggerError::InvalidEnvFilter(e.to_string()))?;

        let result = match self.format {
            LogFormat::Json => {
                let b = base.json();
                match env_filter {
                    Some(f) => b.with_env_filter(f).try_init(),
                    None => b.with_max_level(self.level).try_init(),
                }
            }
            LogFormat::Text => match env_filter {
                Some(f) => base.with_env_filter(f).try_init(),
                None => base.with_max_level(self.level).try_init(),
            },
        };

        result.map_err(|e| LoggerError::TryInitError(e.to_string()))
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}
