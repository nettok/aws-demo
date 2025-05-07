use askama::Error as AskamaError;
use axum::Json;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use lambda_http::RequestExt;
use serde::Serialize;
use tracing;

#[derive(Serialize)]
pub struct ErrorResp {
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
    SampleError(ErrorResp),
    TemplateError(ErrorResp),
}

impl AppError {
    fn into_error_resp(self) -> (StatusCode, ErrorResp) {
        match self {
            AppError::SampleError(r) => (StatusCode::IM_A_TEAPOT, r),
            AppError::TemplateError(r) => (StatusCode::INTERNAL_SERVER_ERROR, r),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, error_resp) = self.into_error_resp();
        // TODO: can I also log the location of where the error originated?
        tracing::error!("{}: {}", error_resp.error.name, error_resp.error.message);
        (status_code, Json(error_resp)).into_response()
    }
}

pub fn sample_error(request: Request, message: String) -> AppError {
    AppError::SampleError(error_resp("SampleError", message, request))
}

pub fn template_error(request: Request, askama_error: AskamaError) -> AppError {
    AppError::TemplateError(error_resp(
        "TemplateError",
        askama_error.to_string(),
        request,
    ))
}

fn error_resp(error_name: &'static str, message: String, request: Request) -> ErrorResp {
    let method = request.method().to_string();
    let path = request.uri().path().to_owned();
    let request_id = request.lambda_context().request_id;
    let trace_id = request.lambda_context().xray_trace_id.unwrap_or_default();

    ErrorResp {
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
