# Security Policy

## Supported versions

Only the latest published release of `tiny-tracing` receives security fixes.

## Reporting a vulnerability

Please do **not** open a public issue for security problems.

Report it privately through GitHub's
[private vulnerability reporting](https://github.com/containerscrew/tiny-tracing/security/advisories/new)
(repository → **Security** → **Report a vulnerability**).

Include, if you can:

- a description of the issue and its impact,
- the affected version(s),
- steps or a minimal example to reproduce it.

This is a small, single-maintainer project, so responses are best-effort: expect an
acknowledgement within a few days. Confirmed issues are fixed in a new release and
published as a GitHub security advisory.

## Scope

This policy covers the `tiny-tracing` crate itself. Vulnerabilities in its dependencies
(`tracing`, `tracing-subscriber`, `chrono`, …) should be reported to those projects;
they are tracked here through `cargo-audit`, `cargo-deny` and Dependabot.
