use axum::{middleware::map_request, response::IntoResponse, routing::get, Router};
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};

use crate::{app_state::AppState, system_user::SystemUser, Result};

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/admin", get(require_admin))
        .layer(map_request(crate::auth::verify_is_admin))
        .route("/", get(say_hello))
        .route("/auth", get(require_login))
        .layer(CookieManagerLayer::new())
        .layer(cors())
        .with_state(state)
}

fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
}

async fn say_hello(
    user: Option<SystemUser>,
) -> Result<impl IntoResponse> {
    let res = user.map(|u| u.name).unwrap_or("Anon".to_string());
    Ok(format!("Hello {}!", res))
}

async fn require_login(
    user: SystemUser,
) -> Result<impl IntoResponse> {
    Ok(format!("Hello authenticated {}!", user.name))
}

async fn require_admin(
    user: SystemUser,
) -> Result<impl IntoResponse> {
    Ok(format!("Hello admin {}!", user.name))
}