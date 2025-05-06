use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;

#[derive(Serialize)]
struct Health {
    status: String,
}

pub async fn get_health() -> impl IntoResponse {
    Json(Health {
        status: "OK".to_owned(),
    })
}
