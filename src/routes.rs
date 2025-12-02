use axum::Router;
use tower_http::trace::TraceLayer;
use tracing::Level;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use crate::{AppState, Error};

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
        .fallback(Error::api_fallback)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::span!(
                        Level::INFO,
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                    )
                })
                .on_response(
                    |response: &axum::http::Response<_>,
                     latency: std::time::Duration,
                     _span: &tracing::Span| {
                        tracing::info!(
                            status = %response.status(),
                            latency = ?latency,
                            "response"
                        );
                    },
                ),
        )
        .with_state(state)
}
