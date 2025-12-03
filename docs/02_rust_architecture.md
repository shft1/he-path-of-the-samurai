# Архитектура rust_iss

## Слоистая схема

| Слой | Путь | Ответственность |
|------|------|-----------------|
| Config | `src/config` | Чтение окружения, интервалы планировщика, таймауты, User-Agent |
| Domain | `src/domain` | DTO (`IssEntry`, `IssTrend`, `SpaceSummary`) c `DateTime<Utc>` |
| Clients | `src/clients` | HTTP-клиенты ISS/OSDR/NASA/SpaceX на общем `reqwest::Client` |
| Repo | `src/repo` | SQLx-репозитории, весь SQL сосредоточен здесь |
| Services | `src/services` | Бизнес-логика, фоновые задачи, rate-limit через `tokio::Mutex` |
| Handlers | `src/handlers` | Тонкие Axum-хендлеры → `Result<Json<ApiEnvelope<T>>, ApiError>` |
| Routes | `src/routes` | Регистрация публичных эндпоинтов и DI state |

Поток зависимостей:
```
routes → handlers → services → {clients, repo}
                            ↘ config
```
`AppState` хранит `AppConfig` и `Arc<ServiceRegistry>` для безопасного шаринга служб.

## Контракт ошибок
- Любой хендлер отдаёт HTTP 200 с `ApiEnvelope<T>`.
- Ошибка сериализуется как `{ "ok": false, "error": { "code": "...", "message": "...", "trace_id": "..." } }`.
- `trace_id` генерируется в `ApiError` и прокидывается в логи.

## Планировщики и запросы наружу
- `spawn_jobs` поднимает фоновые задачи с интервалами из env.
- `tokio::Mutex` предотвращает одновременный `/fetch` и планировщик.
- Один `reqwest::Client` с таймаутом и кастомным User-Agent для всех клиентов.

## Работа со временем
- SQLx конвертирует поля БД прямо в `DateTime<Utc>`, поэтому на выходе всегда RFC3339.
- Входящие payload'ы приводятся в UTC через хелпер `pick_datetime`.

## Upsert вместо слепого INSERT
- `OsdrRepo::upsert_item` использует `ON CONFLICT (dataset_id)` и тем самым избегает дублей при повторной загрузке того же набора.
- Это ускоряет обработку и обеспечивает идемпотентность: обновляем только изменившиеся строки.

