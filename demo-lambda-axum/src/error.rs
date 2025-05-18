use std::error::Error;
use std::panic::Location;

use askama::Error as AskamaError;
use axum::Json;
use axum::extract::rejection::FormRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use util::tracing;
use serde::Serialize;
use strum::IntoStaticStr;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Serialize)]
pub struct ErrorResp {
    error: ErrorDetails,
}

#[derive(Debug, Serialize)]
struct ErrorDetails {
    name: &'static str,
    message: String,
}

#[derive(Debug, Error, IntoStaticStr)]
pub enum AppError {
    #[error("Sample error: {}", message)]
    SampleError {
        message: String,
        location: &'static Location<'static>,
    },

    #[error("Template error: {}", source)]
    TemplateError {
        location: &'static Location<'static>,
        source: AskamaError,
    },

    #[error("Validation error: {}", source)]
    ValidationError {
        location: &'static Location<'static>,
        source: ValidationErrors,
    },

    #[error("Form rejection: {}", source)]
    AxumFormRejection {
        location: &'static Location<'static>,
        source: FormRejection,
    },
}

impl AppError {
    fn to_response(&self) -> (StatusCode, ErrorResp) {
        let resp = Self::error_resp(self.into(), self.to_string());

        let (status, location) = match self {
            AppError::SampleError { location, .. } => (StatusCode::IM_A_TEAPOT, location),
            AppError::TemplateError { location, .. } => {
                (StatusCode::INTERNAL_SERVER_ERROR, location)
            }
            AppError::ValidationError { location, .. } => (StatusCode::BAD_REQUEST, location),
            AppError::AxumFormRejection { location, .. } => (StatusCode::BAD_REQUEST, location),
        };

        tracing::error!(
            location = location.to_string(),
            error = self.to_string(),
            source = self.source().map(|s| s.to_string())
        );

        (status, resp)
    }

    fn error_resp(error_name: &'static str, message: String) -> ErrorResp {
        ErrorResp {
            error: ErrorDetails {
                name: error_name,
                message,
            },
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, error_resp) = self.to_response();
        (status_code, Json(error_resp)).into_response()
    }
}

#[track_caller]
pub fn sample_error(message: String) -> AppError {
    AppError::SampleError {
        message,
        location: Location::caller(),
    }
}

impl From<AskamaError> for AppError {
    #[track_caller]
    fn from(value: AskamaError) -> Self {
        AppError::TemplateError {
            location: Location::caller(),
            source: value,
        }
    }
}

impl From<ValidationErrors> for AppError {
    #[track_caller]
    fn from(value: ValidationErrors) -> Self {
        AppError::ValidationError {
            location: Location::caller(),
            source: value,
        }
    }
}

impl From<FormRejection> for AppError {
    #[track_caller]
    fn from(value: FormRejection) -> Self {
        AppError::AxumFormRejection {
            location: Location::caller(),
            source: value,
        }
    }
}
