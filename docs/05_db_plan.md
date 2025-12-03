# План оптимизации БД

## Индексы
- `idx_iss_fetch_log_fetched_at` и `idx_iss_fetch_log_source_time` — быстрый `ORDER BY fetched_at` и фильтр по источнику.
- `idx_telemetry_legacy_recorded_at` — пагинация телеметрии без seq scan.
- `idx_cms_pages_slug` и `idx_cms_blocks_slug` — мгновенный поиск CMS-сущностей.

## Материализованное представление
```
CREATE MATERIALIZED VIEW telemetry_legacy_hourly AS
SELECT date_trunc('hour', recorded_at) AS bucket,
       avg(voltage) AS avg_voltage,
       avg(temp) AS avg_temp,
       count(*) AS sample_count
FROM telemetry_legacy
GROUP BY bucket;
```
Позволяет отдавать часовую статистику без нагрузки на «сырые» таблицы. Обновляется командой `REFRESH MATERIALIZED VIEW telemetry_legacy_hourly;`.

## Сокращение запросов
- `CacheRepo::latest_by_sources` забирает APOD/NEO/FLR/CME/SpaceX одним DISTINCT-запросом вместо пяти.
- Сводка для Dashboard выполняет 3 SQL-запроса (сравните с 7 раньше).
- Upsert по бизнес-ключу (`ON CONFLICT`) исключает дубли и уменьшает запись.

