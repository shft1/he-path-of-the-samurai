#!/bin/sh
set -e

MODE="${LEGACY_MODE:-loop}"

if [ "$MODE" = "cron" ]; then
  CRON_SCHEDULE="${LEGACY_CRON_SCHEDULE:-*/5 * * * *}"
  echo "[legacy-cli] Starting in cron mode: $CRON_SCHEDULE"
  echo "$CRON_SCHEDULE python -m app.main" > /tmp/crontab
  exec supercronic /tmp/crontab
elif [ "$MODE" = "once" ]; then
  echo "[legacy-cli] Running once"
  exec python -m app.main
else
  echo "[legacy-cli] Starting in loop mode"
  exec python -m app.main
fi

