mod db;
mod error;
mod routes;

use sqlx::PgPool;
use std::net::SocketAddr;
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

    info!("Starting the application...");

    let pool = db::create_pool().await;
    info!("Database connected");

    let state = AppState { pool };
    let app = routes::create_router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Server running at http://{addr}/");

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
