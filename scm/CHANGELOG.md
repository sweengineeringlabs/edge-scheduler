# Changelog

## [0.3.4] — 2026-06-14

- `Configurable` trait added to `api/traits/` — exposes `config_builder`, `service`, `default_tokio_config`, `tokio_config_builder`
- `TokioSchedulerConfig` and `TokioSchedulerConfigBuilder` moved from `spi/` to `api/types/` (SEA rule 160)
- `configurable_svc.rs` and `validator_svc.rs` added to `saf/` (rules 218, 220, 221)
- All `api/traits/` re-exports removed from `saf/`; traits exported via `gateway/` only (rule 126)
- `api/` re-exports now route through `_svc.rs` files; `saf/mod.rs` no longer contains `pub use crate::api::` (rule 232)
- `TokioScheduler` made `pub(crate)` (rule 50); inline unit tests added to all `spi/` fns (rule 124)
- Full `_happy`/`_error`/`_edge` test coverage for all `saf/` fns and `api/` trait fns (rules 221, 222)
- All `expect()`/`expect_err()` calls replaced with `match`/`const` patterns (`expect_used = "deny"`)
- Arch audit: **173/173 passed, 0 failed**

## [0.3.3] — 2026-06-05

- Architecture diagrams (sequence + dataflow) added to `docs/`
- W3H overview, TLDR, and documentation table added
- CI: no-AI-attribution workflow; pre-commit hook path fixes for post-SEA-migration layout
- Dev-dep bumped: `edge-domain` v0.1.0 → v0.8.0

## [0.3.0] — 2026-05-28

- Migrated to formalized SEA package layout (`main/src/{api,spi,saf,gateway}/`)
- `Scheduler` trait moved to `api/traits/`; `TokioScheduler` moved to `spi/tokio/`
- `SchedulerSvc` introduced as the single public factory type in `saf/`

## [0.2.0]

- `swe-edge-runtime-scheduler` introduced as a fully standalone crate (no `edge-runtime` dep)
- `Scheduler` trait + `TokioScheduler` implementation
- `TokioSchedulerConfig` with worker count, stack size, blocking pool, thread name fields
- `Validator` trait + `SchedulerConfigValidator` for config pre-flight checks
- `SchedulerError` variants: `StartFailed`, `InvalidConfig`
