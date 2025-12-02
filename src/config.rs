use std::net::SocketAddr;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub server_addr: SocketAddr,
}

impl Config {
    pub fn from_env() -> Self {
        let user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
        let password = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
        let db = std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
        let host = std::env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".into());
        let port = std::env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".into());

        let database_url = format!("postgres://{user}:{password}@{host}:{port}/{db}");

        let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let server_port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".into());
        let server_addr: SocketAddr = format!("{server_host}:{server_port}")
            .parse()
            .expect("Invalid SERVER_HOST or SERVER_PORT");

        Self {
            database_url,
            server_addr,
        }
    }
}
