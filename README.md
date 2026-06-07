# swe-edge-runtime-scheduler

> **TLDR:** Async executor abstraction for swe-edge — pluggable `Scheduler` trait with a tokio-backed default; swap executors without rewriting application startup. See [Overview](scm/docs/README.md) for details.

Runtime-agnostic async scheduler for `swe-edge-runtime`. Ships a tokio-backed
`TokioScheduler` by default; bring your own runtime by implementing `Scheduler`.

## Documentation

| Document | Description |
|----------|-------------|
| [Overview](scm/docs/README.md) | WHAT + WHY — capabilities and design rationale |
