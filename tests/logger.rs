use slog_rs::Logger;
use slog_rs::errors::LoggerError;

#[test]
fn logger_builds_with_valid_config() {
    let logger = Logger::new().with_level("debug").with_format("json");

    assert_eq!(logger.level(), "debug");
    assert_eq!(logger.format(), "json");
}

#[test]
fn logger_fails_with_invalid_level() {
    let logger = Logger::new().with_level("notalevel");
    let result = logger.init();
    assert!(matches!(result, Err(LoggerError::InvalidLevel(_))));
}

#[test]
fn can_build_logger_with_text_format() {
    let logger = Logger::new().with_level("info").with_format("text");
    assert!(logger.init().is_ok());
}
