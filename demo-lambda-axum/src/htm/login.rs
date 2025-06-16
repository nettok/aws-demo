use crate::db;
use crate::db::DatabaseConnection;
use crate::error::AppError;
use crate::extract::ValidatedForm;
use crate::htm::{RenderResult, render};
use askama::Template;
use axum::response::{IntoResponse, Redirect, Response};
use axum_extra::extract::PrivateCookieJar;
use axum_extra::extract::cookie::Cookie;
use serde::Deserialize;
use time::Duration;
use util::tracing::{self, instrument};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct LoginForm {
    #[validate(length(min = 1, message = "Can not be empty"))]
    username: String,

    #[validate(length(min = 1, message = "Can not be empty"))]
    password: String,
}

#[instrument]
pub async fn get_login() -> RenderResult {
    #[derive(Template)]
    #[template(path = "login.html")]
    struct Htm;

    let template = Htm;
    render(template)
}

#[instrument(skip(login))]
pub async fn post_login(
    jar: PrivateCookieJar,
    DatabaseConnection(db_conn): DatabaseConnection,
    ValidatedForm(login): ValidatedForm<LoginForm>,
) -> Result<(PrivateCookieJar, Response), AppError> {
    #[derive(Template)]
    #[template(path = "login.html")]
    struct Htm;

    if let Some(user) = db::users::get_user_by_name(db_conn, &login.username).await {
        // TODO: password should be hashed
        if login.password == user.password {
            let cookie = Cookie::build(("user_id", user.id.hyphenated().to_string()))
                .path("/")
                .secure(true)
                .http_only(true)
                .max_age(Duration::minutes(30));

            let updated_jar = jar.add(cookie);
            Ok((updated_jar, Redirect::to("/htm/index").into_response()))
        } else {
            let template = Htm;
            render(template).map(|html| (jar, html.into_response()))
        }
    } else {
        let template = Htm;
        render(template).map(|html| (jar, html.into_response()))
    }
}
