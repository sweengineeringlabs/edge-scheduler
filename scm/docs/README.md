# swe-edge-runtime-scheduler

## WHAT

Async executor abstraction for swe-edge — pluggable `Scheduler` trait with a tokio-backed default,
decoupling application startup from runtime choice.

Key capabilities:

- **`Scheduler`** — core trait: `run<F: Future<Output = Result<(), SchedulerError>>>(fut: F) → Result<(), SchedulerError>`; blocks calling thread until completion
- **`TokioScheduler`** — concrete tokio-backed implementation (behind `tokio-rt` feature flag)
- **`TokioSchedulerConfig`** / **`TokioSchedulerConfigBuilder`** — runtime tuning: thread pool size and related settings, loaded from TOML
- **`SchedulerError`** — enum for executor failures: panic, cancellation, runtime unavailable
- **`ApplicationConfigBuilder`** — fluent config assembly for scheduler settings

## WHY

| Problem | Solution |
|---------|----------|
| Application startup tied to `tokio::main`; alternative runtimes require invasive refactoring | `Scheduler` trait decouples the executor; swap tokio for another runtime by providing a different `Scheduler` impl |
| Thread-pool tuning buried in application code | `TokioSchedulerConfig` loaded from `[scheduler]` TOML; changed without recompiling |
| Executor panics surface as opaque `std::panic::catch_unwind` boilerplate | `SchedulerError::Panic` variant provides a typed, matchable error for recovery |
| Diamond dep conflicts when executor types change | One crate, one tag — all consumers pin the same version; kgraph detects conflicts pre-commit |
