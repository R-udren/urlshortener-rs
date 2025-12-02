use crate::{AppState, db::DatabaseConnection};
use axum::{Router, http::StatusCode, routing::get};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

async fn using_connection_extractor(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "urlshortener", description = "URL Shortener API")
    )
)]
struct ApiDoc;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/hello", get(using_connection_extractor))
        .merge(Scalar::with_url("/", ApiDoc::openapi()))
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
