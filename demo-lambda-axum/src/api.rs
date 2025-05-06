use crate::error;

use axum::extract::Request;
use axum::response::IntoResponse;
use tracing::{self, instrument};

#[instrument]
pub async fn get_hello() -> &'static str {
    tracing::info!("Saying hello...");
    "Hola Mundo!"
}

#[instrument]
pub async fn get_error(request: Request) -> impl IntoResponse {
    error::sample_error(request, "I am a sample error".to_owned())
}
