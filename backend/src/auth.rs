use axum::extract::Request;

use crate::system_user::{SystemRole, SystemUser};

use crate::{Error, Result};

pub async fn verify_is_admin(user: SystemUser, request: Request) -> Result<Request> {
    if user.role == SystemRole::Admin {
        Ok(request)
    } else {
        Err(Error::UnauthorizedUser(user.id))
    }
}

pub async fn verify_is_moderator(user: SystemUser, request: Request) -> Result<Request> {
    if user.role > SystemRole::User {
        Ok(request)
    } else {
        Err(Error::UnauthorizedUser(user.id))
    }
}
