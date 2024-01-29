use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};

#[derive(Clone, FromRef)]
pub struct AppState;

#[async_trait]
impl<S> FromRequestParts<S> for AppState
where
    Self: FromRef<S>,
    S: Sync + Send + Clone,
{
    type Rejection = crate::Error;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}
