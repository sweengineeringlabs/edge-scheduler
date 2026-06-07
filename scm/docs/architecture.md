# Architecture — edge-scheduler

## Sequence

> The caller creates a `TokioScheduler` via `SchedulerSvc`, submits a future via `run()`, and the scheduler blocks the calling thread until the future completes or the runtime shuts down.

```mermaid
sequenceDiagram
    participant App
    participant SchedulerSvc
    participant TokioScheduler
    participant TokioRuntime

    App->>SchedulerSvc: create_config_builder()
    SchedulerSvc-->>App: ApplicationConfigBuilder

    App->>App: configure: thread_count, thread_name_prefix

    App->>SchedulerSvc: tokio_scheduler(config, "worker")
    SchedulerSvc->>TokioRuntime: build multi-thread runtime
    TokioRuntime-->>SchedulerSvc: Runtime handle
    SchedulerSvc-->>App: TokioScheduler

    App->>TokioScheduler: run(async { do_work().await })
    TokioScheduler->>TokioRuntime: block_on(future)
    TokioRuntime-->>TokioScheduler: Result<(), SchedulerError>
    TokioScheduler-->>App: Result<(), SchedulerError>
```

## Data Flow

> A `Future` enters `Scheduler::run`; the tokio multi-thread runtime drives it to completion; `Result<(), SchedulerError>` exits.

```mermaid
flowchart LR
    A["TokioSchedulerConfig\n───────────\nworker_threads: usize\nthread_name: String\nenable_io: bool\nenable_time: bool"] --> B["SchedulerSvc\n::tokio_scheduler"]
    B --> C["TokioScheduler\n(wraps tokio::Runtime)"]

    D["Future<Output =\nResult<(), SchedulerError>>\n+ Send + 'static"] --> C
    C --> E["tokio Runtime\n::block_on(future)"]
    E -->|Ok| F["() — completed"]
    E -->|Err| G["SchedulerError\n::StartFailed(reason)\n::InvalidConfig(reason)"]
```
