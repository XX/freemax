//! HTTP-сервер Freemax.

use axum::Router;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use hypertext::prelude::*;
use sqlx::PgPool;
use sqlx::migrate::Migrator;
use tower_http::trace::TraceLayer;

pub use crate::config::Config;
pub use crate::error::{AppError, AppResult};

pub mod config;
pub mod error;

pub static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn index() -> impl IntoResponse {
    rsx! {
        <!DOCTYPE html>
        <html lang="ru">
            <head><meta charset="utf-8"><title>"Freemax"</title></head>
            <body><h1>"Freemax — свободный обмен"</h1></body>
        </html>
    }
}

/// Проверка живости, включая доступность БД.
async fn health(State(state): State<AppState>) -> AppResult<&'static str> {
    sqlx::query("SELECT 1").execute(&state.db).await?;
    Ok("ok")
}
