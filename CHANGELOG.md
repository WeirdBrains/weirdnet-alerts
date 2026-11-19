# Changelog

## [Unreleased]

## [0.1.0] - 2026-11-19

### Added

- `Alerter`: `weirdnet-rules` rule specs over `Observation`s with `fail`, `ok` and `event` actions, per-key dedupe with a repeat interval, and recovery notices.
- `rules::default_rules`: engine crash loops, failed loads and crashes; default route on Wi-Fi or missing; free disk; GPU temperature with hysteresis.
- Sinks: SMTP, JSON webhooks, log and memory, with dry runs; secrets only as `env:` or `file:` references.
- Probes: default route, free disk and GPU temperature.
- `AlertsConfig`: builds the alerter and its sinks from config.
- `push` feature: `PushSink`, wakes through `weirdnet-push` 0.1.
- `backends` feature: `engine::observation` and `engine::monitor` for `weirdnet-backends` 0.1 supervisor events.
