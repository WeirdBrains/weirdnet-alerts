# weirdnet-alerts

Node health alerts for Weirdnet nodes and daemons.

- **Rules**: observations (`engine.crash_loop`, `net.route`, `disk`, `gpu`, or your own kinds)
  are matched against [`weirdnet-rules`](https://crates.io/crates/weirdnet-rules) `RuleSpec`s
  whose then-actions are `fail`, `ok` or `event`. Keys and titles are templates over the
  observation's fields (`{host}` is always there). `rules::default_rules` covers engines, the
  default route (Wi-Fi or missing), free disk and GPU temperature, with hysteresis.
- **Dedupe**: a failing key alerts at most once per repeat interval (default an hour), and one
  recovery notice goes out when it clears. Events are deduped like failures.
- **Sinks**: SMTP (lettre, STARTTLS or TLS; plain SMTP only to a loopback relay), JSON webhooks
  (https, or http to loopback), the log, an in-memory sink, and with the `push` feature phone
  wakes through [`weirdnet-push`](https://crates.io/crates/weirdnet-push). Every network sink has
  a dry-run mode that records what it would send. Sink failures are logged, never returned.
- **Secrets** are references, `env:NAME` or `file:/path`, never values in the config.
- **Probes**: default route (Linux `ip route get`), free disk (Unix `statvfs`), GPU temperature
  (`nvidia-smi`). Elsewhere they report nothing.
- **Config**: `AlertsConfig` (serde, TOML-friendly) builds the alerter and its sinks.

```rust
use weirdnet_alerts::{AlertsConfig, Observation};

let cfg: AlertsConfig = toml::from_str(&text)?;
let alerter = cfg.alerter("gpu-box")?.with_product("mynode");
alerter.observe(&Observation::new("disk", serde_json::json!({"path": "/", "free_pct": 4.0}))).await;
```

## Features

- `push`: `PushSink` wakes the owner's devices (`WakeCard`s) with reason `MessageWaiting`. The
  wake carries no alert content; the woken app fetches alerts from the node over Weirdnet. This
  pulls in `weirdnet-push`'s HTTPS client (reqwest with rustls, which uses aws-lc), so leave it
  off for BSD cross builds.
- `backends`: `engine::observation` turns [`weirdnet-backends`](https://crates.io/crates/weirdnet-backends)
  `SupervisorEvent`s into `engine.<kind>` observations, and `engine::monitor` feeds them and the
  probes to an alerter until stopped.
