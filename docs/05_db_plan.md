# Database optimisation plan

## Structural tweaks
- Added covering indexes:
  - `idx_iss_fetch_log_fetched_at` + `idx_iss_fetch_log_source_time` for `ORDER BY fetched_at DESC` and per-source lookups.
  - `idx_telemetry_legacy_recorded_at` so dashboards can page through historical telemetry without seq scans.
  - `idx_cms_pages_slug` to keep the CMS single-row queries fast.
- Materialized view `telemetry_legacy_hourly` pre-aggregates voltage/temperature by hour, which can be refreshed out-of-band (`REFRESH MATERIALIZED VIEW telemetry_legacy_hourly;`) for analytics widgets without stressing the raw table.

## Query reductions
- `CacheRepo::latest_by_sources` fetches APOD/NEO/FLR/CME/SpaceX snapshots in one DISTINCT query, cutting the previous 5 round-trips down to 1. `SpaceCacheService::summary` now consumes that map directly.
- Rust service only issues two additional queries (ISS last + OSDR count), so the dashboard summary endpoint now runs 3 SQL statements instead of 7.

## Next steps
1. Schedule `REFRESH MATERIALIZED VIEW telemetry_legacy_hourly` every hour (either via the Pascal cron or the future CLI container).
2. Expose the hourly aggregates via rust_iss (`/space/telemetry-hourly`) so the Blade dashboards can plot trends without ad hoc SQL.
3. Consider promoting `osdr_items` & `space_cache` DDL into migrations once we fully remove the self-migrating code from rust_iss.

