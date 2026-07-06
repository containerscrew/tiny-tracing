use tiny_tracing::Level;
use tiny_tracing::config::{LogFormat, Logger};
use tiny_tracing::errors::LoggerError;

#[test]
fn new_should_return_default_values() {
    let logger = Logger::new();
    assert_eq!(logger.level(), Level::INFO);
    assert_eq!(logger.format(), LogFormat::Text);
}

#[test]
fn with_level_should_store_level() {
    let logger = Logger::new().with_level(Level::DEBUG);
    assert_eq!(logger.level(), Level::DEBUG);
}

#[test]
fn with_format_should_store_format() {
    let logger = Logger::new().with_format(LogFormat::Json);
    assert_eq!(logger.format(), LogFormat::Json);
}

#[test]
fn init_should_fail_with_invalid_env_filter() {
    let result = Logger::new().with_env_filter("!!!invalid!!!").init();
    assert!(matches!(result, Err(LoggerError::InvalidEnvFilter(_))));
}

#[test]
fn init_should_set_up_subscriber_and_reject_double_init() {
    let first = Logger::new()
        .with_level(Level::INFO)
        .with_format(LogFormat::Text)
        .init();
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

#[test]
fn log_format_from_str_parses_known_values() {
    assert_eq!("text".parse::<LogFormat>().unwrap(), LogFormat::Text);
    assert_eq!("JSON".parse::<LogFormat>().unwrap(), LogFormat::Json);
    assert!(matches!(
        "yaml".parse::<LogFormat>(),
        Err(LoggerError::InvalidFormat(_))
    ));
}

#[test]
fn log_format_default_is_text() {
    assert_eq!(LogFormat::default(), LogFormat::Text);
}
