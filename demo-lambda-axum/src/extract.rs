use crate::error::{ServerError, axum_form_rejection, validation_error};
use axum::Form;
use axum::extract::rejection::FormRejection;
use axum::extract::{FromRequest, Request};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedForm<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state)
            .await
            .map_err(|rejection| axum_form_rejection(rejection))?;

        value
            .validate()
            .map_err(|errors| validation_error(errors))?;

        Ok(ValidatedForm(value))
    }
}
