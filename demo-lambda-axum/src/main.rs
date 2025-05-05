mod api;

use axum::{Router, routing::get};
use lambda_http::{run, tracing, Error};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/health", get(|| async { "OK" }))
        .nest("/api/v1",
              Router::new()
                  .route("/hello", get(api::get_hello))
        );
    
    run(app).await
}
