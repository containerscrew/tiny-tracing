use thiserror::Error;

/// Errors that can occur when configuring and initialising the logger.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum LoggerError {
    /// The supplied log level string is not valid.
    #[error(
        "Invalid log level: '{0}'. Valid levels are: 'trace', 'debug', 'info', 'warn', 'error'."
    )]
    InvalidLevel(String),
    /// The supplied format string is not valid.
    #[error("Invalid log format: '{0}'. Valid formats are: 'text', 'json'.")]
    InvalidFormat(String),
    /// A global tracing subscriber is already set (double-init).
    #[error("Failed to initialize subscriber: {0}")]
    TryInitError(String),
}
