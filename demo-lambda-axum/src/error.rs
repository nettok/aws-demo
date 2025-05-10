use std::error::Error;
use std::panic::Location;

use askama::Error as AskamaError;
use axum::Json;
use axum::extract::Request;
use axum::extract::rejection::FormRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use lambda_http::RequestExt;
use serde::Serialize;
use thiserror::Error;
use tracing;
use validator::ValidationErrors;

#[derive(Serialize, Debug)]
pub struct ErrorResp {
    error: ErrorDetails,

    #[serde(skip_serializing)]
    location: &'static Location<'static>,
}

#[derive(Serialize, Debug)]
struct ErrorDetails {
    name: &'static str,
    message: String,
    request_context: RequestContext,
}

#[derive(Serialize, Debug, Clone)]
pub struct RequestContext {
    method: String,
    path: String,
    request_id: String,
    trace_id: String,
}

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("{}: {}", resp.error.name, resp.error.message)]
    SampleError { resp: ErrorResp },

    #[error("{}: {}", resp.error.name, resp.error.message)]
    TemplateError {
        resp: ErrorResp,
        source: AskamaError,
    },

    #[error("{}: {}", resp.error.name, resp.error.message)]
    ValidationError {
        resp: ErrorResp,
        source: ValidationErrors,
    },

    #[error("{}: {}", resp.error.name, resp.error.message)]
    AxumFormRejection {
        resp: ErrorResp,
        source: FormRejection,
    },
}

impl ServerError {
    fn error_resp(&self) -> (StatusCode, &ErrorResp) {
        match self {
            ServerError::SampleError { resp, .. } => (StatusCode::IM_A_TEAPOT, resp),
            ServerError::TemplateError { resp, .. } => (StatusCode::INTERNAL_SERVER_ERROR, resp),
            ServerError::ValidationError { resp, .. } => (StatusCode::BAD_REQUEST, resp),
            ServerError::AxumFormRejection { resp, .. } => (StatusCode::BAD_REQUEST, resp),
        }
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status_code, error_resp) = self.error_resp();

        tracing::error!(
            location = error_resp.location.to_string(),
            error = self.to_string(),
            source = self.source().map(|s| s.to_string())
        );

        (status_code, Json(error_resp)).into_response()
    }
}

impl From<&Request> for RequestContext {
    fn from(request: &Request) -> Self {
        let method = request.method().to_string();
        let path = request.uri().path().to_owned();
        let request_id = request.lambda_context().request_id;
        let trace_id = request.lambda_context().xray_trace_id.unwrap_or_default();

        RequestContext {
            method,
            path,
            request_id,
            trace_id,
        }
    }
}

#[track_caller]
pub fn sample_error(request_context: RequestContext, message: String) -> ServerError {
    let location = Location::caller();
    ServerError::SampleError {
        resp: error_resp("SampleError", message, request_context, location),
    }
}

#[track_caller]
pub fn template_error(request_context: RequestContext, source: AskamaError) -> ServerError {
    let location = Location::caller();
    ServerError::TemplateError {
        resp: error_resp(
            "TemplateError",
            source.to_string(),
            request_context,
            location,
        ),
        source,
    }
}

#[track_caller]
pub fn validation_error(request_context: RequestContext, source: ValidationErrors) -> ServerError {
    let location = Location::caller();
    ServerError::ValidationError {
        resp: error_resp(
            "ValidationError",
            source.to_string(),
            request_context,
            location,
        ),
        source,
    }
}

#[track_caller]
pub fn axum_form_rejection(request_context: RequestContext, source: FormRejection) -> ServerError {
    let location = Location::caller();
    ServerError::AxumFormRejection {
        resp: error_resp(
            "AxumFormRejection",
            source.to_string(),
            request_context,
            location,
        ),
        source,
    }
}

fn error_resp(
    error_name: &'static str,
    message: String,
    request_context: RequestContext,
    location: &'static Location<'static>,
) -> ErrorResp {
    ErrorResp {
        error: ErrorDetails {
            name: error_name,
            message,
            request_context,
        },
        location,
    }
}
