# План по легаси-генератору

## Контракт
- CSV: `recorded_at TIMESTAMPTZ`, `voltage NUMERIC(5,2)`, `temp NUMERIC(5,2)`, `source_file TEXT`.
- Целевая таблица: `telemetry_legacy(recorded_at, voltage, temp, source_file)`.
- Переменные окружения:
  - `CSV_OUT_DIR` — директория для CSV (volume `csvdata`).
  - `PG*` — подключение к PostgreSQL.
  - `GEN_PERIOD_SEC` — интервал генерации в режиме daemon.
  - `LEGACY_CRON_SCHEDULE` — расписание cron (по умолчанию `*/5 * * * *`).

## Контейнер
- Pascal заменён на Python CLI (`services/legacy-cli`, образ `python:3.12-alpine`).
- `run.sh` выбирает режим (`cron`, `once`, `daemon`) и отправляет логи в stdout/stderr.
- `supercronic` читает расписание и запускает CLI; каждая задача устанавливает `LEGACY_RUN_ONCE=true`.
- CSV остаются на volume, импорт в Postgres выполняется из файлов.

## Маршрут миграции
1. Pascal-утилита держится «на поддержке», но запускается через cron.
2. Python CLI повторяет логику, использует тот же контракт окружения.
3. После проверки можно отключить Pascal и оставить `legacy_cli` основным сервисом.
4. При необходимости CLI можно переписать на Go/Rust — контракт и расписание не меняются.

## Логирование
- Cron-файл формируется на лету (`/tmp/legacy.cron`), учитывает переменные окружения.
- `supercronic` стримит вывод — `docker logs legacy_cli` показывает каждое выполнение.
- Ошибки CSV/SQL фиксируются строками с префиксом `[legacy]`.

