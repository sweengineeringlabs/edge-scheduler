# swe-edge-runtime-scheduler

Runtime-agnostic async scheduler contract — `Scheduler` trait plus an optional tokio-backed implementation.

Library crates depend on this crate for the `Scheduler` trait only; the concrete tokio runtime is an opt-in feature (`tokio-rt`) so the contract stays executor-neutral in production code.

## Features

- **Runtime-neutral contract** — `Scheduler` trait runs any `Future<Output = Result<(), SchedulerError>>` with no tokio dep in the default feature set
- **Tokio backend** — opt-in `tokio-rt` feature wires a multi-threaded tokio runtime with configurable worker count, stack size, blocking pool, and thread name
- **Typed configuration** — `TokioSchedulerConfig` deserialises from TOML `[scheduler]` sections; `TokioSchedulerConfigBuilder` for programmatic construction
- **Validator contract** — `Validator` trait enforces config validity before a scheduler is built; rejects unsafe values (e.g. stack < 64 KiB)
- **Configurable contract** — `Configurable` trait for types that expose their own scheduler configuration surface
- **SAF factory** — `SchedulerSvc` is the single public construction point; callers never name the concrete runtime type

## Basic Usage

```rust
use swe_edge_runtime_scheduler::{Scheduler, SchedulerError, SchedulerSvc, TokioSchedulerConfig};

let cfg = TokioSchedulerConfig::default();
let scheduler = SchedulerSvc::tokio_scheduler(cfg, "my-app");

scheduler.run(async {
    // your async work here
    Ok(())
})?;
```

## Configuration via TOML

```toml
[scheduler]
workers            = 4      # worker threads (default: tokio picks based on CPU count)
thread_stack_kib   = 512    # per-thread stack in KiB (default: tokio default, min 64)
max_blocking_threads = 32   # blocking pool size (default: tokio default)
thread_name        = "edge" # thread name prefix (default: "tokio-worker")
```

## Programmatic Configuration

```rust
use std::num::NonZeroUsize;
use swe_edge_runtime_scheduler::{SchedulerSvc, TokioSchedulerConfigBuilder};

const WORKERS: NonZeroUsize = match NonZeroUsize::new(4) {
    Some(n) => n,
    None => panic!("4 is nonzero"),
};

let cfg = TokioSchedulerConfigBuilder::new()
    .workers(WORKERS)
    .thread_stack_kib(512)
    .thread_name("edge")
    .build();

let scheduler = SchedulerSvc::tokio_scheduler(cfg, "my-app");
```

## Validation

```rust
use swe_edge_runtime_scheduler::{SchedulerSvc, TokioSchedulerConfig, Validator};

let cfg = TokioSchedulerConfig { thread_stack_kib: Some(16), ..Default::default() };

match SchedulerSvc::validate(&cfg) {
    Ok(()) => { /* safe to build */ }
    Err(msg) => eprintln!("Invalid config: {msg}"),
}
```

## Feature Flags

| Flag | Default | Description |
|------|---------|-------------|
| `tokio-rt` | enabled | Tokio multi-thread runtime backend (`TokioScheduler`, `TokioSchedulerConfig`) |

## Architecture

Follows the [SEA (Structural Engineering Architecture)](https://github.com/sweengineeringlabs/edge) layering convention:

```
api/      — Scheduler, Validator, Configurable traits + all public types
spi/      — TokioScheduler (pub(crate) concrete implementation)
saf/      — SchedulerSvc factory (the only public construction surface)
gateway/  — re-exports; consumers never import below this layer
```
