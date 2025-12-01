mod error;
mod routes;
use std::net::SocketAddr;

pub use error::{Error, Result};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Starting the application...");
    let app = routes::create_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("By visiting http://{addr}/ you should see the API documentation");

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
