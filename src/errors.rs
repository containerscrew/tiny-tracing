use std::fmt;

pub enum LoggerError {
    InvalidLevel(String),
    InvalidFormat(String),
}

impl fmt::Display for LoggerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoggerError::InvalidLevel(level) => {
                write!(
                    f,
                    "Invalid log level: '{}'. Valid levels are: 'trace', 'debug', 'info', 'warn', 'error'.",
                    level
                )
            }
            LoggerError::InvalidFormat(format) => {
                write!(
                    f,
                    "Invalid log format: '{}'. Valid formats are: 'text', 'json'.",
                    format
                )
            }
        }
    }
}

// A unique format for dubugging output
impl fmt::Debug for LoggerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for LoggerError {}
