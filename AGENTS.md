# weirdnet-alerts agents

The collective `AGENTS.md` in `WeirdBrains/weirdnet-agents` is mandatory and wins on conflict.

- No app-specific types or names; the product name in subjects is the application's.
- Secrets are references (`env:`, `file:`), never values; never log them.
- Alerting must never take a node down: sink failures are logged, not returned.
- Keep the default build free of aws-lc (BSD cross builds); `push` may pull it in.
- Build outside the source checkout, with `CARGO_TARGET_DIR` on a scratch disk; clean targets afterwards.
