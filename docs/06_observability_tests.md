# Observability & tests

## Unified logging & errors
- **Rust**: uses `tracing` with EnvFilter, `ApiError` ensures every handler returns `{ok:false,...}` plus `trace_id`. Scheduler jobs log failures under `scheduler` target.
- **Laravel**: `ApiResponder` mirrors the same payload. Controllers bubble up source exceptions so stack traces land in Laravel logs while HTTP clients get deterministic JSON.
- **Legacy**: `run.sh` prefixes every message with `[legacy]`, so container logs can be tailed, and cron output is not swallowed.

## Trace propagation
- `RustIssClient` adds retries + consistent timeout, so PHP proxies can log upstream issues. `trace_id` from rust_iss is surfaced to PHP logs for correlation.

## Test harness
- `services/rust-iss` now includes unit tests (`cargo test`) for geodesic computations and timestamp parsing. Future additions: service-layer tests with `sqlx::test` macros pointing at ephemeral Postgres.
- PHP services become testable thanks to constructor injection; add PHPUnit tests by mocking `RustIssClient` (plan captured in `docs/03_php_refactor.md`).
- Legacy CLI can be exercised via `LEGACY_RUN_ONCE=true python -m app.main`, enabling smoke tests in CI without waiting for cron.

## Sample log excerpt
```
[legacy] starting supercronic with schedule '*/5 * * * *'
2025/12/03 10:15:00 starting job: LEGACY_RUN_ONCE=true /usr/local/bin/legacy
[legacy] running legacy generator in daemon mode
Legacy error: could not connect to server: Connection refused
```

