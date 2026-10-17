# Архитектура

## Стек

- **tokio** — рантайм.
- **axum 0.8** — HTTP, маршрутизация.
- **PostgreSQL + sqlx** — хранилище, миграции (`migrations/`, применяются при старте).
- **hypertext + wingy-hypertext** (`dev`) — серверный рендеринг компонентов Web UI, интерактивность через htmx.

## Крейты

| Крейт | Назначение |
|-------|------------|
| `crates/core` (`freemax-core`) | Доменные типы и правила, без IO. |
| `crates/server` (`freemax-server`, бинарь `freemax`) | HTTP, БД, шаблоны. |

## Конфигурация

Файл `config.toml` (образец — `config.example.toml`), загрузка через `config-load`. Источники по возрастанию приоритета:

1. файл из `FREEMAX_ROOT_CONFIG`;
2. файл из `--config` или первый `config.toml` в текущем и родительских каталогах;
3. переменные окружения `FREEMAX_<СЕКЦИЯ>__<КЛЮЧ>`, например `FREEMAX_HTTP__PORT=8090`.

| Ключ | По умолчанию |
|------|--------------|
| `http.host` | `127.0.0.1` |
| `http.port` | `8080` |
| `database.url` | Postgres из `docker-compose.yml` |
| `database.max_connections` | `10` |
| `log.spec` | `info` (`RUST_LOG` имеет приоритет) |

Вне `config.toml`: `DATABASE_URL` (`.env`) — для `#[sqlx::test]` и sqlx-cli; `FREEMAX_DB_PORT` — порт Postgres в docker-compose.

## Соглашения

- Сборка и проверки — через `cargo make` (`Makefile.toml`): `run`, `lint`, `test`, `lint-test`; профиль `-p debug`.
- Форматирование: `cargo +nightly fmt` (`.rustfmt.toml`, ширина 120); TOML — taplo (`.taplo.toml`).
- Линты: `cargo clippy --all-targets -- -D warnings`.
- CI: `.github/workflows/ci.yml` — `cargo make lint` и `cargo make test` на `main`/`dev`.
- Задачи: `issues/` — по файлу на задачу.
- ID — UUID v7.
- Ошибки обработчиков — `AppError` (`crates/server/src/error.rs`); детали внутренних ошибок только в лог.
- Тесты с БД — `#[sqlx::test(migrator = "freemax_server::MIGRATOR")]` в `crates/server/tests/`.
