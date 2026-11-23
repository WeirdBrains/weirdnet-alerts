# Changelog

## [Unreleased]

## [0.2.2] - 2026-11-23

### Changed

- The `backends` feature accepts weirdnet-backends 0.4 (LoRA adapters) as well as 0.2 and 0.3: `weirdnet-backends = ">=0.2, <0.5"`. No API changes; adapter add and remove events become `engine.added` and `engine.removed` like models.

## [0.2.1] - 2026-11-21

### Changed

- Package metadata and docs follow the WeirdBrains public-content rules (Weird Brains LLC as the copyright holder, m3talsmith as the author, no machine details). No code or API changes.

## [0.2.0] - 2026-11-21

### Changed

- Breaking: the `backends` feature takes `weirdnet-backends` 0.2 or 0.3 (on `weirdnet-agent` 0.5) instead of 0.1, so apps on agent 0.5 get one copy of each.

### Added

- `default_rules`: a model removed at run time (`engine.removed`) clears its engine alert.

## [0.1.0] - 2026-11-19

### Added

- `Alerter`: `weirdnet-rules` rule specs over `Observation`s with `fail`, `ok` and `event` actions, per-key dedupe with a repeat interval, and recovery notices.
- `rules::default_rules`: engine crash loops, failed loads and crashes; default route on Wi-Fi or missing; free disk; GPU temperature with hysteresis.
- Sinks: SMTP, JSON webhooks, log and memory, with dry runs; secrets only as `env:` or `file:` references.
- Probes: default route, free disk and GPU temperature.
- `AlertsConfig`: builds the alerter and its sinks from config.
- `push` feature: `PushSink`, wakes through `weirdnet-push` 0.1.
- `backends` feature: `engine::observation` and `engine::monitor` for `weirdnet-backends` 0.1 supervisor events.
