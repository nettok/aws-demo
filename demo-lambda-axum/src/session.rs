use crate::AppState;
use axum::extract::{FromRef, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Redirect, Response};
use axum_extra::extract::PrivateCookieJar;
use axum_extra::extract::cookie::Key;

pub async fn session_middleware(
    //State(state): State<AppState>,
    jar: PrivateCookieJar,
    request: Request,
    next: Next,
) -> Result<(PrivateCookieJar, Response), StatusCode> {
    if !request.uri().path().starts_with("/htm") || request.uri().path() == "/htm/login" {
        // allow non-protected paths
        return Ok((jar, next.run(request).await));
    }

    if let Some(user_id) = jar.get("user_id") {
        // TODO: validate against DB
        // allow logged in
        return Ok((jar, next.run(request).await));
    }

    Ok((jar, Redirect::temporary("/htm/login").into_response()))
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_key.clone()
    }
}
