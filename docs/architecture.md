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

## Конфигурация (env)

| Переменная | По умолчанию |
|------------|--------------|
| `DATABASE_URL` | — (обязательна) |
| `FREEMAX_ADDR` | `127.0.0.1:8080` |
| `RUST_LOG` | `info` |

## Соглашения

- Сборка и проверки — через `cargo make` (`Makefile.toml`): `run`, `lint`, `test`, `lint-test`; профиль `-p debug`.
- Форматирование: `cargo +nightly fmt` (`.rustfmt.toml`, ширина 120); TOML — taplo (`.taplo.toml`).
- Линты: `cargo clippy --all-targets -- -D warnings`.
- CI: `.github/workflows/ci.yml` — `cargo make lint` и `cargo make test` на `main`/`dev`.
- Задачи: `issues/` — по файлу на задачу.
- ID — UUID v7.
