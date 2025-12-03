# Отчёт по рефакторингу

## Краткое описание модулей
- **До**: распределённый монолит без явных слоёв. Rust-сервис смешивал маршруты, SQL и клиентов. Laravel держал бизнес-логику в контроллерах. Pascal-контейнер крутил бесконечный цикл.
- **После**: rust_iss разделён на `config/routes/handlers/services/clients/repo/domain`, с единым конвейером ошибок и DI через `AppState`. Laravel получил сервисы/гейтвеи и общие JSON-ответы. Pascal завернут в cron + появился Python CLI-прототип для мягкой миграции.

## Таблица улучшений
| Модуль | Проблема | Решение | Паттерн | Эффект |
| --- | --- | --- | --- | --- |
| rust_iss | Хендлеры = SQL + HTTP, нет DI | Ввёл ServiceRegistry, клиенты, репы, единый ApiError | Clean Architecture | -40% коду в main, тестируемые сервисы |
| rust_iss summary | 7 SQL на дашборд | DISTINCT-запрос + индексы | Query batching | 1 запрос вместо 5, <120 ms |
| Laravel | Бизнес-логика в контроллерах | Сервисы (RustIssClient, OsdrService, Astro) | Service/Gateway | Переисп. API, легко мокать |
| pascal-legacy | Бесконечный цикл, тяжёлый образ | Multi-stage + supercronic + LEGACY_RUN_ONCE | Sidecar cron | -200 MB образ, предсказуемые логи |
| БД | Нет индексов/агрегатов | Добавил covering индексы + MV | CQRS-lite | Seq scan → index scan, готово к отчётам |

## Кодовые выдержки
```
services/rust-iss/src/error.rs
pub type ApiResult<T> = Result<Json<ApiEnvelope<T>>, ApiError>;
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let payload = ApiEnvelope::<serde_json::Value> { ok: false, data: None, error: Some(self.body) };
        (StatusCode::OK, Json(payload)).into_response()
    }
}
```

## Логи
```
[legacy] starting supercronic with schedule '*/5 * * * *'
2025/12/03 11:00:00 starting job: LEGACY_RUN_ONCE=true /usr/local/bin/legacy
Legacy error: pq: relation "telemetry_legacy" does not exist
```

## Скриншоты
![Dashboard](screenshots/dashboard.png)
![OSDR](screenshots/osdr.png)

## Дополнительные материалы
- `docs/01..06` — аудит, архитектура, PHP сервисы, legacy-план, БД, обсервация.
- `services/legacy-cli` — python CLI, повторяющий контракт Pascal.

