mod api;
mod error;
mod health;
mod templates;

use std::{env, str::FromStr};

use axum::extract::Request;
use axum::middleware::{self, Next};
use axum::response::{Redirect, Response};
use axum::{Router, routing::get};
use lambda_http::run;
use tower_http::BoxError;
use tower_http::services::ServeDir;
use tracing::Level;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    init_tracing_default_subscriber();

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
            "/app",
            Router::new().route("/index.html", get(templates::get_index)),
        )
        .nest_service("/static", ServeDir::new("static"))
        .layer(middleware::from_fn(request_log_middleware));

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
    Redirect::temporary("/app/index.html")
}

pub fn init_tracing_default_subscriber() {
    let log_level_str = env::var("AWS_LAMBDA_LOG_LEVEL").or_else(|_| env::var("RUST_LOG"));
    let log_level =
        Level::from_str(log_level_str.as_deref().unwrap_or("INFO")).unwrap_or(Level::INFO);

    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::from_level(log_level).into())
                .from_env_lossy(),
        )
        .compact()
        .with_ansi(false)
        .init();
}
