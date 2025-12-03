# rust_iss Architecture Review

## Layered layout
| Layer | Path | Responsibility |
| --- | --- | --- |
| Config | `services/rust-iss/src/config` | Normalises env, scheduler intervals, ports, HTTP timeout/user-agent, `.env` defaults. |
| Domain | `src/domain` | Typed DTOs (`IssEntry`, `IssTrend`, `SpaceSummary`, etc.) with strict `DateTime<Utc>` usage for every `TIMESTAMPTZ`. |
| Clients | `src/clients` | Pure HTTP clients (ISS, OSDR, NASA, SpaceX) with shared `reqwest` builder (`timeout`, `UA`, TLS) and zero business logic. |
| Repo | `src/repo` | SQLx-based repositories. No SQL outside this layer; `INSERT ... RETURNING` for ISS and `ON CONFLICT` upserts for OSDR datasets. |
| Services | `src/services` | Application services with dependency injection via `AppState`. Background schedulers, rate-limits (async mutex), cache refresh orchestration. |
| Handlers | `src/handlers` | Thin Axum handlers → `Result<Json<ApiEnvelope<T>>, ApiError>`, no DB/HTTP calls. |
| Routes | `src/routes` | Declares all public endpoints and wires state. |

## Dependency flow
```
routes → handlers → services → {clients, repo}
                            ↘ config (intervals, URLs, timeouts)
```
`AppState` holds immutable `AppConfig` + `Arc<ServiceRegistry>`, which contains `Arc` instances of each service for DI-friendly cloning.

## Error contract
- `ApiEnvelope<T>` is returned from every handler with HTTP 200.
- Failure path serialises to `{ "ok": false, "error": { "code": "...", "message": "...", "trace_id": "..." } }`.
- `ApiError` generates a UUID `trace_id`; logs include it for correlation.

## Schedulers & rate limiting
- Each external fetch job runs through `spawn_jobs`, driven by configured intervals.
- Services guard critical sections with `tokio::Mutex` so manual `/fetch` calls share the same lock as background workers (prevents overlapping runs and double inserts).
- HTTP clients share a single `reqwest::Client` with per-request timeouts + custom User-Agent to respect upstream limits.

## TIMESTAMPTZ handling
- Repository mappers convert DB rows straight into `DateTime<Utc>` (via SQLx decoding). Domain structs expose UTC-only timestamps, meaning serde always emits RFC3339 + timezone and consumers never see naive time.
- For upstream payloads, helper `pick_datetime` parses ISO strings, `YYYY-MM-DD HH:MM:SS`, or epoch seconds, normalising into UTC before persisting.

## Business logic highlights
- **Upsert vs blind insert**: `OsdrRepo::upsert_item` uses `ON CONFLICT (dataset_id)` to keep a single authoritative row per business key. This avoids duplicate rows that blind inserts would create when upstream replays the same dataset; throughput improves because only deltas incur writes, and consistent dataset IDs enable idempotent cache invalidation.
- **Space cache**: `space_cache` remains the single fan-out table; `SpaceCacheService` owns data refresh & summary building. Any new source only requires a client method + match arm, keeping Blade views untouched.

## Open issues / next actions
1. Expand structured logging (JSON) + add tracing propagation headers once PHP gateway consumes `trace_id`.
2. Harden `reqwest` clients with retry policies (e.g., exponential backoff) when upstreams answer with 429; scheduler already serialises fetches so we can add jitter safely.
3. Cover services with unit & integration tests (SQLx `#[cfg(test)]` + mock clients). Test scenarios will encode acceptance criteria from README once we move to the test step of the plan.

