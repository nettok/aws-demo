mod api;
mod health;
mod templates;

use axum::extract::Request;
use axum::middleware::{self, Next};
use axum::response::{Redirect, Response};
use axum::{Router, routing::get};
use lambda_http::{Error, run, tracing};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let app = Router::new()
        .route("/", get(redirect_to_index))
        .route("/health", get(health::get_health))
        .nest(
            "/api/v1",
            Router::new().route("/hello", get(api::get_hello)),
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
        "Request method={} path={}",
        request.method(),
        request.uri().path()
    );
    let response = next.run(request).await;
    tracing::info!("Response status={}", response.status().as_u16());
    response
}

async fn redirect_to_index() -> Redirect {
    Redirect::temporary("/app/index.html")
}
