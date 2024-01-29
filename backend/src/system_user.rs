use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use lib_backend::vars::AUTH_COOKIE_NAME;
use tower_cookies::Cookies;

use crate::Error;

#[derive(Debug, Clone)]
pub struct SystemUser {
    pub id: i64,
    pub name: String,
    pub role: SystemRole,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SystemRole {
    User,
    Moderator,
    Admin,
}

#[async_trait]
impl<S> FromRequestParts<S> for SystemUser {
    type Rejection = crate::Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let cookies = parts
            .extract::<Cookies>()
            .await
            .map_err(|_| Error::CookiesNotProvided)?;

        if let Some(auth) = cookies.get(AUTH_COOKIE_NAME) {
            Ok(Self {
                id: 0,
                name: auth.value().to_owned(),
                role: SystemRole::Admin,
            })
        } else {
            Err(Error::Unauthorized)
        }
    }
}
