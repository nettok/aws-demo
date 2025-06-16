pub mod db_entries;

use crate::{AppConfig, AppState};
use axum::extract::{FromRef, FromRequestParts, State};
use axum::http::request::Parts;
use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use postgres_native_tls::MakeTlsConnector;
use tower_http::BoxError;

pub type PostgresPool = Pool<PostgresConnectionManager<MakeTlsConnector>>;
pub type PostgresPooledConnection =
    PooledConnection<'static, PostgresConnectionManager<MakeTlsConnector>>;
pub struct DatabaseConnection(pub PostgresPooledConnection);

pub async fn postgres_pool(config: &AppConfig) -> Result<PostgresPool, BoxError> {
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
