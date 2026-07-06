# tiny-tracing

A lightweight, builder-style logging library for Rust that wraps `tracing` and
`tracing-subscriber`. Designed for small to medium projects that want structured or
plain-text output with zero fuss.

<p align="center">
    <a href="https://github.com/containerscrew/tiny-tracing/actions/workflows/test.yml"><img alt="CI" src="https://img.shields.io/github/actions/workflow/status/containerscrew/tiny-tracing/test.yml?branch=main&label=CI"></a>
    <a href="./CHANGELOG.md"><img alt="Changelog" src="https://img.shields.io/badge/changelog-md-blue"></a>
    <a href="https://crates.io/crates/tiny-tracing"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/tiny-tracing"></a>
    <img alt="Crates.io Total Downloads" src="https://img.shields.io/crates/d/tiny-tracing?label=crates.io%20downloads">
    <img alt="GitHub code size in bytes" src="https://img.shields.io/github/languages/code-size/containerscrew/tiny-tracing">
    <img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/containerscrew/tiny-tracing">
    <img alt="GitHub issues" src="https://img.shields.io/github/issues/containerscrew/tiny-tracing">
    <img alt="GitHub pull requests" src="https://img.shields.io/github/issues-pr/containerscrew/tiny-tracing">
    <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/containerscrew/tiny-tracing?style=social">
    <img alt="License" src="https://img.shields.io/badge/License-MIT-blue.svg">
    <img alt="MSRV" src="https://img.shields.io/badge/MSRV-1.96.1-orange">
</p>

---

## Quickstart

Add the crate to your project:

```shell
cargo add tiny-tracing
```

Minimal setup — text output at INFO level, nothing else needed:

```rust
use tiny_tracing::Logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::new().init()?;

    tiny_tracing::info!("Application started");
    Ok(())
}
```

## Configuration

The builder API exposes every knob through chainable methods:

```rust
use tiny_tracing::Logger;

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

`tiny-tracing` is distributed under the terms of the [MIT](./LICENSE) license.

## Development

```bash
git clone https://github.com/containerscrew/tiny-tracing.git
cd tiny-tracing

cargo test                                        # unit + integration + doc-tests
cargo fmt --all -- --check                        # check formatting
cargo clippy --all-targets --all-features -- -D warnings
```

Releases are automated via [cocogitto](https://docs.cocogitto.io/) (Conventional Commits).
See the [release skill](.claude/skills/release/SKILL.md) for the full workflow.
