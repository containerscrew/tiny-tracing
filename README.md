# slog-rs

A lightweight, builder-style logging library for Rust that wraps `tracing` and
`tracing-subscriber`. Designed for small to medium projects that want structured or
plain-text output with zero fuss.

<p align="center">
    <a href="https://github.com/containerscrew/slog-rs/actions/workflows/test.yml"><img alt="CI" src="https://img.shields.io/github/actions/workflow/status/containerscrew/slog-rs/test.yml?branch=main&label=CI"></a>
    <a href="./CHANGELOG.md"><img alt="Changelog" src="https://img.shields.io/badge/changelog-md-blue"></a>
    <a href="https://crates.io/crates/slog-rs"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/slog-rs"></a>
    <img alt="Crates.io Total Downloads" src="https://img.shields.io/crates/d/slog-rs?label=crates.io%20downloads">
    <img alt="GitHub code size in bytes" src="https://img.shields.io/github/languages/code-size/containerscrew/slog-rs">
    <img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/containerscrew/slog-rs">
    <img alt="GitHub issues" src="https://img.shields.io/github/issues/containerscrew/slog-rs">
    <img alt="GitHub pull requests" src="https://img.shields.io/github/issues-pr/containerscrew/slog-rs">
    <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/containerscrew/slog-rs?style=social">
    <img alt="License" src="https://img.shields.io/badge/License-MIT-blue.svg">
    <img alt="MSRV" src="https://img.shields.io/badge/MSRV-1.96.1-orange">
</p>

---

## Quickstart

Add the crate to your project:

```shell
cargo add slog-rs
```

Minimal setup — text output at INFO level, nothing else needed:

```rust
use slog_rs::Logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::new().init()?;

    slog_rs::info!("Application started");
    Ok(())
}
```

## Configuration

The builder API exposes every knob through chainable methods:

```rust
use slog_rs::Logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::new()
        .with_level("debug")                       // trace | debug | info | warn | error
        .with_format("json")                       // text | json
        .with_env_filter("info,my_crate=trace")    // per-target EnvFilter
        .with_file(true)                           // show source file in log lines
        .with_target(false)                        // hide module path
        .init()?;
    Ok(())
}
```

| Method | Default | Description |
|---|---|---|
| `with_level("debug")` | `"info"` | Static log level |
| `with_format("json")` | `"text"` | Output format |
| `with_env_filter("info,my_crate=debug")` | none | Per-target filter via `EnvFilter` |
| `with_file(true)` | `false` | Show source file path in log lines |
| `with_target(false)` | `true` | Show module path in log lines |

## Safety

The library calls `tracing_subscriber::try_init()` internally — calling `init()` more
than once returns a `LoggerError::TryInitError` instead of panicking. No `unsafe` code
anywhere in the crate.

## License

`slog-rs` is distributed under the terms of the [MIT](./LICENSE) license.
