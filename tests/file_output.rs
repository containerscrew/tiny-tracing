use std::fs;
use std::path::PathBuf;

use tiny_tracing::errors::LoggerError;
use tiny_tracing::{Logger, Output, info};

fn temp_path(name: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!("tiny-tracing-{}-{name}.log", std::process::id()));
    p
}

#[test]
fn file_output_writes_log_lines_to_disk() {
    let path = temp_path("file");
    let _ = fs::remove_file(&path);

    Logger::new()
        .with_output(Output::File(path.clone()))
        .init()
        .expect("init should succeed");

    info!(target: "file_output_test", "hello file");

    let contents = fs::read_to_string(&path).expect("log file should exist");
    assert!(
        contents.contains("hello file"),
        "log file should contain the message, got: {contents:?}"
    );

    let _ = fs::remove_file(&path);
}

#[test]
fn file_output_reports_unopenable_path() {
    let path = PathBuf::from("/this/dir/does/not/exist/tiny-tracing.log");
    let result = Logger::new().with_output(Output::File(path)).init();
    assert!(matches!(result, Err(LoggerError::OpenLogFile { .. })));
}
