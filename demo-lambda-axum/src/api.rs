use axum::Json;
use crate::error;
use axum::response::IntoResponse;
use chrono::{DateTime, Utc};
use serde::Serialize;
use tracing::{self, instrument};

#[derive(Serialize)]
pub struct Greeting {
    value: String,
    timestamp: DateTime<Utc>
}

#[instrument]
pub async fn get_hello() -> Json<Greeting> {
    tracing::info!("Saying hello...");
    Json(Greeting {
        value: "Hola mundo!".to_owned(),
        timestamp: Utc::now()
    })
}

#[instrument]
pub async fn get_error() -> impl IntoResponse {
    error::sample_error("I am a sample error".to_owned())
}
