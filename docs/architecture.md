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
| `crates/client` (`freemax-client`) | WASM: поведение компонентов wingy-hypertext; `webassets/` — main.js/css, htmx. |

## Интерфейс

- Обработчик возвращает `View` (заголовок + содержимое основной области). Middleware `view::render_layout` оборачивает его в полную страницу или, для htmx-запроса (`HX-Request`, кроме восстановления истории), во фрагмент: `<title>`, содержимое и навигация с `hx-swap-oob`.
- Ошибки (`AppError`) рендерятся так же, в общем layout; htmx подставляет и ответы 4xx/5xx (`main.js`).
- Локализация (ru по умолчанию, en) — Fluent через `fluent-templates`, см. ниже.
- Статика собирается `cargo make client` в `target/web` (стили — из версии wingy-hypertext в `Cargo.lock`) и раздаётся по `/webassets` из `http.webassets_dir`.

## Локализация

- Сообщения — `crates/server/locales/<язык>/*.ftl`, встраиваются в бинарник при сборке. Набор идентификаторов во всех языках одинаков (тест `i18n::tests::locales_are_complete`).
- Язык запроса (`i18n::Locale`): cookie `lang` → `Accept-Language` → `ru`. Middleware `i18n::detect_locale` кладёт его в расширения запроса; обработчики получают `Locale` экстрактором, `render_layout` — из расширений.
- Текст: `locale.t("id")`. Нет сообщения — ошибка в лог и сам идентификатор вместо текста.
- `AppError` языка не знает: отдаёт `view::ErrorView`, текст подставляет `render_layout`.
- `crates/core` текстов не содержит: ошибки и состояния домена — enum'ы, текстом их делает сервер.
- Переключатель в шапке (`view/language.rs`, `freemax-client/src/language.rs`) пишет cookie, очищает кэш истории htmx и перезагружает страницу.
- Ответы с layout несут `Vary: hx-request, cookie, accept-language`.

## Конфигурация

Файл `config.toml` (образец — `config.example.toml`), загрузка через `config-load`. Источники по возрастанию приоритета:

1. файл из `FREEMAX_ROOT_CONFIG`;
2. файл из `--config` или первый `config.toml` в текущем и родительских каталогах;
3. переменные окружения `FREEMAX_<СЕКЦИЯ>__<КЛЮЧ>`, например `FREEMAX_HTTP__PORT=8090`.

| Ключ | По умолчанию |
|------|--------------|
| `http.host` | `127.0.0.1` |
| `http.port` | `8080` |
| `http.webassets_dir` | `target/web` |
| `database.url` | Postgres из `docker-compose.yml` |
| `database.max_connections` | `10` |
| `log.filter` | `info` (`RUST_LOG` имеет приоритет) |

Вне `config.toml`: `DATABASE_URL` (`.env`) — для `#[sqlx::test]` и sqlx-cli; `FREEMAX_DB_PORT` — порт Postgres в docker-compose.

## Соглашения

- Сборка и проверки — через `cargo make` (`Makefile.toml`): `run`, `client`, `lint`, `test`, `lint-test`; профиль `-p debug`.
- Форматирование: `cargo +nightly fmt` (`.rustfmt.toml`, ширина 120); TOML — taplo (`.taplo.toml`).
- Линты: `cargo clippy --all-targets -- -D warnings`.
- CI: `.github/workflows/ci.yml` — `cargo make lint` и `cargo make test` на `main`/`dev`.
- Задачи: `issues/` — по файлу на задачу.
- ID — UUID v7.
- Ошибки обработчиков — `AppError` (`crates/server/src/error.rs`); детали внутренних ошибок только в лог.
- Тесты с БД — `#[sqlx::test(migrator = "freemax_server::MIGRATOR")]` в `crates/server/tests/`.
