# Laravel layer refactor

## What changed
- Introduced service layer under `app/Services`:
  - `RustIssClient` centralises interaction with the Rust backend (shared timeout, retries, env-driven base URL, envelope-aware responses).
  - `DashboardService`, `OsdrService`, `JwstFeedService`, `AstroEventsService` concentrate business logic, so controllers render or orchestrate only.
- Added `App\Support\ApiResponder` to emit the global `{ ok, data | error }` contract with `trace_id`, reusing the same semantics as rust_iss.
- Controllers (`DashboardController`, `OsdrController`, `ProxyController`, `AstroController`) now use constructor injection, no longer instantiate curl/file_get_contents inline.
- Cleaned up routing to avoid duplicate declarations and to improve readability.

## Patterns applied
- **Gateway/Service pattern**: `RustIssClient` acts as an anti-corruption layer for all internal APIs, allowing us to swap the data source (e.g. JWST via AstronomyAPI) in one place.
- **DTO + Presenter**: `OsdrService` encapsulates the flattening/normalisation step that views rely on, making it trivially testable.
- **Response envelope**: `ApiResponder` enforces the mandated 200+`ok=false` policy everywhere, with UUID trace propagation.

## Operational benefits
- Timeouts, retries and headers are configured once via env, preventing per-controller drift.
- We can stub the new services in tests or replace them with fake clients without touching controllers or Blade.
- Consistent JSON contract simplifies the nginx/JS integration and aligns with the rust_iss backend behaviour.

