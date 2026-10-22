//! HTTP-сервер Freemax.

use std::path::Path;

use axum::extract::State;
use axum::routing::get;
use axum::{Router, middleware};
use sqlx::PgPool;
use sqlx::migrate::Migrator;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

pub use crate::config::Config;
pub use crate::error::{AppError, AppResult};

pub mod config;
pub mod error;
pub mod i18n;
pub mod view;

pub static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

/// Маршруты приложения; статика из `webassets_dir` раздаётся по `/webassets`.
pub fn router(state: AppState, webassets_dir: impl AsRef<Path>) -> Router {
    Router::new()
        .route("/", get(async |locale| view::page::index(locale)))
        .route("/circles", get(async |locale| view::page::circles(locale)))
        .route("/about", get(async |locale| view::page::about(locale)))
        .route("/health", get(health))
        .fallback(async || AppError::NotFound)
        .layer(middleware::from_fn(view::render_layout))
        .layer(middleware::from_fn(i18n::detect_locale))
        .nest_service("/webassets", ServeDir::new(webassets_dir))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// Проверка живости, включая доступность БД.
async fn health(State(state): State<AppState>) -> AppResult<&'static str> {
    sqlx::query("SELECT 1").execute(&state.db).await?;
    Ok("ok")
}
