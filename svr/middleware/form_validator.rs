use crate::svr::error::HttpResError;
use axum::Form;
use axum::extract::rejection::FormRejection;
use axum::extract::{FromRequest, Request};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = HttpResError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await.map_err(|e| HttpResError::ValidationError(e.to_string()))?;
        value.validate().map_err(|e| HttpResError::ValidationError(e.to_string()))?;
        Ok(ValidatedForm(value))
    }
}