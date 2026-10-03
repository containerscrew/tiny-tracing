# tiny-tracing

A lightweight, builder-style logging library for Rust that wraps `tracing` and
`tracing-subscriber`. Designed for small to medium projects that want structured or
plain-text output with zero fuss.

<p align="center">
    <a href="https://github.com/containerscrew/tiny-tracing/actions/workflows/ci.yml"><img alt="CI" src="https://img.shields.io/github/actions/workflow/status/containerscrew/tiny-tracing/ci.yml?branch=main&label=CI"></a>
    <a href="./CHANGELOG.md"><img alt="Changelog" src="https://img.shields.io/badge/changelog-md-blue"></a>
    <a href="https://crates.io/crates/tiny-tracing"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/tiny-tracing"></a>
    <a href="https://docs.rs/tiny-tracing"><img alt="docs.rs" src="https://img.shields.io/docsrs/tiny-tracing"></a>
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

> [!NOTE]
> AI coding assistants are used in this project mainly for best-practice and design
> guidance (through the [agent skills](#agent-skills) listed below), to keep the library
> as simple and productive as possible, and above all to document its functions.

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
use tiny_tracing::{Logger, LogFormat, Level, Output};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::new()
        .with_level(Level::DEBUG)                  // TRACE | DEBUG | INFO | WARN | ERROR
        .with_format(LogFormat::Json)              // Text | Json
        .with_env_filter("info,my_crate=trace")    // per-target EnvFilter, on top of level
        .with_file(true)                           // show source file in log lines
        .with_target(false)                        // hide module path
        .with_output(Output::Both("app.log".into())) // stdout + file
        .colored(false)                            // force ANSI colours off (default: auto)
        .init()?;
    Ok(())
}
```

| Method | Default | Description |
|---|---|---|
| `with_level(Level::DEBUG)` | `Level::INFO` | Global log level (`tracing::Level`); `with_env_filter` refines it per-target |
| `with_format(LogFormat::Json)` | `LogFormat::Text` | Output format |
| `with_env_filter("info,my_crate=debug")` | none | Per-target filter via `EnvFilter`, layered on the level |
| `with_file(true)` | `false` | Show source file path in log lines |
| `with_target(false)` | `true` | Show module path in log lines |
| `with_output(Output::Both("app.log".into()))` | `Output::Stdout` | Write to stdout, a file, or both |
| `colored(false)` | auto | Force ANSI colours on or off for stdout; by default they are on only when stdout is a terminal |

### Output destinations

`with_output` takes an `Output`: `Stdout` (default), `File(path)`, or `Both(path)`.
File output is opened in append mode (created if missing) with synchronised,
blocking writes — no background thread, no guard to keep alive.

### Colours

By default ANSI colours are used only when stdout is a terminal, so piped output,
Docker and CI logs stay free of escape codes. Call `colored(true)` or `colored(false)`
to override that. File output is never coloured, whatever you choose. The `NO_COLOR`
environment variable is not read; use `colored(false)` if you want to honour it
yourself.

### Timestamps

Every log line carries an RFC 3339 timestamp in UTC (for example
`2026-10-01T12:00:00.123456Z`), in both text and JSON output.

Need to load config from a string (env var, TOML)? `LogFormat` implements `FromStr`,
and `tracing::Level` does too:

```rust
use tiny_tracing::{LogFormat, Level};

let format: LogFormat = "json".parse()?;
let level: Level = "debug".parse()?;
# Ok::<_, Box<dyn std::error::Error>>(())
```

## Examples

Runnable examples live under [`examples/`](./examples):

```bash
cargo run --example basic       # text output at INFO
cargo run --example json        # JSON output at DEBUG, with file locations
cargo run --example env_filter  # per-target filter (respects RUST_LOG)
cargo run --example file        # write to stdout + a file at once
cargo run --example colored     # force ANSI colours off
```

## Limitations

`tiny-tracing` is deliberately a thin wrapper, so some things are left out on purpose:

- **File writes are blocking.** Each log line is written synchronously under a mutex,
  with no background thread. That is fine for small and medium workloads, but heavy
  logging from many threads or from async tasks can contend on the lock. If you need
  non-blocking writes, use [`tracing-appender`](https://crates.io/crates/tracing-appender)
  with `tracing-subscriber` directly.
- **No file rotation.** The log file is only ever appended to and grows without bound.
  Rotate it externally (for example with `logrotate`) or use
  [`tracing-appender`](https://crates.io/crates/tracing-appender)'s rolling files.
- **In containers, log to stdout.** In Docker, Kubernetes or systemd, keep the default
  `Output::Stdout` and let the platform collect and rotate the logs, rather than writing
  files inside the container.

Planned improvements are tracked in [`roadmap.md`](./roadmap.md).

## Safety

The library calls `tracing_subscriber::try_init()` internally — calling `init()` more
than once returns a `LoggerError::TryInitError` instead of panicking. No `unsafe` code
anywhere in the crate.

To report a vulnerability, see the [security policy](./SECURITY.md).

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

### Agent skills

AI coding agents working on this repo use these third-party skills (installed under
`.claude/skills/`, pinned in [`skills-lock.json`](./skills-lock.json)):

```bash
npx skills add trailofbits/skills@cargo-fuzz               # fuzzing Rust code with cargo-fuzz
npx skills add apollographql/skills@rust-best-practices    # idiomatic Rust guidelines
npx skills add openai/skills@security-ownership-map        # security-oriented ownership / bus-factor analysis
npx skills add vercel-labs/skills@find-skills              # discover and install other skills
```

The `release` skill is specific to this repo and not installed from anywhere.
