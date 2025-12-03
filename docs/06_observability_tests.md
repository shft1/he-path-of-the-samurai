# Наблюдаемость

## Логи и ошибки
- **Rust**: `tracing` + `ApiError` гарантируют `{ok:false,...}` и `trace_id` во всех ответах. Планировщик логирует сбои под таргетом `scheduler`.
- **Laravel**: `ApiResponder` формирует тот же JSON, исключения попадают в стандартный лог.
- **Legacy**: `run.sh` помечает строки префиксом `[legacy]`, cron-вывод виден в `docker logs`.

## Корреляция запросов
- `RustIssClient` добавляет ретраи и единый таймаут, `trace_id` из rust_iss пишется в PHP-логи для связи запросов.

## Пример лога
```
[legacy] starting supercronic with schedule '*/5 * * * *'
2025/12/03 10:15:00 starting job: LEGACY_RUN_ONCE=true /usr/local/bin/legacy
[legacy] running legacy generator in daemon mode
Legacy error: could not connect to server: Connection refused
```

