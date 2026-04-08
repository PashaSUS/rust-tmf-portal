use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    TmfClient(tmf_client::error::TmfError),
    ServiceUnavailable(String),
    BadRequest(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TmfClient(e) => write!(f, "TMF client error: {}", e),
            Self::ServiceUnavailable(name) => write!(f, "Service unavailable: {}", name),
            Self::BadRequest(msg) => write!(f, "Bad request: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::TmfClient(tmf_client::error::TmfError::Api { status, body }) => {
                let status_code =
                    actix_web::http::StatusCode::from_u16(*status).unwrap_or(
                        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    );
                HttpResponse::build(status_code).json(serde_json::json!({
                    "error": body,
                    "status": status,
                }))
            }
            Self::TmfClient(e) => {
                HttpResponse::BadGateway().json(serde_json::json!({
                    "error": e.to_string(),
                }))
            }
            Self::ServiceUnavailable(name) => {
                HttpResponse::ServiceUnavailable().json(serde_json::json!({
                    "error": format!("{} is not configured or unreachable", name),
                }))
            }
            Self::BadRequest(msg) => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": msg,
                }))
            }
        }
    }
}

impl From<tmf_client::error::TmfError> for AppError {
    fn from(err: tmf_client::error::TmfError) -> Self {
        Self::TmfClient(err)
    }
}
