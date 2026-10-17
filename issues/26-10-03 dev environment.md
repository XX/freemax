# Окружение разработки

Этап 0. Зависимости: —

## Сделать

- `docker-compose.yml` с Postgres для локальной разработки; согласовать с `.env.example`.
- Тесты с БД через `#[sqlx::test]` (временная база на тест, миграции из `migrations/`).
- Единый тип `AppError` в `crates/server` с `IntoResponse`: 400/403/404/500, логирование внутренних ошибок.
- Вынести конфиг (env) в отдельную структуру.

## Готово, когда

- `docker compose up -d && cargo make run` поднимает сервис с чистой базой.
- `cargo make test` выполняет хотя бы один тест с БД (например, `/health` + миграции).

## Результат

- `docker-compose.yml`: Postgres 17 на `127.0.0.1:5442` (порты 5432–5434 на машине разработчика заняты), переопределяется `FREEMAX_DB_PORT`.
- `crates/server` разделён на `lib.rs` (`router`, `AppState`, `MIGRATOR`) и тонкий `main.rs`.
- Конфигурация по образцу Wikihub (`config.rs`, `cli.rs`): `config.toml` через `config-load`, `--config`, переопределение `FREEMAX_<СЕКЦИЯ>__<КЛЮЧ>`; образец — `config.example.toml`.
- `AppError` (`error.rs`): 400/403/404/500, `sqlx::Error::RowNotFound` → 404, внутренние ошибки только в лог.
- `/health` проверяет доступность БД.
- Тесты: `crates/server/tests/health.rs` (`/health`, применённые миграции) + unit-тесты `AppError`.
- CI: сервис Postgres и `DATABASE_URL` в `.github/workflows/ci.yml`.

---

● Задача 03 сделана, но не закоммичена. cargo make lint-test проходит целиком: fmt, clippy, 2 unit-теста и 2 теста с базой. taplo fmt --check тоже чистый. Оба критерия «Готово, когда» проверил вживую: docker 
  compose up -d && cargo make run поднимает сервис, /health отвечает ok, неизвестный путь даёт 404.
     
  Что сделано:
  - docker-compose.yml — Postgres 17 на 127.0.0.1:5442. Порты 5432–5434 у вас заняты другими Postgres, поэтому выбрал 5442. Порт можно поменять через FREEMAX_DB_PORT.
  - crates/server/src/lib.rs — сюда вынесены router, AppState и MIGRATOR, чтобы тесты могли собирать приложение без сети. main.rs теперь только запускает сервер и загружает .env через dotenvy. Раньше .env
    вообще не читался.
  - config.rs — Config::from_env() читает DATABASE_URL, FREEMAX_ADDR и новую переменную FREEMAX_DB_MAX_CONNECTIONS.
  - error.rs — AppError отдаёт 400/403/404/500. «Строка не найдена» из sqlx превращается в 404. Подробности внутренних ошибок пишутся только в лог, клиенту уходит общий текст.
  - /health теперь проверяет и доступность базы.
  - Тесты — crates/server/tests/health.rs через #[sqlx::test]: каждый тест получает свою временную базу с миграциями.
  - Документация — обновил README.md и docs/architecture.md, в файл задачи добавил раздел «Результат».
  
  Что стоит проверить:
  - CI — в .github/workflows/ci.yml добавил сервис Postgres и DATABASE_URL, без них тесты с базой там упадут. Сам workflow я не запускал, он проверится при первом пуше.
  - .env — в вашем локальном .env поменял порт на 5442. Со старым портом он не работал: на 5432 у системного Postgres нет пользователя freemax.
  - Docker — запущен контейнер freemax-db-1 с томом pgdata. Если не нужен, остановите его командой docker compose down.

---

Посмотри как реализовано чтение конфигурации в проекте Wikihub (../../wikihub/wikihub/crates/server/src/config.rs), сделай похожим образом и во freemax. Набросок конфига есть в config.example.toml - его можно и поменять при необходимости.

---

● Конфигурация во freemax теперь читается так же, как в Wikihub. cargo make lint-test проходит, добавился один тест. Проверил вживую:
  - сервер находит config.toml в родительском каталоге;
  - FREEMAX_HTTP__PORT=8091 переопределяет порт;
  - если указан несуществующий --config, сервер выходит с понятной ошибкой.
  
  Что сделано:
  - config.rs — та же схема, что в Wikihub: config-load и секции с #[serde(default)]. Секции такие: http (host, port), database (url, max_connections), log (spec). Файл ищется в том же порядке: сначала
    FREEMAX_ROOT_CONFIG, затем --config или первый config.toml в текущем и родительских каталогах. После файла применяются переменные окружения.
  - cli.rs — параметр --config через clap, как в Wikihub.
  - config.example.toml — переделал ваш набросок: addr разбил на [http] host/port по образцу Wikihub и добавил [log]. Файл config.toml добавлен в .gitignore; для себя я создал его копией примера.
  - Тест — config.example.toml читается и совпадает со значениями по умолчанию, так что пример не разойдётся с кодом.
  - Документация — обновил README.md, раздел «Конфигурация» в docs/architecture.md, CLAUDE.md и «Результат» в задаче 03.
  
  Отличия от Wikihub:
  - Разделитель в переменных окружения — __ вместо _: FREEMAX_DATABASE__MAX_CONNECTIONS. С одиночным _ ключ max_connections не прочитать. В Wikihub, похоже, та же проблема: в комментарии написано
    WIKIHUB_HTTP__PORT, а в коде стоит .separator("_").
  - list_separator не добавлял: при try_parsing(true) он заставляет читать все переменные окружения как списки.
  - Логирование осталось на tracing, а не на flexi_logger. log.spec — фильтр в синтаксисе RUST_LOG, сама переменная RUST_LOG важнее файла.
  - Фильтр по модулю — в примере написано freemax_server=debug, потому что модуль называется по крейту. Прежний freemax=debug ни на что не действовал.
  
  Что осталось вне config.toml: DATABASE_URL в .env всё ещё нужен, потому что #[sqlx::test] берёт адрес базы только оттуда. В .env.example оставил только эту строку. Сервер .env больше не читает, поэтому
  FREEMAX_ADDR и RUST_LOG в вашем .env теперь ни на что не влияют.
  
  Разные порты Postgres. Вы вернули 5432 в README.md и .env.example, а docker-compose.yml, значение по умолчанию database.url и config.example.toml указывают на 5442. Скажите, какой порт оставить, и я приведу
  всё к нему.
  
  По ходу нашёл и остановил процесс сервера, оставшийся от моей проверки в задаче 03: он всё это время занимал порт 8080.

---

Что означает запись
    ports:
      - "127.0.0.1:${FREEMAX_DB_PORT:-5442}:5432"

---

● Это проброс порта из контейнера на вашу машину. Формат записи: адрес_хоста:порт_хоста:порт_контейнера.

  - 127.0.0.1 — порт открывается только на локальном интерфейсе. К базе можно подключиться с этой машины, но не из сети. Без этой части Docker слушал бы на всех интерфейсах (0.0.0.0).
  - ${FREEMAX_DB_PORT:-5442} — порт на хосте. Docker Compose подставляет переменную FREEMAX_DB_PORT, а если она не задана или пустая, берёт 5442. Синтаксис :- такой же, как в shell. Переменную можно задать в
    окружении или в файле .env рядом с docker-compose.yml.
  - 5432 — порт внутри контейнера, стандартный порт Postgres. Его менять не нужно.
  
  Итого: Postgres внутри контейнера слушает 5432, а снаружи доступен как localhost:5442 (или localhost:$FREEMAX_DB_PORT). 
    
  Примеры: 
  docker compose up -d                       # localhost:5442
  FREEMAX_DB_PORT=5432 docker compose up -d  # localhost:5432
  
  Если хотите везде 5432, достаточно один раз добавить FREEMAX_DB_PORT=5432 в .env: Compose читает его автоматически. Тогда поменяется и строка подключения, то есть database.url в config.toml и DATABASE_URL в
  .env.

---
