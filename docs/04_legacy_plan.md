# Legacy generator plan

## Current contract
- CSV schema: `recorded_at TIMESTAMPTZ`, `voltage NUMERIC(5,2)`, `temp NUMERIC(5,2)`, `source_file TEXT`.
- Target table: `telemetry_legacy(recorded_at, voltage, temp, source_file)`.
- Environment surface:
  - `CSV_OUT_DIR` — location for artefacts (bind-mounted volume).
  - `PG*` — PostgreSQL connectivity.
  - `GEN_PERIOD_SEC` — legacy sleep interval (still honoured in daemon mode).
  - `LEGACY_RUN_ONCE` — new flag so the Pascal binary can terminate after one batch.

## Container hardening
- `services/pascal-legacy/Dockerfile` is now multi-stage: Debian build → Alpine runtime (saves ~180 MB).
- `run.sh` no longer compiles on boot; it drives the runtime mode (`cron`, `once`, or `daemon`) and pushes logs to stdout/stderr.
- `supercronic` schedules the job via `LEGACY_CRON_SCHEDULE` (default `*/5 * * * *`), so weak machines don’t keep a busy Pascal loop alive; each cron run sets `LEGACY_RUN_ONCE=true`.
- All CSVs remain on `csvdata` volume; Postgres COPY still happens from the generated files.

## Migration path
1. **Pascal maintenance mode**: keep the existing binary but run it via cron (already wired). Health/log checks are now deterministic.
2. **Python CLI prototype**: `services/legacy-cli` reproduces the behaviour using `psycopg` and shares the same env contract. It’s published behind the optional `legacy_cli` service with `profiles: ["experimental"]` so it doesn’t run in production until toggled.
3. **Rollout plan**:
   - Run the Python CLI in parallel (experimental profile) writing to a shadow table to compare outputs.
   - Once verified, flip `pascal_legacy` off and promote `legacy_cli` in `docker-compose.yml`.
   - Optionally rewrite the CLI in Go/Rust using the same contract; only the service build context changes.

## Logging & cron manifest
- Cron is rendered at runtime (`/tmp/legacy.cron`) to respect env overrides.
- Job output is streamed by supercronic, so K8s/Compose tailing works out of the box.
- CSV/SQL failures surface as explicit log lines (`[legacy] ...`) or as `ApiResponder` payloads when the new CLI is called via HTTP (future scope).

