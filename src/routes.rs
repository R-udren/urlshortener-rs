use axum::Router;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use crate::AppState;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "urlshortener", description = "URL Shortener API")
    )
)]
struct ApiDoc;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(Scalar::with_url("/", ApiDoc::openapi()))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state)
}
