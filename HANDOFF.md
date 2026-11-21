# Handoff

Running log for [weirdnet-alerts](https://github.com/WeirdBrains/weirdnet-alerts), newest first. Shared rules: the collective `AGENTS.md` and this repo's [AGENTS.md](AGENTS.md).

## 2026-11-21: weirdnet-backends 0.2 (weirdnet-agent 0.5)

What changed (commit a47e16e, on main, unreleased; see [CHANGELOG.md](CHANGELOG.md) `Unreleased`):

- Breaking: the `backends` feature takes [weirdnet-backends](https://crates.io/crates/weirdnet-backends) `">=0.2, <0.4"` instead of 0.1, so it no longer pulls weirdnet-agent 0.4 next to an app's 0.5. `engine::observation` and `engine::monitor` take that crate's `SupervisorEvent` and `Supervisor`, which is why this is a minor bump. The range also covers the planned backends 0.3, whose `Supervisor` and `SupervisorEvent` this crate uses unchanged.
- `default_rules`: `engine.removed` (a model removed at run time, backends 0.2.1 and later) clears that model's engine alert.

Checks run: `cargo fmt --check`; clippy `-D warnings` default and `--all-features`; tests default (12), `--all-features` (15) and `--no-default-features`; `cargo doc` with `-D warnings`; `cargo package`. Cargo.lock now holds one weirdnet-agent (0.5.0).

Not tested: against an unreleased backends 0.3 (the range is checked again when it exists); live SMTP, webhook and push sinks (unchanged).

What's left and next: a minor release (proposed 0.2.0) once m3talsmith approves it.
