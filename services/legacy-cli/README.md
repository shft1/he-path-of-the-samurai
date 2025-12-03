# Legacy CLI microservice (Python)

Reference implementation that mirrors the Pascal generator contract:

- Generates a CSV with columns `recorded_at, voltage, temp, source_file`.
- Copies rows into `telemetry_legacy` via `COPY ... CSV HEADER`.
- Respects the same env variables as the Pascal version (`CSV_OUT_DIR`, `GEN_PERIOD_SEC`, `PG*`, `LEGACY_RUN_ONCE`).

Usage:

```bash
docker build -t legacy-cli .
docker run --rm \
  -e PGHOST=db -e PGUSER=monouser -e PGPASSWORD=monopass \
  -e PGDATABASE=monolith -e CSV_OUT_DIR=/tmp/csv \
  -v $(pwd)/csv:/tmp/csv \
  legacy-cli
```

Set `LEGACY_RUN_ONCE=true` to run a single batch (suitable for cron) or provide `GEN_PERIOD_SEC` to let the CLI sleep between cycles.

