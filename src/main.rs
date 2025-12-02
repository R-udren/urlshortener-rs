mod db;
mod error;
mod routes;
use std::{env, net::SocketAddr};

use axum::extract::FromRef;
pub use error::{Error, Result};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::net::TcpListener;
use tracing::info;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

// чтобы PgPool: FromRef<AppState>
impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> PgPool {
        state.pool.clone()
    }
}

#[tokio::main]
async fn main() -> Result {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let db_user = env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string());
    let db_password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let db_name = env::var("POSTGRES_DB").unwrap_or_else(|_| "db".to_string());
    let db_host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
    let db_port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());

    // Собираем URL
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    info!("Starting the application...");
    let app = routes::create_router().with_state(AppState { pool });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("By visiting http://{addr}/ you should see the API documentation");

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
