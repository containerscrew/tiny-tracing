use thiserror::Error;

/// Errors that can occur when configuring and initialising the logger.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum LoggerError {
    /// The supplied format string is not a known [`LogFormat`](crate::LogFormat).
    ///
    /// Returned by `LogFormat::from_str`, not by `Logger::init` — the builder
    /// itself is typed, so this cannot occur when values are passed as enums.
    #[error("Invalid log format: '{0}'. Valid formats are: 'text', 'json'.")]
    InvalidFormat(String),
    /// The env filter string could not be parsed by
    /// [`EnvFilter`](tracing_subscriber::EnvFilter).
    #[error("Invalid env filter: '{0}'.")]
    InvalidEnvFilter(String),
    /// A global tracing subscriber is already set (double-init).
    #[error("Failed to initialize subscriber: {0}")]
    TryInitError(String),
}
