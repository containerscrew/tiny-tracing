//! Timestamp format and `with_timestamp` behaviour.
//!
//! The global subscriber can only be set once per process, so each case runs in a
//! child process: the parent test re-executes this very test binary, selecting the
//! case through environment variables, and then inspects the log file it produced.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use tiny_tracing::{LogFormat, Logger, Output, info};

const CASE_VAR: &str = "TINY_TRACING_TIMESTAMP_CASE";
const FILE_VAR: &str = "TINY_TRACING_TIMESTAMP_FILE";

/// Entry point of the child process. Does nothing unless the parent set the
/// case variables, so a plain `cargo test` run passes through it harmlessly.
#[test]
fn child_logs_one_line() {
    let (Ok(case), Ok(file)) = (std::env::var(CASE_VAR), std::env::var(FILE_VAR)) else {
        return;
    };

    let (format, timestamp) = match case.as_str() {
        "text" => (LogFormat::Text, true),
        "json" => (LogFormat::Json, true),
        "text-off" => (LogFormat::Text, false),
        "json-off" => (LogFormat::Json, false),
        other => panic!("unknown case {other}"),
    };

    Logger::new()
        .with_format(format)
        .with_timestamp(timestamp)
        .with_output(Output::File(PathBuf::from(file)))
        .init()
        .expect("init should succeed");

    info!("hello timestamp");
}

/// Runs `case` in a child process and returns the single log line it wrote.
fn log_line(case: &str) -> String {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "tiny-tracing-{}-timestamp-{case}.log",
        std::process::id()
    ));
    let _ = fs::remove_file(&path);

    let status = Command::new(std::env::current_exe().expect("test binary path"))
        .args(["--exact", "child_logs_one_line", "--test-threads=1"])
        .env(CASE_VAR, case)
        .env(FILE_VAR, &path)
        .status()
        .expect("child process should start");
    assert!(status.success(), "child process for case {case} failed");

    let contents = fs::read_to_string(&path).expect("log file should exist");
    let _ = fs::remove_file(&path);

    let mut lines = contents.lines();
    let line = lines
        .next()
        .expect("log file should not be empty")
        .to_string();
    assert!(
        lines.next().is_none(),
        "expected a single line, got: {contents:?}"
    );
    line
}

/// `YYYY-MM-DDTHH:MM:SS[.f…]Z`: UTC, RFC 3339. The fractional digits are
/// optional and variable in length because trailing zeros are trimmed.
fn is_utc_rfc3339(s: &str) -> bool {
    let Some(s) = s.strip_suffix('Z') else {
        return false;
    };
    let (date_time, fraction) = match s.split_once('.') {
        Some((head, frac)) => (head, Some(frac)),
        None => (s, None),
    };
    let b = date_time.as_bytes();
    let base_ok = b.len() == 19
        && b.iter().enumerate().all(|(i, c)| match i {
            4 | 7 => *c == b'-',
            10 => *c == b'T',
            13 | 16 => *c == b':',
            _ => c.is_ascii_digit(),
        });
    base_ok
        && fraction
            .is_none_or(|f| (1..=9).contains(&f.len()) && f.bytes().all(|c| c.is_ascii_digit()))
}

#[test]
fn text_line_starts_with_utc_rfc3339_timestamp() {
    let line = log_line("text");
    let first = line.split_whitespace().next().unwrap_or_default();
    assert!(
        is_utc_rfc3339(first),
        "text line should start with a UTC RFC 3339 timestamp, got: {line:?}"
    );
    assert!(line.contains("hello timestamp"), "got: {line:?}");
}

#[test]
fn json_line_has_utc_rfc3339_timestamp_field() {
    let line = log_line("json");
    let prefix = "\"timestamp\":\"";
    let start = line
        .find(prefix)
        .unwrap_or_else(|| panic!("JSON line should have a timestamp field, got: {line:?}"))
        + prefix.len();
    let value = line[start..].split('"').next().unwrap_or_default();
    assert!(
        is_utc_rfc3339(value),
        "JSON timestamp should be UTC RFC 3339, got: {line:?}"
    );
    assert!(line.contains("hello timestamp"), "got: {line:?}");
}

#[test]
fn text_line_has_no_timestamp_when_disabled() {
    let line = log_line("text-off");
    assert!(
        !line.trim_start().starts_with(|c: char| c.is_ascii_digit()),
        "text line should not start with a timestamp, got: {line:?}"
    );
    assert!(
        line.contains("INFO") && line.contains("hello timestamp"),
        "got: {line:?}"
    );
}

#[test]
fn json_line_has_no_timestamp_field_when_disabled() {
    let line = log_line("json-off");
    assert!(
        !line.contains("\"timestamp\":"),
        "JSON line should not have a timestamp field, got: {line:?}"
    );
    assert!(line.contains("hello timestamp"), "got: {line:?}");
}
