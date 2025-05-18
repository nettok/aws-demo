mod api;
mod db;
mod error;
mod extract;
mod health;
mod htm;
mod serde_decorators;

use axum::Router;
use axum::extract::Request;
use axum::middleware::{self, Next};
use axum::response::{Redirect, Response};
use axum::routing::{delete, get, post};
use util::config::load_app_config;
use util::tracing;
use dotenvy::dotenv;
use lambda_http::run;
use serde::Deserialize;
use tower_http::BoxError;
use tower_http::services::ServeDir;

#[derive(Clone, Deserialize)]
struct AppConfig {}

#[derive(Clone)]
struct AppState {
    config: AppConfig,
}

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    dotenv().ok();
    tracing::init_tracing_default_subscriber();

    let state = AppState {
        config: load_app_config::<AppConfig>()?,
    };

    let app = Router::new()
        .route("/", get(redirect_to_index))
        .route("/health", get(health::get_health))
        .nest(
            "/api/v1",
            Router::new()
                .route("/hello", get(api::get_hello))
                .route("/error", get(api::get_error)),
        )
        .nest(
            "/htm",
            Router::new()
                .route("/index.html", get(htm::get_index))
                .nest(
                    "/journal",
                    Router::new()
                        .route("/entries", get(htm::journal::get_journal_entries))
                        .route("/entries", post(htm::journal::update_journal_entry))
                        .route("/entries/{id}", delete(htm::journal::delete_journal_entry)),
                ),
        )
        .nest_service("/static", ServeDir::new("static"))
        .layer(middleware::from_fn(request_log_middleware))
        .with_state(state);

    run(app).await
}

async fn request_log_middleware(request: Request, next: Next) -> Response {
    tracing::info!(
        method = request.method().to_string(),
        path = request.uri().path(),
        "Handling request"
    );
    let response = next.run(request).await;
    tracing::info!(status = response.status().as_u16(), "Returning response");
    response
}

async fn redirect_to_index() -> Redirect {
    Redirect::temporary("/htm/index.html")
}
