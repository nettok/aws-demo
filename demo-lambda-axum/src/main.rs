mod api;
mod templates;

use axum::{Router, routing::get};
use axum::response::Redirect;
use lambda_http::{run, tracing, Error};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let app = Router::new()
        .route("/", get(redirect_to_index))
        .route("/health", get(|| async { "OK" }))
        .nest("/api/v1",
              Router::new()
                  .route("/hello", get(api::get_hello))
        )
        .nest("/app",
              Router::new()
                  .route("/index.html", get(templates::get_index))
        )
        .nest_service("/static", ServeDir::new("static"));
    
    run(app).await
}

async fn redirect_to_index() -> Redirect {
    Redirect::temporary("/app/index.html")
}
