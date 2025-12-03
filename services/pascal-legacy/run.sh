#!/usr/bin/env bash
set -euo pipefail

mode=${LEGACY_MODE:-cron}

log() {
  printf '[legacy] %s\n' "$*"
}

if [[ "$mode" == "cron" ]]; then
  schedule=${LEGACY_CRON_SCHEDULE:-'*/5 * * * *'}
  cron_file=/tmp/legacy.cron
  log "starting supercronic with schedule '${schedule}'"
  printf '%s LEGACY_RUN_ONCE=true /usr/local/bin/legacy\n' "$schedule" > "$cron_file"
  exec /usr/local/bin/supercronic "$cron_file"
fi

if [[ "$mode" == "once" ]]; then
  log "running legacy generator once"
  export LEGACY_RUN_ONCE=true
  exec /usr/local/bin/legacy
fi

log "running legacy generator in daemon mode"
export LEGACY_RUN_ONCE=false
exec /usr/local/bin/legacy
