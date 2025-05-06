use axum::Json;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::http::header::AsHeaderName;
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
    method: String,
    path: String,
    request_id: String,
    trace_id: String,
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

pub fn sample_error(request: Request, message: String) -> AppError {
    AppError::SampleError(error_response("SampleError", message, request))
}

fn error_response(error_name: &'static str, message: String, request: Request) -> ErrorResponse {
    let method = request.method().to_string();
    let path = request.uri().path().to_owned();
    let request_id = get_header(&request, "lambda-runtime-aws-request-id").unwrap_or_default();
    let trace_id = get_header(&request, "x-amzn-trace-id").unwrap_or_default();

    ErrorResponse {
        error: Error {
            name: error_name,
            message,
            method,
            path,
            request_id,
            trace_id,
        },
    }
}

fn get_header<K>(request: &Request, key: K) -> Option<String>
where
    K: AsHeaderName,
{
    request
        .headers()
        .get(key)
        .map(|value| value.to_str().unwrap_or_default())
        .map(|value| value.to_owned())
}
