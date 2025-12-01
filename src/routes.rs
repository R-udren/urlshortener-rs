use axum::Router;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "urlshortener", description = "URL Shortener API")
    )
)]
struct ApiDoc;

pub fn create_router() -> Router {
    Router::new()
        .merge(Scalar::with_url("/docs", ApiDoc::openapi()))
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
