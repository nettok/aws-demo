use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tracing;

#[derive(Serialize)]
pub struct ErrorResponse {
    error: Error,
}

#[derive(Serialize)]
struct Error {
    name: &'static str,
    message: String,
    // TODO: add request_id
}

pub enum AppError {
    SampleError(ErrorResponse),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::SampleError(response) => {
                // TODO: can I also log the location of where the error originated?
                tracing::error!("{}: {}", response.error.name, response.error.message);
                (StatusCode::IM_A_TEAPOT, Json(response)).into_response()
            }
        }
    }
}

pub fn sample_error(message: String) -> AppError {
    AppError::SampleError(ErrorResponse {
        error: Error {
            name: "SampleError",
            message,
        },
    })
}
