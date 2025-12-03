# Infrastructure & Compose Audit

## Service Footprint
- `db` (`postgres:16`): runs with default Debian image, exposes 5432 to host, seeds via `db/init.sql`, no CPU/memory caps, secrets embedded in compose.
- `rust_iss`: multi-stage build (rust:slim → debian:12-slim), publishes 8081→3000, depends on DB only via healthcheck, exposes multiple external API URLs/keys via env with fallbacks to empty strings.
- `php` (Laravel dashboard): built from `php:8.3-fpm-alpine`, volume-mounts mutable state into `appdata`; secrets/API keys hardcoded in compose; depends on both DB and rust service; entrypoint script applied at runtime.
- `nginx`: proxy layer referencing same `appdata` volume, but no rate limiting, gzip, or cache controls configured.
- `pascal_legacy`: Debian-based image installing full FreePascal toolchain + `postgresql-client`; no scheduler/cron definition, runs single binary loop via `run.sh`, logs unspecified.

## Compliance Gaps
1. **Secrets**: API keys and DB creds are in `docker-compose.yml`, violating “no hardcoded secrets”; no `.env` indirection.
2. **Resource efficiency**: heavy Debian runtime layers for Rust & Pascal; Postgres not pinned to slim/alpine variant; no `deploy.resources` or environment-based tuning ⇒ too heavy for “слабые машины”.
3. **Observability/security**: HTTP 200 + `{ok:false}` contract is not enforced across services; no structured logging or trace ids; Nginx lacks basic protections (rate limit, security headers).
4. **Initialization**: Compose relies on implicit state in `appdata` volume; first-run experience is unclear and may violate “собирается с нуля”.
5. **Networking**: Single `backend` network used for both internal API and public exposure; no separation or firewalling; services bind to host ports unconditionally.

## Light-Weighting & Hardening Actions
1. Introduce `.env` (and `.env.example`) consumed by compose; strip secrets from YAML; document mandatory variables.
2. Switch to slimmer base images:
   - `db`: `postgres:16-alpine` with tuned shared buffers.
   - `rust_iss`: builder `rust:1.82-alpine3.20`, runtime `gcr.io/distroless/cc` or `alpine:3.20`.
   - `pascal_legacy`: `alpine` + `fpc` cross-package or replace with custom CLI (see future steps).
3. Ensure all services mount read-only volumes where possible, remove unused `appdata` writes by baking code into image.
4. Add explicit healthchecks/timeouts for `php`, `rust_iss`, `nginx`, `pascal_legacy`; ensure `depends_on` uses `condition: service_healthy`.
5. Apply resource constraints + shared `.env` toggles (`COMPOSE_PROJECT_NAME`, `FETCH_EVERY_SECONDS`, rate-limit thresholds).
6. Harden Nginx (security headers, buffering, gzip, upstream timeouts) to reduce PHP load.
7. Document cold start procedure in README + include diagrams, satisfying audit trail requirement.

## Next Steps
- Refactor rust/PHP services to follow clean architecture layers and unified error schema.
- Redesign legacy Pascal execution as sidecar CLI with cron + stdout/stderr logging.
- Produce acceptance evidence (code excerpts, logs, screenshots) for final report.

