use clap::Parser;
use freemax_server::{AppState, Config, MIGRATOR, router};
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::EnvFilter;

mod cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = cli::Opts::parse();
    let config = Config::load(opts.config)?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new(&config.log.filter))?)
        .init();

    let db = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await?;
    MIGRATOR.run(&db).await?;

    let addr = (config.http.host.as_str(), config.http.port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("listening on http://{}", listener.local_addr()?);
    axum::serve(listener, router(AppState { db }, &config.http.webassets_dir)).await?;

    Ok(())
}
