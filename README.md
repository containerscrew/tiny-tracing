<h1 align="center">tiny-tracing</h1>

<p align="center">
    A lightweight, builder-style logging library for Rust that wraps <code>tracing</code> and
    <code>tracing-subscriber</code>. Designed for small to medium projects that want structured or
    plain-text output with zero fuss.
</p>

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
    <img alt="MSRV" src="https://img.shields.io/badge/MSRV-1.88-orange">
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
use tiny_tracing::{Logger, info};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = Logger::new().init()?;

    info!("hello from tiny-tracing");
    Ok(())
}
```

## Configuration

The builder API exposes every knob through chainable methods:

```rust
use tiny_tracing::{Logger, LogFormat, Level, Output};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = Logger::new()
        .with_level(Level::DEBUG)                  // TRACE | DEBUG | INFO | WARN | ERROR
        .with_format(LogFormat::Json)              // Text | Json
        .with_env_filter("info,my_crate=trace")    // per-target EnvFilter, on top of level
        // .with_env_filter_from_env()            // or read it from RUST_LOG instead
        .with_file(true)                           // show source file in log lines
        .with_target(false)                        // hide module path
        .with_timestamp(false)                     // omit the timestamp (default: on, UTC RFC 3339)
        .with_output(Output::Both("app.log".into())) // stdout + file
        .colored(false)                            // force ANSI colours off (default: auto)
        .init()?;
    Ok(())
}
```

| Method                                        | Default           | Description                                                                                                       |
| --------------------------------------------- | ----------------- | ----------------------------------------------------------------------------------------------------------------- |
| `with_level(Level::DEBUG)`                    | `Level::INFO`     | Global log level (`tracing::Level`); `with_env_filter` refines it per-target                                      |
| `with_format(LogFormat::Json)`                | `LogFormat::Text` | Output format                                                                                                     |
| `with_env_filter("info,my_crate=debug")`      | none              | Per-target filter via `EnvFilter`, layered on the level                                                           |
| `with_env_filter_from_env()`                  | none              | Reads the filter from `RUST_LOG`; if it is unset, only the level applies                                          |
| `with_file(true)`                             | `false`           | Show source file path in log lines                                                                                |
| `with_target(false)`                          | `true`            | Show module path in log lines                                                                                     |
| `with_timestamp(false)`                       | `true`            | Prefix each line with a UTC RFC 3339 timestamp                                                                    |
| `with_output(Output::Both("app.log".into()))` | `Output::Stdout`  | Write to stdout, stderr, a file, or both stdout and a file                                                        |
| `colored(false)`                              | auto              | Force ANSI colours on or off; by default they are on only when the output stream (stdout or stderr) is a terminal |

### Output destinations

`with_output` takes an `Output`: `Stdout` (default), `Stderr`, `File(path)`, or `Both(path)` (stdout plus a file). Use `Stderr` in
command-line tools so stdout stays free for program output.
File output is opened in append mode (created if missing) and written by a background
thread, so logging calls do not wait for the disk. See [The guard](#the-guard).

### The guard

`init()` returns a `LoggerGuard`. Keep it alive for as long as you log, typically as the
first binding in `main`:

```rust
let _guard = Logger::new()
    .with_output(Output::File("app.log".into()))
    .init()?;
```

The guard owns the background thread that writes the file. Dropping it flushes every
queued line and stops the thread, so:

- Bind it to `_guard`, not to a bare `_`: `let _ = ...init()?;` drops it immediately and
  nothing reaches the file. The type is `#[must_use]`, so the compiler warns about a
  discarded guard.
- `std::process::exit` skips destructors, so lines still queued at that point are lost.
  Return from `main` instead.
- The queue holds up to 128 000 lines and no line is ever dropped; if it fills up, the
  logging call waits for room.
- With `Stdout` or `Stderr` the guard holds nothing, but keep binding it so the code does
  not depend on the output.

### Colours

By default ANSI colours are used only when the stream being written to is a terminal, so piped output,
Docker and CI logs stay free of escape codes. Call `colored(true)` or `colored(false)`
to override that. File output is never coloured, whatever you choose. The `NO_COLOR`
environment variable is not read; use `colored(false)` if you want to honour it
yourself.

### Timestamps

By default every log line carries an RFC 3339 timestamp in UTC (for example
`2026-10-01T12:00:00.123456Z`), in both text and JSON output. The sub-second
fraction has a variable length because trailing zeros are trimmed. Local time is not
supported, so lines from different hosts are easy to correlate.

Call `with_timestamp(false)` to leave the timestamp out, for example when journald or
your container runtime already adds its own. In JSON output the `timestamp` field is
then omitted too.

### Environment filter

`with_env_filter("info,my_crate=debug")` takes the directives as a string.
`with_env_filter_from_env()` reads them from `RUST_LOG` instead. If `RUST_LOG` is unset
only the level applies, and an invalid value makes `init()` return
`LoggerError::InvalidEnvFilter`. A global directive in the filter (such as
`RUST_LOG=warn`) takes precedence over `with_level`.

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
cargo run --example env_filter  # per-target filter passed as a string
cargo run --example env_filter_from_env  # filter read from RUST_LOG
cargo run --example file        # write to stdout + a file at once
cargo run --example colored     # force ANSI colours off
cargo run --example no_timestamp  # omit the timestamp
cargo run --example stderr      # log to stderr, keep stdout for program output
```

## Limitations

`tiny-tracing` is deliberately a thin wrapper, so some things are left out on purpose:

- **File writes need the guard.** They are non-blocking (a background thread, via
  [`tracing-appender`](https://crates.io/crates/tracing-appender)), which means the
  `LoggerGuard` returned by `init()` must stay alive and the process must end normally to
  flush the queue.
- **Stdout and stderr writes are synchronous too.** Each log line is written on the
  calling thread, which waits until the write finishes. That is fine at normal log
  volumes, but a slow consumer (a full pipe, a stalled log shipper) can stall the
  application. There is no non-blocking option for these streams.
- **No file rotation.** The log file is only ever appended to and grows without bound.
  Rotate it externally (for example with `logrotate`) or use
  [`tracing-appender`](https://crates.io/crates/tracing-appender)'s rolling files.
- **In containers, log to stdout.** In Docker, Kubernetes or systemd, keep the default
  `Output::Stdout` and let the platform collect and rotate the logs, rather than writing
  files inside the container. `LogFormat::Json` is usually the best fit for log
  aggregators, and colours switch off by themselves because stdout is not a terminal.
  Use `Output::Stderr` instead for command-line tools whose stdout carries data.

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

cog install-hook --all                            # install the git pre-commit hook (once per clone)
```

### Pre-commit hook

The hook is defined in `cog.toml` (`[git_hooks.pre-commit]`). On every commit it runs
[`prek`](https://github.com/j178/prek) with `.pre-commit-config.yaml`, then
`cargo nextest run`, `cargo fmt --all -- --check` and `cargo check`.

Git does not version hooks, so install it once after cloning with
`cog install-hook --all`, and re-run it with `--overwrite` after editing `cog.toml`.
Do not run `prek install`: it would replace the hook that `cog` generates. You need
`cog`, `prek` and `cargo-nextest` installed locally.

Releases are automated via [cocogitto](https://docs.cocogitto.io/) (Conventional Commits).
See the [release skill](.claude/skills/release/SKILL.md) for the full workflow.

### Minimum supported Rust version (MSRV)

`rust-version` in `Cargo.toml` is the oldest Rust the crate promises to build with. It is
independent of `rust-toolchain.toml`, which only pins the toolchain used for development.
CI checks it with the `msrv` job. After bumping dependencies or adding code that needs a
newer compiler, measure the real minimum with
[`cargo-msrv`](https://github.com/foresterre/cargo-msrv):

```bash
cargo install cargo-msrv --locked
cargo msrv find        # tries older toolchains until the build breaks
```

Then update `rust-version` in `Cargo.toml`, the MSRV badge at the top of this file and the
`msrv` job in `.github/workflows/ci.yml` to the same version.

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
