use axum::{
    Json,
    http::{HeaderMap, HeaderValue, StatusCode, Uri, header::WWW_AUTHENTICATE},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::borrow::Cow;
use std::collections::HashMap;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

/// Can be returned in a `Result` from an API handler function.
///
/// For convenience, this represents both API errors as well as internal recoverable errors,
/// and maps them to appropriate status codes along with at least a minimally useful error
/// message in a JSON body.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Return `400 Bad Request`
    #[error("{0}")]
    BadRequest(String),

    /// Return `401 Unauthorized`
    #[error("Authentication Required")]
    Unauthorized,

    /// Return `403 Forbidden`
    #[error("User may not perform that action")]
    Forbidden,

    /// Return `404 Not Found`
    #[error("Resource Not Found")]
    NotFound,

    /// Return `409 Conflict`
    #[error("Conflict: {0}")]
    Conflict(String),

    /// Return `422 Unprocessable Entity`
    ///
    /// This serializes the `errors` map to JSON for detailed validation errors.
    #[error("Error in the Request Body")]
    UnprocessableEntity {
        errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
    },

    /// Automatically return `500 Internal Server Error` on a `sqlx::Error`.
    #[error("An error occurred with the database")]
    Sqlx(#[from] sqlx::Error),

    /// Return `500 Internal Server Error` on a `std::io::Error`.
    #[error("An internal server error occurred")]
    Io(#[from] std::io::Error),

    /// Return `500 Internal Server Error` on a `anyhow::Error`.
    #[error("An internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    reason: Option<String>,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>>,
}

impl ErrorResponse {
    fn new(code: StatusCode, message: String) -> Self {
        Self {
            code: code.as_u16(),
            reason: code.canonical_reason().map(String::from),
            message,
            details: None,
        }
    }

    fn with_details(mut self, details: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>) -> Self {
        self.details = Some(details);
        self
    }

    fn into_response(self, status: StatusCode) -> Response {
        (status, Json(self)).into_response()
    }
}

impl Error {
    /// Convenient constructor for `Error::UnprocessableEntity`.
    ///
    /// Multiple errors for the same key are collected into a list for that key.
    pub fn unprocessable_entity<K, V>(errors: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let mut error_map = HashMap::new();

        for (key, val) in errors {
            error_map
                .entry(key.into())
                .or_insert_with(Vec::new)
                .push(val.into());
        }

        Self::UnprocessableEntity { errors: error_map }
    }

    /// Get the HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Sqlx(_) | Self::Io(_) | Self::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    // 404 Error handling
    pub async fn api_fallback(uri: Uri) -> Response {
        let status = StatusCode::NOT_FOUND;
        let body = ErrorResponse::new(status, format!("Request path Not Found: {uri}"));
        body.into_response(status)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match &self {
            Self::Sqlx(e) => {
                tracing::error!("SQLx error: {:?}", e);
            }
            Self::Io(e) => {
                tracing::error!("IO error: {:?}", e);
            }
            Self::Anyhow(e) => {
                tracing::error!("Internal error: {:?}", e);
            }
            _ => {}
        }

        let status = self.status_code();

        match self {
            Self::UnprocessableEntity { errors } => {
                let body = ErrorResponse::new(status, "Error in the Request Body".to_string())
                    .with_details(errors);
                return (status, Json(body)).into_response();
            }
            Self::Unauthorized => {
                let body = ErrorResponse::new(status, self.to_string());
                let mut headers = HeaderMap::new();
                headers.insert(WWW_AUTHENTICATE, HeaderValue::from_static("Bearer"));
                return (status, headers, Json(body)).into_response();
            }
            _ => {}
        }

        // Default JSON response for all other errors
        let body = ErrorResponse::new(status, self.to_string());

        (status, Json(body)).into_response()
    }
}
