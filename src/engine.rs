//! `weirdnet-backends` engines as alert observations (feature `backends`).

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{broadcast, watch};
use weirdnet_backends::{Supervisor, SupervisorEvent};

use crate::{Alerter, Observation, probe};

/// A supervisor event as an observation: `engine.<kind>` (`engine.crash_loop`,
/// `engine.load_failed`, `engine.ready`, `engine.crashed`, ...) with the event's fields.
pub fn observation(ev: &SupervisorEvent) -> Observation {
    let v = serde_json::to_value(ev).unwrap_or_default();
    let kind = v.get("kind").and_then(|k| k.as_str()).unwrap_or("unknown").to_string();
    Observation::new(format!("engine.{kind}"), v)
}

/// The probes: default route, free space on `disks`, GPU temperatures.
pub async fn probes(disks: &[PathBuf]) -> Vec<Observation> {
    let mut out = Vec::new();
    out.extend(probe::route().await);
    out.extend(disks.iter().filter_map(|d| probe::disk(d)));
    out.extend(probe::gpus().await);
    out
}

/// Runs until `stop` is true: forwards the supervisor's events and runs the probes every
/// `every`.
pub async fn monitor(alerter: Arc<Alerter>, supervisor: Option<Supervisor>, disks: Vec<PathBuf>, every: Duration, mut stop: watch::Receiver<bool>) {
    let mut events = supervisor.as_ref().map(Supervisor::subscribe);
    let mut tick = tokio::time::interval(every);
    tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    loop {
        tokio::select! {
            _ = async { let _ = stop.wait_for(|s| *s).await; } => return,
            ev = next_event(&mut events) => match ev {
                Ok(ev) => { alerter.observe(&observation(&ev)).await; }
                Err(broadcast::error::RecvError::Lagged(n)) => tracing::warn!("alerts: skipped {n} engine events"),
                Err(broadcast::error::RecvError::Closed) => events = None,
            },
            _ = tick.tick() => {
                for obs in probes(&disks).await {
                    alerter.observe(&obs).await;
                }
            }
        }
    }
}

async fn next_event(rx: &mut Option<broadcast::Receiver<SupervisorEvent>>) -> Result<SupervisorEvent, broadcast::error::RecvError> {
    match rx {
        Some(r) => r.recv().await,
        None => std::future::pending().await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::{Thresholds, default_rules};

    #[test]
    fn supervisor_events_raise_and_clear_engine_alerts() {
        let a = Alerter::new("node-a", default_rules(&Thresholds::default()), Vec::new());
        let t = std::time::Instant::now();
        let looped: SupervisorEvent = serde_json::from_value(serde_json::json!({"kind": "crash_loop", "model": "qwen", "crashes": 5})).unwrap();
        let obs = observation(&looped);
        assert_eq!(obs.kind, "engine.crash_loop");
        let r = a.evaluate(&obs, t);
        assert_eq!((r.len(), r[0].key.as_str()), (1, "engine.qwen"));
        assert_eq!(r[0].title, "qwen on node-a is in a crash loop (5 crashes); restarts stopped until it is loaded again with force");
        let ready: SupervisorEvent = serde_json::from_value(serde_json::json!({"kind": "ready", "model": "qwen", "load_secs": 2.5})).unwrap();
        assert_eq!(a.evaluate(&observation(&ready), t)[0].status, crate::AlertStatus::Recovered);
        a.evaluate(&obs, t);
        // `SupervisorEvent::Removed` (backends 0.2.1) as `observation` makes it.
        let removed = Observation::new("engine.removed", serde_json::json!({"kind": "removed", "model": "qwen"}));
        let r = a.evaluate(&removed, t);
        assert_eq!(r.len(), 1);
        assert_eq!((&r[0].status, r[0].title.as_str()), (&crate::AlertStatus::Recovered, "qwen on node-a was removed"));
    }

    #[tokio::test]
    async fn monitor_stops() {
        let a = Arc::new(Alerter::new("node-a", Vec::new(), Vec::new()));
        let (tx, rx) = watch::channel(false);
        let h = tokio::spawn(monitor(a, None, vec![], Duration::from_secs(3600), rx));
        tx.send(true).unwrap();
        tokio::time::timeout(Duration::from_secs(5), h).await.unwrap().unwrap();
    }
}
