use crate::error;
use crate::serde_decorators::empty_string_as_none;
use axum::Json;
use axum::response::IntoResponse;
use axum_extra::extract::OptionalQuery;
use chrono::{DateTime, Utc};
use util::tracing::{self, instrument};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GreetingParams {
    #[serde(deserialize_with = "empty_string_as_none")]
    name: Option<String>,
}

#[derive(Serialize)]
pub struct GreetingResponse {
    value: String,
    timestamp: DateTime<Utc>,
}

#[instrument(skip(params))]
pub async fn get_hello(
    OptionalQuery(params): OptionalQuery<GreetingParams>,
) -> Json<GreetingResponse> {
    tracing::info!("Saying hello...");
    Json(GreetingResponse {
        value: format!(
            "Hola {}!",
            params
                .map(|p| p.name)
                .flatten()
                .unwrap_or_else(|| "mundo".to_owned())
        ),
        timestamp: Utc::now(),
    })
}

#[instrument]
pub async fn get_error() -> impl IntoResponse {
    error::sample_error("I am a sample error".to_owned())
}
