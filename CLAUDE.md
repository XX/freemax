# CLAUDE.md

Freemax — сервис свободного обмена (Rust: tokio, axum, sqlx/PostgreSQL, wingy-hypertext `dev`).

- Требования и модель: `docs/`. Документацию писать кратко, по-русски.
- `crates/core` — домен без IO; `crates/server` — axum + БД + рендеринг.
- Схема БД меняется только новой миграцией в `migrations/`.
- Команды: `cargo make run` (`-p debug` — debug), `cargo make lint-test` — fmt + clippy + тесты, как в CI.
- TOML форматируется taplo (`.taplo.toml`).
- `issues/` — задачи в markdown, по файлу на задачу.
