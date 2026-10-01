# Roadmap

Planned improvements to make `tiny-tracing` safe to rely on in production while keeping
it a thin, simple wrapper around `tracing-subscriber`. Items are ordered by priority.
Because several of them change behaviour or public API, ship them together as a
`0.3.0` release.

## Production issues

### 1. ANSI colours are always on for stdout

**Where:** `Logger::init` in `src/config.rs` builds the stdout layer with
`fmt_layer(std::io::stdout, true)`, so `ansi` is hardcoded to `true`.

**Problem:** when stdout is not a terminal (a pipe, `> file`, Docker/journald, CI logs)
the text output still contains escape codes.

**Goal:** enable colours only when stdout is a TTY and `NO_COLOR` is not set.

**Hints:**
- `std::io::IsTerminal` is in the standard library; no new dependency needed.
- Decide the precedence between TTY detection and `NO_COLOR` (the
  [NO_COLOR](https://no-color.org) convention: any non-empty value disables colour).
- Keep file output colour-free, as it is today.

**Done when:** tests cover colour on/off, and the README states when colours are used.

### 2. Ambiguous timestamps

**Where:** `LocalTimer` in `src/time.rs` formats `%Y-%m-%d %H:%M:%S` in local time.

**Problem:** no UTC offset and no sub-second precision. In JSON output, which is what
log aggregators (Loki, Datadog, CloudWatch, …) ingest, this is ambiguous and hard to
correlate across hosts.

**Goal:** emit RFC 3339 timestamps that carry the offset (or are in UTC).

**Open decision:** UTC always, or local time with offset? Either way, JSON should be
unambiguous.

**Hints:**
- `tracing-subscriber` has a `chrono` feature with `ChronoUtc` and `ChronoLocal`
  timers; using them could make `LocalTimer` unnecessary.
- `pub mod time` is public API, so removing or changing `LocalTimer` is a breaking
  change — another reason to do it in `0.3.0`.

**Done when:** a test asserts the timestamp format in both text and JSON output.

## Cheap improvements

### 3. `Output::Stderr`

CLI tools usually log to stderr so stdout stays free for program output. Add an
`Output::Stderr` variant and handle it in `Logger::init` (apply the same TTY/`NO_COLOR`
rule as stdout, checking stderr instead). Update the `Output` docs and the README.

### 4. Read the filter from the environment

The library never reads `RUST_LOG`; callers must pass the string themselves, even though
the crate docs mention `RUST_LOG`. Consider a builder method such as
`with_env_filter_from_env()` that reads `RUST_LOG` when set and falls back to
`with_level` otherwise. Keep the precedence rules documented on `with_level`.

### 5. `#[non_exhaustive]` on public enums

Mark `Output`, `LogFormat` and `LoggerError` as `#[non_exhaustive]` so adding variants
later (such as `Output::Stderr`) is not a breaking change for users who `match` on them.
Do this in the same release as item 3. Adding it is itself breaking, which is fine in
`0.x`.

### 6. Lower the MSRV

`rust-version` in `Cargo.toml` is set to the latest stable toolchain (the same one pinned
in `rust-toolchain.toml`), which forces every user to be on it. Edition 2024 already
sets a floor of 1.85. Measure the real minimum with `cargo-msrv`, set `rust-version` to
it, and keep the CI toolchain independent. Remember to update the MSRV badge in the
README and add an MSRV job to CI.

## Not planned

These would add complexity beyond the goal of a thin wrapper. Users who need them should
use `tracing-subscriber` directly:

- File rotation (use `logrotate`, or log to stdout in containers).
- Non-blocking file writes (`tracing-appender`).
- Custom layers, OpenTelemetry or Sentry integration.
- Further format customisation (thread ids, span events, flattened JSON, …).

## Checklist before releasing

- `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`,
  `cargo fmt --all -- --check`.
- Sweep `README.md`, `AGENTS.md` and crate docs for drift (see `AGENTS.md`).
- Release with `cog bump` as described in the
  [release skill](../.claude/skills/release/SKILL.md).
