use std::error::Error;
use std::panic::Location;

use askama::Error as AskamaError;
use axum::Json;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use lambda_http::RequestExt;
use serde::Serialize;
use thiserror::Error;
use tracing;

#[derive(Serialize, Debug)]
pub struct ErrorResp {
    error: ErrorDetails,
}

#[derive(Serialize, Debug)]
struct ErrorDetails {
    name: &'static str,
    message: String,
    method: String,
    path: String,
    request_id: String,
    trace_id: String,

    #[serde(skip_serializing)]
    location: &'static Location<'static>,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{}: {}", resp.error.name, resp.error.message)]
    SampleError { resp: ErrorResp },

    #[error("{}: {}", resp.error.name, resp.error.message)]
    TemplateError {
        resp: ErrorResp,
        source: AskamaError,
    },
}

impl AppError {
    fn error_resp(&self) -> (StatusCode, &ErrorResp) {
        match self {
            AppError::SampleError { resp, .. } => (StatusCode::IM_A_TEAPOT, resp),
            AppError::TemplateError { resp, .. } => (StatusCode::INTERNAL_SERVER_ERROR, resp),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, error_resp) = self.error_resp();

        tracing::error!(
            location = error_resp.error.location.to_string(),
            error = self.to_string(),
            source = self.source().map(|s| s.to_string())
        );

        (status_code, Json(error_resp)).into_response()
    }
}

#[track_caller]
pub fn sample_error(request: Request, message: String) -> AppError {
    let location = Location::caller();
    AppError::SampleError {
        resp: error_resp("SampleError", message, request, location),
    }
}

#[track_caller]
pub fn template_error(request: Request, askama_error: AskamaError) -> AppError {
    let location = Location::caller();
    AppError::TemplateError {
        resp: error_resp("TemplateError", askama_error.to_string(), request, location),
        source: askama_error,
    }
}

fn error_resp(
    error_name: &'static str,
    message: String,
    request: Request,
    location: &'static Location<'static>,
) -> ErrorResp {
    let method = request.method().to_string();
    let path = request.uri().path().to_owned();
    let request_id = request.lambda_context().request_id;
    let trace_id = request.lambda_context().xray_trace_id.unwrap_or_default();

    ErrorResp {
        error: ErrorDetails {
            name: error_name,
            message,
            method,
            path,
            request_id,
            trace_id,
            location,
        },
    }
}
