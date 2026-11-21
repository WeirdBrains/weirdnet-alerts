# Handoff

Running log for [weirdnet-alerts](https://github.com/WeirdBrains/weirdnet-alerts), newest first. Shared rules: the collective `AGENTS.md` and this repo's [AGENTS.md](AGENTS.md).

## 2026-11-21: release 0.2.0

What changed: release `v0.2.0` (tag [v0.2.0](https://github.com/WeirdBrains/weirdnet-alerts/releases/tag/v0.2.0), the `release: v0.2.0` commit right after this entry), approved by m3talsmith. It ships the `weirdnet-backends` 0.2/0.3 move and the `engine.removed` rule (a47e16e) to [crates.io](https://crates.io/crates/weirdnet-alerts/0.2.0); docs at [docs.rs](https://docs.rs/weirdnet-alerts/0.2.0).

Checks run: the release script's checks from [release.toml](release.toml) (fmt, clippy `-D warnings` and tests with all features, `cargo doc -D warnings`), then `cargo publish`.

Not tested: as in the entry below; weirdnet-backends 0.3.0 is released next and checked against this range then.

What's left and next: nothing here until the next change.

## 2026-11-21: weirdnet-backends 0.2 (weirdnet-agent 0.5)

What changed (commit a47e16e, on main, unreleased; see [CHANGELOG.md](CHANGELOG.md) `Unreleased`):

- Breaking: the `backends` feature takes [weirdnet-backends](https://crates.io/crates/weirdnet-backends) `">=0.2, <0.4"` instead of 0.1, so it no longer pulls weirdnet-agent 0.4 next to an app's 0.5. `engine::observation` and `engine::monitor` take that crate's `SupervisorEvent` and `Supervisor`, which is why this is a minor bump. The range also covers the planned backends 0.3, whose `Supervisor` and `SupervisorEvent` this crate uses unchanged.
- `default_rules`: `engine.removed` (a model removed at run time, backends 0.2.1 and later) clears that model's engine alert.

Checks run: `cargo fmt --check`; clippy `-D warnings` default and `--all-features`; tests default (12), `--all-features` (15) and `--no-default-features`; `cargo doc` with `-D warnings`; `cargo package`. Cargo.lock now holds one weirdnet-agent (0.5.0).

Not tested: against an unreleased backends 0.3 (the range is checked again when it exists); live SMTP, webhook and push sinks (unchanged).

What's left and next: a minor release (proposed 0.2.0) once m3talsmith approves it.
