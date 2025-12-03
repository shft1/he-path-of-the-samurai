# Аудит инфраструктуры и Docker Compose

## Состав сервисов
- `db` (`postgres:16-alpine`): БД PostgreSQL, инициализация `db/init.sql`, порт 5434.
- `rust_iss`: многоэтапная сборка (rust → distroless), выдаёт 8081→3000, получает ключи API из переменных окружения.
- `php` (Laravel dashboard): `php:8.4-fpm-alpine`, код встраивается на этапе build, миграции выполняются автоматически.
- `nginx`: прокси и раздача статики, общая сеть `backend`.
- `legacy_cli`: Python CLI, выполняется по cron, пишет в stdout/stderr.

## Проблемы, которые нашли
1. Секреты были в `docker-compose.yml`.
2. Использовались тяжёлые образы Debian.
3. Не было лимитов ресурсов и healthcheck.
4. Непрозрачный запуск на «чистой» машине.
5. Логи и формат ошибок отличались между сервисами.

## Что сделали
1. Секреты вынесены в `.env` / `.env.example`, compose использует `${VAR:-default}`.
2. Перешли на slim/alpine/distroless образы и включили кэширование сборки (`--mount=type=cache`).
3. Добавили `mem_limit`, healthcheck PostgreSQL, детерминированный порядок старта.
4. Laravel собирается целиком во время docker build, начальные миграции выполняются в entrypoint.
5. Введение единого ответа `{ok, data|error}` с `trace_id`, одинакового для Rust и PHP.

