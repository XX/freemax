# CLAUDE.md

Freemax — сервис свободного обмена (Rust: tokio, axum, sqlx/PostgreSQL, wingy-hypertext `dev`).

- Требования и модель: `docs/`. Документацию писать кратко, по-русски.
- `crates/core` — домен без IO; `crates/server` — axum + БД + рендеринг; `crates/client` — WASM-клиент и `webassets/`.
- Страницы: обработчик возвращает `view::View`, layout и htmx-фрагменты делает middleware `view::render_layout`.
- Тексты интерфейса — только через `Locale::t` и `crates/server/locales/{ru,en}/*.ftl` (оба языка сразу).
- Схема БД меняется только новой миграцией в `migrations/`.
- Команды: `cargo make run` (`-p debug` — debug; собирает и клиент), `cargo make client` — только статика, `cargo make lint-test` — fmt + clippy + тесты, как в CI.
- Конфигурация сервера: `config.toml` (образец `config.example.toml`), переопределение через `FREEMAX_<СЕКЦИЯ>__<КЛЮЧ>`.
- TOML форматируется taplo (`.taplo.toml`).
- `issues/` — задачи в markdown, по файлу на задачу.
