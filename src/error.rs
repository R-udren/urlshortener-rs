pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Generic error: {0}")]
    GenericError(#[from] std::io::Error),
}
