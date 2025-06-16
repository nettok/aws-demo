mod api;
mod db;
mod error;
mod extract;
mod health;
mod htm;
mod serde_decorators;

use axum::Router;
use axum::extract::{FromRef, FromRequestParts, Request, State};
use axum::http::request::Parts;
use axum::middleware::{self, Next};
use axum::response::{Redirect, Response};
use axum::routing::{delete, get, post};
use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use chrono::Utc;
use dotenvy::dotenv;
use lambda_http::run;
use postgres_native_tls::MakeTlsConnector;
use serde::Deserialize;
use tower_http::BoxError;
use tower_http::services::ServeDir;
use util::config::load_app_config;
use util::tracing;

type PostgresPool = Pool<PostgresConnectionManager<MakeTlsConnector>>;
type PostgresPooledConnection =
    PooledConnection<'static, PostgresConnectionManager<MakeTlsConnector>>;
struct DatabaseConnection(PostgresPooledConnection);

#[derive(Clone, Deserialize)]
struct AppConfig {
    ca_certs: String,
    postgres: String,
}

#[derive(Clone)]
struct AppState {
    config: AppConfig,
    postgres_pool: PostgresPool,
}

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    dotenv().ok();
    tracing::init_tracing_default_subscriber();

    let config = load_app_config::<AppConfig>()?;
    let shared_config = &config.clone();

    let state = AppState {
        config,
        postgres_pool: postgres_pool(&shared_config).await?,
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
                .route("/index.html", get(redirect_to_index_with_date))
                .route("/index.html/{date}", get(htm::get_index))
                .nest(
                    "/journal",
                    Router::new()
                        .route("/entries/{date}", get(htm::journal::get_journal_entries))
                        .route("/entries/{date}", post(htm::journal::update_journal_entry))
                        .route(
                            "/entries/{date}/{id}",
                            delete(htm::journal::delete_journal_entry),
                        ),
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

async fn redirect_to_index_with_date() -> Redirect {
    let date = format!("{}", Utc::now().format("%Y-%m-%d"));
    Redirect::temporary(&format!("/htm/index.html/{}", date))
}

async fn postgres_pool(config: &AppConfig) -> Result<PostgresPool, BoxError> {
    use native_tls::{Certificate, TlsConnector};
    use postgres_native_tls::MakeTlsConnector;
    use std::fs;

    let cert = fs::read(&config.ca_certs)?;
    let cert = Certificate::from_pem(&cert)?;
    let connector = TlsConnector::builder().add_root_certificate(cert).build()?;

    let connector = MakeTlsConnector::new(connector);

    let manager =
        PostgresConnectionManager::new_from_stringlike(&config.postgres, connector).unwrap();

    Pool::builder()
        // AWS Lambdas only process one request at a time, so we only need one connection
        .max_size(1)
        .build(manager)
        .await
        .map_err(|e| BoxError::from(e))
}

impl<S> FromRequestParts<S> for DatabaseConnection
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (axum::http::StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let State(app_state): State<AppState> = State::from_request_parts(parts, state)
            .await
            .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let conn = app_state
            .postgres_pool
            .get_owned()
            .await
            .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Self(conn))
    }
}
