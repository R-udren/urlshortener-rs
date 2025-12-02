mod config;
mod db;
mod error;
mod routes;

use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::info;

pub use error::{Error, Result};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

#[tokio::main]
async fn main() -> Result {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = config::Config::from_env();
    info!("Starting server at http://{}/", config.server_addr);

    let pool = db::create_pool(&config.database_url).await;
    info!("Database connected");

    let state = AppState { pool };
    let app = routes::create_router(state);

    let listener = TcpListener::bind(config.server_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
