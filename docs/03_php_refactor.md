# Рефакторинг Laravel-слоя

## Что сделано
- В `app/Services` добавлены сервисы:
  - `RustIssClient` — единый клиент к rust_iss c таймаутами, retries и базовым URL из env.
  - `DashboardService`, `OsdrService`, `JwstFeedService`, `AstroEventsService` — бизнес-логика вынесена из контроллеров.
- `App\Support\ApiResponder` формирует единый ответ `{ ok, data | error }` с `trace_id`.
- Контроллеры используют DI через конструктор и не вызывают curl/file_get_contents напрямую.
- Маршруты сгруппированы без дублей.

## Паттерны

| Паттерн | Назначение |
|---------|------------|
| Gateway/Service | `RustIssClient` экранирует внутренние API и envelope |
| DTO / Presenter | Сервисы готовят данные для Blade без доступа к HTTP/SQL |
| Response Envelope | `ApiResponder` поддерживает политику 200 + `ok=false` |

## Польза
- Таймауты, заголовки и ретраи задаются централизованно.
- Сервисы легко подменяются другими реализациями благодаря DI.
- JSON-контракт совпадает с rust_iss, что упрощает интеграцию с фронтом и логирование.

