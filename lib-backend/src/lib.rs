pub mod application;
pub mod env;
pub mod var_names;
pub mod redis;
pub mod cookies;
pub mod crypt;

pub use application::Application;
use tracing::info;
use tracing_subscriber::EnvFilter;


pub fn init() {
    dotenvy::dotenv().ok();
    
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Backend init complete.");
}