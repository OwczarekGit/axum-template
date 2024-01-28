use app_state::AppState;
use lib_backend::{env::read_env, Application};

mod auth;
mod error;
mod app_state;
mod routes;
mod system_user;

pub use error::{Result, Error};

#[tokio::main]
async fn main() -> Result<()> {
    lib_backend::init();
    
    let port = read_env("PORT").parse()?;

    let state = AppState;

    Application::new(port)
        .await?
        .run(routes::routes(state.clone()), state)
        .await
}
