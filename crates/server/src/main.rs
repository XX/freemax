use std::net::SocketAddr;

use axum::Router;
use axum::response::IntoResponse;
use axum::routing::get;
use hypertext::prelude::*;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct AppState {
    #[allow(dead_code)]
    db: sqlx::PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let database_url = std::env::var("DATABASE_URL")?;
    let addr: SocketAddr = std::env::var("FREEMAX_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8080".into())
        .parse()?;

    let db = PgPoolOptions::new().max_connections(10).connect(&database_url).await?;
    sqlx::migrate!("../../migrations").run(&db).await?;

    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(|| async { "ok" }))
        .layer(TraceLayer::new_for_http())
        .with_state(AppState { db });

    tracing::info!("listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
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
