use app_state::AppState;
use lib_backend::{env::get_env, Application};

mod app_state;
mod auth;
mod error;
mod routes;
mod system_user;

pub use error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    lib_backend::init();

    let port = get_env("PORT").parse()?;

    let state = AppState;

    Application::new(port)
        .await?
        .run(routes::routes(state.clone()), state)
        .await
}
