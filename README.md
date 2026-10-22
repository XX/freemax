# Freemax

Сервис и протокол свободного обмена. Участники (люди и организации) публикуют **потребности** и **предложения** (предметы, ресурсы, услуги) в **кругах**. Обмен безвозмездный: без эквивалентности и взаимности — достаточно состоять в общем круге.

## Документация

- [Требования](docs/requirements.md)
- [Доменная модель](docs/domain.md)
- [Протокол обмена](docs/protocol.md)
- [Архитектура](docs/architecture.md)

## Запуск

```sh
cp config.example.toml config.toml   # конфигурация сервера
cp .env.example .env                 # DATABASE_URL для тестов с БД
docker compose up -d                 # Postgres на 127.0.0.1:5432
cargo make run                       # клиент + сервер, release; `cargo make -p debug run` — debug-сборка
cargo make lint-test                 # fmt + clippy + тесты (как в CI)
```

Для сборки клиента нужны target `wasm32-unknown-unknown`, `wasm-bindgen-cli` той же версии, что крейт `wasm-bindgen` в `Cargo.lock`, и `wasm-opt` (только release).

Миграции применяются при старте сервера. Тесты с БД (`#[sqlx::test]`) создают временную базу на каждый тест через `DATABASE_URL`.

Лицензия: MIT или Apache-2.0.

Стек: Rust, tokio, axum, PostgreSQL (sqlx), [wingy-hypertext](https://github.com/XX/wingy-hypertext) (`dev`).
