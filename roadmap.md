# Roadmap

Planned improvements for `tiny-tracing`, roughly ordered by priority. They should ship
together as the `0.3.0` release.

Nothing pending: every planned item has shipped. Add new ideas here.

## Not planned

These would go beyond a thin wrapper. Use `tracing-subscriber` directly for them:

- File rotation (use `logrotate`, or log to stdout in containers).
- Custom layers, OpenTelemetry or Sentry integration.
- Further format customisation (thread ids, span events, flattened JSON, …).

## Before releasing

- Run `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo fmt --all -- --check` and `cargo deny check`.
- Sweep `README.md`, `AGENTS.md` and the crate docs for drift (see `AGENTS.md`).
- Mention the breaking changes since `0.2.0` in the release notes: `#[non_exhaustive]` on `Output`, `LogFormat` and `LoggerError` (committed as `feat!:`), and the removal of `chrono`, `LocalTimer` and `pub mod time` (commit `7abc3ff`, not marked as breaking, so `--auto` would not notice it).
- Release with the explicit `cog bump --version 0.3.0` rather than `--auto`. See the [release skill](.claude/skills/release/SKILL.md).
