//! `Output::Stderr` writes log lines to stderr and nothing to stdout.
//!
//! The global subscriber can only be set once per process, so the parent test
//! re-executes this test binary as a child process, captures both of its streams
//! and inspects them. The child is selected through an environment variable.

use std::process::Command;

use tiny_tracing::errors::LoggerError;
use tiny_tracing::{Logger, Output, info};

const CHILD_VAR: &str = "TINY_TRACING_STDERR_CHILD";

/// Entry point of the child process. Does nothing unless the parent set the
/// variable, so a plain `cargo test` run passes through it harmlessly.
#[test]
fn child_logs_to_stderr() {
    if std::env::var(CHILD_VAR).is_err() {
        return;
    }

    Logger::new()
        .with_output(Output::Stderr)
        .init()
        .expect("init should succeed");

    let second = Logger::new().with_output(Output::Stderr).init();
    assert!(matches!(second, Err(LoggerError::TryInitError(_))));

    info!("log line for stderr");
    println!("data line for stdout");
}

#[test]
fn stderr_output_keeps_stdout_free_of_logs() {
    let output = Command::new(std::env::current_exe().expect("test binary path"))
        .args([
            "--exact",
            "child_logs_to_stderr",
            "--nocapture",
            "--test-threads=1",
        ])
        .env(CHILD_VAR, "1")
        .output()
        .expect("child process should start");
    assert!(output.status.success(), "child process failed: {output:?}");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stderr.contains("log line for stderr"),
        "stderr should contain the log line, got: {stderr:?}"
    );
    assert!(
        !stdout.contains("log line for stderr"),
        "stdout should not contain the log line, got: {stdout:?}"
    );
    assert!(
        stdout.contains("data line for stdout"),
        "program output should stay on stdout, got: {stdout:?}"
    );
    assert!(
        !stderr.contains("data line for stdout"),
        "stderr should only hold log lines, got: {stderr:?}"
    );
    assert!(
        !stderr.contains('\x1b'),
        "stderr is not a terminal, so no ANSI escapes are expected, got: {stderr:?}"
    );
}
