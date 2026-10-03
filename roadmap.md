# Roadmap

Planned improvements to make `tiny-tracing` safe to rely on in production while keeping
it a thin, simple wrapper around `tracing-subscriber`. Items are ordered by priority.
Because several of them change behaviour or public API, ship them together as a
`0.3.0` release.

## Pending work on changes already in the code

Automatic ANSI colour detection (`Logger::colored` is an `Option<bool>`, `None` meaning
"only when stdout is a terminal") and UTC RFC 3339 timestamps (`UtcTime::rfc_3339()`)
are implemented and documented in `README.md` and `AGENTS.md`. They still lack tests.

### 1. Test the colour decision

Extract the decision into a small pure function, for example
`fn use_ansi(colored: Option<bool>, is_terminal: bool) -> bool`, and unit-test the
three cases (`None` with and without a terminal, `Some(true)`, `Some(false)`). The
global subscriber can only be set once per process, so testing through `init()` is
awkward.

Optionally mention the colour behaviour in the crate docs (`src/lib.rs` feature list).

### 2. Test the timestamp format

Add `tests/timestamp.rs` (its own file, so it gets its own process and its own global
subscriber). Log to a temp file, as `tests/file_output.rs` does, and assert the
timestamp looks like `YYYY-MM-DDTHH:MM:SS.ffffffZ` (UTC, RFC 3339) in both text and
JSON output.

Optionally mention in the crate docs (`src/lib.rs`) that timestamps are UTC.

## Cheap improvements

### 3. `Output::Stderr`

CLI tools usually log to stderr so stdout stays free for program output. Add an
`Output::Stderr` variant and handle it in `Logger::init`. The automatic colour decision
must check whether **stderr** is a terminal, not stdout. Update the `Output` docs, the
`colored` doc comment (it says "stdout") and the README.

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
- `cargo deny check` (the `time` dependency tree is new since `chrono` was dropped).
- Sweep `README.md`, `AGENTS.md` and crate docs for drift (see `AGENTS.md`).
- Commit the breaking changes with a `!` type (`feat!:`) or a `BREAKING CHANGE:` footer.
- Release with the explicit `cog bump --version 0.3.0` rather than `--auto`, since
  cocogitto may treat a breaking change as a major bump. See the
  [release skill](.claude/skills/release/SKILL.md).
