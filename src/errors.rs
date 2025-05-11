use std::fmt;

#[derive(Debug)]
pub enum LoggerError {
    InvalidLevel(String),
    InvalidFormat(String),
}

impl fmt::Display for LoggerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoggerError::InvalidLevel(level) => {
                write!(f, "Invalid log level: {}", level)
            }
            LoggerError::InvalidFormat(format) => {
                write!(f, "Invalid log format: {}", format)
            }
        }
    }
}

impl std::error::Error for LoggerError {}
