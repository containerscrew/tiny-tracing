use slog_rs::config::Logger;
use slog_rs::errors::LoggerError;

#[test]
fn new_should_return_default_values() {
    let logger = Logger::new();
    assert_eq!(logger.level(), "info");
    assert_eq!(logger.format(), "text");
}

#[test]
fn with_level_should_store_level() {
    let logger = Logger::new().with_level("debug");
    assert_eq!(logger.level(), "debug");
}

#[test]
fn with_format_should_store_format() {
    let logger = Logger::new().with_format("json");
    assert_eq!(logger.format(), "json");
}

#[test]
fn init_should_fail_with_invalid_level() {
    let result = Logger::new().with_level("not_a_level").init();
    assert!(matches!(result, Err(LoggerError::InvalidLevel(_))));
}

#[test]
fn init_should_fail_with_invalid_format() {
    let result = Logger::new().with_format("yaml").init();
    assert!(matches!(result, Err(LoggerError::InvalidFormat(_))));
}

#[test]
fn init_should_fail_with_invalid_env_filter() {
    let result = Logger::new().with_env_filter("!!!invalid!!!").init();
    assert!(matches!(result, Err(LoggerError::InvalidLevel(_))));
}

#[test]
fn init_should_set_up_subscriber_and_reject_double_init() {
    let first = Logger::new().with_level("info").with_format("text").init();
    assert!(first.is_ok());

    let second = Logger::new().init();
    assert!(matches!(second, Err(LoggerError::TryInitError(_))));
}

#[test]
fn default_should_match_new() {
    let from_new = Logger::new();
    let from_default = Logger::default();
    assert_eq!(from_new.level(), from_default.level());
    assert_eq!(from_new.format(), from_default.format());
}
