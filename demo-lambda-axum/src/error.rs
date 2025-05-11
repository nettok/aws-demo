use std::error::Error;
use std::panic::Location;

use askama::Error as AskamaError;
use axum::Json;
use axum::extract::rejection::FormRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
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

#[track_caller]
pub fn sample_error(message: String) -> ServerError {
    let location = Location::caller();
    ServerError::SampleError {
        resp: error_resp("SampleError", message, location),
    }
}

#[track_caller]
pub fn template_error(source: AskamaError) -> ServerError {
    let location = Location::caller();
    ServerError::TemplateError {
        resp: error_resp(
            "TemplateError",
            source.to_string(),
            location,
        ),
        source,
    }
}

#[track_caller]
pub fn validation_error(source: ValidationErrors) -> ServerError {
    let location = Location::caller();
    ServerError::ValidationError {
        resp: error_resp(
            "ValidationError",
            source.to_string(),
            location,
        ),
        source,
    }
}

#[track_caller]
pub fn axum_form_rejection(source: FormRejection) -> ServerError {
    let location = Location::caller();
    ServerError::AxumFormRejection {
        resp: error_resp(
            "AxumFormRejection",
            source.to_string(),
            location,
        ),
        source,
    }
}

fn error_resp(
    error_name: &'static str,
    message: String,
    location: &'static Location<'static>,
) -> ErrorResp {
    ErrorResp {
        error: ErrorDetails {
            name: error_name,
            message,
        },
        location,
    }
}
