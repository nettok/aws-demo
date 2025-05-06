use crate::error;

use axum::response::IntoResponse;
use tracing::{self, instrument};

#[instrument]
pub async fn get_hello() -> &'static str {
    tracing::info!("Saying hello...");
    "Hola Mundo!"
}

#[instrument]
pub async fn get_error() -> impl IntoResponse {
    error::sample_error("I am a sample error".to_owned())
}
