# Roadmap

Planned improvements for `tiny-tracing`, roughly ordered by priority. They should ship
together as the `0.3.0` release.

1. **Test the colour decision.** Extract it into a pure function such as `use_ansi(colored: Option<bool>, is_terminal: bool) -> bool` and unit-test the cases (`None` with and without a terminal, `Some(true)`, `Some(false)`). `init()` repeats the decision for stdout and for stderr; calling the function from both places removes the duplication. Testing through `init()` is awkward because the global subscriber can only be set once per process.
2. **Test `Output::Stderr`.** Check that `init()` succeeds and a second call returns `LoggerError::TryInitError` (own test file), cover the stderr colour decision through item 1, and verify the lines land on stderr and not on stdout by running the compiled `examples/stderr` binary with `std::process::Command` and capturing both streams.

## Not planned

These would go beyond a thin wrapper. Use `tracing-subscriber` directly for them:

- File rotation (use `logrotate`, or log to stdout in containers).
- Non-blocking file writes (`tracing-appender`).
- Custom layers, OpenTelemetry or Sentry integration.
- Further format customisation (thread ids, span events, flattened JSON, …).

## Before releasing

- Run `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo fmt --all -- --check` and `cargo deny check`.
- Sweep `README.md`, `AGENTS.md` and the crate docs for drift (see `AGENTS.md`).
- Mention the breaking changes since `0.2.0` in the release notes: `#[non_exhaustive]` on `Output`, `LogFormat` and `LoggerError` (committed as `feat!:`), and the removal of `chrono`, `LocalTimer` and `pub mod time` (commit `7abc3ff`, not marked as breaking, so `--auto` would not notice it).
- Release with the explicit `cog bump --version 0.3.0` rather than `--auto`. See the [release skill](.claude/skills/release/SKILL.md).
