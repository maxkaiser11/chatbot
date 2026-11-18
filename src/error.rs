use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Conversation not found")]
    NotFound,

    #[error("{0}")]
    BadRequest(String),

    #[error("AI Provider error: {0}")]
    Llm(String),

    #[error("Chat storage error: {0}")]
    Storage(String),
}

impl IntoResponse for AppError {
    // converts error into http response
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Llm(_) => StatusCode::BAD_GATEWAY,
            AppError::Storage(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        if status.is_server_error() {
            tracing::error!("{self}");
        }

        (
            status,
            Json(json!({
                "error": self.to_string()
            })),
        )
            .into_response()
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Llm(err.to_string())
    }
}
