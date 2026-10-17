# Freemax

Сервис и протокол свободного обмена. Участники (люди и организации) публикуют **потребности** и **предложения** (предметы, ресурсы, услуги) в **кругах**. Обмен безвозмездный: без эквивалентности и взаимности — достаточно состоять в общем круге.

## Документация

- [Требования](docs/requirements.md)
- [Доменная модель](docs/domain.md)
- [Протокол обмена](docs/protocol.md)
- [Архитектура](docs/architecture.md)

## Запуск

```sh
cp .env.example .env    # указать DATABASE_URL
cargo make run          # release; `cargo make run -p debug` — debug-сборка
cargo make lint-test    # fmt + clippy + тесты (как в CI)
```

Миграции применяются при старте сервера.

Лицензия: MIT или Apache-2.0.

Стек: Rust, tokio, axum, PostgreSQL (sqlx), [wingy-hypertext](https://github.com/XX/wingy-hypertext) (`dev`).
